use wasm_bindgen::prelude::*;
// use wasm_bindgen::{JsCast, JsValue};
use log::debug;

#[wasm_bindgen(module = "/assets/js/greatviewer.js")]
extern "C" {
    type GreatViewer;

    #[wasm_bindgen(constructor)]
    fn new(
        patch_to_model: &str,
        size_flag: bool,
    ) -> GreatViewer;

    #[wasm_bindgen(method)]
    fn starter(this: &GreatViewer);
}

// fn check_it(label: &str, value: &JsValue) {
//     let result =
//         if value.has_type::<GreatViewer>() {
//             "checked cast will succeed"
//         } else {
//             "checked cast will fail"
//         };

//     debug!("{}: {}, {:#?}", label, result, value);
// }

pub(crate) fn preview_model(patch_to_model: &str, size_flag: bool) {
    debug!("viewer");
    show_it(&patch_to_model, size_flag);
}

fn show_it(patch_to_model: &str, size_flag: bool) {
    let x = GreatViewer::new(
        patch_to_model,
        size_flag,
    );
    // check_it("one", &x);
    x.starter();
}