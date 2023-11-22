use wasm_bindgen::prelude::*;
// use wasm_bindgen::{JsCast, JsValue};
use log::debug;

use crate::services::get_value_field;

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
    let x = Clipboard::new(value);
    debug!("run click");
    x.click();
}
