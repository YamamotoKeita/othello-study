use std::sync::Mutex;
use crate::game_manager::GameManager;

pub struct Storage {
    pub store: Mutex<Option<GameManager>>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
           store: Mutex::new(None),
        }
    }
}