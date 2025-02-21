
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Window;

pub type ClosureHandle = i32;

pub trait ClosureWrapper: Clone {
    fn new(callback: Box<dyn FnMut(f64)>) -> Self;
}

pub struct WasmClosureWrapper {
    pub closure: Rc<RefCell<Closure<dyn FnMut(f64)>>>
}

pub trait FrameScheduler<CW: ClosureWrapper> {
    fn request_frame(&self, callback: &CW) -> ClosureHandle;
    fn request_frame_after(&self, callback: &CW, timeout: i32);
    fn cancel(&self, handle: ClosureHandle);
}

pub struct WebFrameScheduler {
    window: Window,
}

impl WebFrameScheduler {
    pub fn new(window: Window) -> Self {
        Self {
            window
        }
    }
}

impl ClosureWrapper for WasmClosureWrapper {
    fn new(callback: Box<dyn FnMut(f64)>) -> Self {
        let closure = Closure::wrap(callback);
        let closure = Rc::new(RefCell::new(closure));

        Self {
            closure,
        }
    }
}

impl Clone for WasmClosureWrapper {
    fn clone(&self) -> Self {
        Self { closure: self.closure.clone() }
    }
}

impl FrameScheduler<WasmClosureWrapper> for WebFrameScheduler {
    fn request_frame(&self, handler: &WasmClosureWrapper) -> ClosureHandle {
        let borrowed_closure = handler.closure.borrow();
        let js_function = borrowed_closure.as_ref().unchecked_ref();
        self.window.request_animation_frame(js_function).unwrap()
    }

    fn request_frame_after(&self, handler: &WasmClosureWrapper, timeout: i32) {
        
        let window = self.window.clone();
        let request_frame_handler = handler.clone();
        let timeout_closure = Closure::wrap(Box::new(move || {
            let request_frame_handler = request_frame_handler.closure.borrow();
            let js_function = request_frame_handler.as_ref().unchecked_ref();
            window.request_animation_frame(js_function).unwrap();
        }) as Box<dyn FnMut()>);

        self.window.set_timeout_with_callback_and_timeout_and_arguments_0(timeout_closure.as_ref().unchecked_ref(), timeout).unwrap();
        timeout_closure.forget();
    }
    
    fn cancel(&self, handle: ClosureHandle) {
        self.window.cancel_animation_frame(handle).unwrap();
    }
}
