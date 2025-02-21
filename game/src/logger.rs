pub trait Logger {

}

pub struct WasmLogger {

}

impl Logger for WasmLogger {

}

impl WasmLogger {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct VoidLogger {
}

impl Logger for VoidLogger {

}

impl VoidLogger {
    pub fn new() -> Self {
        Self {}
    }
}