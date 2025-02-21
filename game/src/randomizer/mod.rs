
const COLORS: &[[f32; 4]] = &[
    [1.0, 0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0, 1.0],
    [0.0, 0.0, 1.0, 1.0],
    [1.0, 1.0, 0.0, 1.0],
    [1.0, 0.647, 0.0, 1.0],
    [0.5, 0.0, 0.5, 1.0],
    [0.0, 1.0, 1.0, 1.0],
    [1.0, 0.75, 0.8, 1.0], 
];

pub trait Randomizer {
    fn get_random_color(&mut self) -> [f32; 4];
    fn get_random_position_on_grid(&mut self, grid_size: i32) -> (i32, i32);
}

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

#[cfg(test)]
pub struct OsRandomizer {
    rng: rand::rngs::ThreadRng,
}

#[cfg(test)]
impl OsRandomizer {
    pub fn new() -> Self {
        Self { rng: rand::rng() }
    }
}

#[cfg(test)]
impl Randomizer for OsRandomizer {
    fn get_random_color(&mut self) -> [f32; 4] {
        use rand::seq::IndexedRandom;

        *COLORS.choose(&mut self.rng).unwrap()
    }

    fn get_random_position_on_grid(&mut self, grid_size: i32) -> (i32, i32) {
        use rand::Rng;

        let x = self.rng.random_range(1..grid_size);
        let y = self.rng.random_range(1..grid_size);
        (x, y)
    }
}