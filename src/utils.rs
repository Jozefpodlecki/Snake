use std::rc::Rc;
use web_sys::HtmlCanvasElement;

#[derive(Clone)]
pub struct Style {
    pub color: String,
    pub font_size: String,
    pub font_family: String
}

#[derive(Clone)]
pub struct Layout {
    pub row_size: f64,
    pub column_size: f64,
    pub row_span: (u32, u32),
    pub column_span: (u32, u32),
    pub alignment: (String, String)
}

impl Layout {
    pub fn compute_dim(&self, text_width: f64) -> (f64, f64) {
        let row_start = self.row_span.0 as f64 * self.row_size;
        let row_end = self.row_span.1 as f64 * self.row_size;

        let y = row_start + (row_end - row_start) / 2_f64;

        let column_start = self.column_span.0 as f64 * self.column_size;
        let column_end = self.column_span.1 as f64 * self.column_size;

        let x = column_start + (column_end - column_start - text_width) / 2_f64;

        (x, y)
    }
}

pub fn resize_canvas(canvas: &HtmlCanvasElement, width: u32, height: u32) {
    canvas.set_width(width);
    canvas.set_height(height);
}

pub fn get_current_timestamp() -> f64 {
    js_sys::Date::now()
}