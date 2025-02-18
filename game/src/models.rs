use js_sys::{Float32Array, Math};
use wasm_bindgen::JsValue;
use web_sys::{console, WebGlRenderingContext};

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

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
    pub can_run: bool
}

pub struct Food {
    position: (i32, i32),
    grid_size: i32,
    cell_size: f32,
}

impl Food {
    pub fn new(grid_size: i32) -> Self {
        let position = Self::random_position(grid_size);
        let cell_size = 2.0 / grid_size as f32;

        Food { position, grid_size, cell_size }
    }

    pub fn respawn(&mut self) {
        self.position = Self::random_position(self.grid_size);
    }

    fn random_position(grid_size: i32) -> (i32, i32) {
        let x = (Math::random() * grid_size as f64) as i32;
        let y = (Math::random() * grid_size as f64) as i32;
        console_log!("x {} y {}", x, y);
        (x, y)

    }

    pub fn draw(&self, context: &WebGlRenderingContext) {
        let vertices_array = self.compute_vertices();
        let length = vertices_array.length();

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (length / 2) as i32);
    }


    fn compute_vertices(&self) -> Float32Array {
        let (x, y) = self.position;
        let x1 = x as f32 * self.cell_size - 1.0 + 0.01;
        let y1 = y as f32 * self.cell_size - 1.0 + 0.01;
        let x2 = x1 + self.cell_size - 0.01;
        let y2 = y1 + self.cell_size - 0.01;

        let vertices: [f32; 12] = [
            x1, y1,
            x2, y1,
            x1, y2,
            x1, y2,
            x2, y1,
            x2, y2,
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
}

impl Snake {
    pub fn new(direction: Direction, grid_size: i32) -> Self {
        let body = Self::initialize_body();
        let cell_size = 2.0 / grid_size as f32;
        let spacing = 0.01;

        Snake {
            body,
            vertices: vec![],
            direction,
            grid_size,
            cell_size,
            spacing
        }
    }

    pub fn draw(&self, context: &WebGlRenderingContext) {
        let vertices_array = self.compute_vertices();
        let length = vertices_array.length();

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
        
        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (length / 2) as i32);
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

    pub fn overlaps(&self, food: &Food) -> bool {
        self.body[0] == food.position
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

    fn initialize_body() -> Vec<(i32, i32)> {
        let mut body = Vec::new();
        body.push((10, 10));
        body.push((9, 10));
        body.push((8, 10));
        body
    }

    fn compute_vertices(&self) -> Float32Array {
        
        let mut all_vertices = Vec::new();
        

        for &(x, y) in &self.body {
            let x1 = x as f32 * self.cell_size - 1.0 + self.spacing;
            let y1 = y as f32 * self.cell_size - 1.0 + self.spacing;
            let x2 = x1 + self.cell_size - self.spacing;
            let y2 = y1 + self.cell_size - self.spacing * 1.5;

            let vertices: [f32; 12] = [
                x1, y1,
                x2, y1,
                x1, y2,
                x1, y2,
                x2, y1,
                x2, y2,
            ];
    
            all_vertices.extend_from_slice(&vertices);
        }
    
        unsafe { Float32Array::view(&all_vertices) }
    }
    
}