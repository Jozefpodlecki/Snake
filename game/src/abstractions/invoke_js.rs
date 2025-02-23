use js_sys::Function;
use wasm_bindgen::JsValue;

pub trait InvokeJs  {
    fn invoke(&self);
}

impl InvokeJs for Function {
    fn invoke(&self) {
        self.call0(&JsValue::null()).unwrap();
    }
}

#[cfg(test)]
mockall::mock! {
    pub InvokeJsMock {}
    impl InvokeJs for InvokeJsMock {
        fn invoke(&self);
    }
}