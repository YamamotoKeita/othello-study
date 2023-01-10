use crate::model::board::Board;
use crate::model::points::Points;

pub trait Player {
    fn play(&self, board: &Board) -> Points;
    fn message_before_play(&self, board: &Board) -> Option<String>;
}