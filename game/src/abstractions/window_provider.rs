use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Window;

pub trait WindowProvider {
    fn get_inner_width(&self) -> f64;
    fn get_inner_height(&self) -> f64;
    fn on_resize(&self, handler: Box<dyn FnMut()>);
}


impl WindowProvider for Window {
    fn get_inner_width(&self) -> f64 {
        self.inner_width().unwrap().as_f64().unwrap()
    }

    fn get_inner_height(&self) -> f64 {
        self.inner_height().unwrap().as_f64().unwrap()
    }

    fn on_resize(&self, handler: Box<dyn FnMut()>) {
        let closure = Closure::new(handler);
        self.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}