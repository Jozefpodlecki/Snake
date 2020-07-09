extern crate web_sys;
extern crate wasm_bindgen;
extern crate js_sys;

use core::cell::RefMut;
use core::fmt::Display;
use core::fmt::Formatter;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::window;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;
use web_sys::KeyEvent;
use web_sys::KeyboardEvent;
use js_sys::Math;
use js_sys::Date;
use web_sys::CanvasRenderingContext2d;
use crate::wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;


fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn onKeydown(event: MouseEvent) {

}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
pub struct Cell {
    x: f64,
    y: f64,
    row: u32,
    column: u32
}


pub struct SnakeCell {
    cell: Cell,
    direction: Direction
}

pub fn generateFood(rows: u32, columns: u32, rowChunk: f64, columnChunk: f64) -> Cell {
    let randomRow = (Math::random() * rows as f64) as u32;
    let randomColumn = (Math::random() * columns as f64) as u32;
    log(&format!("{} {} {} {} {} {}", randomRow, randomColumn, randomRow as f64 * rowChunk, randomColumn as f64 * columnChunk, rowChunk, columnChunk));
    let cell = Cell {
        x: randomColumn as f64 * columnChunk,
        y: randomRow as f64 * rowChunk,
        row: randomRow,
        column: randomColumn
    };
    
    cell
}

