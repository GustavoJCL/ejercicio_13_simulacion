// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod lib;
use lib::huecos::*;
use lib::poker::*;
use lib::prueba_arriba_abajo::*;
use lib::prueba_arriba_abajo_media::*;
use lib::series::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            huecos,
            poker,
            prueba_arriba_abajo,
            prueba_arriba_abajo_media,
            series
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
