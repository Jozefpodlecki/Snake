use js_sys::{Float32Array, Function};
use wasm_bindgen::JsValue;

use crate::{colors::get_random_color, food::Food, models::Direction, snake::Snake, utils::get_random_position_on_grid};

pub trait InvokeJs  {
    fn invoke(&self);
}

impl InvokeJs for Function {
    fn invoke(&self) {
        self.call0(&JsValue::null()).unwrap();
    }
}

pub struct Game<T: InvokeJs> {
    pub can_run: bool,
    score: u32,
    snake: Snake,
    foods: Vec<Food>,
    on_score: T,
    on_game_over: T,
    grid_size: i32,
    cell_size: f32,
    food_count: u32,
}

impl<T: InvokeJs> Game<T> {
    pub fn new(grid_size: i32,
        food_count: u32,
        on_score: T,
        on_game_over: T) -> Self {
        let cell_size = 2.0 / grid_size as f32;
        let snake_color = [1.0, 1.0, 1.0, 1.0];
        let snake = Snake::new(Direction::Right, grid_size, cell_size, snake_color);
        let foods = (0..food_count).map(|_| Food::new( get_random_color(), get_random_position_on_grid(grid_size), cell_size)).collect();

        Game {
            can_run: true,
            score: 0,
            grid_size,
            snake,
            foods,
            cell_size,
            on_score,
            on_game_over,
            food_count
        }
    }

    pub fn is_over(&self) -> bool {
        self.snake.is_self_collision()
    }

    pub fn update_and_notify_ui(&mut self) {
        self.snake.traverse();

        for food in &mut self.foods {
            if self.snake.overlaps(food) {
                
                self.snake.grow();
                food.position = get_random_position_on_grid(self.grid_size);
                self.score += 1;
                self.on_score.invoke();

                break;
            }
        }
    }

    pub fn apply_options_and_reset(&mut self, food_count: u32) {
        self.food_count = food_count;

        self.reset();
    }

    pub fn reset_and_notify_ui(&mut self) {
        self.on_game_over.invoke();

        self.reset();
    }

    fn reset(&mut self) {
        self.score = 0;
        self.snake.reset();
        self.foods = (0..self.food_count).map(|_| Food::new(get_random_color(), get_random_position_on_grid(self.grid_size), self.cell_size)).collect();
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