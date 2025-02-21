use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::{food::Food, models::Direction, randomizer::Randomizer, snake::Snake};

pub trait InvokeJs  {
    fn invoke(&self);
}

impl InvokeJs for Function {
    fn invoke(&self) {
        self.call0(&JsValue::null()).unwrap();
    }
}

pub struct Game<R: Randomizer> {
    pub is_played_by_ai: bool,
    direction: Direction,
    score: u32,
    snake: Snake,
    foods: Vec<Food>,
    grid_size: i32,
    cell_size: f32,
    food_count: u32,
    randomizer: R,
}

impl<R: Randomizer> Game< R> {
    pub fn new(grid_size: i32, food_count: u32, randomizer: R) -> Self {
        let cell_size = 2.0 / grid_size as f32;
        let snake = Snake::new();

        Game {
            direction: Direction::Right,
            is_played_by_ai: true,
            score: 0,
            grid_size,
            snake,
            foods: vec![],
            cell_size,
            food_count,
            randomizer
        }
    }

    pub fn initialize(&mut self) {
        let body_length = 5;
        let snake_color = [1.0, 1.0, 1.0, 1.0];
        
        let foods = (0..self.food_count).map(|_| Food::new(
            self.randomizer.get_random_color(),
            self.randomizer.get_random_position_on_grid(self.grid_size), self.cell_size)).collect();
        self.foods = foods;
        self.snake.initialize(body_length, self.grid_size, self.cell_size, snake_color);
    }

    fn get_closest_food(&self) -> Option<&Food> {
        let snake_head = self.snake.get_head_position();
        
        self.foods.iter().min_by_key(|food| {
            let dx = (snake_head.0 - food.position.0).abs();
            let dy = (snake_head.1 - food.position.1).abs();
            dx + dy
        })
    }

    fn get_free_position(
        randomizer: &mut R,
        snake: &Snake,
        food_positions: &[(i32, i32)],
        grid_size: i32) -> (i32, i32) {
        loop {
            let position = randomizer.get_random_position_on_grid(grid_size);
            
            let is_occupied = snake.occupies(position) || food_positions.iter().any(|food_position| *food_position == position);
            
            if !is_occupied {
                return position;
            }
        }
    }

    fn update_ai(&mut self) {
        if let Some(target_food) = self.get_closest_food() {
            let snake_head = self.snake.get_head_position();
            let mut possible_moves = vec![];
    
            let directions = [
                (Direction::Right, (snake_head.0 + 1, snake_head.1)),
                (Direction::Left, (snake_head.0 - 1, snake_head.1)),
                (Direction::Up, (snake_head.0, snake_head.1 + 1)),
                (Direction::Down, (snake_head.0, snake_head.1 - 1)),
            ];
    
            for (dir, pos) in directions.iter() {
                if !self.snake.will_collide(*pos) {
                    let open_space = self.count_reachable_cells(*pos);
                    possible_moves.push((*dir, *pos, open_space));
                }
            }
    
            if let Some((best_direction, _, _)) = possible_moves
                .into_iter()
                .filter(|(_, _, open_space)| *open_space > 2)
                .min_by_key(|(_, pos, _)| {
                    let dx = (pos.0 - target_food.position.0).abs();
                    let dy = (pos.1 - target_food.position.1).abs();
                    dx + dy
                })
            {
                self.change_direction(best_direction);
            }
        }
    }
    
    // Function to count open cells from a given position (basic flood-fill)
    fn count_reachable_cells(&self, start: (i32, i32)) -> usize {
        let mut visited = vec![vec![false; self.grid_size as usize]; self.grid_size as usize];
        let mut queue = vec![start];
        let mut count = 0;
    
        while let Some((x, y)) = queue.pop() {
            
            let is_out_of_bounds_or_visited = 
                x < 0 || y < 0 || x >= self.grid_size || y >= self.grid_size || visited[x as usize][y as usize];

            if is_out_of_bounds_or_visited {
                continue;
            }
            
            if self.snake.will_collide((x, y)) {
                continue;
            }

            visited[x as usize][y as usize] = true;
            count += 1;
    
            queue.push((x + 1, y));
            queue.push((x - 1, y));
            queue.push((x, y + 1));
            queue.push((x, y - 1));
        }
        
        count
    }

    pub fn is_over(&self) -> bool {
        self.snake.is_self_collision()
    }

    pub fn update(&mut self) -> bool {
        let mut has_grown = false;

        if self.is_played_by_ai {
            self.update_ai();
        }

        self.snake.traverse(self.direction);

        let food_positions: Vec<_> = self.foods.iter().map(|food| food.position).collect();

        for food in &mut self.foods {
            if self.snake.head_overlaps(food.position) {
                
                self.snake.grow();
                food.position = Self::get_free_position(&mut self.randomizer, &self.snake, &food_positions, self.grid_size);

                if self.is_played_by_ai {
                    self.score += 1;
                    has_grown = true;
                }

                break;
            }
        }

        has_grown
    }

    pub fn apply_options_and_reset(&mut self, grid_size: i32, food_count: u32) {
        self.grid_size = grid_size;
        self.food_count = food_count;
        self.cell_size = 2.0 / grid_size as f32;

        self.snake.resize(grid_size, self.cell_size);

        self.reset();
    }

    pub fn reset(&mut self) {
        self.is_played_by_ai = false;
        self.score = 0;
        self.snake.reset();
        self.foods = (0..self.food_count).map(|_| Food::new(
            self.randomizer.get_random_color(),
            Self::get_free_position(&mut self.randomizer, &self.snake, &vec![], self.grid_size),
            self.cell_size)).collect();
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if !matches!(
            (self.direction, direction),
            (Direction::Up, Direction::Down) 
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
        ) {
            self.direction = direction;
        }
    }

    pub fn get_vertices(&self) -> Vec<f32> {
        let mut all_vertices = Vec::new();

        all_vertices.extend_from_slice(&self.snake.as_vertices());

        for food in &self.foods {
            all_vertices.extend_from_slice(&food.as_vertices());
        }

        all_vertices
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::randomizer::OsRandomizer;

    struct MockInvokeJs;
    impl InvokeJs for MockInvokeJs {
        fn invoke(&self) {}
    }

    #[test]
    fn test_game_initialization() {
        // let mut randomizer = OsRandomizer::new();

        // let grid_size = 10;
        // let food_count = 3;
        // let mut game = Game::new(grid_size, food_count, MockInvokeJs, MockInvokeJs, randomizer);

        // assert_eq!(game.grid_size, grid_size);
        // assert_eq!(game.foods.len(), food_count as usize);
        // assert_eq!(game.score, 0);
        // assert!(game.can_run);
    }
}
