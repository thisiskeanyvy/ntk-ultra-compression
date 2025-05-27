//! Core library for NTK Ultra-Compression
//! 
//! This library provides the core compression and decompression functionality
//! with support for encryption and parallel processing.

use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter, Seek, SeekFrom};
use std::time::{SystemTime, Instant};
use std::sync::{Arc, Mutex};
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
use bincode;
use image::{ImageBuffer, Rgba, RgbaImage, GenericImageView, ImageFormat};
use std::cmp::min;

// Constantes pour le format de fichier
const MAGIC_BYTES: &[u8] = b"NTK1";
const HEADER_SIZE: usize = 512;
const DEFAULT_BLOCK_SIZE: usize = 16 * 1024 * 1024; // 16MB pour de meilleures performances
const SALT_SIZE: usize = 32;
const KEY_SIZE: usize = 32;
const NONCE_SIZE: usize = 12;
const TAG_SIZE: usize = 16; // Taille du tag d'authentification AES-GCM

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
    #[error("Steganography error: {0}")]
    SteganographyError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionOptions {
    /// Niveau de compression (1-22 pour zstd)
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
    /// Utiliser la stéganographie
    pub use_steganography: bool,
    /// Chemin de l'image pour la stéganographie
    pub steganography_image: Option<String>,
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
    pub estimated_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileHeader {
    magic: String,
    version: u32,
    encrypted: bool,
    original_name: String,
    original_size: u64,
}

#[derive(Debug, Clone)]
pub struct ProgressInfo {
    pub processed_bytes: u64,
    pub total_bytes: u64,
    pub current_speed: f64,
    pub estimated_remaining_time: f64,
}

pub type ProgressCallback = Arc<Mutex<dyn FnMut(ProgressInfo) + Send + 'static>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EncryptedBlock {
    nonce: Vec<u8>,
    data: Vec<u8>,
}

pub struct Compressor {
    options: CompressionOptions,
    progress_callback: Option<Arc<Mutex<dyn FnMut(ProgressInfo) + Send + 'static>>>,
}

impl Default for CompressionOptions {
    fn default() -> Self {
        Self {
            level: 19,
            block_size: DEFAULT_BLOCK_SIZE,
            threads: num_cpus::get(),
            dictionary_size: 64 * 1024 * 1024,
            use_encryption: false,
            password: None,
            use_steganography: false,
            steganography_image: None,
        }
    }
}

impl Compressor {
    pub fn new(options: CompressionOptions) -> Self {
        Self {
            options,
            progress_callback: None,
        }
    }

    pub fn set_progress_callback<F>(&mut self, callback: F)
    where
        F: FnMut(ProgressInfo) + Send + 'static,
    {
        self.progress_callback = Some(Arc::new(Mutex::new(callback)));
    }

