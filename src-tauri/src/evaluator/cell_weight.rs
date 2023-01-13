use crate::evaluator::Evaluator;
use crate::model::board_node::BoardNode;
use crate::model::evaluation::Evaluation;
use crate::model::points::Points;

const CELL_WEIGHT: [i32; 64] = [
    30, -12,   0, -1, -1,  0, -12,  30,
   -12, -15,  -3, -3, -3, -3, -15, -12,
     0,  -3,   0, -1, -1,  0,  -3,   0,
    -1,  -3,  -1, -1, -1, -1,  -3,  -1,
    -1,  -3,  -1, -1, -1, -1,  -3,  -1,
     0,  -3,   0, -1, -1,  0,  -3,   0,
   -12,  -15, -3, -3, -3, -3, -15, -12,
    30,  -12,  0, -1, -1,  0, -12,  30,
];

pub struct CellWeightEvaluator {}

impl Evaluator for CellWeightEvaluator {
    fn evaluate(&self, board: &BoardNode) -> Evaluation {
        let mut player = 0_i32;
        let mut opponent = 0_i32;

        for n in 0..CELL_WEIGHT.len() {
            let mask: Points = 1 << (63 - n);
            if board.player_stones & mask != 0 {
                player += CELL_WEIGHT[n];
            }
            if board.opponent_stones & mask != 0 {
                opponent += CELL_WEIGHT[n];
            }
        }

        player - opponent
    }
}