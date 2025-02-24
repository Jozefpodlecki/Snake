use log::debug;

use csscolorparser::Color;
use crate::{models::{Difficulty, Direction, GameOptions, GameResult}, objects::{Food, Obstacle, Snake}, randomizer::Randomizer};

pub struct Game<R: Randomizer> {
    options: GameOptions,
    pub direction: Direction,
    pub snake: Snake,
    pub foods: Vec<Food>,
    pub obstacles: Vec<Obstacle>,
    cell_size: f32,
    randomizer: R
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
            self.create_obstacles(2);
        }
    }

    fn create_obstacles(&mut self, count: usize) {
        let obstacles = (0..count).map(|_| Obstacle::new(
            [0.7, 0.7, 0.7, 1.0],
            self.randomizer.get_random_position_on_grid(self.options.grid_size), self.cell_size)).collect();
        self.obstacles = obstacles;
    }

    fn get_free_position(
        randomizer: &mut R,
        snake: &Snake,
        obstacles: &[Obstacle],
        food_positions: &[(i32, i32)],
        grid_size: i32) -> (i32, i32) {
        loop {
            let position = randomizer.get_random_position_on_grid(grid_size);
            
            let is_occupied = snake.occupies(position) 
                || obstacles.iter().any(|obstacle| obstacle.occupies(position))
                || food_positions.iter().any(|object_position| *object_position == position);
            
            if !is_occupied {
                return position;
            }
        }
    }

    pub fn is_over(&self) -> bool {
        self.snake.is_self_collision()
            || self.obstacles.iter().any(|obstacle| obstacle.occupies(self.snake.get_head_position()))
    }

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

    pub fn update(&mut self) -> GameResult {
        let mut game_result = GameResult::Noop;

        self.update_snake_position(self.direction);

        let food_positions: Vec<_> = self.foods.iter().map(|food| food.position).collect();

        for food in &mut self.foods {
            if self.snake.head_overlaps(food.position) {
                
                self.snake.grow();
                food.position = Self::get_free_position(
                    &mut self.randomizer,
                    &self.snake, 
                    &self.obstacles,
                    &food_positions,
                    self.options.grid_size);

                game_result = GameResult::Score;

                break;
            }
        }

        if self.is_over() {
            game_result = GameResult::Over;
        }

        game_result
    }

    pub fn apply_options_and_reset(&mut self, options: GameOptions) {
        debug!("apply_options_and_reset");

        if self.options.difficulty != options.difficulty {
            if options.difficulty == Difficulty::Hard {
                self.create_obstacles(2);
            }
            else {
                self.obstacles = vec![];
            }
        }

        self.options = options;

        self.cell_size = 2.0 / self.options.grid_size as f32;

        let color = self.options.snake_color.parse::<Color>().unwrap();
        self.snake.set_color(color.to_array());
        self.snake.resize(self.cell_size);

        self.reset();
    }

    pub fn reset(&mut self) {
        self.snake.reset();
        self.foods = (0..self.options.food_count).map(|_| Food::new(
            self.randomizer.get_random_color(),
            Self::get_free_position(
                &mut self.randomizer,
                &self.snake,
                &vec![],
                &vec![],
                self.options.grid_size),
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
    use crate::{models::{GameOptions, Difficulty}, objects::Snake, randomizer::OsRandomizer};

    fn default_game_options() -> GameOptions {
        GameOptions {
            id: "".into(),
            fps: 10,
            frame_threshold_ms: 10.0,
            grid_size: 10,
            food_count: 3,
            difficulty: Difficulty::Easy,
            snake_color: "#00FF00".to_string(),
        }
    }

    fn setup_game(difficulty: Difficulty) -> Game<OsRandomizer> {
        let randomizer = OsRandomizer::new();
        let mut options = default_game_options();
        options.difficulty = difficulty;
    
        let mut game = Game::new(options, randomizer);
        game.initialize();
        game
    }

    #[test]
    fn test_game_initialization() {
        let game = setup_game(Difficulty::Easy);

        assert_eq!(game.foods.len(), 3, "Should initialize with correct number of food items");
        assert_eq!(game.obstacles.len(), 0, "Should have no obstacles in Easy mode");
    }

    #[test]
    fn test_snake_moves() {
        let mut game = setup_game(Difficulty::Easy);

        let initial_head_position = game.snake.get_head_position();
        game.change_direction(Direction::Right);
        game.update();
        let new_head_position = game.snake.get_head_position();

        assert_ne!(initial_head_position, new_head_position, "Snake should move when updated");
        assert_eq!(new_head_position.0, (initial_head_position.0 + 1) % game.options.grid_size, "Snake should move correctly to the right");
    }

    #[test]
    fn test_food_consumption() {
        let mut game = setup_game(Difficulty::Easy);

        let food_position = game.foods[0].position;
        let position = (food_position.0 - 1, food_position.1);
        game.snake.move_to(position);

        let result = game.update();

        assert_eq!(result, GameResult::Score, "Snake should score when consuming food");
        assert!(game.foods.iter().any(|food| food.position != food_position), "Food should be repositioned after being eaten");
    }

    #[test]
    fn test_game_over_on_self_collision() {
        let mut game = setup_game(Difficulty::Easy);

        game.snake.grow();
        game.snake.grow();
        game.snake.move_to((1, 1));
        game.snake.move_to((1, 2));
        game.snake.move_to((2, 2));
        game.snake.move_to((2, 1));
        game.snake.move_to((1, 1)); // Colliding with itself

        let result = game.update();
        assert_eq!(result, GameResult::Over, "Game should be over if the snake collides with itself");
    }

    #[test]
    fn test_obstacle_avoidance() {
        let mut game = setup_game(Difficulty::Hard);

        assert!(!game.obstacles.is_empty(), "Obstacles should be generated in Hard mode");

        let obstacle_pos = game.obstacles[0].position;
        let position = (obstacle_pos.0 - 1, obstacle_pos.1);
        game.snake.move_to(position);

        let result = game.update();
        assert_eq!(result, GameResult::Over, "Game should be over if the snake hits an obstacle");
    }

    #[test]
    fn test_change_direction() {
        let mut game = setup_game(Difficulty::Easy);

        game.change_direction(Direction::Up);
        assert_eq!(game.direction, Direction::Up, "Snake should be able to change direction");

        game.change_direction(Direction::Down);
        assert_eq!(game.direction, Direction::Up, "Snake should not be able to reverse direction immediately");
    }

    #[test]
    fn test_reset_game() {
        let mut game = setup_game(Difficulty::Easy);

        game.change_direction(Direction::Right);
        game.reset();

        assert_eq!(game.foods.len() as u32, game.options.food_count, "Food count should reset");
    }
}
