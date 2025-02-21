use std::sync::{Arc, Mutex};

use js_sys::Function;
use web_sys::{Document, HtmlCanvasElement, Window};

use crate::{abstractions::{frame_scheduler::{WasmClosureWrapper, WebFrameScheduler}, *}, console_log, game::{Game, InvokeJs}, models::{GameOptions, GameState, VerticePayload}, randomizer::{JsRandomizer, Randomizer}, utils::create_key_direction_map};

pub type WasmGameOrchestrator = GameOrchestrator<HtmlCanvasElement, Document, Window, WasmClosureWrapper, Function, JsRandomizer, WebGl2Renderer, WebFrameScheduler>;

pub struct GameOrchestrator <C, D, W, CW, T, R, RE, FS>
where
    C: CanvasProvider + 'static,
    D: DocumentProvider + 'static,
    W: WindowProvider + 'static,
    CW: ClosureWrapper + 'static,
    T: InvokeJs + 'static,
    R: Randomizer + 'static,
    RE: Renderer + 'static,
    FS: FrameScheduler<CW> + 'static
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
    last_timestamp: f64,
    callback: Option<CW>,
    callback_handle: ClosureHandle
}

impl<C, D, W, CW, T, R, RE, FS> GameOrchestrator <C, D, W, CW, T, R, RE, FS>
where
    C: CanvasProvider + 'static,
    D: DocumentProvider,
    W: WindowProvider + 'static,
    CW: ClosureWrapper + 'static,
    T: InvokeJs + 'static,
    R: Randomizer + 'static,
    RE: Renderer + 'static,
    FS: FrameScheduler<CW> + 'static
 {
    pub fn new(
        options: GameOptions,
        canvas_provider: C,
        document_provider: D,
        window_provider: W,
        frame_scheduler: FS,
        renderer: RE,
        randomizer: R,
        on_score: T,
        on_game_over: T) -> Self {
        let game = Game::new(options.grid_size, options.food_count, randomizer);

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
            callback_handle: 0
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

    pub fn is_playing(&self) -> bool {
        self.state == GameState::Playing || self.state == GameState::AiPlaying
    }

    pub fn stop(&mut self) {
        console_log!("stop Paused");
        self.state = GameState::Paused;
        self.frame_scheduler.cancel(self.callback_handle);
    }

    pub fn reset(&mut self) {
        self.game.reset();
    }

    pub fn start_game_loop(game_orchestrator: &Arc<Mutex<Self>>, is_ai_playing: bool) {

        let callback: Box<dyn FnMut(f64) + 'static> = {
            let game_orchestrator = game_orchestrator.clone();

            Box::new(move |timestamp: f64| {

                if let Ok(mut orchestrator) = game_orchestrator.lock() {

                    if orchestrator.state == GameState::GameOver
                        || orchestrator.state == GameState::Paused {
                        return;
                    }
    
                    let diff = timestamp - orchestrator.last_timestamp;
    
                    if diff < orchestrator.options.frame_threshold_ms {
                        let callback = orchestrator.callback.as_ref().unwrap();
                        orchestrator.frame_scheduler.request_frame_after(callback, diff as i32);
                        return;
                    }
    
                    orchestrator.last_timestamp = timestamp;
    
                    if !orchestrator.on_game_loop() {
                        return;
                    }
                    
                    let callback = orchestrator.callback.as_ref().unwrap();
                    orchestrator.frame_scheduler.request_frame(callback);
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
                orchestrator.state = GameState::Playing;
            }
        }

    }

    fn on_game_loop(&mut self) -> bool {
    
        if self.game.update() {
            self.on_score.invoke();
        }
    
        let vertices = self.game.get_vertices();
        let length = vertices.len();
        let payload = VerticePayload {
            data: vertices,
            length: length,
            vertice_size: length as i32 / 6
        };
        self.renderer.draw(&payload);

        if self.game.is_over() {
    
            if self.game.is_played_by_ai {
                self.reset();
                self.game.is_played_by_ai = true;
            }
            else {
                self.on_game_over.invoke();
                self.state = GameState::GameOver;
                return false;
            }
        }

        true
    }

    pub fn apply_options_and_reset(&mut self, options: GameOptions) {
        self.options = options;
        self.game.apply_options_and_reset(self.options.grid_size, self.options.food_count);
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

                if game_orchestrator.state != GameState::Playing {
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
