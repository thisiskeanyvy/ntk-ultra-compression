//keany-vy.khun
use std::fs::{self, File};
use std::io::{self, Write, Read};

// Fonction de chiffrement
pub fn chiffrer_fichier(source: &str, destination: &str, password: &[u8]) -> io::Result<()> {
    let data = fs::read(source)?;
    let encrypted = operation_xor(&data, password);
    fs::write(destination, encrypted)
}

// Fonction de déchiffrement 
pub fn dechiffrer_fichier(source: &str, destination: &str, password: &[u8]) -> io::Result<()> {
    let encrypted_data = fs::read(source)?;
    let decrypted = operation_xor(&encrypted_data, password);
    fs::write(destination, decrypted)
}

// Opération XOR sous-jacente
fn operation_xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

// Utilitaire de saisie
fn saisir_mot_de_passe() -> io::Result<Vec<u8>> {
    print!("Entrez le mot de passe : ");
    io::stdout().flush()?;
    
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    
    Ok(password.trim().as_bytes().to_vec())
}
