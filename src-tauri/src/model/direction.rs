use std::slice::Iter;
use self::Direction::*;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    UpperRight,
    Right,
    LowerRight,
    Down,
    LowerLeft,
    Left,
    UpperLeft,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [Up, UpperRight, Right, LowerRight, Down, LowerLeft, Left, UpperLeft];
        DIRECTIONS.iter()
    }

    pub fn v_move(&self) -> i32 {
        match self {
            Direction::Up | Direction::UpperLeft | Direction::UpperRight => -1,
            Direction::Down | Direction::LowerLeft | Direction::LowerRight => 1,
            _ => 0,
        }
    }

    pub fn h_move(&self) -> i32 {
        match self {
            Direction::Left | Direction::UpperLeft | Direction::LowerLeft => -1,
            Direction::Right | Direction::UpperRight | Direction::LowerRight => 1,
            _ => 0,
        }
    }
}
