use std::sync::{Arc, Mutex};

use js_sys::Function;
use log::debug;
use web_sys::{Document, HtmlCanvasElement, Window};

use crate::{abstractions::{frame_scheduler::{WasmClosureWrapper, WebFrameScheduler}, *}, game::Game, models::{GameOptions, GameResult, GameState, VerticePayload}, randomizer::{JsRandomizer, Randomizer}, utils::create_key_direction_map};

pub type WasmGameOrchestrator = GameOrchestrator<
    HtmlCanvasElement,
    Document,
    Window,
    WasmClosureWrapper,
    Function,
    JsRandomizer,
    WebGl2Renderer,
    WebFrameScheduler,
    GBFSAiController>;

pub struct GameOrchestrator <C, D, W, CW, T, R, RE, FS, A>
where
    C: CanvasProvider + 'static,
    D: DocumentProvider + 'static,
    W: WindowProvider + 'static,
    CW: ClosureWrapper + 'static,
    T: InvokeJs + 'static,
    R: Randomizer + 'static,
    RE: Renderer + 'static,
    FS: FrameScheduler<CW> + 'static,
    A: AiController + 'static
{
    options: GameOptions,
    state: GameState,
    canvas_provider: C,
    document_provider: D,
    window_provider: W,
    game: Game<R>,
    renderer: RE,
    frame_scheduler: FS,
    on_score: T,
    on_game_over: T,
    callback: Option<CW>,
    callback_handle: ClosureHandle,
    last_timestamp: f64,
    ai_controller: A
}

