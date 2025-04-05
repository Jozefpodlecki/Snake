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