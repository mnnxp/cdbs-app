use wasm_bindgen::prelude::*;
// use wasm_bindgen::{JsCast, JsValue};
use log::debug;

use crate::services::get_value_field;

#[wasm_bindgen(module = "/assets/js/greatviewer.js")]
extern "C" {
    type GreatViewer;

    #[wasm_bindgen(constructor)]
    fn new(
        patch_to_model: &str,
        size_flag: bool,
        i18n_str: &str,
    ) -> GreatViewer;

    #[wasm_bindgen(method)]
    fn starter(this: &GreatViewer);
}

pub(crate) fn preview_model(patch_to_model: &str, size_flag: bool) {
    debug!("viewer");
    let i18n_str = format!("{}#{}#{}#{}#{}#{}",
        get_value_field(&302), // "Coordinate axes"
        get_value_field(&303), // "Rotation"
        get_value_field(&304), // "Frame"
        get_value_field(&305), // "Model color"
        get_value_field(&306), // "Background color"
        get_value_field(&307), // "Model Scale
    );
    show_it(&patch_to_model, size_flag, &i18n_str);
}

fn show_it(patch_to_model: &str, size_flag: bool, i18n_str: &str) {
    let x = GreatViewer::new(
        patch_to_model,
        size_flag,
        i18n_str,
    );
    // check_it("one", &x);
    x.starter();
}