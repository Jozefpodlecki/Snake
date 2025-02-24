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

        if let GameState::AiPlaying = self.state {
            let ai_direction = self.ai_controller.get_direction(
                &self.game.snake,
                &self.game.foods,
                &self.game.obstacles,
                self.options.grid_size);

            if let Some(direction) = ai_direction {
                debug!("direction from ai {:?}", direction);
                self.game.change_direction(direction);
            }
        }

        let game_result = self.game.update();
    
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


#[cfg(test)]
mod tests {
    use crate::abstractions::canvas_provider::MockCanvasProvider;
    use crate::game_orchestrator::GameOrchestrator;
    use crate::models::{Difficulty, Direction, GameOptions, GameResult, GameState};
    use crate::randomizer::{MockRandomizer, Randomizer};
    use crate::abstractions::frame_scheduler::MockFrameScheduler;
    use crate::abstractions::renderer::MockRenderer;
    use crate::game_orchestrator::document_provider::MockDocumentProvider;
    use crate::game_orchestrator::window_provider::MockWindowProvider;
    use crate::game_orchestrator::frame_scheduler::MockClosureWrapper;
    use crate::game_orchestrator::ai_controller::MockAiController;
    use crate::abstractions::invoke_js::MockInvokeJsStub;

    use std::sync::{Arc, Mutex};

    use mockall::{mock, predicate::*};

    type TestGameOrchestrator = GameOrchestrator<
        MockCanvasProvider,
        MockDocumentProvider,
        MockWindowProvider,
        MockClosureWrapper,
        MockInvokeJsStub,
        MockRandomizer,
        MockRenderer,
        MockFrameScheduler,
        MockAiController,
    >;

    struct Dependencies {
        pub mock_canvas_provider: MockCanvasProvider,
        pub mock_document_provider: MockDocumentProvider,
        pub mock_window_rovider: MockWindowProvider,
        pub mock_frame_scheduler: MockFrameScheduler,
        pub mock_renderer: MockRenderer,
        pub mock_randomizer: MockRandomizer,
        pub mock_ai_controller: MockAiController,
        pub mock_on_score: MockInvokeJsStub,
        pub mock_on_game_over: MockInvokeJsStub,
    }

    #[test]
    fn test_initialization() {
        let options = GameOptions::default();
        let canvas = MockCanvasProvider::new();
        let document = MockDocumentProvider::new();
        let window = MockWindowProvider::new();
        let scheduler = MockFrameScheduler::new();
        let renderer = MockRenderer::new();
        let randomizer = MockRandomizer::new();
        let ai_controller = MockAiController::new();
        let on_score = MockInvokeJsStub::new();
        let on_game_over = MockInvokeJsStub::new();
        
        let orchestrator = GameOrchestrator::new(
            options,
            canvas,
            document,
            window,
            scheduler,
            renderer,
            randomizer,
            ai_controller,
            on_score,
            on_game_over,
        );
        
        assert_eq!(orchestrator.state, GameState::Idle);
    }

    #[test]
    fn test_state_transitions() {
        let dependencies = setup_dependencies();
        let mut orchestrator = setup_orchestrator(dependencies);
        assert!(!orchestrator.is_playing());
        
        orchestrator.state = GameState::UserPlaying;
        assert!(orchestrator.is_playing());
        
        orchestrator.state = GameState::GameOver;
        assert!(orchestrator.is_game_over());
    }

    #[test]
    fn test_ai_moves_snake() {
        let mut dependencies = setup_dependencies();

        dependencies.mock_ai_controller
            .expect_get_direction()
            .with(always(), always(), always(), always())
            .returning(|_, _, _, _| Some(Direction::Up));

        let mut orchestrator = setup_orchestrator(dependencies);
        orchestrator.initialize();
        orchestrator.state = GameState::AiPlaying;

        orchestrator.on_game_loop();
        
        assert_ne!(orchestrator.game.direction, Direction::Right);
    }

    #[test]
    fn test_game_over_triggers_callback() {
        let mut dependencies = setup_dependencies();

        dependencies
            .mock_on_game_over
            .expect_invoke()
            .returning(|| {});

        let mut orchestrator = setup_orchestrator(dependencies);
        orchestrator.initialize();

        orchestrator.state = GameState::UserPlaying;

        orchestrator.on_game_loop();
        orchestrator.game.change_direction(Direction::Up);
        orchestrator.on_game_loop();
        orchestrator.game.change_direction(Direction::Left);
        orchestrator.on_game_loop();
        orchestrator.game.change_direction(Direction::Down);
        orchestrator.on_game_loop();

        assert_eq!(orchestrator.state, GameState::GameOver);
    }

    #[test]
    fn test_setup_key_bindings() {
        let mut dependencies = setup_dependencies();

        dependencies
            .mock_document_provider
            .expect_on_key_down()
            .returning(|_| {});

        let orchestrator = Arc::new(Mutex::new(setup_orchestrator(dependencies)));
        GameOrchestrator::setup_key_bindings(&orchestrator);
        
        // Simulate key press event and check if direction changes
        let mut orchestrator = orchestrator.lock().unwrap();
        orchestrator.game.change_direction(Direction::Up);
        assert_eq!(orchestrator.game.direction, Direction::Up);
    }

    fn setup_dependencies() -> Dependencies {
        let mut dependencies = Dependencies {
            mock_canvas_provider: MockCanvasProvider::new(),
            mock_document_provider: MockDocumentProvider::new(),
            mock_window_rovider: MockWindowProvider::new(),
            mock_frame_scheduler: MockFrameScheduler::new(),
            mock_renderer: MockRenderer::new(),
            mock_randomizer: MockRandomizer::new(),
            mock_ai_controller: MockAiController::new(),
            mock_on_score: MockInvokeJsStub::new(),
            mock_on_game_over: MockInvokeJsStub::new(),
        };

        dependencies
            .mock_randomizer
            .expect_get_random_color()
            .returning(|| [1.0, 1.0, 1.0, 1.0]);

        dependencies
            .mock_randomizer
            .expect_get_random_position_on_grid()
            .returning(|_| (5, 5));

        dependencies.mock_renderer
            .expect_draw()
            .returning(|_| {});

    
       dependencies
    }

    fn setup_orchestrator(dependencies: Dependencies) -> TestGameOrchestrator {
        let game_options = GameOptions {
            id: "".into(),
            fps: 10,
            frame_threshold_ms: 10.0,
            grid_size: 20,
            food_count: 1,
            difficulty: Difficulty::Easy,
            snake_color: "#00FF00".to_string(),
        };

        let orchestrator = GameOrchestrator::new(
            game_options,
            dependencies.mock_canvas_provider,
            dependencies.mock_document_provider,
            dependencies.mock_window_rovider,
            dependencies.mock_frame_scheduler,
            dependencies.mock_renderer,
            dependencies.mock_randomizer,
            dependencies.mock_ai_controller,
            dependencies.mock_on_score,
            dependencies.mock_on_game_over,
        );

        orchestrator
    }
    

}
