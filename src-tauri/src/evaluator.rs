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
     * # Sings of number
     * - If player_stones have the advantage, the evaluation is a positive number.
     * - If opponent_stones have the advantage, the evaluation is a negative number.
     * - If player_stones and opponent_stones are equal in the advantage, the evaluation is zero.
     *
     * # Values of number
     * Evaluation values are equivalent to stone number in the end of game.
     * For example, Victory with difference of 10 stones corresponds to 10, and defeat with difference of 10 stones corresponds to -10.
     * They will be between -64 and 64.
     *
     */
    fn evaluate(&self, board: &BoardNode) -> Evaluation;
}