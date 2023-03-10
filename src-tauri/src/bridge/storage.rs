use std::sync::Mutex;
use crate::bridge::game_info::GameInfo;

pub struct Storage {
    pub store: Mutex<GameInfo>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
           store: Mutex::new(GameInfo::new()),
        }
    }
}