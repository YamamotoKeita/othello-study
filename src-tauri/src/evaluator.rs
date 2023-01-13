pub mod stone_count;

use crate::model::evaluation::Evaluation;
use crate::model::board_node::BoardNode;

pub trait Evaluator {
    /*
     * Evaluates a board.
     *
     */
    fn evaluate(&self, board: &BoardNode) -> Evaluation;
}