//! Core library for NTK Ultra-Compression
//! 
//! This library provides the core compression and decompression functionality
//! with support for encryption and parallel processing.

use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::time::SystemTime;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use rayon::prelude::*;
use memmap2::Mmap;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

// Constantes pour le format de fichier
const MAGIC_BYTES: &[u8] = b"NTK1";
const HEADER_SIZE: usize = 512;
const DEFAULT_BLOCK_SIZE: usize = 1024 * 1024; // 1MB
const SALT_SIZE: usize = 32;
const KEY_SIZE: usize = 32;
const NONCE_SIZE: usize = 12;

#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Invalid file format")]
    InvalidFormat,
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Compression error: {0}")]
    CompressionError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionOptions {
    /// Niveau de compression (1-9)
    pub level: u32,
    /// Taille des blocs en octets
    pub block_size: usize,
    /// Nombre de threads à utiliser
    pub threads: usize,
    /// Taille du dictionnaire
    pub dictionary_size: usize,
    /// Utiliser le chiffrement
    pub use_encryption: bool,
    /// Mot de passe pour le chiffrement
    pub password: Option<String>,
}

impl Default for CompressionOptions {
    fn default() -> Self {
        Self {
            level: 6,
            block_size: DEFAULT_BLOCK_SIZE,
            threads: num_cpus::get(),
            dictionary_size: 32 * 1024 * 1024, // 32MB
            use_encryption: false,
            password: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub original_name: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub encrypted: bool,
    pub creation_time: u64,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileHeader {
    magic: String,
    version: u32,
    encrypted: bool,
    original_name: String,
    original_size: u64,
}

pub struct Compressor {
    options: CompressionOptions,
}

impl Compressor {
    pub fn new(options: CompressionOptions) -> Self {
        Self { options }
    }

    pub fn compress<P: AsRef<Path>>(&self, input: P, output: P) -> Result<FileMetadata> {
        let input_path = input.as_ref();
        let output_path = output.as_ref();

        // Ouvrir les fichiers
        let input_file = File::open(input_path)
            .with_context(|| format!("Failed to open input file: {}", input_path.display()))?;
        let output_file = File::create(output_path)
            .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;

        let input_size = input_file.metadata()?.len();
        let input_name = input_path.file_name()
            .ok_or_else(|| CompressionError::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid input filename"
            )))?
            .to_string_lossy()
            .into_owned();

        // Mapper le fichier en mémoire pour une lecture efficace
        let mmap = unsafe { Mmap::map(&input_file)? };

        // Préparer le chiffrement si nécessaire
        let (key, salt, nonce) = if self.options.use_encryption {
            self.prepare_encryption()?
        } else {
            (vec![], vec![], vec![])
        };

        // Diviser le fichier en blocs
        let chunks: Vec<_> = mmap.chunks(self.options.block_size).collect();
        
        // Compresser les blocs en parallèle
        let compressed_blocks: Vec<_> = chunks.par_iter()
            .map(|chunk| self.compress_block(chunk, &key))
            .collect::<Result<_>>()?;

        // Écrire l'en-tête et les blocs
        let mut writer = BufWriter::new(output_file);
        self.write_header(&mut writer, &input_name, input_size, &salt, &nonce)?;

        let mut compressed_size = HEADER_SIZE as u64;
        for block in compressed_blocks {
            writer.write_u32::<LittleEndian>(block.len() as u32)?;
            writer.write_all(&block)?;
            compressed_size += block.len() as u64 + 4;
        }

        writer.flush()?;

        // Calculer le checksum final
        let checksum = blake3::hash(&mmap);

        Ok(FileMetadata {
            original_name: input_name,
            original_size: input_size,
            compressed_size,
            compression_ratio: input_size as f64 / compressed_size as f64,
            encrypted: self.options.use_encryption,
            creation_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs(),
            checksum: hex::encode(checksum.as_bytes()),
        })
    }

    pub fn decompress<P: AsRef<Path>>(&self, input: P, output: P) -> Result<()> {
        let input_path = input.as_ref();
        let output_path = output.as_ref();

        let mut input_file = BufReader::new(File::open(input_path)?);
        let mut output_file = BufWriter::new(File::create(output_path)?);

        // Lire et vérifier l'en-tête
        let (header, salt, nonce) = self.read_header(&mut input_file)?;

        // Préparer le déchiffrement si nécessaire
        let key = if header.encrypted {
            if let Some(password) = &self.options.password {
                self.derive_key(password, &salt)?
            } else {
                return Err(CompressionError::EncryptionError("Password required for encrypted file".into()).into());
            }
        } else {
            vec![]
        };

        // Lire et décompresser les blocs
        while let Ok(block_size) = input_file.read_u32::<LittleEndian>() {
            let mut block = vec![0u8; block_size as usize];
            input_file.read_exact(&mut block)?;

            let decompressed = self.decompress_block(&block, &key, &nonce)?;
            output_file.write_all(&decompressed)?;
        }

        output_file.flush()?;
        Ok(())
    }

    fn write_header<W: Write>(&self, writer: &mut W, name: &str, size: u64, salt: &[u8], nonce: &[u8]) -> Result<()> {
        let header = FileHeader {
            magic: String::from_utf8_lossy(MAGIC_BYTES).into_owned(),
            version: 1,
            encrypted: self.options.use_encryption,
            original_name: name.to_string(),
            original_size: size,
        };

        let header_json = serde_json::to_string(&header)?;
        let header_bytes = header_json.as_bytes();
        
        writer.write_all(&header_bytes)?;
        
        // Padding to HEADER_SIZE
        let padding = vec![0u8; HEADER_SIZE - header_bytes.len()];
        writer.write_all(&padding)?;

        if self.options.use_encryption {
            writer.write_all(salt)?;
            writer.write_all(nonce)?;
        }

        Ok(())
    }

    fn read_header<R: Read>(&self, reader: &mut R) -> Result<(FileHeader, Vec<u8>, Vec<u8>)> {
        let mut header_bytes = vec![0u8; HEADER_SIZE];
        reader.read_exact(&mut header_bytes)?;

        // Trouver la fin du JSON en cherchant le premier 0
        let json_end = header_bytes.iter()
            .position(|&b| b == 0)
            .unwrap_or(HEADER_SIZE);

        let header: FileHeader = serde_json::from_slice(&header_bytes[..json_end])?;

        if header.magic.as_bytes() != MAGIC_BYTES {
            return Err(CompressionError::InvalidFormat.into());
        }

        let mut salt = Vec::new();
        let mut nonce = Vec::new();

        if header.encrypted {
            salt = vec![0u8; SALT_SIZE];
            nonce = vec![0u8; NONCE_SIZE];
            reader.read_exact(&mut salt)?;
            reader.read_exact(&mut nonce)?;
        }

        Ok((header, salt, nonce))
    }

    fn compress_block(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        // Compression avec zstd
        let mut compressed = Vec::new();
        let mut encoder = zstd::Encoder::new(&mut compressed, self.options.level as i32)?;
        encoder.write_all(data)?;
        encoder.finish()?;

        // Chiffrement si nécessaire
        if !key.is_empty() {
            self.encrypt_block(&compressed, key)
        } else {
            Ok(compressed)
        }
    }

    fn decompress_block(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        // Déchiffrement si nécessaire
        let data = if !key.is_empty() {
            self.decrypt_block(data, key, nonce)?
        } else {
            data.to_vec()
        };

        // Décompression
        let mut decompressed = Vec::new();
        let mut decoder = zstd::Decoder::new(&data[..])?;
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }

    // Méthodes privées pour la gestion du chiffrement
    fn prepare_encryption(&self) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let password = self.options.password.as_ref()
            .ok_or_else(|| CompressionError::EncryptionError("Password required for encryption".into()))?;

        let mut salt = vec![0u8; SALT_SIZE];
        getrandom::getrandom(&mut salt)?;

        let mut nonce = vec![0u8; NONCE_SIZE];
        getrandom::getrandom(&mut nonce)?;

        let key = self.derive_key(password, &salt)?;

        Ok((key, salt, nonce))
    }

