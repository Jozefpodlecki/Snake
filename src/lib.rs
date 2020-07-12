extern crate web_sys;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate base64;
use core::cell::RefMut;
use core::fmt::Display;
use core::fmt::Formatter;
use std::cell::RefCell;
use std::rc::Rc;

use web_sys::window;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlImageElement;
use web_sys::MouseEvent;
use web_sys::KeyEvent;
use web_sys::KeyboardEvent;
use web_sys::Event;
use js_sys::Math;
use js_sys::Date;
use web_sys::CanvasRenderingContext2d;
use crate::wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub mod game;
pub mod utils;
pub mod styles;

use crate::game::*;
use utils::*;
use crate::styles::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = window, js_name = requestAnimationFrame)]
    fn request_animation_frame(f: &Closure<dyn FnMut()>);
}

pub fn on_resize_closure(game_context_rc: Rc<RefCell<GameContext>>) -> Closure<dyn FnMut(Event)> {
    Closure::wrap(Box::new(move |event: Event| {

        let mut game_context = game_context_rc.borrow_mut();
        let columns = game_context.columns;
        let rows = game_context.rows;
        let canvas = &mut game_context.canvas;

        let (width, height) = get_window_dim();
        resize_canvas(&canvas, width, height);
        game_context.column_size = width as f64 / columns as f64;
        game_context.row_size = height as f64 / rows as f64;
        game_context.width = width;
        game_context.height = height;
        game_context.columns = ((width as f64 / height as f64) * rows as f64) as u32;
        
    }) as Box<dyn FnMut(Event)>)
}

pub fn on_mouse_click_closure(game_context_rc: Rc<RefCell<GameContext>>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |event: MouseEvent| {

        let game_context = &mut game_context_rc.borrow_mut();

        if game_context.first_time {
            game_context.first_time = false;
        }

        if game_context.has_lost {
            game_context.has_lost = false;
            game_context.score = 0;
        }    
        
    }) as Box<dyn FnMut(MouseEvent)>)
}

pub fn on_key_down_closure(game_context_rc: Rc<RefCell<GameContext>>) -> Closure<dyn FnMut(KeyboardEvent)> {
    let key_direction_map = create_key_direction_map();
    let key_direction_disallowed_map = create_key_direction_disallowed_map();

    Closure::wrap(Box::new(move |event: KeyboardEvent| {

        let key = event.key().to_lowercase();
        let direction = &mut game_context_rc.borrow_mut().direction;

        match key_direction_map.get(&key) {
            Some(&next_direction) => {
                
                if *direction == next_direction || key_direction_disallowed_map[&direction] == next_direction {
                    return;
                }

                *direction = next_direction;
            },
            _ => {
                
            }
        };

    }) as Box<dyn FnMut(_)>)
}

pub fn repaint_screen(context: &CanvasRenderingContext2d, width: u32, height: u32) {
    let style = JsValue::from("black");
    context.set_fill_style(&style);
    context.fill_rect(0_f64, 0_f64, f64::from(width), f64::from(height));
}

pub fn render_signature(game_context: &mut GameContext) {
    let column_size = game_context.column_size;
    let row_size = game_context.row_size;
    let rows = game_context.rows;
    let columns = game_context.columns;

    let signature_style = Style {
        color: "white".to_string(),
        font_size: "30px".to_string(),
        font_family: "Arial".to_string()
    };

    let signature_layout: Layout = Layout {
        column_size,
        row_size,
        row_span: (rows - 5, rows),
        column_span: (0, columns),
        alignment: ("center".to_string(), "center".to_string())
    };

    let date = Date::new_0();
    let text = format!("{} {}", date.get_full_year(), "Józef Podlecki");
    render_text(&mut game_context.context, text, signature_style, signature_layout);

    match &game_context.image {
        Some(image) => {
            game_context.context.draw_image_with_html_image_element(image, 100_f64, 1000_f64).unwrap();
        }
        _ => {}
    };
}

