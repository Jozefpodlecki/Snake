#![allow(static_mut_refs)]

use std::rc::Rc;
use std::{cell::RefCell, panic};
use abstractions::{frame_scheduler::WebFrameScheduler, GBFSAiController, WebGl2Renderer};
use game_orchestrator::{GameOrchestrator, WasmGameOrchestrator};
use js_sys::Function;
use log::{debug, info};
use models::GameOptions;
use randomizer::JsRandomizer;
use utils::*;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGl2RenderingContext};
use crate::models::GameState;

mod utils;
mod constants;
mod models;
mod macros;
mod game;
mod randomizer;
mod abstractions;
mod game_orchestrator;
mod objects;

static mut GAME_ORCHESTRATOR: Option<Rc<RefCell<WasmGameOrchestrator>>> = None;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            #[cfg(debug_assertions)]
            console_log::init_with_level(Level::Debug).unwrap();
            
            #[cfg(not(debug_assertions))]
            console_log::init_with_level(Level::Warn).unwrap();
        }
    } else {
        fn init_log() {}
    }
}

#[wasm_bindgen]
pub unsafe fn setup(options: JsValue,
    on_score: Function,
    on_game_over: Function) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_log();
    let options: GameOptions = serde_wasm_bindgen::from_value(options).unwrap();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id(&options.id).unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2").unwrap().unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let randomizer = JsRandomizer;
    let frame_scheduler = WebFrameScheduler::new(window.clone());
    let renderer = WebGl2Renderer::new(context.clone());
    let ai_controller = GBFSAiController::new();
    let mut game_orchestrator=  WasmGameOrchestrator::new(
        options,
        canvas,
        document,
        window,
        frame_scheduler,
        renderer,
        randomizer,
        ai_controller,
        on_score,
        on_game_over);
        
    game_orchestrator.initialize();
    game_orchestrator.resize();


    let shared_game_orchestrator = Rc::new(RefCell::new(game_orchestrator));
    GAME_ORCHESTRATOR = Some(shared_game_orchestrator.clone());

    WasmGameOrchestrator::setup_on_resize(shared_game_orchestrator.clone());
    WasmGameOrchestrator::setup_key_bindings(shared_game_orchestrator);

    setup_webgl(&context);

    let version = context.get_parameter(WebGl2RenderingContext::VERSION).unwrap();
    let max_texture_size = context.get_parameter(WebGl2RenderingContext::MAX_TEXTURE_SIZE).unwrap();
    info!("Version: {}", version.as_string().unwrap());
    info!("Max texture size: {}", max_texture_size.as_f64().unwrap());

    Ok(())
}

#[wasm_bindgen(js_name = "applyOptions")]
pub unsafe fn apply_options(options: JsValue) -> Result<(), JsValue> {
    let options: GameOptions = serde_wasm_bindgen::from_value(options).unwrap();

    let orchestrator = GAME_ORCHESTRATOR.clone().unwrap();
    let mut orchestrator = orchestrator.borrow_mut();
    orchestrator.apply_options_and_reset(options);

    Ok(())
}

#[wasm_bindgen]
pub unsafe fn play(#[wasm_bindgen(js_name = "isAiPlaying")]is_ai_playing: bool) -> Result<(), JsValue> {
    debug!("play");

    {
        let orchestrator = GAME_ORCHESTRATOR.clone().unwrap();
        let mut orchestrator = orchestrator.borrow_mut();
        orchestrator.play();
    }

    GameOrchestrator::start_game_loop( GAME_ORCHESTRATOR.clone().unwrap(), is_ai_playing);

    Ok(())
}

#[wasm_bindgen]
pub unsafe fn stop() -> Result<(), JsValue> {

    let orchestrator = GAME_ORCHESTRATOR.clone().unwrap();
    let mut orchestrator = orchestrator.borrow_mut();
    orchestrator.stop();

    Ok(())
}