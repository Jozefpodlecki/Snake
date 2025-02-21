use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Idle = 0,
    AiPlaying = 1,
    Playing = 2,
    Paused = 3,
    GameOver = 4,
}

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

pub struct VerticePayload {
    pub data: Vec<f32>,
    pub length: usize,
    pub vertice_size: i32,
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