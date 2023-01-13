use crate::evaluator_old::Evaluator;
use crate::evaluator_old::stone_count::StoneCountEvaluator;
use crate::model::board::Board;
use crate::model::evaluation::Evaluation;


pub struct GameEvaluator<T: Evaluator> {
    end_evaluator: StoneCountEvaluator,
    evaluator: T,
}

impl <T: Evaluator> Evaluator for GameEvaluator<T> {
    fn evaluate(&self, board: &Board) -> Evaluation {
        if board.is_game_end() {
            self.end_evaluator.evaluate(board)
        } else {
            self.evaluator.evaluate(board)
        }
    }
}

impl <T: Evaluator> GameEvaluator<T> {
    pub fn new(evaluator: T) -> GameEvaluator<T> {
        GameEvaluator {
            end_evaluator: StoneCountEvaluator::new_with_weight(100),
            evaluator,
        }
    }
}