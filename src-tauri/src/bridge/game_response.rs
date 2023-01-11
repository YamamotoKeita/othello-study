use serde::{Serialize, Deserialize};
use crate::model::board::Board;
use crate::model::direction::Direction;
use crate::model::player_type::PlayerType;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameResponse {
    board: [[u32; 8]; 8],
    placed_stone: Option<u32>,
    placed_point: Option<Point>,
    reversed_lines: Option<Vec<Vec<Point>>>,
    next_player: Option<u32>,
    next_candidates: Vec<Point>,
}

impl GameResponse {
    pub fn new(board: Board) -> GameResponse {
        GameResponse {
            board: GameResponse::board_to_array(board),
            placed_stone: None,
            placed_point: None,
            reversed_lines: None,
            next_player: Some(GameResponse::stone_to_int(PlayerType::First)),
            next_candidates: vec![],
        }
    }

    pub fn new_by_move(board: Board, previous_board: Board, player: PlayerType, placed_point: Point) -> GameResponse {
        let reversed_points = GameResponse::reversed_points(previous_board, player, placed_point.x, placed_point.y);

        GameResponse {
            board: GameResponse::board_to_array(board),
            placed_stone: Some(GameResponse::stone_to_int(player)),
            placed_point: Some(placed_point),
            reversed_lines: Some(reversed_points),
            next_player: Some(GameResponse::stone_to_int(board.player)),
            next_candidates: vec![],
        }
    }

    pub fn board_to_array(board: Board) -> [[u32; 8]; 8] {
        let mut result: [[u32; 8]; 8] = [[0; 8]; 8];
        for y in 0..=7 {
            for x in 0..=7 {
                let stone = board.get_stone(x, y);
                result[y as usize][x as usize] = GameResponse::stone_to_int(stone);
            }
        }
        result
    }

    pub fn reversed_points(board: Board, player: PlayerType, x: u32, y: u32) -> Vec<Vec<Point>> {
        let mut result = vec![vec![]];

        for direction in Direction::iterator() {
            let mut line: Vec<Point> = vec![];
            let mut next_point = GameResponse::shift_point(Point{x, y}, *direction);

            while let Some(point) = next_point {
                if !board.has_stone(player.opposite(), point.x, point.y) {
                    if board.has_stone(player, point.x, point.y) && !line.is_empty() {
                        result.push(line);
                    }
                    break;
                }
                line.push(point);
                next_point = GameResponse::shift_point(point, *direction);
            }
        }

        return result;
    }

    pub fn shift_point(point: Point, direction: Direction) -> Option<Point> {
        let mut x: i32 = point.x as i32;
        let mut y: i32 = point.y as i32;

        x += direction.h_move();
        y += direction.v_move();

        if x > 7 || x < 0 || y > 7 || y < 0 {
            return None;
        }
        return Some(Point{ x: x as u32, y: y as u32 });
    }

    pub fn stone_to_int(stone: PlayerType) -> u32 {
        match stone {
            PlayerType::None =>  0,
            PlayerType::First =>  1,
            PlayerType::Second =>  2,
        }
    }
}