pub fn on_animation_frame_closure(
    game_context_rc: Rc<RefCell<GameContext>>,
    func_ref: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
    middle_row: u32,
    middle_column: u32
) -> Closure<dyn FnMut()> {
        
    let rows = game_context_rc.borrow().rows;
    let columns = game_context_rc.borrow().columns;
    let row_size = game_context_rc.borrow().row_size;
    let column_size = game_context_rc.borrow().column_size;

    let fps_interval = 1000_f64 / 30_f64;
    let mut then = get_current_timestamp();
    let mut food = generateFood(rows, columns, row_size, column_size);

    Closure::wrap(Box::new(move || {
       
        let mut game_context = game_context_rc.borrow_mut();
        let first_time = game_context.first_time;
        let start_direction = game_context.start_direction;
        let direction = game_context.direction;
        let tail_length = game_context.tail_length;
        let has_lost = game_context.has_lost;
        let score = game_context.score;
        let height = game_context.height;
        let width = game_context.width;
        let row_size = game_context.row_size;
        let column_size = game_context.column_size;
        let rows = game_context.rows;
        let columns = game_context.columns;

        let now = get_current_timestamp();
        let elapsed = now - then;
        if elapsed < fps_interval {
            
            request_animation_frame(func_ref.borrow().as_ref().unwrap());
            return;
        }

        then = now - (elapsed % fps_interval);
        repaint_screen(&mut game_context.context, width, height);

        if first_time {
            let title_style = Style {
                color: "white".to_string(),
                font_size: "150px".to_string(),
                font_family: "Arial".to_string()
            };
    
            let title_layout: Layout = Layout {
                column_size,
                row_size,
                row_span: (0, 20),
                column_span: (0, columns),
                alignment: ("center".to_string(), "center".to_string())
            };
    
            render_text(&mut game_context.context, "Snake".to_string(), title_style, title_layout);

            let click_action_style = Style {
                color: "white".to_string(),
                font_size: "60px".to_string(),
                font_family: "Arial".to_string()
            };

            let click_action_layout: Layout = Layout {
                column_size,
                row_size,
                row_span: (0, rows),
                column_span: (0, columns),
                alignment: ("center".to_string(), "center".to_string())
            };
            
            let text = "Click to play".to_string();
            render_text(&mut game_context.context, text, click_action_style, click_action_layout);

            render_signature(&mut game_context);
    
            request_animation_frame(func_ref.borrow().as_ref().unwrap());
            return;
        }

        if has_lost {

            let score_style = Style {
                color: "white".to_string(),
                font_size: "100px".to_string(),
                font_family: "Arial".to_string()
            };

            let score_layout: Layout = Layout {
                column_size,
                row_size,
                row_span: (0, 30),
                column_span: (0, columns),
                alignment: ("center".to_string(), "center".to_string())
            };

            let text = format!("Your score {}", score);
            
            render_text(&mut game_context.context, text, score_style, score_layout);

            let click_action_style = Style {
                color: "white".to_string(),
                font_size: "60px".to_string(),
                font_family: "Arial".to_string()
            };

            let click_action_layout: Layout = Layout {
                column_size,
                row_size,
                row_span: (0, rows),
                column_span: (0, columns),
                alignment: ("center".to_string(), "center".to_string())
            };
            
            let text = "Click to play again".to_string();
            render_text(&mut game_context.context, text, click_action_style, click_action_layout);

            render_signature(&mut game_context);

            request_animation_frame(func_ref.borrow().as_ref().unwrap());
            return;
        }

        render_snake(&mut game_context);
        render_food(&mut game_context.context, food, column_size, row_size);

        render_signature(&mut game_context);

        let score_style = Style {
            color: "white".to_string(),
            font_size: "30px".to_string(),
            font_family: "Arial".to_string()
        };

        let score_layout: Layout = Layout {
            column_size,
            row_size,
            row_span: (0, 5),
            column_span: (columns - 10, columns),
            alignment: ("center".to_string(), "center".to_string())
        };

        let text = format!("Score {}", game_context.score);
        render_text(&mut game_context.context, text, score_style, score_layout);

        let snake_head = game_context.snake_tail.last().unwrap().cell;
        
        if check_if_snake_overlap(&mut game_context.snake_tail, snake_head) {
            game_context.snake_tail = initialize_snake_tail(tail_length, middle_row, middle_column, row_size, column_size, start_direction);
            game_context.has_lost = true;
            request_animation_frame(func_ref.borrow().as_ref().unwrap());
            return;
        }

        if snake_head.row == food.row && snake_head.column == food.column {
            game_context.score = game_context.score + 1;

            food = generateFood(rows, columns, row_size, column_size);
            while food_overlaps_snake(&mut game_context.snake_tail, food) {
                food = generateFood(rows, columns, row_size, column_size);
            } 
            
            prolong_tail(&mut game_context.snake_tail, column_size, row_size);
        }

        move_snake(&mut game_context.snake_tail, direction, rows, columns, column_size, row_size);

        request_animation_frame(func_ref.borrow().as_ref().unwrap());
        
    }) as Box<dyn FnMut()>)  
}

