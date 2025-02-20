use std::{cell::RefCell, collections::HashMap, rc::Rc};

use js_sys::{Function, Math};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Document, EventTarget, HtmlCanvasElement, KeyboardEvent, WebGlProgram, WebGl2RenderingContext, WebGlShader, Window};

use crate::{constants::{FS_SOURCE, VS_SOURCE}, game::Game, models::Direction, randomizer::JsRandomizer};

pub type Sharedf64Closure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

pub fn create_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Option<WebGlShader> {
    let shader = context.create_shader(shader_type).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    let is_success = context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS);

    if !is_success.as_bool().unwrap_or_default() {
        context.get_shader_info_log(&shader);
        context.delete_shader(Some(&shader));
        return None
    }

    Some(shader)
}

pub fn create_program(
    context: &WebGl2RenderingContext,
    vertex_shader: WebGlShader,
    fragment_shader: WebGlShader) -> Option<WebGlProgram> {
    let program = context.create_program().unwrap();

    context.attach_shader(&program, &vertex_shader);
    context.attach_shader(&program, &fragment_shader);
    context.link_program(&program);

    let is_success = context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS);

    if !is_success.as_bool().unwrap_or_default() {
        context.get_program_info_log(&program);
        context.delete_program(Some(&program));
        return None
    }

    Some(program)
}

pub fn setup_webgl(context: &WebGl2RenderingContext) {

    let vertex_shader = create_shader(&context, WebGl2RenderingContext::VERTEX_SHADER, VS_SOURCE).unwrap();
    let fragment_shader = create_shader(&context, WebGl2RenderingContext::FRAGMENT_SHADER, FS_SOURCE).unwrap();
    let program = create_program(&context, vertex_shader, fragment_shader).unwrap();

    context.use_program(Some(&program));
    
    let position_buffer = context.create_buffer();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, position_buffer.as_ref());

    let position_location = context.get_attrib_location(&program, "a_position") as u32;
    let stride = 6 * 4;

    context.enable_vertex_attrib_array(position_location);
    context.vertex_attrib_pointer_with_i32(position_location, 2, WebGl2RenderingContext::FLOAT, false, stride, 0);

    let color_location = context.get_attrib_location(&program, "a_color") as u32;
    context.vertex_attrib_pointer_with_i32(color_location, 4, WebGl2RenderingContext::FLOAT, false, stride, 2 * 4);
    context.enable_vertex_attrib_array(color_location);
}

pub fn create_key_direction_map() -> HashMap<String, Direction> {
    let key_direction_pairs = vec![
        ("a", Direction::Left),
        ("arrowleft", Direction::Left),
        ("w", Direction::Up),
        ("arrowup", Direction::Up),
        ("s", Direction::Down),
        ("arrowdown", Direction::Down),
        ("d", Direction::Right),
        ("arrowright", Direction::Right),
    ];

    key_direction_pairs.into_iter().map(|(key, direction)| {
        (key.to_string(), direction)
    }).collect()
}

pub fn request_animation_frame(window: &Window, closure: &Sharedf64Closure) -> i32 {
    let borrowed_closure = closure.borrow();
    let closure_function = borrowed_closure.as_ref().unwrap().as_ref().unchecked_ref();
    window.request_animation_frame(closure_function).unwrap()
}

pub fn setup_key_bindings(document: Document, game: Rc<RefCell<Game<Function, JsRandomizer>>>) {

    let key_direction_map = create_key_direction_map();

    let closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {

        if !game.borrow_mut().can_run {
            return;
        }

        let key = event.key().to_lowercase();

        if let Some(direction) = key_direction_map.get(&key) {
            game.borrow_mut().change_direction(*direction);
        }

    });

    let listener = closure.as_ref().unchecked_ref();
    document.add_event_listener_with_callback("keydown", listener).unwrap();
    closure.forget();
}

pub fn on_resize(
    window: Window,
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext) {
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
    context.viewport(0, 0, context.drawing_buffer_width(), context.drawing_buffer_height());
}

pub fn setup_on_resize(
    window: Window,
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext
) {

    let window_inner = window.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |_: EventTarget| {
        on_resize(window_inner.clone(), canvas.clone(), context.clone());
    });

    window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn performance_now(window: &Window) -> f64 {
    let timestamp = window.performance().unwrap().now();
    timestamp
}

pub fn set_timeout_with_param(window: Window, closure: &Sharedf64Closure, timeout: i32, argument: JsValue) {
    let borrowed_closure = closure.borrow();
    let closure_function = borrowed_closure.as_ref().unwrap().as_ref().unchecked_ref();
    
    let arguments = js_sys::Array::new_with_length(1);
    arguments.set(0, argument);
    window.set_timeout_with_callback_and_timeout_and_arguments(closure_function, timeout, &arguments).unwrap();
}

pub fn get_random_position_on_grid(grid_size: i32) -> (i32, i32) {
    #[cfg(test)]
    {
        use rand::Rng;
        let mut rng = rand::rng();
        let x = rng.random_range(1..grid_size);
        let y = rng.random_range(1..grid_size);
        (x, y)
    }
 
    #[cfg(not(test))]
    {
        let grid_size = grid_size - 1;
        let x = (1.0 + Math::random() * grid_size as f64) as i32;
        let y = (1.0 + Math::random() * grid_size as f64) as i32;
        (x, y)
    }
   
}