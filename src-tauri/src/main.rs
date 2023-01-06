#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::State;
use crate::bridge::ai_config::AiConfig;
use crate::bridge::storage::Storage;
use crate::game_manager::GameManager;

mod tests;
mod model;
mod bridge;
mod game_manager;

#[tauri::command]
fn init_game(ai_config1: Option<AiConfig>, ai_config2: Option<AiConfig>, state: State<'_, Storage>) {
    //state.store = GameManager::new()

}

#[tauri::command]
fn click(player: u32, x: u32, y: u32, state: State<'_, Storage>) {
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
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