    pub fn compress<P: AsRef<Path>>(&self, input: P, output: P) -> Result<FileMetadata> {
        let start = Instant::now();
        let input_path = input.as_ref();
        let output_path = output.as_ref();

        let input_file = File::open(input_path)?;
        let mut output_file = BufWriter::new(File::create(output_path)?);

        let input_size = input_file.metadata()?.len();
        let input_name = input_path.file_name()
            .ok_or_else(|| CompressionError::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid input filename"
            )))?
            .to_string_lossy()
            .into_owned();

        // Préparer le chiffrement si nécessaire
        let (key, salt, nonce) = if self.options.use_encryption {
            self.prepare_encryption()?
        } else {
            (vec![], vec![], vec![])
        };

        // Mapper le fichier en mémoire pour une lecture efficace
        let mmap = unsafe { Mmap::map(&input_file)? };

        // Diviser le fichier en blocs
        let chunk_size = self.options.block_size;
        let chunks: Vec<_> = mmap.chunks(chunk_size).collect();
        
        // Compresser les blocs en parallèle
        let processed_bytes = Arc::new(Mutex::new(0u64));
        let start_time = Arc::new(Instant::now());

        let compressed_blocks: Vec<_> = chunks.par_iter()
            .map(|chunk| {
                let result = self.compress_block(chunk, &key, &nonce);
                
                // Mise à jour de la progression
                if let Some(ref callback) = &self.progress_callback {
                    let mut bytes = processed_bytes.lock().unwrap();
                    *bytes += chunk.len() as u64;
                    
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = *bytes as f64 / elapsed;
                    let remaining = (input_size - *bytes) as f64 / speed;
                    
                    if let Ok(mut guard) = callback.lock() {
                        guard(ProgressInfo {
                            processed_bytes: *bytes,
                            total_bytes: input_size,
                            current_speed: speed,
                            estimated_remaining_time: remaining,
                        });
                    }
                }
                
                result
            })
            .collect::<Result<_>>()?;

        // Écrire l'en-tête
        self.write_header(&mut output_file, &input_name, input_size, &salt, &nonce)?;

        let mut compressed_size = HEADER_SIZE as u64;
        for block in compressed_blocks {
            let block_size = block.len() as u32;
            output_file.write_all(&block_size.to_le_bytes())?;
            output_file.write_all(&block)?;
            compressed_size += block_size as u64 + 4;
        }

        output_file.flush()?;

        let checksum = blake3::hash(&mmap);
        let elapsed = start.elapsed().as_secs_f64();

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
            estimated_time: elapsed,
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

        let file_size = input_file.seek(SeekFrom::End(0))?;
        input_file.seek(SeekFrom::Start(HEADER_SIZE as u64))?;

        let mut processed_bytes = 0u64;
        let start = Instant::now();

        // Lire et décompresser les blocs
        while processed_bytes < file_size - HEADER_SIZE as u64 {
            // Lire la taille du bloc
            let mut size_buf = [0u8; 4];
            if let Err(e) = input_file.read_exact(&mut size_buf) {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(e.into());
            }
            let block_size = u32::from_le_bytes(size_buf);

            // Vérification de sécurité sur la taille du bloc
            if block_size == 0 || block_size as usize > self.options.block_size * 4 {
                return Err(CompressionError::InvalidFormat.into());
            }

            // Lire le bloc
            let mut block = vec![0u8; block_size as usize];
            if let Err(e) = input_file.read_exact(&mut block) {
                return Err(CompressionError::IoError(io::Error::new(
                    e.kind(),
                    format!("Failed to read block at offset {}: {}", processed_bytes, e)
                )).into());
            }

            // Décompresser le bloc
            let decompressed = match self.decompress_block(&block, &key, &nonce) {
                Ok(data) => data,
                Err(e) => {
                    return Err(CompressionError::CompressionError(
                        format!("Failed to decompress block at offset {}: {}", processed_bytes, e)
                    ).into());
                }
            };

            // Écrire le bloc décompressé
            if let Err(e) = output_file.write_all(&decompressed) {
                return Err(CompressionError::IoError(io::Error::new(
                    e.kind(),
                    format!("Failed to write decompressed block at offset {}: {}", processed_bytes, e)
                )).into());
            }

            processed_bytes += block_size as u64 + 4;

            // Mise à jour de la progression
            if let Some(ref callback) = &self.progress_callback {
                let elapsed = start.elapsed().as_secs_f64();
                let speed = processed_bytes as f64 / elapsed;
                let remaining = (file_size - HEADER_SIZE as u64 - processed_bytes) as f64 / speed;

                if let Ok(mut guard) = callback.lock() {
                    guard(ProgressInfo {
                        processed_bytes,
                        total_bytes: file_size - HEADER_SIZE as u64,
                        current_speed: speed,
                        estimated_remaining_time: remaining,
                    });
                }
            }
        }

        output_file.flush()?;
        Ok(())
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
            checksum: String::new(),
            estimated_time: 0.0,
        })
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
        let mut header_bytes = header_json.into_bytes();
        
        // S'assurer que l'en-tête ne dépasse pas la taille maximale
        if header_bytes.len() > HEADER_SIZE {
            return Err(CompressionError::InvalidFormat.into());
        }
        
        // Padding jusqu'à HEADER_SIZE
        header_bytes.resize(HEADER_SIZE, 0);
        writer.write_all(&header_bytes)?;

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

        let header: FileHeader = serde_json::from_slice(&header_bytes[..json_end])
            .map_err(|_| CompressionError::InvalidFormat)?;

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

    fn compress_block(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        // Compression avec zstd
        let mut compressed = Vec::with_capacity(data.len());
        {
            let mut encoder = zstd::Encoder::new(&mut compressed, self.options.level as i32)?;
            encoder.write_all(data)?;
            encoder.finish()?;
        }

        // Chiffrement si nécessaire
        if !key.is_empty() {
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|e| CompressionError::EncryptionError(e.to_string()))?;
            let nonce = Nonce::from_slice(nonce);
            
            cipher.encrypt(nonce, compressed.as_ref())
                .map_err(|e| CompressionError::EncryptionError(e.to_string()).into())
        } else {
            Ok(compressed)
        }
    }

    fn decompress_block(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let to_decompress = if !key.is_empty() {
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|e| CompressionError::EncryptionError(e.to_string()))?;
            let nonce = Nonce::from_slice(nonce);
            
            cipher.decrypt(nonce, data)
                .map_err(|e| CompressionError::EncryptionError(format!("Decryption failed: {}", e)))?
        } else {
            data.to_vec()
        };

        let mut decoder = zstd::Decoder::new(&to_decompress[..])?;
        let mut decoded = Vec::with_capacity(self.options.block_size);
        decoder.read_to_end(&mut decoded)?;
        Ok(decoded)
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

    pub fn hide_in_image<P: AsRef<Path>>(&self, archive_path: P, image_path: P, output_path: P) -> Result<()> {
        let archive_data = std::fs::read(archive_path)?;
        let img = image::open(image_path)
            .map_err(|e| CompressionError::SteganographyError(e.to_string()))?;
        
        let img_rgba = img.to_rgba8();
        let (width, height) = img_rgba.dimensions();
        let max_bytes = (width * height * 3) / 8; // 3 bits par pixel (1 par canal RGB)
        
        if archive_data.len() as u32 > max_bytes {
            return Err(CompressionError::SteganographyError(
                format!("Archive too large for this image. Max size: {} bytes", max_bytes)
            ).into());
        }
        
        let mut stego_img = RgbaImage::new(width, height);
        let mut bit_index = 0;
        
        // Écrire la taille de l'archive sur les 32 premiers pixels
        let size_bytes = (archive_data.len() as u32).to_le_bytes();
        for (i, &byte) in size_bytes.iter().enumerate() {
            for bit in 0..8 {
                let x = (i * 8 + bit) as u32 % width;
                let y = (i * 8 + bit) as u32 / width;
                let mut pixel = img_rgba.get_pixel(x, y).clone();
                pixel[0] &= 0xFE;
                pixel[0] |= ((byte >> bit) & 1) as u8;
                stego_img.put_pixel(x, y, pixel);
            }
        }
        
        // Écrire les données de l'archive
        for &byte in archive_data.iter() {
            for bit in 0..8 {
                let pixel_index = 32 + bit_index; // 32 pixels après la taille
                let x = pixel_index as u32 % width;
                let y = pixel_index as u32 / width;
                let mut pixel = img_rgba.get_pixel(x, y).clone();
                
                // Modifier le LSB du canal rouge
                pixel[0] &= 0xFE;
                pixel[0] |= ((byte >> bit) & 1) as u8;
                
                stego_img.put_pixel(x, y, pixel);
                bit_index += 1;
            }
        }
        
        // Copier le reste de l'image
        for y in 0..height {
            for x in 0..width {
                if (y * width + x) as usize >= 32 + bit_index {
                    stego_img.put_pixel(x, y, *img_rgba.get_pixel(x, y));
                }
            }
        }
        
        stego_img.save(output_path)
            .map_err(|e| CompressionError::SteganographyError(e.to_string()).into())
    }

    pub fn extract_from_image<P: AsRef<Path>>(&self, image_path: P, output_path: P) -> Result<()> {
        let img = image::open(image_path)
            .map_err(|e| CompressionError::SteganographyError(e.to_string()))?;
        let img_rgba = img.to_rgba8();
        let (width, height) = img_rgba.dimensions();
        
        // Lire la taille de l'archive
        let mut size_bytes = [0u8; 4];
        for (i, byte) in size_bytes.iter_mut().enumerate() {
            for bit in 0..8 {
                let x = (i * 8 + bit) as u32 % width;
                let y = (i * 8 + bit) as u32 / width;
                let pixel = img_rgba.get_pixel(x, y);
                *byte |= ((pixel[0] & 1) as u8) << bit;
            }
        }
        let archive_size = u32::from_le_bytes(size_bytes) as usize;
        
        // Vérifier que la taille est valide
        let max_bytes = (width * height * 3) / 8;
        if archive_size as u32 > max_bytes {
            return Err(CompressionError::SteganographyError("Invalid archive size in image".into()).into());
        }
        
        // Extraire les données de l'archive
        let mut archive_data = vec![0u8; archive_size];
        for (byte_idx, byte) in archive_data.iter_mut().enumerate() {
            for bit in 0..8 {
                let pixel_index = 32 + (byte_idx * 8 + bit); // 32 pixels après la taille
                let x = pixel_index as u32 % width;
                let y = pixel_index as u32 / width;
                let pixel = img_rgba.get_pixel(x, y);
                *byte |= ((pixel[0] & 1) as u8) << bit;
            }
        }
        
        std::fs::write(output_path, archive_data)?;
        Ok(())
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