    fn derive_key(&self, password: &str, salt: &[u8]) -> Result<Vec<u8>> {
        let mut key = vec![0u8; KEY_SIZE];
        pbkdf2::pbkdf2_hmac::<sha2::Sha256>(
            password.as_bytes(),
            salt,
            100_000,
            &mut key,
        );
        Ok(key)
    }

    fn encrypt_block(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| CompressionError::EncryptionError(e.to_string()))?;

        let nonce = Nonce::from_slice(&[0u8; NONCE_SIZE]);
        cipher.encrypt(nonce, data)
            .map_err(|e| CompressionError::EncryptionError(e.to_string()).into())
    }

    fn decrypt_block(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| CompressionError::EncryptionError(e.to_string()))?;

        let nonce = Nonce::from_slice(nonce);
        cipher.decrypt(nonce, data)
            .map_err(|e| CompressionError::EncryptionError(e.to_string()).into())
    }

    pub fn get_metadata<P: AsRef<Path>>(&self, path: P) -> Result<FileMetadata> {
        let path_ref = path.as_ref();
        let mut file = BufReader::new(File::open(path_ref)?);
        let (header, _, _) = self.read_header(&mut file)?;
        let file_size = std::fs::metadata(path_ref)?.len();
        
        Ok(FileMetadata {
            original_name: header.original_name,
            original_size: header.original_size,
            compressed_size: file_size,
            compression_ratio: header.original_size as f64 / file_size as f64,
            encrypted: header.encrypted,
            creation_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs(),
            checksum: String::new(), // Le checksum n'est pas stocké dans l'en-tête
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_compression_decompression() -> Result<()> {
        let dir = tempdir()?;
        let input_path = dir.path().join("test.txt");
        let compressed_path = dir.path().join("test.ntk");
        let output_path = dir.path().join("test_out.txt");

        // Créer un fichier de test
        let test_data = b"Hello, World!".repeat(1000);
        fs::write(&input_path, &test_data)?;

        // Compresser
        let options = CompressionOptions::default();
        let compressor = Compressor::new(options.clone());
        let metadata = compressor.compress(&input_path, &compressed_path)?;

        // Vérifier les métadonnées
        assert!(metadata.compression_ratio > 1.0);
        assert_eq!(metadata.original_size, test_data.len() as u64);
        assert!(!metadata.encrypted);

        // Décompresser
        compressor.decompress(&compressed_path, &output_path)?;

        // Vérifier le résultat
        let decompressed = fs::read(&output_path)?;
        assert_eq!(decompressed, test_data);

        Ok(())
    }

    #[test]
    fn test_encrypted_compression() -> Result<()> {
        let dir = tempdir()?;
        let input_path = dir.path().join("test.txt");
        let compressed_path = dir.path().join("test.ntk");
        let output_path = dir.path().join("test_out.txt");

        // Créer un fichier de test
        let test_data = b"Secret data!".repeat(1000);
        fs::write(&input_path, &test_data)?;

        // Compresser avec chiffrement
        let options = CompressionOptions {
            use_encryption: true,
            password: Some("test123".to_string()),
            ..Default::default()
        };
        let compressor = Compressor::new(options.clone());
        let metadata = compressor.compress(&input_path, &compressed_path)?;

        // Vérifier les métadonnées
        assert!(metadata.encrypted);

        // Décompresser
        compressor.decompress(&compressed_path, &output_path)?;

        // Vérifier le résultat
        let decompressed = fs::read(&output_path)?;
        assert_eq!(decompressed, test_data);

        Ok(())
    }
} 