use std::{cell::RefCell, rc::Rc};

use constants::{FS_SOURCE, VS_SOURCE};
use utils::{create_program, create_shader};
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGlRenderingContext};

mod utils;
mod constants;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;

    canvas.set_width(width);
    canvas.set_height(height);

    let context = canvas.get_context("webgl").unwrap().unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    setup_webgl(&context);

    // let cb = Closure::wrap(Box::new(move || {
    //     web_sys::console::log_1(&"raf called".into());
    // }) as Box<dyn FnMut()>);

    // let closure = Rc::new(RefCell::new(None));
    // let closure_clone = closure.clone();


    // {
    //     // let window = window.clone();
    //     *closure_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        
    
    //         let test = closure.borrow().as_ref().unwrap();
    //         // web_sys::window().unwrap().request_animation_frame(test).unwrap();

    //     }) as Box<dyn FnMut()>));
    // }

    // window.request_animation_frame(aaa);

    Ok(())
}

fn setup_webgl(context: &WebGlRenderingContext) {

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