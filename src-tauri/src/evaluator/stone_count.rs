use crate::evaluator::Evaluator;
use crate::model::board_node::BoardNode;
use crate::model::evaluation::Evaluation;
use crate::model::player_type::PlayerType;

pub struct StoneCountEvaluator {
    stone_weight: i32,
}

impl Evaluator for StoneCountEvaluator {
    fn evaluate(&self, board: &BoardNode) -> Evaluation {
        (board.player_stone_count() - board.opponent_stone_count()) * self.stone_weight
    }
}

#[allow(dead_code)]
impl StoneCountEvaluator {
    pub fn new() -> StoneCountEvaluator {
        StoneCountEvaluator {stone_weight: 1}
    }
    pub fn new_with_weight(stone_weight: i32) -> StoneCountEvaluator {
        StoneCountEvaluator {stone_weight}
    }
}