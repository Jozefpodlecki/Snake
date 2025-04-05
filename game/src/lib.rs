#![allow(static_mut_refs)]

use std::rc::Rc;
use std::{cell::RefCell, panic};
use game_orchestrator::{GameOrchestrator, WasmGameOrchestrator};
use game_orchestrator_factory::{GameOrchestratorFactory, WasmGameOrchestratorFactory};
use js_sys::Function;
use log::{debug, info};
use models::GameOptions;
use utils::*;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

mod utils;
mod constants;
mod models;
mod macros;
mod game;
mod randomizer;
mod abstractions;
mod game_orchestrator;
mod game_orchestrator_factory;
mod objects;

static mut GAME_ORCHESTRATOR: Option<Rc<RefCell<WasmGameOrchestrator<Function>>>> = None;

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
pub unsafe fn setup(
    options: JsValue,
    on_score: Function,
    on_game_over: Function) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_log();

    let options: GameOptions = serde_wasm_bindgen::from_value(options).unwrap();

    let game_orchestrator = WasmGameOrchestratorFactory::create(options, on_score, on_game_over);

    {
        let mut orchestrator = game_orchestrator.borrow_mut();
        orchestrator.initialize();
        orchestrator.resize();
    }
    

    GAME_ORCHESTRATOR = Some(game_orchestrator.clone());

    WasmGameOrchestrator::setup_on_resize(game_orchestrator.clone());
    WasmGameOrchestrator::setup_key_bindings(game_orchestrator);

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

#[cfg(test)]
mod tests {
    use crate::abstractions::invoke_js::MockInvokeJsStub;

    use super::*;


    #[test]
    fn should_setup() {
     
    }
}