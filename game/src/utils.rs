use std::{cell::RefCell, collections::HashMap, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Document, Event, EventTarget, HtmlCanvasElement, KeyboardEvent, WebGlProgram, WebGlRenderingContext, WebGlShader, Window};

use crate::{constants::{FS_SOURCE, VS_SOURCE}, models::{Direction, Snake}};

pub type ShareduEventClosure = Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>;
pub type Sharedf64Closure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

pub fn create_shader(context: &WebGlRenderingContext, shader_type: u32, source: &str) -> Option<WebGlShader> {
    let shader = context.create_shader(shader_type).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    let is_success = context.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS);

    if !is_success.as_bool().unwrap_or_default() {
        context.get_shader_info_log(&shader);
        context.delete_shader(Some(&shader));
        return None
    }

    Some(shader)
}

pub fn create_program(
    context: &WebGlRenderingContext,
    vertex_shader: WebGlShader,
    fragment_shader: WebGlShader) -> Option<WebGlProgram> {
    let program = context.create_program().unwrap();

    context.attach_shader(&program, &vertex_shader);
    context.attach_shader(&program, &fragment_shader);
    context.link_program(&program);

    let is_success = context.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS);

    if !is_success.as_bool().unwrap_or_default() {
        context.get_program_info_log(&program);
        context.delete_program(Some(&program));
        return None
    }

    Some(program)
}

pub fn setup_webgl(context: &WebGlRenderingContext) {

    let vertex_shader = create_shader(&context, WebGlRenderingContext::VERTEX_SHADER, VS_SOURCE).unwrap();
    let fragment_shader = create_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER, FS_SOURCE).unwrap();
    let program = create_program(&context, vertex_shader, fragment_shader).unwrap();

    context.use_program(Some(&program));
    let position_buffer = context.create_buffer();

    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, position_buffer.as_ref());
    let position_location = context.get_attrib_location(&program, "a_position") as u32;

    context.enable_vertex_attrib_array(position_location);
    context.vertex_attrib_pointer_with_i32(position_location, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
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

pub fn request_animation_frame(window: &Window, closure: &Sharedf64Closure) {
    let borrowed_closure = closure.borrow();
    let closure_function = borrowed_closure.as_ref().unwrap().as_ref().unchecked_ref();
    window.request_animation_frame(closure_function).unwrap();
    // window.cancel_animation_frame(handle)
}

pub fn setup_key_bindings(document: Document, snake: Rc<RefCell<Snake>>) {

    let key_direction_map = create_key_direction_map();

    let closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {

        let key = event.key().to_lowercase();

        if let Some(direction) = key_direction_map.get(&key) {
            snake.borrow_mut().change_direction(*direction);
        }

    });

    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn on_resize(
    window: Window,
    canvas: HtmlCanvasElement,
    context: WebGlRenderingContext) {
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
    context.viewport(0, 0, context.drawing_buffer_width(), context.drawing_buffer_height());
}

pub fn setup_on_resize(
    window: Window,
    canvas: HtmlCanvasElement,
    context: WebGlRenderingContext
) {

    let window_inner = window.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |_: EventTarget| {
        on_resize(window_inner.clone(), canvas.clone(), context.clone());
    });

    window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}