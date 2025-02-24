use js_sys::Float32Array;
use web_sys::WebGl2RenderingContext;

use crate::models::VerticePayload;

pub trait Renderer {
    fn set_viewport(&self, width: i32, height: i32);
    fn draw(&mut self, vertices: &VerticePayload);
}

pub struct WebGl2Renderer {
    context: WebGl2RenderingContext,
    last_buffer_size: i32
}

impl Renderer for WebGl2Renderer {

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
        fn set_viewport(&self, width: i32, height: i32);
        fn draw(&mut self, vertices: &VerticePayload);
    }
}