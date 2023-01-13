pub mod nega_alpha;
pub mod game_tree_searcher;
mod searcher_tests;

use crate::model::board::Board;
use crate::model::points::Points;

pub trait Searcher {
    fn search(&self, board: &Board, max_depth: u32) -> Points;
}