pub mod frame_scheduler;
pub mod renderer;

use js_sys::Function;
pub use renderer::{Renderer, WebGl2Renderer};
pub use frame_scheduler::{FrameScheduler, ClosureWrapper, ClosureHandle};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Document, HtmlCanvasElement, KeyboardEvent, Window};

pub trait DocumentProvider {
    fn on_key_down(&self, handler: Box<dyn FnMut(String)>);
}

impl DocumentProvider for Document {
    fn on_key_down(&self, mut handler: Box<dyn FnMut(String) + 'static>) {
        let closure = Closure::new(Box::new(move |event: KeyboardEvent| {
            handler(event.key().to_lowercase());
        }) as Box<dyn FnMut(_)>);
        self.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

pub trait WindowProvider {
    fn get_inner_width(&self) -> f64;
    fn get_inner_height(&self) -> f64;
    fn on_resize(&self, handler: Box<dyn FnMut()>);
}

pub trait CanvasProvider {
    fn set_size(&self, width: u32, height: u32);
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

impl CanvasProvider for HtmlCanvasElement {
    fn set_size(&self, width: u32, height: u32) {
        self.set_width(width);
        self.set_height(height);
    }
}

pub trait InvokeJs  {
    fn invoke(&self);
}

impl InvokeJs for Function {
    fn invoke(&self) {
        self.call0(&JsValue::null()).unwrap();
    }
}