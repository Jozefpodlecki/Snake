use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};


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

// function createProgram(vs, fs) {
//     const program = gl.createProgram();
//     gl.attachShader(program, vs);
//     gl.attachShader(program, fs);
//     gl.linkProgram(program);
//     if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
//         console.error(gl.getProgramInfoLog(program));
//         gl.deleteProgram(program);
//         return null;
//     }
//     return program;
// }
// function createShader(type, source) {
//     const shader = gl.createShader(type);
//     gl.shaderSource(shader, source);
//     gl.compileShader(shader);
//     if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
//         console.error(gl.getShaderInfoLog(shader));
//         gl.deleteShader(shader);
//         return null;
//     }
//     return shader;
// }