#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use crate::bridge::ai_config::AiConfig;
use crate::bridge::storage::Storage;

mod tests;
mod model;
mod bridge;
mod game_manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init_game(ai_config1: Option<AiConfig>, ai_config2: Option<AiConfig>) {
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            init_game,
        ])
        .setup(|app| {
            let storage = Storage::new();
            app.manage(storage);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
