#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::State;
use crate::bridge::ai_config::AiConfig;
use crate::bridge::game_info::GameInfo;
use crate::bridge::game_response::GameResponse;
use crate::bridge::storage::Storage;

mod tests;
mod model;
mod bridge;
mod searcher;
mod evaluator;

#[tauri::command]
fn init_game(ai_config1: Option<AiConfig>, ai_config2: Option<AiConfig>, state: State<'_, Storage>) -> GameResponse {
    let mut game_info = state.store.lock().unwrap();
    game_info.init(ai_config1, ai_config2);
    GameResponse::new(&game_info.board)
}

#[tauri::command]
fn click(x: u32, y: u32, state: State<'_, Storage>) -> Option<GameResponse> {
    let mut game_info = state.store.lock().unwrap();
    if game_info.click(x, y) {
        return Some(GameResponse::new(&game_info.board));
    }
    return None;
}

#[tauri::command]
fn wait_ai(state: State<'_, Storage>) -> Option<GameResponse> {
    let mut game_info = state.store.lock().unwrap();
    if game_info.play_ai() {
        return Some(GameResponse::new(&game_info.board));
    }
    return None;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            init_game,
            click,
            wait_ai,
        ])
        .setup(|app| {
            let storage = Storage::new();
            app.manage(storage);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
