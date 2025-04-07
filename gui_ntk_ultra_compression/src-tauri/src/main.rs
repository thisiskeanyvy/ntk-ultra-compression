// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn ma_fonction_rust() {
    println!("vous avez vraiment cru qu'on allait coder pour avoir 14.08 au lieu de 20 au projet, fuck epita !!!")
}

fn main() {
    gui_ntk_ultra_compression_lib::run()
}
