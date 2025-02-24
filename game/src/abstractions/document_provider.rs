use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Document, KeyboardEvent};

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

#[cfg(test)]
mockall::mock! {
    pub DocumentProvider {}
    impl DocumentProvider for DocumentProvider {
        fn on_key_down(&self, handler: Box<dyn FnMut(String)>);
    }
}