use crate::{models::Direction, objects::{Food, Obstacle, Snake}};

pub trait AiController {
    fn get_direction(
        &self,
        snake: &Snake,
        foods: &[Food],
        obstacles: &[Obstacle],
        grid_size: i32,
    ) -> Option<Direction>;
}

// Greedy Best-First Search (GBFS) with basic flood-fill for open space evaluation.
pub struct GBFSAiController {}

impl AiController for GBFSAiController {
    fn get_direction(
        &self,
        snake: &Snake,
        foods: &[Food],
        obstacles: &[Obstacle],
        grid_size: i32,
    ) -> Option<Direction> {
        if let Some(target_position) = self.get_closest_food(snake, foods) {
            let snake_head = snake.get_head_position();
            let mut possible_moves = vec![];
    
            let directions = [
                (Direction::Right, (snake_head.0 + 1, snake_head.1)),
                (Direction::Left, (snake_head.0 - 1, snake_head.1)),
                (Direction::Up, (snake_head.0, snake_head.1 + 1)),
                (Direction::Down, (snake_head.0, snake_head.1 - 1)),
            ];
    
            for (dir, position) in directions.iter() {
                if snake.will_collide(*position) || self.is_obstacle(*position, obstacles) {
                    continue;
                }
    
                let open_space = self.count_reachable_cells(*position, snake, obstacles, grid_size);
                possible_moves.push((*dir, *position, open_space));
            }
    
            if let Some((best_direction, _, _)) = possible_moves
                .into_iter()
                .filter(|(_, _, open_space)| *open_space > 2) // Avoid tight spaces
                .max_by_key(|(_, _, open_space)| *open_space) // Prefer safest path
                .into_iter()
                .min_by_key(|(_, pos, _)| {
                    let dx = (pos.0 - target_position.0).abs();
                    let dy = (pos.1 - target_position.1).abs();
                    dx + dy
                })
            {
                return Some(best_direction);
            }
        }
    
        None
    }
}

impl GBFSAiController {
    pub fn new() -> Self {
        GBFSAiController {}
    }
    

    fn get_closest_food(&self, snake: &Snake, foods: &[Food]) -> Option<(i32, i32)> {
        let snake_head = snake.get_head_position();
        
        let food = foods.iter().min_by_key(|food| {
            let dx = (snake_head.0 - food.position.0).abs();
            let dy = (snake_head.1 - food.position.1).abs();
            dx + dy
        });

        food.map(|food| food.position)
    }
    
    // Function to count open cells from a given position (basic flood-fill)
    fn count_reachable_cells(
        &self,
        start: (i32, i32),
        snake: &Snake,
        obstacles: &[Obstacle],
        grid_size: i32,
    ) -> usize {
        let mut visited = vec![vec![false; grid_size as usize]; grid_size as usize];
        let mut queue = vec![start];
        let mut count = 0;
    
        while let Some((x, y)) = queue.pop() {
            let is_out_of_bounds_or_visited =
                x < 0 || y < 0 || x >= grid_size || y >= grid_size || visited[x as usize][y as usize];
    
            if is_out_of_bounds_or_visited || snake.will_collide((x, y)) || self.is_obstacle((x, y), obstacles) {
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
    
    fn is_obstacle(&self, position: (i32, i32), obstacles: &[Obstacle]) -> bool {
        obstacles.iter().any(|obstacle| obstacle.occupies(position))
    }
}

#[cfg(test)]
mockall::mock! {
    pub AiControllerMock {}
    impl AiController for AiControllerMock {
        fn get_direction(
            &self,
            snake: &Snake,
            foods: &[Food],
            obstacles: &[Obstacle],
            grid_size: i32,
        ) -> Option<Direction>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Direction;
    use crate::objects::{Food, Obstacle, Snake};

    fn setup_snake(body_length: usize, cell_size: f32) -> Snake {
        let mut snake = Snake::new();
        snake.initialize(body_length, cell_size);
        snake
    }

    fn setup_food(position: (i32, i32)) -> Food {
        let color = [1.0, 0.0, 0.0, 1.0]; // Red color
        let cell_size = 1.0;
        Food::new(color, position, cell_size)
    }

    fn setup_obstacle(position: (i32, i32)) -> Obstacle {
        let color = [0.5, 0.5, 0.5, 1.0]; // Gray color
        let cell_size = 1.0;
        Obstacle::new(color, position, cell_size)
    }

    #[test]
    fn test_moves_toward_closest_food() {
        let ai = GBFSAiController::new();
        let mut snake = setup_snake(3, 1.0);
        snake.move_to((5, 5)); // Manually move head to (5,5)
        let foods = vec![setup_food((7, 5))];
        let obstacles = vec![];

        let direction = ai.get_direction(&snake, &foods, &obstacles, 10);

        assert_eq!(direction, Some(Direction::Down));
    }

    #[test]
    fn test_avoids_obstacles() {
        let ai = GBFSAiController::new();
        let mut snake = setup_snake(3, 1.0);
        snake.move_to((5, 5));
        let foods = vec![setup_food((7, 5))];
        let obstacles = vec![setup_obstacle((6, 5))];

        let direction = ai.get_direction(&snake, &foods, &obstacles, 10);

        // Should not go Right because of obstacle; should pick an alternative route
        assert_ne!(direction, Some(Direction::Right));
    }

    #[test]
    fn test_avoids_dead_ends() {
        let ai = GBFSAiController::new();
        let mut snake = setup_snake(3, 1.0);
        snake.move_to((5, 5));
        let foods = vec![setup_food((7, 5))];
        let obstacles = vec![
            setup_obstacle((6, 5)),
            setup_obstacle((6, 6)),
            setup_obstacle((5, 6)),
        ];

        let direction = ai.get_direction(&snake, &foods, &obstacles, 10);

        // Should go left to escape, not right into a dead-end
        assert_eq!(direction, Some(Direction::Down));
    }

    #[test]
    fn test_no_move_if_trapped() {
        let ai = GBFSAiController::new();
        let mut snake = setup_snake(3, 1.0);
        snake.move_to((5, 5));
        let foods = vec![setup_food((7, 5))];
        let obstacles = vec![
            setup_obstacle((6, 5)),
            setup_obstacle((4, 5)),
            setup_obstacle((5, 6)),
            setup_obstacle((5, 4)),
        ];

        let direction = ai.get_direction(&snake, &foods, &obstacles, 10);

        // No valid moves left
        assert_eq!(direction, None);
    }

    #[test]
    fn test_picks_safest_path() {
        let ai = GBFSAiController::new();
        let mut snake = setup_snake(3, 1.0);
        snake.move_to((5, 5));
        let foods = vec![setup_food((7, 5))];
        let obstacles = vec![
            setup_obstacle((6, 5)),
            setup_obstacle((4, 6)),
        ];

        let direction = ai.get_direction(&snake, &foods, &obstacles, 10);

        // Should go down to avoid immediate collision
        assert_eq!(direction, Some(Direction::Down));
    }
}
