use crate::evaluator::Evaluator;
use crate::model::board_node::BoardNode;
use crate::model::evaluation::Evaluation;
use crate::model::player_type::PlayerType;

pub struct PlaceablePointEvaluator {}

impl Evaluator for PlaceablePointEvaluator {
    fn evaluate(&self, board: &BoardNode) -> Evaluation {
        return board.player_stone_count();
    }
}