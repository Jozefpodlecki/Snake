use std::{cell::RefCell, rc::Rc, sync::{Arc, LazyLock, Mutex}, task::Context};

use js_sys::{Float32Array, Function};
use models::{Direction, Food, GameContext, Snake};
use options::Options;
use utils::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGl2RenderingContext, Window};

mod utils;
mod constants;
mod options;
mod models;
mod macros;

static mut GAME_CONTEXT: GameContext = GameContext {
    score: 0,
    current_direction: Direction::Right,
    height: 0.0,
    width: 0.0,
    grid_size: 30,
    can_run: true,
    position_buffer: None
};

#[wasm_bindgen]
pub unsafe fn run(options: JsValue,
    on_score: Function,
    on_game_over: Function) -> Result<(), JsValue> {
    let options: Options = serde_wasm_bindgen::from_value(options).unwrap();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id(&options.id).unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let snake = Rc::new(RefCell::new(Snake::new(Direction::Right, GAME_CONTEXT.grid_size)));

    let context = canvas
        .get_context("webgl2").unwrap().unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    on_resize(window.clone(), canvas.clone(), context.clone());

    setup_webgl(&context, &mut GAME_CONTEXT);

    let version = context.get_parameter(WebGl2RenderingContext::VERSION).unwrap();
    // let max_element_index = context.get_parameter(WebGlRenderingContext::MAX);
    let max_texture_size = context.get_parameter(WebGl2RenderingContext::MAX_TEXTURE_SIZE).unwrap();
    // console_log!("Max element index: {}", max_element_index.as_f64().unwrap());
    console_log!("Version: {}", version.as_string().unwrap());
    console_log!("Max texture size: {}", max_texture_size.as_f64().unwrap());
  
    setup_on_resize(window.clone(), canvas, context.clone());
    setup_key_bindings(document, snake.clone());
    setup_request_animation_frame(
        window,
        context,
        Rc::new(options),
        snake,
        on_score,
        on_game_over);

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
    context: WebGl2RenderingContext,
    options: Rc<Options>,
    snake: Rc<RefCell<Snake>>,
    on_score: Function,
    on_game_over: Function) {
    let closure: Sharedf64Closure = Rc::new(RefCell::new(None));

    let closure_mut = closure.clone();
    let window_inner = window.clone();
    let closure_inner = closure.clone();
    let last_timestamp = Rc::new(RefCell::new(0.0));
    // let mut food = Food::new(GAME_CONTEXT.grid_size);
    let mut foods: Vec<Food> = (0..3).map(|_| Food::new(GAME_CONTEXT.grid_size)).collect();

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
    
        let mut snake = snake.borrow_mut();

        snake.traverse();

        if snake.is_self_collision() {
            snake.reset();
            on_game_over.call0(&JsValue::null()).unwrap();
            // let empty_data = Float32Array::view(&vec![]);
            // context.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &empty_data, WebGlRenderingContext::STATIC_DRAW);

            // context.delete_buffer(GAME_CONTEXT.position_buffer.as_ref());
            // let new_buffer = context.create_buffer();
            // GAME_CONTEXT.position_buffer = new_buffer.clone();
            // context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, new_buffer.as_ref());
        }

        let mut score_updated = false;
        let mut eaten_foods = Vec::new();
        
        for (index, food) in foods.iter_mut().enumerate() {
            if snake.overlaps(&food) {
                snake.grow();
                food.respawn();
                eaten_foods.push(index);
                score_updated = true;
            }
        }

        for index in eaten_foods.iter().rev() {
            foods.remove(*index);
        }

        while foods.len() < 3 {
            foods.push(Food::new(GAME_CONTEXT.grid_size));
        }

        if score_updated {
            on_score.call0(&JsValue::null()).unwrap();
        }

        // context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        snake.draw(&context);

        for food in &foods {
            food.draw(&context);
        }

        request_animation_frame(&window_inner, &closure_inner);

    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(&window, &closure);   
}