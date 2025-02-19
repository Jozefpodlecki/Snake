use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOptions {
    pub id: String,
    pub difficulty: Difficulty,
    pub grid_size: i32,
    pub food_count: u32,
    pub fps: i32,
    pub frame_threshold_ms: f64
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Difficulty {
    Easy,
    Hard
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}