pub fn get_window_dim() -> (u32, u32) {
    let window = window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32 -1;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32 -1;

    (width, height)
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let _window = window().unwrap();
    let document = _window.document().unwrap();
    
    let canvas = document.get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let (width, height) = get_window_dim();
    resize_canvas(&canvas, width, height);

    let context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
        
    let start_direction = Direction::Right;
    let rows = 50;
    let columns = ((width as f64 / height as f64) * rows as f64) as u32;
    let column_size = width as f64 / columns as f64;
    let row_size = height as f64 / rows as f64;
    let middle_row = rows / 2;
    let middle_column = columns / 2;
    let tail_length = 4;

    let snake_tail = initialize_snake_tail(tail_length, middle_row, middle_column, row_size, column_size, start_direction);

    let game_context_rc = Rc::new(RefCell::new(GameContext {
        window: window().unwrap(),
        context,
        canvas,
        width,
        height,
        rows,
        columns,
        column_size,
        row_size,
        has_lost: false,
        first_time: true,
        score: 0,
        direction: start_direction,
        start_direction,
        snake_tail,
        tail_length,
        image: None
    }));

    let on_resize = on_resize_closure(game_context_rc.clone());
    let on_mouse_click = on_mouse_click_closure(game_context_rc.clone());
    let on_key_down = on_key_down_closure(game_context_rc.clone());
    
    let func_ref = Rc::new(RefCell::new(None));
    let copied_func_ref = func_ref.clone();
    let on_animation_frame = on_animation_frame_closure(
        game_context_rc.clone(),
        func_ref,
        middle_row,
        middle_column
    );
    *copied_func_ref.borrow_mut() = Some(on_animation_frame);
    
    let image = Rc::new(HtmlImageElement::new().unwrap());
    //let on_image_load = on_image_load_closure(game_context_rc.clone(), image.clone());

    let game_context = game_context_rc.borrow();
    let window = &game_context.window;

    request_animation_frame(copied_func_ref.borrow().as_ref().unwrap());
    //image.add_event_listener_with_callback("load", on_image_load.as_ref().unchecked_ref());
    window.add_event_listener_with_callback("keydown", on_key_down.as_ref().unchecked_ref());
    window.add_event_listener_with_callback("click", on_mouse_click.as_ref().unchecked_ref());
    window.add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref());
    on_key_down.forget();
    on_mouse_click.forget();
    on_resize.forget();
    //on_image_load.forget();
    
    Ok(())
}

pub fn on_image_load_closure(game_context_rc: Rc<RefCell<GameContext>>, image_rc: Rc<HtmlImageElement>) -> Closure<dyn FnMut(Event)> {
    let github_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M0 0v24h24v-24h-24zm14.534 19.59c-.406.078-.534-.171-.534-.384v-2.195c0-.747-.262-1.233-.55-1.481 1.782-.198 3.654-.875 3.654-3.947 0-.874-.311-1.588-.824-2.147.083-.202.357-1.016-.079-2.117 0 0-.671-.215-2.198.82-.639-.18-1.323-.267-2.003-.271-.68.003-1.364.091-2.003.269-1.528-1.035-2.2-.82-2.2-.82-.434 1.102-.16 1.915-.077 2.118-.512.56-.824 1.273-.824 2.147 0 3.064 1.867 3.751 3.645 3.954-.229.2-.436.552-.508 1.07-.457.204-1.614.557-2.328-.666 0 0-.423-.768-1.227-.825 0 0-.78-.01-.055.487 0 0 .525.246.889 1.17 0 0 .463 1.428 2.688.944v1.489c0 .211-.129.459-.528.385-3.18-1.057-5.472-4.056-5.472-7.59 0-4.419 3.582-8 8-8s8 3.581 8 8c0 3.533-2.289 6.531-5.466 7.59z"/></svg>"#.to_string();   
    let result = window().unwrap().btoa(&github_svg).unwrap();
    let data_url = format!("{}{}", "data:image/svg+xml;base64,", result);
    image_rc.set_src(&data_url);
    
    let closure = Closure::wrap(Box::new(move |event: Event| {
        let mut game_context = game_context_rc.borrow_mut();    
        let image = event
            .target()
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        game_context.context.draw_image_with_html_image_element(&image, 100_f64, 1000_f64).unwrap();
        game_context.image = Some(image);
        
    }) as Box<dyn FnMut(Event)>);

    closure
}