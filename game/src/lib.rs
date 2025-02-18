use std::{cell::RefCell, rc::Rc, sync::{Arc, LazyLock, Mutex}, task::Context};

use js_sys::Function;
use models::{Direction, Food, GameContext, Snake};
use options::Options;
use utils::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGlRenderingContext, Window};

mod utils;
mod constants;
mod options;
mod models;

static mut GAME_CONTEXT: GameContext = GameContext {
    score: 0,
    current_direction: Direction::Right,
    height: 0.0,
    width: 0.0,
    grid_size: 30,
    can_run: true
};

#[wasm_bindgen]
pub unsafe fn run(options: JsValue, on_score: Function) -> Result<(), JsValue> {
    let options: Options = serde_wasm_bindgen::from_value(options).unwrap();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id(&options.id).unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let snake = Rc::new(RefCell::new(Snake::new(Direction::Right, GAME_CONTEXT.grid_size)));

    let context = canvas
        .get_context("webgl").unwrap().unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    on_resize(window.clone(), canvas.clone(), context.clone());

    setup_webgl(&context);
  
    setup_on_resize(window.clone(), canvas, context.clone());
    setup_key_bindings(document, snake.clone());
    setup_request_animation_frame(
        window,
        context,
        Rc::new(options),
        snake,
        on_score);

    GAME_CONTEXT.can_run = true;

    Ok(())
}

#[wasm_bindgen]
pub unsafe fn stop() -> Result<(), JsValue> {

    GAME_CONTEXT.can_run = false;

    Ok(())
}

unsafe fn setup_request_animation_frame(
    window: Window,
    context: WebGlRenderingContext,
    options: Rc<Options>,
    snake: Rc<RefCell<Snake>>,
    on_score: Function) {
    let closure: Sharedf64Closure = Rc::new(RefCell::new(None));

    let closure_mut = closure.clone();
    let window_inner = window.clone();
    let closure_inner = closure.clone();
    let last_timestamp = Rc::new(RefCell::new(0.0));
    let mut food = Food::new(GAME_CONTEXT.grid_size);

    *closure_mut.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {

        if !GAME_CONTEXT.can_run {
            return;
        }

        let mut last_timestamp = last_timestamp.borrow_mut();

        if timestamp - *last_timestamp < options.frame_threshold_ms {
            request_animation_frame(&window_inner, &closure_inner);
            return;
        }

        *last_timestamp = timestamp;
    
        snake.borrow_mut().traverse();

        if snake.borrow().overlaps(&food) {
            snake.borrow_mut().grow();
            food.respawn();
            on_score.call0(&JsValue::null()).unwrap();
        }

        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        snake.borrow().draw(&context);
        food.draw(&context);

        request_animation_frame(&window_inner, &closure_inner);

    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(&window, &closure);   
}