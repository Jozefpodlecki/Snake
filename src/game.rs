extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
use web_sys::Window;
use web_sys::HtmlCanvasElement;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlImageElement;
use js_sys::Math;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::utils::{Style, Layout};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(str: &str);
}

pub struct GameContext {
    pub window: Window,
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub row_size: f64,
    pub column_size: f64,
    pub rows: u32,
    pub columns: u32,
    pub width: u32,
    pub height: u32,
    pub has_lost: bool,
    pub first_time: bool,
    pub score: u32,
    pub tail_length: u32,
    pub direction: Direction,
    pub start_direction: Direction,
    pub snake_tail: Vec<SnakeCell>,
    pub image: Option<HtmlImageElement>
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub x: f64,
    pub y: f64,
    pub row: u32,
    pub column: u32
}

#[derive(Copy, Clone)]
pub struct SnakeCell {
    pub cell: Cell,
    pub direction: Direction
}

pub fn create_key_direction_map() -> HashMap<String, Direction> {
    let mut key_direction_map = HashMap::new();
    key_direction_map.insert("a".to_string(), Direction::Left);
    key_direction_map.insert("arrowleft".to_string(), Direction::Left);
    key_direction_map.insert("w".to_string(), Direction::Up);
    key_direction_map.insert("arrowup".to_string(), Direction::Up);
    key_direction_map.insert("s".to_string(), Direction::Down);
    key_direction_map.insert("arrowdown".to_string(), Direction::Down);
    key_direction_map.insert("d".to_string(), Direction::Right);
    key_direction_map.insert("arrowright".to_string(), Direction::Right);
    key_direction_map
}

pub fn create_key_direction_disallowed_map() -> HashMap<Direction, Direction> {
    let mut key_direction_disallowed_map = HashMap::new();
    key_direction_disallowed_map.insert(Direction::Left, Direction::Right);
    key_direction_disallowed_map.insert(Direction::Right, Direction::Left);
    key_direction_disallowed_map.insert(Direction::Up, Direction::Down);
    key_direction_disallowed_map.insert(Direction::Down, Direction::Up);
    key_direction_disallowed_map
}

pub fn generateFood(rows: u32, columns: u32, rowChunk: f64, columnChunk: f64) -> Cell {
    let randomRow = (Math::random() * rows as f64) as u32;
    let randomColumn = (Math::random() * columns as f64) as u32;
    
    let cell = Cell {
        x: randomColumn as f64 * columnChunk,
        y: randomRow as f64 * rowChunk,
        row: randomRow,
        column: randomColumn
    };
    
    cell
}

pub fn initialize_snake_tail(tail_length: u32, row: u32, column: u32, rowChunk: f64, columnChunk: f64, start_direction: Direction) -> Vec<SnakeCell> {
    let mut vector: Vec<SnakeCell> = Vec::new();

    for i in 0..tail_length {
        vector.push(SnakeCell {
            cell: Cell {
                x: (column as f64 + f64::from(i)) * columnChunk,
                y: row as f64 * rowChunk,
                column: column + i as u32,
                row: row,
            },
            direction: start_direction
        })
    }

    vector
}

pub fn render_snake(game_context: &mut GameContext) {
    let context = &mut game_context.context;
    let snake_tail = &mut game_context.snake_tail;
    let column_size = game_context.column_size;
    let row_size = game_context.row_size;

    let style = JsValue::from("#FFFFFF77");
    context.set_fill_style(&style);
    for snake_cell in snake_tail.iter() {
        let cell = snake_cell.cell;
        context.fill_rect(cell.x, cell.y, column_size, row_size);
    }
}

pub fn check_if_snake_overlap(snake_tail: &[SnakeCell], snake_head: Cell) -> bool {

    for snake_cell in snake_tail.iter().take(snake_tail.len() - 1) {
        if snake_head.row == snake_cell.cell.row && snake_head.column == snake_cell.cell.column {
            return true;
        }
    }

    false
}

pub fn render_food(context: &CanvasRenderingContext2d, food: Cell, column_size: f64, row_size: f64) {
    context.fill_rect(food.x, food.y, column_size, row_size);
}

pub fn render_text(context: &mut CanvasRenderingContext2d, text: String, style: Style, layout: Layout) {
    let color = JsValue::from(&style.color);
    context.set_fill_style(&color);
    context.set_font(&format!("{} {}", style.font_size, style.font_family));
    let text_metric = context.measure_text(&text).unwrap();
    let text_width = text_metric.width();
    let (x, y) = layout.compute_dim(text_width);
    
    context.fill_text(&text, x, y).unwrap();
}

pub fn food_overlaps_snake(snake_tail: &mut Vec<SnakeCell>, food: Cell) -> bool {

    for snake_cell in snake_tail.iter() {
        if snake_cell.cell.row == food.row && snake_cell.cell.column == food.column {
            return true;
        }
    }

    false
}

pub fn prolong_tail(snake_tail: &mut Vec<SnakeCell>, column_size: f64, row_size: f64) {
    let mut snake_cell = *snake_tail.first_mut().unwrap();
    let cell = &mut snake_cell.cell;

    match snake_cell.direction {
        Direction::Up => {
            cell.row = cell.row + 1;
        },
        Direction::Down => {
            cell.row = cell.row - 1;
        },
        Direction::Left => {
            cell.column = cell.column + 1;
        },
        Direction::Right => {
            cell.column = cell.column - 1; 
        },
    }

    cell.x = cell.column as f64 * column_size;
    cell.y = cell.row as f64 * row_size;

    snake_tail.insert(0, snake_cell);
}

pub fn move_snake_cell(snake_cell: &mut SnakeCell, rows: u32, columns: u32, column_size: f64, row_size: f64, direction: Direction) {
    let mut drow = 1 as i32;
    let mut dcolumn = 0 as i32;
    let cell = &mut snake_cell.cell;

    match snake_cell.direction {
        Direction::Up => {
            drow = -1;
            dcolumn = 0;
        },
        Direction::Down => {
            drow = 1;
            dcolumn = 0;
        },
        Direction::Left => {
            drow = 0;
            dcolumn = -1;
        },
        Direction::Right => {
            drow = 0;
            dcolumn = 1;
        },
    }

    let row = cell.row as i32 + drow;
    let column = cell.column as i32 + dcolumn;
    cell.row = row as u32;
    cell.column = column as u32;
    cell.x = cell.column as f64 * column_size;
    cell.y = cell.row as f64 * row_size;

    if column < 0 {
        cell.x = columns as f64 * column_size;
        cell.column = columns;
    }

    if column >= columns as i32 {
        cell.x = 0_f64;
        cell.column = 0;
    }

    if row < 0 {
        cell.y = rows as f64 * row_size;
        cell.row = rows;
    }

    if row >= rows as i32 {
        cell.y = 0_f64;
        cell.row = 0;
    }

    snake_cell.direction = direction;
}

pub fn move_snake(snake_tail: &mut Vec<SnakeCell>, direction: Direction, rows: u32, columns: u32, column_size: f64, row_size: f64) {
    for index in 0..snake_tail.len() {
        let next_cell = snake_tail.get(index + 1);
        let mut next_direction = direction;

        if next_cell.is_some() {
            next_direction = next_cell.unwrap().direction;
        }

        let mut snake_cell = snake_tail.get_mut(index).unwrap();
        move_snake_cell(&mut snake_cell, rows, columns, column_size, row_size, next_direction);
    }
}