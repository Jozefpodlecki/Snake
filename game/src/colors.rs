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

pub fn get_random_color() -> [f32; 4] {
    #[cfg(test)]
    {
        use rand::{rng, seq::IndexedRandom};
        let mut rng = rng();
        *COLORS.choose(&mut rng).unwrap()
    }

    #[cfg(not(test))]
    {
        use js_sys::Math;
        let index = (Math::random() * COLORS.len() as f64) as usize;
        COLORS[index]
    }
}