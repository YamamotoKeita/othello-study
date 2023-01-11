use std::io;

use crate::model::board::Board;
use crate::model::player::Player;
use crate::model::points::{Points, text_to_point_safely};

pub struct CuiPlayer {
}

impl CuiPlayer {
    #[allow(dead_code)]
    pub fn new() -> CuiPlayer {
        CuiPlayer{}
    }
}

impl Player for CuiPlayer {
    fn play(&self, board: &Board) -> Points {
        println!("Enter the place. (e.g. \"F5\" or \"f5\")");

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");

            if let Some(point) = text_to_point_safely(&input.trim()) {
                if board.can_place(point) {
                    return point;
                } else {
                    println!("You can't put a stone there.")
                }
            } else {
                println!("Invalid input.");
            }
        }
    }

    fn message_before_play(&self, _board: &Board) -> Option<String> {
        None
    }
}