use log::debug;

use csscolorparser::Color;
use crate::{models::{Difficulty, Direction, GameOptions}, objects::{obstacle, Food, Obstacle, Snake}, randomizer::Randomizer};

pub struct Game<R: Randomizer> {
    pub is_played_by_ai: bool,
    options: GameOptions,
    direction: Direction,
    score: u32,
    snake: Snake,
    foods: Vec<Food>,
    obstacles: Vec<Obstacle>,
    cell_size: f32,
    randomizer: R,
}

impl<R: Randomizer> Game< R> {
    pub fn new(
        options: GameOptions,
        randomizer: R) -> Self {
        let cell_size = 2.0 / options.grid_size as f32;
        let snake = Snake::new();

        Game {
            options,
            direction: Direction::Right,
            is_played_by_ai: true,
            score: 0,
            snake,
            foods: vec![],
            obstacles: vec![],
            cell_size,
            randomizer
        }
    }

    pub fn initialize(&mut self) {
        let body_length = 5;
        
        let foods = (0..self.options.food_count).map(|_| Food::new(
            self.randomizer.get_random_color(),
            self.randomizer.get_random_position_on_grid(self.options.grid_size), self.cell_size)).collect();
        self.foods = foods;
        self.snake.initialize(body_length, self.cell_size);
        
        let color = self.options.snake_color.parse::<Color>().unwrap();
        self.snake.set_color(color.to_array());

        if self.options.difficulty == Difficulty::Hard {
            let obstacles = (0..2).map(|_| Obstacle::new(
                [0.7, 0.7, 0.7, 1.0],
                self.randomizer.get_random_position_on_grid(self.options.grid_size), self.cell_size)).collect();
            self.obstacles = obstacles;
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
        let grid_size = self.options.grid_size;

        let mut visited = vec![vec![false; grid_size as usize]; grid_size as usize];
        let mut queue = vec![start];
        let mut count = 0;
    
        while let Some((x, y)) = queue.pop() {
            
            let is_out_of_bounds_or_visited = 
                x < 0 || y < 0 || x >= grid_size || y >= grid_size || visited[x as usize][y as usize];

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

    // pub fn traverse(&mut self, direction: Direction) {
    //     let (head_x, head_y) = self.body[0];
    //     let unit = 1;

    //     let mut new_head = match direction {
    //         Direction::Up => (head_x, head_y + unit),
    //         Direction::Down => (head_x, head_y - unit),
    //         Direction::Left => (head_x - unit, head_y),
    //         Direction::Right => (head_x + unit, head_y),
    //     };

    //     new_head.0 = (new_head.0 + self.grid_size) % self.grid_size;
    //     new_head.1 = (new_head.1 + self.grid_size) % self.grid_size;

    //     for i in (1..self.body.len()).rev() {
    //         self.body[i] = self.body[i - 1];
    //     }

    //     self.body[0] = new_head;
    // }

    fn update_snake_position(&mut self, direction: Direction) {
        let grid_size = self.options.grid_size;

        let (head_x, head_y) = self.snake.get_head_position();
        let unit = 1;
    
        let mut new_head = match direction {
            Direction::Up => (head_x, head_y + unit),
            Direction::Down => (head_x, head_y - unit),
            Direction::Left => (head_x - unit, head_y),
            Direction::Right => (head_x + unit, head_y),
        };

        new_head.0 = (new_head.0 + grid_size) % grid_size;
        new_head.1 = (new_head.1 + grid_size) % grid_size;
    
        self.snake.move_to(new_head);
    }

    pub fn update(&mut self) -> bool {
        let mut has_grown = false;

        if self.is_played_by_ai {
            debug!("update_ai");
            self.update_ai();
        }

        self.update_snake_position(self.direction);

        let food_positions: Vec<_> = self.foods.iter().map(|food| food.position).collect();

        for food in &mut self.foods {
            if self.snake.head_overlaps(food.position) {
                
                self.snake.grow();
                food.position = Self::get_free_position(&mut self.randomizer, &self.snake, &food_positions, self.options.grid_size);

                if self.is_played_by_ai {
                    self.score += 1;
                    has_grown = true;
                }

                break;
            }
        }

        has_grown
    }

    pub fn apply_options_and_reset(&mut self, options: GameOptions) {
        debug!("apply_options_and_reset");
        self.options = options;
        self.cell_size = 2.0 / self.options.grid_size as f32;

        let color = self.options.snake_color.parse::<Color>().unwrap();
        self.snake.set_color(color.to_array());
        self.snake.resize(self.cell_size);

        self.reset();
    }

    pub fn reset(&mut self) {
        self.is_played_by_ai = false;
        self.score = 0;
        self.snake.reset();
        self.foods = (0..self.options.food_count).map(|_| Food::new(
            self.randomizer.get_random_color(),
            Self::get_free_position(&mut self.randomizer, &self.snake, &vec![], self.options.grid_size),
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

        for obstacle in &self.obstacles {
            all_vertices.extend_from_slice(&obstacle.as_vertices());
        }
        

        all_vertices
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::{abstractions::InvokeJs, randomizer::OsRandomizer};

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
