use serde::{Serialize, Deserialize};
use crate::model::board::Board;
use crate::model::player_type::PlayerType;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GameResponse {
    board: [[u32; 8]; 8],
}

impl GameResponse {
    pub fn new(board: &Board) -> GameResponse {
        let mut result: [[u32; 8]; 8] = [[0; 8]; 8];

        for y in 0..=7 {
            for x in 0..=7 {
                let stone = if board.has_stone(PlayerType::First, x, y) {
                    1
                } else if board.has_stone(PlayerType::Second, x, y) {
                    2
                } else {
                    0
                };
                result[y as usize][x as usize] = stone;
            }
        }

        GameResponse {
            board: result,
        }
    }
}