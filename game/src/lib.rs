use std::{cell::RefCell, rc::Rc};

use game::Game;
use js_sys::Function;
use models::GameOptions;
use renderer::Renderer;
use utils::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGl2RenderingContext, Window};

mod utils;
mod constants;
mod models;
mod macros;
mod game;
mod food;
mod snake;
mod renderer;
mod colors;

static mut OPTIONS: Option<Rc<RefCell<GameOptions>>> = None;
static mut GAME: Option<Rc<RefCell<Game<Function>>>> = None;

#[wasm_bindgen]
pub unsafe fn run(options: JsValue,
    on_score: Function,
    on_game_over: Function) -> Result<(), JsValue> {
    let options: GameOptions = serde_wasm_bindgen::from_value(options).unwrap();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id(&options.id).unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let game = Rc::new(RefCell::new(Game::new(
        options.grid_size,
        options.food_count,
        on_score,
        on_game_over)));

    GAME = Some(game.clone());
    OPTIONS = Some(Rc::new(RefCell::new(options)));

    let context = canvas
        .get_context("webgl2").unwrap().unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let renderer = Rc::new(RefCell::new(Renderer::new(context.clone())));

    on_resize(window.clone(), canvas.clone(), context.clone());

    setup_webgl(&context);

    let version = context.get_parameter(WebGl2RenderingContext::VERSION).unwrap();
    let max_texture_size = context.get_parameter(WebGl2RenderingContext::MAX_TEXTURE_SIZE).unwrap();
    console_log!("Version: {}", version.as_string().unwrap());
    console_log!("Max texture size: {}", max_texture_size.as_f64().unwrap());
  
    setup_on_resize(window.clone(), canvas, context.clone());
    setup_key_bindings(document, game.clone());
    start_game_loop(
        window,
        game.clone(),
        renderer);

    game.borrow_mut().can_run = true;

    Ok(())
}

#[wasm_bindgen]
pub unsafe fn apply_options(options: JsValue) -> Result<(), JsValue> {
    let options: GameOptions = serde_wasm_bindgen::from_value(options).unwrap();

    if let Some(game_options) = &OPTIONS {
        let mut game_options = game_options.borrow_mut();
        game_options.fps = options.fps;
        game_options.frame_threshold_ms = options.frame_threshold_ms;
    }

    if let Some(game) = &GAME {
        game.borrow_mut().apply_options_and_reset(options.food_count);
    }

    Ok(())
}

#[wasm_bindgen]
pub unsafe fn play() -> Result<(), JsValue> {

    set_run(true);

    Ok(())
}

#[wasm_bindgen]
pub unsafe fn pause() -> Result<(), JsValue> {

    set_run(false);

    Ok(())
}

unsafe fn set_run(flag: bool) {
    if let Some(game) = &GAME {
        game.borrow_mut().can_run = flag;
    }
}

unsafe fn on_game_loop(
    game: Rc<RefCell<Game<Function>>>,
    renderer: Rc<RefCell<Renderer>>) {
    let mut game = game.borrow_mut();

    game.update_and_notify_ui();

    if game.is_over() {
        game.reset_and_notify_ui();
    }

    let vertices = game.get_vertices();
    let vertices_length = (vertices.length() / 6) as i32;
    renderer.borrow_mut().draw_vertices(&vertices, vertices_length);
}

unsafe fn start_game_loop(
    window: Window,
    game: Rc<RefCell<Game<Function>>>,
    renderer: Rc<RefCell<Renderer>>) {
    let closure: Sharedf64Closure = Rc::new(RefCell::new(None));

    let closure_mut = closure.clone();
    let window_inner = window.clone();
    let closure_inner = closure.clone();
    let last_timestamp = Rc::new(RefCell::new(0.0));

    *closure_mut.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {

        if !game.borrow().can_run {
            let timestamp = JsValue::from_f64(performance_now(&window_inner));
            set_timeout_with_param(window_inner.clone(), &closure_inner, 500, timestamp);
            return;
        }

        let mut last_timestamp = last_timestamp.borrow_mut();
        let frame_threshold_ms = OPTIONS.as_ref().unwrap().borrow().frame_threshold_ms;
    
        if timestamp - *last_timestamp < frame_threshold_ms {
            request_animation_frame(&window_inner, &closure_inner);
            return;
        }
    
        *last_timestamp = timestamp;

        on_game_loop(game.clone(), renderer.clone());

        request_animation_frame(&window_inner, &closure_inner);

    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(&window, &closure);
}