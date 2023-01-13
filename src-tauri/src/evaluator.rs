pub mod stone_count;
pub mod cell_weight;
pub mod open_count;
pub mod placeable_point;
pub mod simple_prediction;

use crate::model::evaluation::Evaluation;
use crate::model::board_node::BoardNode;

pub trait Evaluator {
    /*
     * Evaluates a board from a standpoint of "BoardNode.player_stones".
     *
     * If player_stones have the advantage, the evaluation is a positive number.
     * If opponent_stones have the advantage, the evaluation is a negative number.
     * If player_stones and opponent_stones are equal in the advantage, the evaluation is zero.
     *
     */
    fn evaluate(&self, board: &BoardNode) -> Evaluation;
}