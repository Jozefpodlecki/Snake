use crate::models::Direction;


pub struct Snake {
    body_length: usize,
    body: Vec<(i32, i32)>,
    grid_size: i32,
    cell_size: f32,
    spacing: f32,
    color: [f32; 4],
}

impl Snake {
    pub fn new() -> Self {
      
        let spacing = 0.01;

        Snake {
            body: vec![],
            grid_size: 0,
            cell_size: 0.0,
            spacing,
            color: [0.0, 0.0, 0.0, 0.0],
            body_length: 0,
        }
    }

    pub fn initialize(&mut self, body_length: usize, grid_size: i32, cell_size: f32, color: [f32; 4]) {
        self.body = Self::initialize_body(body_length);
        self.grid_size = grid_size;
        self.cell_size = cell_size;
        self.color = color;
        self.body_length = body_length;
    }

    pub fn grow(&mut self) {
        let length = self.body.len();

        let last = self.body[length - 1];
        let second_last = self.body[length - 2];
    
        let dx = last.0 - second_last.0;
        let dy = last.1 - second_last.1;
    
        let new_segment = (last.0 + dx, last.1 + dy);
        self.body.push(new_segment);
    }

    pub fn is_self_collision(&self) -> bool {
        let head = self.body[0];

        for &segment in self.body.iter().skip(1) {
            if segment == head {
                return true;
            }
        }

        false
    }

    pub fn occupies(&self, position: (i32, i32)) -> bool {
        self.body.iter().any(|&segment| segment == position)
    }

    pub fn head_overlaps(&self, position: (i32, i32)) -> bool {
        self.body[0] == position
    }

    pub fn resize(&mut self, grid_size: i32, cell_size: f32) {
        self.grid_size = grid_size;
        self.cell_size = cell_size;
    }

    pub fn reset(&mut self) {
        self.body = Self::initialize_body(self.body_length);
    }

    pub fn traverse(&mut self, direction: Direction) {
        let (head_x, head_y) = self.body[0];
        let unit = 1;

        let mut new_head = match direction {
            Direction::Up => (head_x, head_y + unit),
            Direction::Down => (head_x, head_y - unit),
            Direction::Left => (head_x - unit, head_y),
            Direction::Right => (head_x + unit, head_y),
        };

        new_head.0 = (new_head.0 + self.grid_size) % self.grid_size;
        new_head.1 = (new_head.1 + self.grid_size) % self.grid_size;

        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1];
        }

        self.body[0] = new_head;
    }

    pub fn get_head_position(&self) -> (i32, i32) {
        self.body[0]
    }

    pub fn will_collide(&self, new_position: (i32, i32)) -> bool {
        self.body.contains(&new_position)
    }

    fn initialize_body(length: usize) -> Vec<(i32, i32)> {
        let mut body = Vec::new();
        
        let start_x = 10;
        let start_y = 10;
    
        for i in 0..length {
            body.push((start_x - i as i32, start_y));
        }

        body
    }

    pub fn as_vertices(&self) -> Vec<f32> {
        
        let mut all_vertices = Vec::new();

        for &(x, y) in &self.body {
            let x1 = x as f32 * self.cell_size - 1.0 + self.spacing;
            let y1 = y as f32 * self.cell_size - 1.0 + self.spacing;
            let x2 = x1 + self.cell_size - self.spacing;
            let y2 = y1 + self.cell_size - self.spacing * 1.4;

            let vertices: [f32; 36] = [
                x1, y1, self.color[0], self.color[1], self.color[2], self.color[3],
                x2, y1, self.color[0], self.color[1], self.color[2], self.color[3],
                x1, y2, self.color[0], self.color[1], self.color[2], self.color[3],
                x1, y2, self.color[0], self.color[1], self.color[2], self.color[3],
                x2, y1, self.color[0], self.color[1], self.color[2], self.color[3],
                x2, y2, self.color[0], self.color[1], self.color[2], self.color[3],
            ];
    
            all_vertices.extend_from_slice(&vertices);
        }
    
        all_vertices
    }
    
}