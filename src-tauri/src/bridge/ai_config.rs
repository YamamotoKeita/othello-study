use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AiConfig {
    pub is_ai: bool,
    pub level: u32,
}
