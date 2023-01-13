use crate::model::points::*;
use crate::model::direction::Direction;
use crate::model::player_type::PlayerType;

/// Representation of Othello board.
#[derive(Clone, Copy)]
pub struct BoardNode {
    pub player_stones: Points,
    pub opponent_stones: Points,
}

impl BoardNode {

    #[inline(always)]
    pub fn stone_count(&self) -> i32 {
        (self.player_stones.count_ones() + self.opponent_stones.count_ones()) as i32
    }

    #[inline(always)]
    fn placeable_points(&self) -> Points {
        let horizontal_targets = self.opponent_stones & MASK_LEFT_RIGHT_ZERO;
        let vertical_targets = self.opponent_stones & MASK_TOP_BOTTOM_ZERO;
        let diagonal_targets = self.opponent_stones & MASK_ALL_SIDES_ZERO;

        let vacant_points = !(self.player_stones | self.opponent_stones);

        let mut result: u64 = 0;

        for direction in Direction::iterator() {
            let targets = match *direction {
                Direction::Left | Direction::Right => horizontal_targets,
                Direction::Up | Direction::Down => vertical_targets,
                _ => diagonal_targets,
            };

            // Shift the player stones 6 squares and mark where the opponent stones are in the direction of movement.
            let mut tmp = targets & shift_points_without_guard(self.player_stones, *direction);
            for _ in 0..5 {
                tmp |= targets & shift_points_without_guard(tmp, *direction);
            }

            let placeable_points = vacant_points & shift_points_without_guard(tmp, *direction);

            result |= placeable_points;
        }

        return result;
    }

    #[inline(always)]
    pub fn clone(&self) -> BoardNode {
        BoardNode {
            player_stones: self.player_stones,
            opponent_stones: self.opponent_stones,
        }
    }
}
