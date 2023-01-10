use crate::evaluator::Evaluator;
use crate::model::board::Board;
use crate::model::evaluation::Evaluation;
use crate::model::player_type::PlayerType;

pub struct PlaceablePointEvaluator {}

impl Evaluator for PlaceablePointEvaluator {
    fn evaluate(&self, board: &Board) -> Evaluation {
        let count = board.count_stones(board.player);

        if board.player == PlayerType::First {
            count
        } else {
            -count
        }
    }
}