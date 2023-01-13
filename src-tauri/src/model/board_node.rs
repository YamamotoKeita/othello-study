use std::mem;
use crate::model::points::*;
use crate::model::direction::Direction;

/// Representation of Othello board.
#[derive(Clone, Copy)]
pub struct BoardNode {
    pub player_stones: Points,
    pub opponent_stones: Points,
}

impl BoardNode {
    #[inline(always)]
    pub fn change_player(&mut self) {
        mem::swap(&mut self.player_stones, &mut self.opponent_stones);
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn stone_count(&self) -> i32 {
        (self.player_stones.count_ones() + self.opponent_stones.count_ones()) as i32
    }

    #[inline(always)]
    pub fn player_stone_count(&self) -> i32 {
        self.player_stones.count_ones() as i32
    }

    #[inline(always)]
    pub fn opponent_stone_count(&self) -> i32 {
        self.opponent_stones.count_ones() as i32
    }

    #[inline(always)]
    pub fn placeable_points(&self) -> Points {
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
    pub fn place_stone(&self, point: Points) -> BoardNode {
        let mut reversed: Points = 0;
        let mut player_stones = self.player_stones;
        let mut opponent_stones = self.opponent_stones;

        for direction in Direction::iterator() {
            let mut line: Points = 0;
            let mut next_point = shift_points(point, *direction);

            while (next_point != 0) && ((next_point & opponent_stones) != 0) {
                line |= next_point;
                next_point = shift_points(next_point, *direction);
            }

            if (next_point & player_stones) != 0 {
                reversed |= line;
            }
        }

        // Both addition and removal can be done with the XOR operation.
        player_stones ^= point | reversed;
        opponent_stones ^= reversed;

        BoardNode {
            player_stones: opponent_stones,
            opponent_stones: player_stones,
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn clone(&self) -> BoardNode {
        BoardNode {
            player_stones: self.player_stones,
            opponent_stones: self.opponent_stones,
        }
    }
}
