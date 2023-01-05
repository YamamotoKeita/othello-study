use serde::{Serialize};

#[derive(Debug, Serialize, Clone, Copy)]
pub struct AiConfig {
    level: u32,
}
