use js_sys::{Float32Array, Function};
use wasm_bindgen::JsValue;

use crate::{food::Food, models::Direction, randomizer::Randomizer, snake::Snake, utils::get_random_position_on_grid};

pub trait InvokeJs  {
    fn invoke(&self);
}

impl InvokeJs for Function {
    fn invoke(&self) {
        self.call0(&JsValue::null()).unwrap();
    }
}

pub struct Game<T: InvokeJs, R: Randomizer> {
    pub can_run: bool,
    pub is_played_by_ai: bool,
    score: u32,
    snake: Snake,
    foods: Vec<Food>,
    on_score: T,
    on_game_over: T,
    grid_size: i32,
    cell_size: f32,
    food_count: u32,
    randomizer: R,
}

impl<T: InvokeJs, R: Randomizer> Game<T, R> {
    pub fn new(grid_size: i32,
        food_count: u32,
        on_score: T,
        on_game_over: T,
        mut randomizer: R) -> Self {
        let cell_size = 2.0 / grid_size as f32;
        let snake_color = [1.0, 1.0, 1.0, 1.0];
        let snake = Snake::new(Direction::Right, grid_size, cell_size, snake_color);
        let foods = (0..food_count).map(|_| Food::new( randomizer.get_random_color(), randomizer.get_random_position_on_grid(grid_size), cell_size)).collect();

        Game {
            is_played_by_ai: true,
            can_run: true,
            score: 0,
            grid_size,
            snake,
            foods,
            cell_size,
            on_score,
            on_game_over,
            food_count,
            randomizer
        }
    }

    fn get_closest_food(&self) -> Option<&Food> {
        let snake_head = self.snake.get_head_position();
        
        self.foods.iter().min_by_key(|food| {
            let dx = (snake_head.0 - food.position.0).abs();
            let dy = (snake_head.1 - food.position.1).abs();
            dx + dy
        })
    }

    fn update_ai(&mut self) {
        if let Some(target_food) = self.get_closest_food() {
            let snake_head = self.snake.get_head_position();
            
            let mut possible_moves = vec![];

            if !self.snake.will_collide((snake_head.0 + 1, snake_head.1)) {
                possible_moves.push((Direction::Right, (snake_head.0 + 1, snake_head.1)));
            }
            if !self.snake.will_collide((snake_head.0 - 1, snake_head.1)) {
                possible_moves.push((Direction::Left, (snake_head.0 - 1, snake_head.1)));
            }
            if !self.snake.will_collide((snake_head.0, snake_head.1 + 1)) {
                possible_moves.push((Direction::Up, (snake_head.0, snake_head.1 + 1)));
            }
            if !self.snake.will_collide((snake_head.0, snake_head.1 - 1)) {
                possible_moves.push((Direction::Down, (snake_head.0, snake_head.1 - 1)));
            }

            if let Some((best_direction, _)) = possible_moves.into_iter().min_by_key(|(_, pos)| {
                let dx = (pos.0 - target_food.position.0).abs();
                let dy = (pos.1 - target_food.position.1).abs();
                dx + dy
            }) {
                self.snake.change_direction(best_direction);
            }
        }
    }

    pub fn is_over(&self) -> bool {
        self.snake.is_self_collision()
    }

    pub fn update_and_notify_ui(&mut self) {
        
        if self.is_played_by_ai {
            self.update_ai();
        }

        self.snake.traverse();

        for food in &mut self.foods {
            if self.snake.overlaps(food) {
                
                self.snake.grow();
                food.position = get_random_position_on_grid(self.grid_size);
                
                if self.is_played_by_ai {
                    self.score += 1;
                    self.on_score.invoke();
                }

                break;
            }
        }
    }

    pub fn apply_options_and_reset(&mut self, grid_size: i32, food_count: u32) {
        self.grid_size = grid_size;
        self.food_count = food_count;
        self.cell_size = 2.0 / grid_size as f32;

        self.snake.resize(grid_size, self.cell_size);

        self.reset();
    }

    pub fn reset_and_notify_ui(&mut self) {
        self.on_game_over.invoke();

        self.reset();
    }

    pub fn reset(&mut self) {
        self.is_played_by_ai = false;
        self.score = 0;
        self.snake.reset();
        self.foods = (0..self.food_count).map(|_| Food::new(self.randomizer.get_random_color(), self.randomizer.get_random_position_on_grid(self.grid_size), self.cell_size)).collect();
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    pub fn get_vertices(&self) -> Float32Array {
        let mut all_vertices = Vec::new();

        all_vertices.extend_from_slice(&self.snake.as_vertices());

        for food in &self.foods {
            all_vertices.extend_from_slice(&food.as_vertices());
        }

        unsafe { Float32Array::view(&all_vertices) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::randomizer::TestRandomizer;

    struct MockInvokeJs;
    impl InvokeJs for MockInvokeJs {
        fn invoke(&self) {}
    }

    #[test]
    fn test_game_initialization() {
        let mut randomizer = TestRandomizer::new();

        let grid_size = 10;
        let food_count = 3;
        let mut game = Game::new(grid_size, food_count, MockInvokeJs, MockInvokeJs, randomizer);

        assert_eq!(game.grid_size, grid_size);
        assert_eq!(game.foods.len(), food_count as usize);
        assert_eq!(game.score, 0);
        assert!(game.can_run);
    }
}
