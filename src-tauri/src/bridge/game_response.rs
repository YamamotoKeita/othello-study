use serde::{Serialize, Deserialize};
use crate::bridge::point::{Point, shift_point, to_point_vec};
use crate::model::board::Board;
use crate::model::direction::Direction;
use crate::model::player_type::PlayerType;
use crate::model::points::{Points, xy_to_point};


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
            next_player: Some(PlayerType::First.code()),
            next_candidates: to_point_vec(board.placeable_points),
        }
    }

    pub fn new_by_move(board: Board, previous_board: Board, player: PlayerType, placed_point: Point) -> GameResponse {
        let reversed_points = GameResponse::reversed_points(previous_board, player, placed_point.x, placed_point.y);

        GameResponse {
            board: GameResponse::board_to_array(board),
            placed_stone: Some(player.code()),
            placed_point: Some(placed_point),
            reversed_lines: Some(reversed_points),
            next_player: Some(board.player.code()),
            next_candidates: to_point_vec(board.placeable_points),
        }
    }

    pub fn board_to_array(board: Board) -> [[u32; 8]; 8] {
        let mut result: [[u32; 8]; 8] = [[0; 8]; 8];
        for y in 0..=7 {
            for x in 0..=7 {
                let stone = board.get_stone(x, y);
                result[y as usize][x as usize] = stone.code();
            }
        }
        result
    }

    pub fn reversed_points(board: Board, player: PlayerType, x: u32, y: u32) -> Vec<Vec<Point>> {
        let mut result = vec![vec![]];

        for direction in Direction::iterator() {
            let mut line: Vec<Point> = vec![];
            let mut next_point = shift_point(Point{x, y}, *direction);

            while let Some(point) = next_point {
                if !board.has_stone(player.opposite(), point.x, point.y) {
                    if board.has_stone(player, point.x, point.y) && !line.is_empty() {
                        result.push(line);
                    }
                    break;
                }
                line.push(point);
                next_point = shift_point(point, *direction);
            }
        }

        return result;
    }

}