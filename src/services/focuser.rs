use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};
use log::debug;

/// Sets focus on the element with the specified `element_id` and scrolls it into view.
pub(crate) fn set_focus(element_id: &str) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // Find the element by ID
    if let Some(element) = document.get_element_by_id(element_id) {
        if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
            debug!("Focus {:?}, html_element {:?}", element_id, html_element);
            html_element.focus().expect("failed to set focus");
            html_element.scroll_into_view();
        }
    } else {
        debug!("Element {:?} not found", element_id);
    }
}