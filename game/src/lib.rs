#![allow(static_mut_refs)]

use std::{panic, sync::{Arc, Mutex}};
use abstractions::{frame_scheduler::WebFrameScheduler, WebGl2Renderer};
use game_orchestrator::{GameOrchestrator, WasmGameOrchestrator};
use js_sys::Function;
use logger::WasmLogger;
use models::GameOptions;
use randomizer::JsRandomizer;
use utils::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGl2RenderingContext};

mod utils;
mod constants;
mod models;
mod macros;
mod game;
mod food;
mod snake;
mod randomizer;
mod abstractions;
mod game_orchestrator;
mod logger;

static mut GAME_ORCHESTRATOR: Option<Arc<Mutex<WasmGameOrchestrator>>> = None;

#[wasm_bindgen]
pub unsafe fn setup(options: JsValue,
    on_score: Function,
    on_game_over: Function) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
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
    let logger = WasmLogger::new();
    let mut game_orchestrator=  WasmGameOrchestrator::new(
        options,
        logger,
        canvas,
        document,
        window,
        frame_scheduler,
        renderer,
        randomizer,
        on_score,
        on_game_over);
        
    game_orchestrator.initialize();
    game_orchestrator.resize();

    let shared_game_orchestrator = Arc::new(Mutex::new(game_orchestrator));
    GAME_ORCHESTRATOR = Some(shared_game_orchestrator.clone());

    WasmGameOrchestrator::setup_on_resize(&shared_game_orchestrator);
    WasmGameOrchestrator::setup_key_bindings(&shared_game_orchestrator);

    setup_webgl(&context);

    let version = context.get_parameter(WebGl2RenderingContext::VERSION).unwrap();
    let max_texture_size = context.get_parameter(WebGl2RenderingContext::MAX_TEXTURE_SIZE).unwrap();
    console_log!("Version: {}", version.as_string().unwrap());
    console_log!("Max texture size: {}", max_texture_size.as_f64().unwrap());

    Ok(())
}

#[wasm_bindgen(js_name = "applyOptions")]
pub unsafe fn apply_options(options: JsValue) -> Result<(), JsValue> {
    let options: GameOptions = serde_wasm_bindgen::from_value(options).unwrap();

    if let Some(game_orchestrator) = &GAME_ORCHESTRATOR {

        {
            let mut game_orchestrator = game_orchestrator.lock().unwrap();

            game_orchestrator.apply_options_and_reset(options);
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub unsafe fn play(#[wasm_bindgen(js_name = "isAiPlaying")]is_ai_playing: bool) -> Result<(), JsValue> {
    console_log!("wasm play");

    if let Some(game_orchestrator) = &GAME_ORCHESTRATOR {
        if let Ok(mut orchestrator) = game_orchestrator.lock() {

            if orchestrator.is_game_over() {
                console_log!("orchestrator.is_game_over()");
                orchestrator.reset();
            }

            if orchestrator.is_playing() {
                console_log!("orchestrator.is_playing");
                orchestrator.stop();
                orchestrator.reset();
            }

            drop(orchestrator);

            GameOrchestrator::start_game_loop(game_orchestrator, is_ai_playing);
        }
        else {
            console_log!("play could not lock")
        }

        if game_orchestrator.is_poisoned() {
            console_log!("play is_poisoned")
        }
    }


    Ok(())
}

#[wasm_bindgen]
pub unsafe fn stop() -> Result<(), JsValue> {

    if let Some(game_orchestrator) = &GAME_ORCHESTRATOR {
        if let Ok(mut orchestrator) = game_orchestrator.lock() {
            orchestrator.stop();
        }

        if game_orchestrator.is_poisoned() {
            console_log!("stop is_poisoned")
        }
    }

    Ok(())
}