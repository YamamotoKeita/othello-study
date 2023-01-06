use serde::{Serialize};

#[derive(Debug, Serialize, Clone, Copy)]
pub struct GameInfo {
    board: [[u32; 8]; 8],
    turn: u32,
    player: u32,
}

impl GameInfo {
    pub fn new() -> GameInfo {
        GameInfo {
            board: [],
            turn: 0,
            player: 0,
        }
    }
}