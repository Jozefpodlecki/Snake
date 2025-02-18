use js_sys::{Float32Array, Math};
use wasm_bindgen::JsValue;
use web_sys::{console, WebGlBuffer, WebGl2RenderingContext};

use crate::console_log;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct GameContext {
    pub score: u32,
    pub width: f32,
    pub height: f32,
    pub current_direction: Direction,
    pub grid_size: i32,
    pub can_run: bool,
    pub position_buffer: Option<WebGlBuffer>
}

pub struct Food {
    position: (i32, i32),
    grid_size: i32,
    cell_size: f32,
    spacing: f32,
    color: [f32; 4]
}

impl Food {
    pub fn new(grid_size: i32) -> Self {
        let position = Self::random_position(grid_size);
        let cell_size = 2.0 / grid_size as f32;
        let spacing = 0.01;
        let color = [1.0, 0.647, 0.0, 1.0];

        Food { 
            position,
            grid_size,
            cell_size,
            spacing,
            color
        }
    }

    pub fn respawn(&mut self) {
        self.position = Self::random_position(self.grid_size);
    }

    fn random_position(grid_size: i32) -> (i32, i32) {
        let x = (Math::random() * grid_size as f64) as i32;
        let y = (Math::random() * grid_size as f64) as i32;
        (x, y)

    }

    pub fn draw(&self, context: &WebGl2RenderingContext) {
        let vertices_array = self.compute_vertices();
        let length = vertices_array.length();
        let buffer_size = (length * 4) as i32;

        // context.buffer_data_with_array_buffer_view(
        //     WebGlRenderingContext::ARRAY_BUFFER,
        //     &vertices_array,
        //     WebGlRenderingContext::STATIC_DRAW,
        // );

        // context.buffer_data_with_i32(WebGlRenderingContext::ARRAY_BUFFER, buffer_size, WebGlRenderingContext::DYNAMIC_DRAW);

        // context.buffer_sub_data_with_i32_and_array_buffer_view(
        //     WebGlRenderingContext::ARRAY_BUFFER,
        //     0,
        //     &vertices_array,
        // );

        // context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (length / 2) as i32);
    }

    fn compute_vertices(&self) -> Float32Array {
        let (x, y) = self.position;
        let x1 = x as f32 * self.cell_size - 1.0 + self.spacing;
        let y1 = y as f32 * self.cell_size - 1.0 + self.spacing;
        let x2 = x1 + self.cell_size - self.spacing;
        let y2 = y1 + self.cell_size - self.spacing;

        let vertices: [f32; 36] = [
            x1, y1, self.color[0], self.color[1], self.color[2], self.color[3],
            x2, y1, self.color[0], self.color[1], self.color[2], self.color[3],
            x1, y2, self.color[0], self.color[1], self.color[2], self.color[3],
            x1, y2, self.color[0], self.color[1], self.color[2], self.color[3],
            x2, y1, self.color[0], self.color[1], self.color[2], self.color[3],
            x2, y2, self.color[0], self.color[1], self.color[2], self.color[3],
        ];

        unsafe { Float32Array::view(&vertices) }
    }
}

pub struct Snake {
    body: Vec<(i32, i32)>,
    vertices: Vec<f32>,
    direction: Direction,
    grid_size: i32,
    cell_size: f32,
    spacing: f32,
    color: [f32; 4]
}

impl Snake {
    pub fn new(direction: Direction, grid_size: i32) -> Self {
        let body = Self::initialize_body(5);
        let cell_size = 2.0 / grid_size as f32;
        let spacing = 0.01;
        let color = [0.5, 0.5, 0.5, 1.0];

        Snake {
            body,
            vertices: vec![],
            direction,
            grid_size,
            cell_size,
            spacing,
            color
        }
    }

    pub fn draw(&self, context: &WebGl2RenderingContext) {
        let vertices_array = self.compute_vertices();
        let length = vertices_array.length();
        let buffer_size = (length * 4) as i32;
        let verticle_size = (length / 6) as i32;

        // context.buffer_data_with_array_buffer_view(
        //     WebGlRenderingContext::ARRAY_BUFFER,
        //     &vertices_array,
        //     WebGlRenderingContext::STATIC_DRAW,
        // );

        context.buffer_data_with_i32(WebGl2RenderingContext::ARRAY_BUFFER, buffer_size, WebGl2RenderingContext::DYNAMIC_DRAW);

        context.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            0,
            &vertices_array,
        );
        
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, verticle_size);
        // context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (length / 2) as i32);
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

    pub fn overlaps(&self, food: &Food) -> bool {
        self.body[0] == food.position
    }

    pub fn reset(&mut self) {
        self.body = Self::initialize_body(3);
        self.direction = Direction::Right;
    }

    pub fn traverse(&mut self) {
        let (head_x, head_y) = self.body[0];
        let unit = 1;

        let mut new_head = match self.direction {
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

    fn initialize_body(length: usize) -> Vec<(i32, i32)> {
        let mut body = Vec::new();
        
        let start_x = 10;
        let start_y = 10;
    
        for i in 0..length {
            body.push((start_x - i as i32, start_y));
        }

        body
    }

    fn compute_vertices(&self) -> Float32Array {
        
        let mut all_vertices = Vec::new();

        for &(x, y) in &self.body {
            let x1 = x as f32 * self.cell_size - 1.0 + self.spacing;
            let y1 = y as f32 * self.cell_size - 1.0 + self.spacing;
            let x2 = x1 + self.cell_size - self.spacing;
            let y2 = y1 + self.cell_size - self.spacing * 1.5;

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
    
        unsafe { Float32Array::view(&all_vertices) }
    }
    
}