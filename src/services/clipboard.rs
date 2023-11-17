use wasm_bindgen::prelude::*;
// use wasm_bindgen::{JsCast, JsValue};
use log::debug;

use crate::services::get_value_field;

#[wasm_bindgen(module = "/assets/js/clipboard.js")]
extern "C" {
    // #[wasm_bindgen(js_name = ClipboardJS)]
    // pub type JSClipboard;

    // #[wasm_bindgen(constructor)]
    // pub fn new(trigger: &str) -> JSClipboard; 

    // type ClipboardJS;
    // #[wasm_bindgen(constructor, js_namespace = ["window"])]
    // fn new(trigger: &str) -> ClipboardJS;

    // #[wasm_bindgen(js_name = default)]
    type Clipboard;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Clipboard;
}

pub(crate) fn set_clipboard(trigger: &str) {
    debug!("set_clipboard");
    Clipboard::new();
}
