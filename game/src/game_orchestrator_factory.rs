use std::rc::Rc;
use std::cell::RefCell;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlCanvasElement, WebGl2RenderingContext, Window};

use crate::abstractions::frame_scheduler::{WasmClosureWrapper, WebFrameScheduler};
use crate::abstractions::{GreedyBfsAi, InvokeJs, WebGl2Renderer};
use crate::game_orchestrator::{GameOrchestrator, WasmGameOrchestrator};
use crate::models::GameOptions;
use crate::randomizer::JsRandomizer;

pub trait GameOrchestratorFactory<T: InvokeJs> {
    fn create(
        options: GameOptions,
        on_score: T,
        on_game_over: T
    ) -> Rc<RefCell<WasmGameOrchestrator<T>>>;
}
pub struct WasmGameOrchestratorFactory;

impl<T: InvokeJs> GameOrchestratorFactory<T> for WasmGameOrchestratorFactory {
    fn create(
        options: GameOptions,
        on_score: T,
        on_game_over: T
    ) -> Rc<RefCell<WasmGameOrchestrator<T>>> {

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id(&options.id).unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let context = canvas
        .get_context("webgl2").unwrap().unwrap()
        .dyn_into::<WebGl2RenderingContext>().unwrap();

    let randomizer = JsRandomizer;
    let frame_scheduler = WebFrameScheduler::new(window.clone());
    let renderer = WebGl2Renderer::new(context.clone());
    let ai_controller = GreedyBfsAi::new();
    let orchestrator=  GameOrchestrator::new(
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

        Rc::new(RefCell::new(orchestrator))
    }
}
