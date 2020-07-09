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
    row: i32,
    column: i32
}


pub struct SnakeCell {
    cell: Cell,
    direction: Direction
}

pub fn generateFood(width: u32, height: u32) -> Cell {
    let randomRow = (Math::random() * width as f64) as i32;
    let randomColumn = (Math::random() * height as f64) as i32;

    Cell {
        x: 0_f64,
        y: 0_f64,
        row: randomRow,
        column: randomColumn
    }
}

pub fn initialize_snake_tail(tail_length: i32, middle_x: f64, middle_y: f64, cell_distance: f64, start_direction: Direction) -> Vec<SnakeCell> {
    let mut vector: Vec<SnakeCell> = Vec::new();

    for i in 0..tail_length {
        vector.push(SnakeCell {
            cell: Cell {
                x: middle_x + f64::from(i) * cell_distance,
                y: middle_y,
                column: 5 + i,
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

    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
    
    canvas.set_width(width);
    canvas.set_height(height);

    let _context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
        
    let context_rc = Rc::new(_context);

    let start_direction = Direction::Right;
    let cell_size = 10_f64;
    let cell_space = 2_f64;
    let cell_distance = cell_size + cell_space;
    let rows = 50;
    let columns = 50;

    let middle_x = (width as f64) / 2_f64;
    let middle_y = (height as f64) / 2_f64;

    let snake_tail_rc = Rc::new(RefCell::new(initialize_snake_tail(4, middle_x, middle_y, cell_distance, start_direction)));

    let mut direction_rc = Rc::new(RefCell::new(start_direction));

    let on_key_down: Closure<dyn FnMut(_)>;

    {
        let context = context_rc.clone();
        let direction = direction_rc.clone();

        on_key_down = Closure::wrap(Box::new(move |event: KeyboardEvent| {

            let key = event.key();
            let mut borrowed = *direction.borrow_mut();

            if borrowed == Direction::Left {
                
            }
    
            if key == "a" || key == "ArrowLeft" {
                borrowed = Direction::Left;     
            }
            
            if key == "w" || key == "ArrowUp" {
                borrowed = Direction::Up;
            }
    
            if key == "s" || key == "ArrowDown" {
                borrowed = Direction::Down;         
            }
    
            if key == "d" || key == "ArrowRight" {
                borrowed = Direction::Right;
            }
    
        }) as Box<dyn FnMut(_)>);
    }

    

    let func_ref = Rc::new(RefCell::new(None));
    let copied_func_ref = func_ref.clone();
    let on_animation_frame: Closure<dyn FnMut()>;
    

    //let mut snake_tail = initialize_snake_tail(4, middle_x, middle_y, cell_distance, start_direction).to_owned();
    {
        let context = context_rc.clone();
        let direction = direction_rc.clone();
        let fps_interval = 1000_f64 / 30_f64;
        let mut then = js_sys::Date::now();
        let mut food = generateFood(width, height);
        //let mut snake_tail = *snake_tail_rc.borrow_mut();
        //let mut snake_tail = snake_tail_rc.clone().borrow_mut();

        let mut snake_tail = vec![
            SnakeCell {
                cell: Cell {
                    x: middle_x + f64::from(0) * cell_distance,
                    y: middle_y,
                    column: 5 + 0,
                    row: 5,
                },
                direction: start_direction
            },
            SnakeCell {
                cell: Cell {
                    x: middle_x + f64::from(1) * cell_distance,
                    y: middle_y,
                    column: 5 + 1,
                    row: 5,
                },
                direction: start_direction
            },
        ];

        snake_tail.get_mut(1).unwrap().cell.y = 200_f64;

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
                context.fill_rect(cell.x, cell.y, cell_size, cell_size);
            }

            context.fill_rect(food.x, food.y, cell_size, cell_size);

            style = JsValue::from("white");
            context.set_fill_style(&style);
            context.set_font("100px Arial");
            context.fill_text("Snake", (width as f64) / 2_f64, 200_f64).unwrap();

            let snake_head = snake_tail.last().unwrap().cell;

            if snake_head.x == food.x && snake_head.y == food.y {

            }
            
            for index in 0..snake_tail.len() {
                let next_cell = snake_tail.get(index + 1);
                let mut next_direction = *direction.borrow();
    
                if next_cell.is_some() {
                    next_direction = next_cell.unwrap().direction;
                }
    
                let mut snake_cell = snake_tail.get_mut(index).unwrap();
                let mut cell = snake_cell.cell;
                let mut dx = 1;
                let mut dy = 0;
                let mut drow = 1;
                let mut dcolumn = 0;
                let cell_distance_int = cell_distance as i32;

                match snake_cell.direction {
                    Direction::Up => {
                        dx = 0;
                        dy = -1 * cell_distance_int;
                        drow = 0;
                        dcolumn = -1 * cell_distance_int;
                    },
                    Direction::Down => {
                        dx = 0;
                        dy = 1 * cell_distance_int;
                        drow = 0;
                        dcolumn = 1 * cell_distance_int;
                    },
                    Direction::Left => {
                        dx = -1 * cell_distance_int;
                        dy = 0;
                        drow = -1 * cell_distance_int;
                        dcolumn = 0;
                    },
                    Direction::Right => {
                        dx = 1 * cell_distance_int;
                        dy = 0;
                        drow = 1 * cell_distance_int;
                        dcolumn = 0;
                    },
                }
                
                cell.row = cell.row + drow;
                cell.column = cell.column + dcolumn;
                cell.x = cell.x + dx as f64;
                cell.y = cell.y + dy as f64;
                //log(&format!("{}, {}", cell.x, cell.y));
                if cell.x < 0_f64 {
                    cell.x = width as f64;
                }

                if cell.x > width as f64 {
                    cell.x = 0_f64;
                }

                if cell.y < 0_f64 {
                    cell.y = height as f64;
                }

                if cell.y > height as f64 {
                    cell.y = 0_f64;
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
