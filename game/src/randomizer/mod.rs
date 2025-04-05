mod js;
mod os;

pub use js::JsRandomizer;

#[cfg(test)]
pub use os::tests::{MockRandomizer, OsRandomizer};

pub trait Randomizer {
    fn get_random_color(&mut self) -> [f32; 4];
    fn get_random_position_on_grid(&mut self, grid_size: i32) -> (i32, i32);
}