use std::collections::HashMap;

use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};

use crate::{constants::{FS_SOURCE, VS_SOURCE}, models::Direction};

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