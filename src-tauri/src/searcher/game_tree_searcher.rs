use crate::model::board::Board;
use crate::model::board_node::BoardNode;
use crate::model::evaluation::{Evaluation, EVALUATION_MIN};
use crate::model::points::{POINT_ITERATOR, Points};
use crate::searcher::Searcher;

pub trait GameTreeSearcher {
    fn evaluate_node(&self, board: &mut BoardNode, depth: u32) -> Evaluation;

    fn evaluate_next_moves(&self, board: &Board, max_depth: u32) -> Vec<(Points, Evaluation)> {
        let mut result: Vec<(Points, Evaluation)> = vec![];

        let board = BoardNode {
            player_stones: board.player_stones(),
            opponent_stones: board.opponent_stones(),
        };

        let placeable_points = board.placeable_points();
        for point in *POINT_ITERATOR {
            if (point & placeable_points) != 0 {
                let mut new_board = board.place_stone(point);
                let score = self.evaluate_node(&mut new_board, max_depth - 1);
                result.push((point, score));
            }
        }

        return result;
    }
}

impl <T: GameTreeSearcher> Searcher for T {

    fn search(&self, board: &Board, max_depth: u32) -> Points {
        let mut result: Option<Points> = None;
        let mut max_score = EVALUATION_MIN;

        let children = self.evaluate_next_moves(board, max_depth);

        for (points, value) in children {
            if value > max_score {
                result = Some(points);
                max_score = value;
            }
        }
        return result.unwrap();
    }
}