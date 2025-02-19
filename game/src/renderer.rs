use js_sys::Float32Array;
use web_sys::WebGl2RenderingContext;

pub struct Renderer {
    context: WebGl2RenderingContext,
    last_buffer_size: i32
}

impl Renderer {
    pub fn new(context: WebGl2RenderingContext) -> Self {
        Renderer { context, last_buffer_size: 0 }
    }

    pub fn draw_vertices(&mut self, vertices: &Float32Array, vertice_size: i32) {
        let length = vertices.length();
        let buffer_size = (length * 4) as i32;
        
        if self.last_buffer_size != buffer_size {
            self.context.buffer_data_with_i32(WebGl2RenderingContext::ARRAY_BUFFER, buffer_size, WebGl2RenderingContext::DYNAMIC_DRAW);
            self.last_buffer_size = buffer_size;
        }

        self.context.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            0,
            vertices,
        );
        
        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertice_size);
    }
}