impl<C, D, W, CW, T, R, RE, FS, A> GameOrchestrator<C, D, W, CW, T, R, RE, FS, A>
where
    C: CanvasProvider + 'static,
    D: DocumentProvider,
    W: WindowProvider + 'static,
    CW: ClosureWrapper + 'static,
    T: InvokeJs + 'static,
    R: Randomizer + 'static,
    RE: Renderer + 'static,
    FS: FrameScheduler<CW> + 'static,
    A: AiController + 'static {
    pub fn new(
        options: GameOptions,
        canvas_provider: C,
        document_provider: D,
        window_provider: W,
        frame_scheduler: FS,
        renderer: RE,
        randomizer: R,
        ai_controller: A,
        on_score: T,
        on_game_over: T) -> Self {
        let game = Game::new(options.clone(), randomizer);

        GameOrchestrator {
            options,
            state: GameState::Idle,
            game,
            canvas_provider,
            document_provider,
            window_provider,
            frame_scheduler,
            renderer,
            on_score,
            on_game_over,
            last_timestamp: 0.0,
            callback: None,
            callback_handle: 0,
            ai_controller
        }
    }

    pub fn initialize(&mut self) {
        self.game.initialize();
    }

    pub fn resize(&mut self) {
        let width = self.window_provider.get_inner_width() as u32;
        let height = self.window_provider.get_inner_height() as u32;
        
        self.canvas_provider.set_size(width, height);
        self.renderer.set_viewport(width as i32, height as i32);
    }

    pub fn is_game_over(&self) -> bool {
        self.state == GameState::GameOver
    }

    pub fn is_playing(&self) -> bool {
        self.state == GameState::UserPlaying || self.state == GameState::AiPlaying
    }

    pub fn stop(&mut self) {
        debug!("stop");
        self.state = GameState::Paused;
        self.frame_scheduler.cancel(self.callback_handle);
    }

    pub fn reset(&mut self) {
        debug!("reset");
        self.game.reset();
    }

    pub fn start_game_loop(game_orchestrator: &Arc<Mutex<Self>>, is_ai_playing: bool) {

        let callback: Box<dyn FnMut(f64) + 'static> = {
            let game_orchestrator = game_orchestrator.clone();

            Box::new(move |timestamp: f64| {
                debug!("request_frame");

                if let Ok(mut orchestrator) = game_orchestrator.lock() {

                    if orchestrator.state == GameState::GameOver
                        || orchestrator.state == GameState::Paused {
                        debug!("game over exit loop");
                        return;
                    }
    
                    let diff = timestamp - orchestrator.last_timestamp;
    
                    if diff < orchestrator.options.frame_threshold_ms {
                        let callback = orchestrator.callback.as_ref().unwrap();
                        orchestrator.frame_scheduler.request_frame_after(callback, diff as i32);
                        return;
                    }
    
                    orchestrator.last_timestamp = timestamp;
    
                    orchestrator.on_game_loop();
                    
                    let callback = orchestrator.callback.as_ref().unwrap();
                    orchestrator.frame_scheduler.request_frame(callback);
                }
                else {
                    debug!("on_frame could not lock")
                }

                if game_orchestrator.is_poisoned() {
                    debug!("on_frame is_poisoned")
                }
            })
        };

        if let Ok(mut orchestrator) = game_orchestrator.lock() {
            let closure_wrapper = CW::new(callback);
            orchestrator.callback = Some(closure_wrapper.clone());
            let callback_handle = orchestrator.frame_scheduler.request_frame(&closure_wrapper);
            orchestrator.callback_handle = callback_handle;

            if is_ai_playing {
                orchestrator.state = GameState::AiPlaying;
            }
            else {
                orchestrator.state = GameState::UserPlaying;
            }
        }
        else {
            debug!("start_game_loop could not lock")
        }

        if game_orchestrator.is_poisoned() {
            debug!("start_game_loop is_poisoned")
        }

    }

    fn on_game_loop(&mut self) {
    
        let mut direction = self.game.direction;

        if let GameState::AiPlaying = self.state {
            let ai_direction = self.ai_controller.get_direction(
                &self.game.snake,
                &self.game.foods,
                &self.game.obstacles,
                self.options.grid_size);

            if let Some(ai_direction) = ai_direction {
                debug!("direction from ai {:?}", ai_direction);
                direction = ai_direction;
            }
        }

        let game_result = self.game.update(direction);
    
        let vertices = self.game.get_vertices();
        let length = vertices.len();
        let payload = VerticePayload {
            data: vertices,
            length: length,
            vertice_size: length as i32 / 6
        };

        self.renderer.draw(&payload);

        match game_result {
            GameResult::Noop => {

            },
            GameResult::Score => {
                if let GameState::UserPlaying = self.state {
                    self.on_score.invoke();
                }
            },
            GameResult::Over => {
                if let GameState::AiPlaying = self.state {
                    self.reset();
                }
                else {
                    self.on_game_over.invoke();
                    self.state = GameState::GameOver;
                }
            },
        }

    }

    pub fn apply_options_and_reset(&mut self, options: GameOptions) {
        self.options = options;
        self.game.apply_options_and_reset(self.options.clone());
    }

    pub fn setup_on_resize(game_orchestrator: &Arc<Mutex<Self>>) {
     
        let handler: Box<dyn FnMut() + 'static> = {
            let game_orchestrator = game_orchestrator.clone();
            Box::new(move || {
                game_orchestrator.lock().unwrap().resize();
            })
        };
    
        {
            let game_orchestrator = game_orchestrator.lock().unwrap();
            game_orchestrator.window_provider.on_resize(handler);
        }
    }
    
    pub fn setup_key_bindings(game_orchestrator: &Arc<Mutex<Self>>) {

        let key_direction_map = create_key_direction_map();
    
        let handler: Box<dyn FnMut(String) + 'static> = {
            let game_orchestrator = game_orchestrator.clone();
            Box::new(move |key: String| {
                let mut game_orchestrator = game_orchestrator.lock().unwrap();

                if game_orchestrator.state != GameState::UserPlaying {
                    return;
                }
        
                if let Some(direction) = key_direction_map.get(&key) {
                    game_orchestrator.game.change_direction(*direction);
                }
            })
        };
    
        {
            let game_orchestrator = game_orchestrator.lock().unwrap();
            game_orchestrator.document_provider.on_key_down(handler);
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::models::{Direction, GameOptions};
//     use crate::randomizer::MockRandomizer;
//     use crate::abstractions::frame_scheduler::MockFrameScheduler;
//     use crate::abstractions::renderer::MockRenderer;
//     use std::sync::{Arc, Mutex};

//     use mockall::{mock, predicate::*};

//     #[test]
//     fn test_initialization() {
//         let options = GameOptions::default();
//         let canvas = MockCanvasProvider::new();
//         let document = MockDocumentProvider::new();
//         let window = MockWindowProvider::new();
//         let scheduler = MockFrameScheduler::new();
//         let renderer = MockRenderer::new();
//         let randomizer = MockRandomizer::new();
//         let ai_controller = MockAiController::new();
//         let on_score = MockJsInvoker::new();
//         let on_game_over = MockJsInvoker::new();
        
//         let orchestrator = GameOrchestrator::new(
//             options,
//             canvas,
//             document,
//             window,
//             scheduler,
//             renderer,
//             randomizer,
//             ai_controller,
//             on_score,
//             on_game_over,
//         );
        
//         assert_eq!(orchestrator.state, GameState::Idle);
//     }

//     #[test]
//     fn test_state_transitions() {
//         let mut orchestrator = setup_orchestrator();
//         assert!(!orchestrator.is_playing());
        
//         orchestrator.state = GameState::UserPlaying;
//         assert!(orchestrator.is_playing());
        
//         orchestrator.state = GameState::GameOver;
//         assert!(orchestrator.is_game_over());
//     }

//     #[test]
//     fn test_ai_moves_snake() {
//         let mut orchestrator = setup_orchestrator();
//         orchestrator.state = GameState::AiPlaying;
        
//         orchestrator.on_game_loop();
        
//         // Check if the game received a direction from AI
//         assert_ne!(orchestrator.game.direction, Direction::default());
//     }

//     #[test]
//     fn test_game_reset() {
//         let mut orchestrator = setup_orchestrator();
//         orchestrator.state = GameState::GameOver;
//         orchestrator.reset();
        
//         assert_eq!(orchestrator.state, GameState::Idle);
//     }

//     #[test]
//     fn test_game_over_triggers_callback() {
//         let mut orchestrator = setup_orchestrator();
//         orchestrator.state = GameState::UserPlaying;
        
//         orchestrator.game.result = GameResult::Over;
//         orchestrator.on_game_loop();
        
//         assert_eq!(orchestrator.state, GameState::GameOver);
//     }

//     #[test]
//     fn test_setup_key_bindings() {
//         let orchestrator = Arc::new(Mutex::new(setup_orchestrator()));
//         GameOrchestrator::setup_key_bindings(&orchestrator);
        
//         // Simulate key press event and check if direction changes
//         let mut orchestrator = orchestrator.lock().unwrap();
//         orchestrator.game.change_direction(Direction::Up);
//         assert_eq!(orchestrator.game.direction, Direction::Up);
//     }

//     fn setup_orchestrator() -> GameOrchestrator<
//         MockCanvasProvider,
//         MockDocumentProvider,
//         MockWindowProvider,
//         MockClosureWrapper,
//         MockJsInvoker,
//         MockRandomizer,
//         MockRenderer,
//         MockFrameScheduler,
//         MockAiController,
//     > {
//         let options = GameOptions::default();
//         let canvas = MockCanvasProvider::new();
//         let document = MockDocumentProvider::new();
//         let window = MockWindowProvider::new();
//         let scheduler = MockFrameScheduler::new();
//         let renderer = MockRenderer::new();
//         let randomizer = MockRandomizer::new();
//         let ai_controller = MockAiController::new();
//         let on_score = MockJsInvoker::new();
//         let on_game_over = MockJsInvoker::new();

//         GameOrchestrator::new(
//             options,
//             canvas,
//             document,
//             window,
//             scheduler,
//             renderer,
//             randomizer,
//             ai_controller,
//             on_score,
//             on_game_over,
//         )
//     }

// }
