use crate::constants::COLORS;

use super::Randomizer;

pub struct JsRandomizer;

impl Randomizer for JsRandomizer {
    fn get_random_color(&mut self) -> [f32; 4] {
        let index = (js_sys::Math::random() * COLORS.len() as f64) as usize;
        COLORS[index]
    }

    fn get_random_position_on_grid(&mut self, grid_size: i32) -> (i32, i32) {
        let x = (js_sys::Math::random() * grid_size as f64) as i32;
        let y = (js_sys::Math::random() * grid_size as f64) as i32;
        (x, y)
    }
}