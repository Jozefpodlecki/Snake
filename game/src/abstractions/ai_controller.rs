use std::collections::{HashSet, VecDeque};

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
            // BFS to find shortest path
            let path = self.bfs_pathfinding(snake, obstacles, grid_size, snake_head, target_position);

            if let Some(next_move) = path {
                return Some(self.get_direction_from_move(snake_head, next_move));
            }
        }

        None
    }
}

impl GBFSAiController {
    pub fn new() -> Self {
        GBFSAiController {}
    }
    
    fn bfs_pathfinding(
        &self,
        snake: &Snake,
        obstacles: &[Obstacle],
        grid_size: i32,
        start: (i32, i32),
        target: (i32, i32),
    ) -> Option<(i32, i32)> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent_map = std::collections::HashMap::new();
        
        // Directions: Right, Left, Up, Down
        let directions = [
            (1, 0),  // Right
            (-1, 0), // Left
            (0, 1),  // Up
            (0, -1), // Down
        ];

        queue.push_back(start);
        visited.insert(start);

        while let Some(curr_pos) = queue.pop_front() {
            if curr_pos == target {
                // Reached the target, build the path
                let mut path = Vec::new();
                let mut pos = target;
                while let Some(&parent) = parent_map.get(&pos) {
                    path.push(pos);
                    pos = parent;
                }

                path.reverse();
                return path.first().cloned(); // Return the next move
            }

            for (dx, dy) in &directions {
                let new_pos = (curr_pos.0 + dx, curr_pos.1 + dy);

                if self.is_valid_move(new_pos, obstacles, &visited, grid_size, snake) {
                    visited.insert(new_pos);
                    queue.push_back(new_pos);
                    parent_map.insert(new_pos, curr_pos);
                }
            }
        }

        None // No path found
    }

    fn is_valid_move(
        &self,
        position: (i32, i32),
        obstacles: &[Obstacle],
        visited: &HashSet<(i32, i32)>,
        grid_size: i32,
        snake: &Snake,
    ) -> bool {
        // Check if within bounds
        if position.0 < 0 || position.1 < 0 || position.0 >= grid_size || position.1 >= grid_size {
            return false;
        }

        // Check if visited or if snake collides
        if visited.contains(&position) || snake.will_collide(position) || self.is_obstacle(position, obstacles) {
            return false;
        }

        true
    }

    fn get_direction_from_move(&self, start: (i32, i32), next_move: (i32, i32)) -> Direction {
        match (next_move.0 - start.0, next_move.1 - start.1) {
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            (0, 1) => Direction::Up,
            (0, -1) => Direction::Down,
            _ => Direction::Up, // Default, shouldn't happen
        }
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
    
    fn is_obstacle(&self, position: (i32, i32), obstacles: &[Obstacle]) -> bool {
        obstacles.iter().any(|obstacle| obstacle.occupies(position))
    }
}

#[cfg(test)]
mockall::mock! {
    pub AiController {}
    impl AiController for AiController {
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
