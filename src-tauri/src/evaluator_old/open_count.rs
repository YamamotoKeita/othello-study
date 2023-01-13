use crate::model::points::*;
use crate::evaluator_old::Evaluator;
use crate::model::board::Board;
use crate::model::direction::Direction;
use crate::model::evaluation::Evaluation;

pub struct OpenCountEvaluator {}

impl Evaluator for OpenCountEvaluator {
    fn evaluate(&self, board: &Board) -> Evaluation {
        let open_points1 = self.open_points(board.first_stones, board.second_stones);
        let open_points2 = self.open_points(board.second_stones, board.first_stones);
        open_points2.count_ones() as i32 - open_points1.count_ones() as i32
    }
}

impl OpenCountEvaluator {
    fn open_points(&self, player_stones: Points, opponent_stones: Points) -> Points {
        let mut result: Points = 0;
        let vacant_points = !(player_stones | opponent_stones);

        for direction in Direction::iterator() {
            let open_points = vacant_points & shift_points(player_stones, *direction);
            result |= open_points;
        }

        return result;
    }
}