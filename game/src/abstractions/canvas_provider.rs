use web_sys::HtmlCanvasElement;

pub trait CanvasProvider {
    fn set_size(&self, width: u32, height: u32);
}

impl CanvasProvider for HtmlCanvasElement {
    fn set_size(&self, width: u32, height: u32) {
        self.set_width(width);
        self.set_height(height);
    }
}