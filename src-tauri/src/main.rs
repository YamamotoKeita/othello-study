#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

use tauri::Manager;
use tauri::State;
use crate::bridge::ai_config::AiConfig;
use crate::bridge::game_response::GameResponse;
use crate::bridge::storage::Storage;

mod tests;
mod model;
mod bridge;
mod searcher;
mod evaluator;

#[tauri::command]
fn init_game(ai_config_1: AiConfig, ai_config_2: AiConfig, state: State<'_, Storage>) -> GameResponse {
    let mut game_info = state.store.lock().unwrap();
    game_info.init(ai_config_1, ai_config_2);
    GameResponse::new(game_info.board)
}

#[tauri::command]
fn click(x: u32, y: u32, state: State<'_, Storage>) -> Option<GameResponse> {
    let mut game_info = state.store.lock().unwrap();
    if let Some(point) = game_info.get_player_move(x, y) {
        return Some(game_info.play(point));
    }
    return None;
}

#[tauri::command]
async fn wait_ai(state: State<'_, Storage>) -> Result<GameResponse, &str> {
    let mut game_info = state.store.lock().unwrap();
    if let Some(point) = game_info.get_ai_move() {
        return Ok(game_info.play(point));
    }
    return Err("No AI move");
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
