#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ntk_core::{Compressor, CompressionOptions, FileMetadata};
use std::sync::{Arc, Mutex};
use tauri::State;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct CompressionRequest {
    input_path: String,
    output_path: String,
    options: CompressionOptions,
}

#[derive(Debug, Deserialize)]
pub struct DecompressRequest {
    input_path: String,
    output_path: String,
    password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SteganographyRequest {
    archive_path: String,
    image_path: String,
    output_path: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProgressEvent {
    processed_bytes: u64,
    total_bytes: u64,
    percent: f64,
    speed_mbps: f64,
    remaining_seconds: f64,
}

struct ProgressState(Arc<Mutex<Option<Box<dyn Fn(ProgressEvent) + Send>>>>);

#[tauri::command]
async fn compress(request: CompressionRequest, progress: State<'_, ProgressState>) -> Result<FileMetadata, String> {
    let mut compressor = Compressor::new(request.options);
    
    // Configurer le callback de progression
    let progress_state = Arc::clone(&progress.0);
    compressor.set_progress_callback(move |info| {
        if let Ok(callback) = progress_state.lock() {
            if let Some(cb) = callback.as_ref() {
                let event = ProgressEvent {
                    processed_bytes: info.processed_bytes,
                    total_bytes: info.total_bytes,
                    percent: (info.processed_bytes as f64 / info.total_bytes as f64) * 100.0,
                    speed_mbps: info.current_speed / (1024.0 * 1024.0),
                    remaining_seconds: info.estimated_remaining_time,
                };
                cb(event);
            }
        }
    });

    compressor
        .compress(request.input_path, request.output_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn decompress(request: DecompressRequest, progress: State<'_, ProgressState>) -> Result<(), String> {
    let mut compressor = Compressor::new(CompressionOptions {
        use_encryption: request.password.is_some(),
        password: request.password,
        ..Default::default()
    });

    // Configurer le callback de progression
    let progress_state = Arc::clone(&progress.0);
    compressor.set_progress_callback(move |info| {
        if let Ok(callback) = progress_state.lock() {
            if let Some(cb) = callback.as_ref() {
                let event = ProgressEvent {
                    processed_bytes: info.processed_bytes,
                    total_bytes: info.total_bytes,
                    percent: (info.processed_bytes as f64 / info.total_bytes as f64) * 100.0,
                    speed_mbps: info.current_speed / (1024.0 * 1024.0),
                    remaining_seconds: info.estimated_remaining_time,
                };
                cb(event);
            }
        }
    });

    compressor
        .decompress(request.input_path, request.output_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_metadata(path: String) -> Result<FileMetadata, String> {
    let compressor = Compressor::new(CompressionOptions::default());
    compressor
        .get_metadata(path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_progress_handler(
    window: tauri::Window,
    progress: State<'_, ProgressState>,
) -> Result<(), String> {
    let mut progress_guard = progress.0.lock().map_err(|e| e.to_string())?;
    *progress_guard = Some(Box::new(move |event: ProgressEvent| {
        window
            .emit("progress", &event)
            .expect("failed to emit progress event");
    }));
    Ok(())
}

#[tauri::command]
async fn clear_progress_handler(progress: State<'_, ProgressState>) -> Result<(), String> {
    let mut progress_guard = progress.0.lock().map_err(|e| e.to_string())?;
    *progress_guard = None;
    Ok(())
}

#[tauri::command]
async fn hide_in_image(request: SteganographyRequest) -> Result<(), String> {
    let compressor = Compressor::new(CompressionOptions::default());
    compressor
        .hide_in_image(request.archive_path, request.image_path, request.output_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn extract_from_image(request: SteganographyRequest) -> Result<(), String> {
    let compressor = Compressor::new(CompressionOptions::default());
    compressor
        .extract_from_image(request.image_path, request.output_path)
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(ProgressState(Arc::new(Mutex::new(None))))
        .invoke_handler(tauri::generate_handler![
            compress,
            decompress,
            get_metadata,
            set_progress_handler,
            clear_progress_handler,
            hide_in_image,
            extract_from_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 