pub fn initialize_snake_tail(tail_length: i32, row: f64, column: f64, rowChunk: f64, columnChunk: f64, start_direction: Direction) -> Vec<SnakeCell> {
    let mut vector: Vec<SnakeCell> = Vec::new();

    for i in 0..tail_length {
        vector.push(SnakeCell {
            cell: Cell {
                x: (column + f64::from(i)) * columnChunk,
                y: row * rowChunk,
                column: 5 + i as u32,
                row: 5,
            },
            direction: start_direction
        })
    }

    vector
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    
    let canvas = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let width = window.inner_width().unwrap().as_f64().unwrap() as u32 -1;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32 -1;
    
    canvas.set_width(width);
    canvas.set_height(height);

    let _context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
        
    let context_rc = Rc::new(_context);

    let start_direction = Direction::Right;
    let rows = 50;
    let columns = 50;
    let columnChunk = width as f64 / columns as f64;
    let rowChunk = height as f64 / rows as f64;
    let middleRow = rows as f64 / 2_f64;
    let middleColumn = columns as f64 / 2_f64;

    let middle_x = (width as f64) / 2_f64;
    let middle_y = (height as f64) / 2_f64;

    let mut snake_tail = initialize_snake_tail(4, middleRow, middleColumn, rowChunk, columnChunk, start_direction);

    let mut direction_rc = Rc::new(RefCell::new(start_direction));

    let on_key_down: Closure<dyn FnMut(_)>;

    {
        let context = context_rc.clone();
        let direction = direction_rc.clone();
        on_key_down = Closure::wrap(Box::new(move |event: KeyboardEvent| {

            let key = event.key();

            if *direction.borrow_mut() == Direction::Left {
                
            }
    
            if key == "a" || key == "ArrowLeft" {
                *direction.borrow_mut() = Direction::Left;     
            }
            
            if key == "w" || key == "ArrowUp" {
                *direction.borrow_mut() = Direction::Up;
            }
    
            if key == "s" || key == "ArrowDown" {
                *direction.borrow_mut() = Direction::Down;         
            }
    
            if key == "d" || key == "ArrowRight" {
                *direction.borrow_mut() = Direction::Right;
            }
    
        }) as Box<dyn FnMut(_)>);
    }

    let func_ref = Rc::new(RefCell::new(None));
    let copied_func_ref = func_ref.clone();
    let on_animation_frame: Closure<FnMut()>;

    {
        let context = context_rc.clone();
        let direction = direction_rc.clone();
        let fps_interval = 1000_f64 / 20_f64;
        let mut then = js_sys::Date::now();
        let mut food = generateFood(rows, columns, rowChunk, columnChunk);
        on_animation_frame = Closure::wrap(Box::new(move || {
            //func_ref.borrow_mut().take();

            let now = js_sys::Date::now();
            let elapsed = now - then;
            if elapsed < fps_interval {
                
                request_animation_frame(func_ref.borrow().as_ref().unwrap());
                return;
            }

            then = now - (elapsed % fps_interval);
            let mut style = JsValue::from("black");
            context.set_fill_style(&style);
            context.fill_rect(0_f64, 0_f64, f64::from(width), f64::from(height));
    
            style = JsValue::from("white");
            context.set_fill_style(&style);
            for snake_cell in snake_tail.iter() {
                let cell = snake_cell.cell;
                context.fill_rect(cell.x, cell.y, columnChunk, rowChunk);
            }

            

            context.fill_rect(food.x, food.y, columnChunk, rowChunk);

            style = JsValue::from("white");
            context.set_fill_style(&style);
            context.set_font("100px Arial");
            context.fill_text("Snake", (width as f64) / 2_f64, 200_f64).unwrap();

            let snake_head = snake_tail.last().unwrap().cell;

            if snake_head.row == food.row && snake_head.column == food.column {
                food = generateFood(rows, columns, rowChunk, columnChunk);
                
            }

            log(&format!(" Snake x {} y {} row {} column {} Food {} {}", snake_head.row, snake_head.column, food.row, food.column));

            for index in 0..snake_tail.len() {
                let next_cell = snake_tail.get(index + 1);
                let mut next_direction = *direction.borrow();
    
                if next_cell.is_some() {
                    next_direction = next_cell.unwrap().direction;
                }
    
                let mut snake_cell = snake_tail.get_mut(index).unwrap();
                let mut cell = &mut snake_cell.cell;
                let mut dx = 1_f64;
                let mut dy = 0_f64;
                let mut drow = 1 as i32;
                let mut dcolumn = 0 as i32;
                
                match snake_cell.direction {
                    Direction::Up => {
                        dx = 0_f64;
                        dy = -1_f64 * rowChunk;
                        drow = 0;
                        dcolumn = -1;
                    },
                    Direction::Down => {
                        dx = 0_f64;
                        dy = 1_f64 * rowChunk;
                        drow = 0;
                        dcolumn = 1;
                    },
                    Direction::Left => {
                        dx = -1_f64 * columnChunk;
                        dy = 0_f64;
                        drow = -1;
                        dcolumn = 0;
                    },
                    Direction::Right => {
                        dx = 1_f64 * columnChunk;
                        dy = 0_f64;
                        drow = 1;
                        dcolumn = 0;
                    },
                }

                let row = cell.row as i32 + drow;
                let column = cell.column as i32 + dcolumn;
                let x = cell.x + dx as f64;
                let y = cell.y + dy as f64;
                cell.row = row as u32;
                cell.column = column as u32;
                cell.x = x;
                cell.y = y;

                if x < 0_f64 {
                    cell.x = (columns - 1) as f64 * columnChunk;
                    cell.column = columns;
                }

                if x > width as f64 {
                    cell.x = 0_f64;
                    cell.column = 0;
                }

                if y < 0_f64 {
                    cell.y = (rows - 1) as f64 * rowChunk;
                    cell.row = rows;
                }

                if y > height as f64 {
                    cell.y = 0_f64;
                    cell.row = 0;
                }

                snake_cell.direction = next_direction;
            }

            request_animation_frame(func_ref.borrow().as_ref().unwrap());
            
        }) as Box<dyn FnMut()>);
        
    }
    
    *copied_func_ref.borrow_mut() = Some(on_animation_frame);

    request_animation_frame(copied_func_ref.borrow().as_ref().unwrap());
    window.add_event_listener_with_callback("keydown", on_key_down.as_ref().unchecked_ref());
    on_key_down.forget();

    Ok(())
}
