
use wasm_bindgen::prelude::*;
use log::debug;

#[wasm_bindgen(module = "/assets/js/clipboard.js")]
extern "C" {
    type Clipboard;
    #[wasm_bindgen(constructor)]
    pub fn new(value: &str) -> Clipboard;
    #[wasm_bindgen(method)]
    pub fn click(this: &Clipboard);
}

pub(crate) fn set_clipboard(value: &str) {
    debug!("set_clipboard");
    Clipboard::new(value);
    debug!("run click");
    // x.click();
}