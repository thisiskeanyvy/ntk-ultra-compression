#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ntk_core::{Compressor, CompressionOptions, FileMetadata};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

struct CompressionState {
    compressor: Mutex<Compressor>,
}

#[derive(serde::Deserialize)]
struct CompressRequest {
    input_path: String,
    output_path: String,
    options: CompressionOptions,
}

#[derive(serde::Deserialize)]
struct DecompressRequest {
    input_path: String,
    output_path: String,
    password: Option<String>,
}

#[tauri::command]
async fn compress(
    state: State<'_, CompressionState>,
    request: CompressRequest,
) -> Result<FileMetadata, String> {
    let input = PathBuf::from(&request.input_path);
    let output = PathBuf::from(&request.output_path);

    if !input.exists() {
        return Err(format!("Input file does not exist: {}", request.input_path));
    }

    let compressor = state.compressor.lock().map_err(|e| e.to_string())?;
    
    compressor
        .compress(&input, &output)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn decompress(
    state: State<'_, CompressionState>,
    request: DecompressRequest,
) -> Result<(), String> {
    let input = PathBuf::from(&request.input_path);
    let output = PathBuf::from(&request.output_path);

    if !input.exists() {
        return Err(format!("Input file does not exist: {}", request.input_path));
    }

    let mut options = CompressionOptions::default();
    if let Some(password) = request.password {
        options.use_encryption = true;
        options.password = Some(password);
    }

    let compressor = Compressor::new(options);
    compressor
        .decompress(&input, &output)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_metadata(
    state: State<'_, CompressionState>,
    path: String,
) -> Result<FileMetadata, String> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    let compressor = state.compressor.lock().map_err(|e| e.to_string())?;
    
    compressor
        .get_metadata(&path)
        .map_err(|e| e.to_string())
}

fn main() {
    let compression_state = CompressionState {
        compressor: Mutex::new(Compressor::new(CompressionOptions::default())),
    };

    tauri::Builder::default()
        .manage(compression_state)
        .invoke_handler(tauri::generate_handler![compress, decompress, get_metadata])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 