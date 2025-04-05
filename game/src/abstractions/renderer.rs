use js_sys::Float32Array;
use web_sys::WebGl2RenderingContext;

use crate::{constants::{FS_SOURCE, VS_SOURCE}, models::VerticePayload, utils::{create_program, create_shader}};

pub trait Renderer {
    fn setup(&self);
    fn set_viewport(&self, width: i32, height: i32);
    fn draw(&mut self, vertices: &VerticePayload);
}

pub struct WebGl2Renderer {
    context: WebGl2RenderingContext,
    last_buffer_size: i32
}

impl Renderer for WebGl2Renderer {

    fn setup(&self) {

        let context = &self.context;
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

    fn set_viewport(&self, _width: i32, _height: i32) {
        // self.context.viewport(0, 0, width, height);
        let width = self.context.drawing_buffer_width();
        let height = self.context.drawing_buffer_height();
        self.context.viewport(0, 0, width, height);
    }

    fn draw(&mut self, payload: &VerticePayload) {
        let buffer_size = (payload.length * 4) as i32;
        let vertices = unsafe { Float32Array::view(&payload.data) };
        
        if self.last_buffer_size != buffer_size {
            self.context.buffer_data_with_i32(WebGl2RenderingContext::ARRAY_BUFFER, buffer_size, WebGl2RenderingContext::DYNAMIC_DRAW);
            self.last_buffer_size = buffer_size;
        }

        self.context.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            0,
            &vertices,
        );
        
        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, payload.vertice_size);
    }
}

impl WebGl2Renderer {
    pub fn new(context: WebGl2RenderingContext) -> Self {
        WebGl2Renderer { context, last_buffer_size: 0 }
    }
}

#[cfg(test)]
mockall::mock! {
    pub Renderer {}
    impl Renderer for Renderer {
        fn setup(&self);
        fn set_viewport(&self, width: i32, height: i32);
        fn draw(&mut self, vertices: &VerticePayload);
    }
}