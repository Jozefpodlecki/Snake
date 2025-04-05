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

pub struct GreedyBfsAi;

impl AiController for GreedyBfsAi {
    fn get_direction(
        &self,
        snake: &Snake,
        foods: &[Food],
        obstacles: &[Obstacle],
        grid_size: i32,
    ) -> Option<Direction> {
        if let Some(target_position) = self.find_closest_food(snake, foods) {
            let snake_head = snake.get_head_position();
            let path = self.bfs(snake, obstacles, grid_size, snake_head, target_position);

            if let Some(next_step) = path {
                return Some(self.get_direction_from_move(snake_head, next_step));
            }
        }
        None
    }
}

impl GreedyBfsAi {
    pub fn new() -> Self {
        Self {}
    }

    fn bfs(
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

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        queue.push_back(start);
        visited.insert(start);

        while let Some(curr) = queue.pop_front() {
            if curr == target {
                let mut path = Vec::new();
                let mut pos = target;
                while let Some(&parent) = parent_map.get(&pos) {
                    path.push(pos);
                    pos = parent;
                }
                path.reverse();
                return path.first().copied();
            }

            for (dx, dy) in directions.iter() {
                let new_pos = (curr.0 + dx, curr.1 + dy);

                if self.is_valid(new_pos, obstacles, &visited, grid_size, snake) {
                    visited.insert(new_pos);
                    queue.push_back(new_pos);
                    parent_map.insert(new_pos, curr);
                }
            }
        }
        None
    }

    fn is_valid(
        &self,
        pos: (i32, i32),
        obstacles: &[Obstacle],
        visited: &HashSet<(i32, i32)>,
        grid_size: i32,
        snake: &Snake,
    ) -> bool {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= grid_size || pos.1 >= grid_size {
            return false;
        }
        if visited.contains(&pos) || snake.will_collide(pos) || self.is_obstacle(pos, obstacles) {
            return false;
        }
        true
    }

    fn get_direction_from_move(&self, start: (i32, i32), next: (i32, i32)) -> Direction {
        match (next.0 - start.0, next.1 - start.1) {
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            (0, 1) => Direction::Up,
            (0, -1) => Direction::Down,
            _ => Direction::Up,
        }
    }

    fn find_closest_food(&self, snake: &Snake, foods: &[Food]) -> Option<(i32, i32)> {
        let head = snake.get_head_position();
        foods.iter().min_by_key(|food| {
            let dx = (head.0 - food.position.0).abs();
            let dy = (head.1 - food.position.1).abs();
            dx + dy
        }).map(|food| food.position)
    }

    fn is_obstacle(&self, pos: (i32, i32), obstacles: &[Obstacle]) -> bool {
        obstacles.iter().any(|o| o.occupies(pos))
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
    use crate::objects::{Food, Obstacle, Snake};
    use crate::models::Direction;

    fn snake_at(pos: (i32, i32)) -> Snake {
        let mut snake = Snake::new();
        snake.initialize(3, 1.0);
        snake.move_to(pos);
        snake
    }

    fn food_at(pos: (i32, i32)) -> Food {
        Food::new([1.0, 0.0, 0.0, 1.0], pos, 1.0)
    }

    fn obstacle_at(pos: (i32, i32)) -> Obstacle {
        Obstacle::new([0.5, 0.5, 0.5, 1.0], pos, 1.0)
    }

    #[test]
    fn moves_toward_food() {
        let ai = GreedyBfsAi::new();
        let snake = snake_at((5, 5));
        let food = vec![food_at((7, 5))];
        let direction = ai.get_direction(&snake, &food, &[], 10);
        assert_eq!(direction, Some(Direction::Right));
    }

    #[test]
    fn avoids_single_obstacle() {
        let ai = GreedyBfsAi::new();
        let snake = snake_at((5, 5));
        let food = vec![food_at((7, 5))];
        let obstacles = vec![obstacle_at((6, 5))];
        let direction = ai.get_direction(&snake, &food, &obstacles, 10);
        assert_ne!(direction, Some(Direction::Right));
    }

    #[test]
    fn trapped_snake_returns_none() {
        let ai = GreedyBfsAi::new();
        let snake = snake_at((5, 5));
        let food = vec![food_at((6, 5))];
        let obstacles = vec![
            obstacle_at((6, 5)),
            obstacle_at((4, 5)),
            obstacle_at((5, 6)),
            obstacle_at((5, 4)),
        ];
        let direction = ai.get_direction(&snake, &food, &obstacles, 10);
        assert_eq!(direction, None);
    }

    #[test]
    fn picks_available_direction() {
        let ai = GreedyBfsAi::new();
        let snake = snake_at((3, 3));
        let food = vec![food_at((0, 0))];
        let obstacles = vec![obstacle_at((2, 3))];
        let direction = ai.get_direction(&snake, &food, &obstacles, 5);
        assert!(direction == Some(Direction::Up) || direction == Some(Direction::Down));
    }
}
