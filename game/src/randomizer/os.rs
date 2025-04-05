#[cfg(test)]
pub mod tests {
    use crate::{constants::COLORS, randomizer::Randomizer};

    pub struct OsRandomizer {
        rng: rand::rngs::ThreadRng,
    }
    
    impl OsRandomizer {
        pub fn new() -> Self {
            Self { rng: rand::rng() }
        }
    }
    
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
    
    mockall::mock! {
        pub Randomizer {}
        impl Randomizer for Randomizer {
            fn get_random_color(&mut self) -> [f32; 4];
            fn get_random_position_on_grid(&mut self, grid_size: i32) -> (i32, i32);
        }
    }
}

