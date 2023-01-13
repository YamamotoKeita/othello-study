use std::cmp::max;
use crate::evaluator::Evaluator;
use crate::model::board_node::BoardNode;
use crate::model::evaluation::{Evaluation, EVALUATION_MAX, EVALUATION_MIN};
use crate::model::points::POINT_ITERATOR;
use crate::searcher::game_tree_searcher::GameTreeSearcher;

pub struct NegaAlpha<T: Evaluator> {
    evaluator: T,
}

impl <T: Evaluator> NegaAlpha<T> {
    pub fn new(evaluator: T) -> NegaAlpha<T> {
        NegaAlpha {
            evaluator
        }
    }

    fn nega_alpha(&self, board: &mut BoardNode, depth: u32, mut alpha: i32, beta: i32, passed: bool) -> i32 {
        // Evaluates a board on a terminal node
        if depth == 0 {
            return self.evaluator.evaluate(&board);
        }

        let placeable_points = board.placeable_points();

        // Skip and turn change
        if placeable_points == 0 {
            if passed {
                return self.evaluator.evaluate(&board);
            }

            board.change_player();
            return -self.nega_alpha(board, depth, -beta, -alpha, true);
        }

        for point in *POINT_ITERATOR {
            if (point & placeable_points) == 0 { continue; }

            let mut new_board = board.place_stone(point);
            let score = -self.nega_alpha(&mut new_board, depth - 1, -beta, -alpha, false);

            // pruning
            if score >= beta {
                return score;
            }

            alpha = max(alpha, score);
        }

        return alpha;
    }
}

impl <T: Evaluator> GameTreeSearcher for NegaAlpha<T> {
    fn evaluate_node(&self, board: &mut BoardNode, depth: u32) -> Evaluation {
        -self.nega_alpha(board, depth, -EVALUATION_MAX, -EVALUATION_MIN, false)
    }
}