use std::{cell::RefCell, rc::Rc};

use log::debug;
use web_sys::{Document, HtmlCanvasElement, Window};

use crate::{abstractions::{frame_scheduler::{WasmClosureWrapper, WebFrameScheduler}, *}, game::Game, models::{GameOptions, GameResult, GameState, VerticePayload}, randomizer::{JsRandomizer, Randomizer}, utils::create_key_direction_map};

pub type WasmGameOrchestrator<T> = GameOrchestrator<
    HtmlCanvasElement,
    Document,
    Window,
    WasmClosureWrapper,
    T,
    JsRandomizer,
    WebGl2Renderer,
    WebFrameScheduler,
    GreedyBfsAi>;

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
    closure_wrapper: CW,
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
        closure_wrapper: CW,
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
            closure_wrapper,
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
        self.renderer.setup();
        self.game.initialize();
    }

    pub fn resize(&mut self) {
        let width = self.window_provider.get_inner_width() as u32;
        let height = self.window_provider.get_inner_height() as u32;
        
        self.canvas_provider.set_size(width, height);
        self.renderer.set_viewport(width as i32, height as i32);
    }

    pub fn play(&mut self) {
        self.game.reset();
    }

    pub fn stop(&mut self) {
        self.state = GameState::Paused;
    }

    pub fn reset(&mut self, state: GameState) {
        self.state = state;
        self.game.reset();
    }

    pub fn start_game_loop(game_orchestrator: Rc<RefCell<Self>>, is_ai_playing: bool) {

        let callback: Box<dyn FnMut(f64) + 'static> = {
            let game_orchestrator = game_orchestrator.clone();

            Box::new(move |timestamp: f64| {

                let mut orchestrator = game_orchestrator.borrow_mut();

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

            })
        };

        let mut orchestrator = game_orchestrator.borrow_mut();
        let mut closure_wrapper = orchestrator.closure_wrapper.clone();
        closure_wrapper.create(callback);
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

    fn on_game_loop(&mut self) {

        if let GameState::AiPlaying = self.state {
            let ai_direction = self.ai_controller.get_direction(
                &self.game.snake,
                &self.game.foods,
                &self.game.obstacles,
                self.options.grid_size);

            if let Some(direction) = ai_direction {
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
                    self.reset(self.state);
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

    pub fn setup_on_resize(game_orchestrator: Rc<RefCell<Self>>) {
     
        let handler: Box<dyn FnMut() + 'static> = {
            let game_orchestrator = game_orchestrator.clone();
            Box::new(move || {
                game_orchestrator.borrow_mut().resize();
            })
        };
    
        {
            let game_orchestrator = game_orchestrator.borrow_mut();
            game_orchestrator.window_provider.on_resize(handler);
        }
    }
    
    pub fn setup_key_bindings(game_orchestrator: Rc<RefCell<Self>>) {

        let key_direction_map = create_key_direction_map();
    
        let handler: Box<dyn FnMut(String) + 'static> = {
            let game_orchestrator = game_orchestrator.clone();
            Box::new(move |key: String| {
                let mut game_orchestrator = game_orchestrator.borrow_mut();

                if game_orchestrator.state != GameState::UserPlaying {
                    return;
                }
        
                if let Some(direction) = key_direction_map.get(&key) {
                    game_orchestrator.game.change_direction(*direction);
                }
            })
        };
    
        {
            let game_orchestrator = game_orchestrator.borrow_mut();
            game_orchestrator.document_provider.on_key_down(handler);
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::abstractions::canvas_provider::MockCanvasProvider;
    use crate::game_orchestrator::GameOrchestrator;
    use crate::models::{Difficulty, Direction, GameOptions, GameState};
    use crate::randomizer::MockRandomizer;
    use crate::abstractions::frame_scheduler::MockFrameScheduler;
    use crate::abstractions::renderer::MockRenderer;
    use crate::game_orchestrator::document_provider::MockDocumentProvider;
    use crate::game_orchestrator::window_provider::MockWindowProvider;
    use crate::game_orchestrator::frame_scheduler::MockClosureWrapper;
    use crate::game_orchestrator::ai_controller::MockAiController;
    use crate::abstractions::invoke_js::MockInvokeJsStub;
    
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
        pub mock_closure_wrapper: MockClosureWrapper,
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
        let closure_wrapper = MockClosureWrapper::new();
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
            closure_wrapper,
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
    fn test_ai_moves_snake() {
        let mut dependencies = setup_dependencies();

        dependencies
            .mock_renderer
            .expect_setup()
            .return_const(());

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
    fn should_reset_state() {
        let dependencies = setup_dependencies();

        let mut orchestrator = setup_orchestrator(dependencies);
        orchestrator.play();
        orchestrator.stop();
        orchestrator.reset(GameState::GameOver);
    }

    #[test]
    fn test_resize_should_call_dom() {
        let mut dependencies = setup_dependencies();

        dependencies
            .mock_window_rovider
            .expect_get_inner_width()
            .return_const(600);

        dependencies
            .mock_window_rovider
            .expect_get_inner_height()
            .return_const(480);

        dependencies
            .mock_canvas_provider
            .expect_set_size()
            .return_const(());

        dependencies
            .mock_window_rovider
            .expect_on_resize()
            .return_const(());

        dependencies
            .mock_renderer
            .expect_set_viewport()
            .return_const(());

        let mut orchestrator = setup_orchestrator(dependencies);
        orchestrator.resize();
    }

    #[test]
    fn test_game_over_triggers_callback() {
        let mut dependencies = setup_dependencies();

        dependencies
            .mock_renderer
            .expect_setup()
            .return_const(());

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

        let orchestrator = Rc::new(RefCell::new(setup_orchestrator(dependencies)));
        GameOrchestrator::setup_key_bindings(orchestrator.clone());
        
        // Simulate key press event and check if direction changes
        let mut orchestrator = orchestrator.borrow_mut();
        orchestrator.game.change_direction(Direction::Up);
        assert_eq!(orchestrator.game.direction, Direction::Up);
    }

    #[test]
    fn should_start_game_loop() {
        let mut dependencies = setup_dependencies();

        dependencies
            .mock_document_provider
            .expect_on_key_down()
            .returning(|_| {});

        dependencies
            .mock_closure_wrapper
            .expect_clone()
            .returning(|| {
                let mut wrapper = MockClosureWrapper::new();

                wrapper
                    .expect_clone()
                    .returning(|| MockClosureWrapper::new());

                wrapper
                    .expect_create()
                    .return_const(());

                wrapper
            });

        dependencies
            .mock_frame_scheduler
            .expect_request_frame()
            .return_const(0);

        let orchestrator = Rc::new(RefCell::new(setup_orchestrator(dependencies)));
        GameOrchestrator::start_game_loop(orchestrator.clone(), false);
    }

    fn setup_dependencies() -> Dependencies {
        let mut dependencies = Dependencies {
            mock_canvas_provider: MockCanvasProvider::new(),
            mock_document_provider: MockDocumentProvider::new(),
            mock_window_rovider: MockWindowProvider::new(),
            mock_closure_wrapper: MockClosureWrapper::new(),
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
            dependencies.mock_closure_wrapper,
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
