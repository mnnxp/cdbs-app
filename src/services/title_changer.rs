use web_sys;
use log::debug;

const CDBS_NAME: &'static str = "CADBase";

/// Sets the text for the browser tab title bar.
/// Adds ` | CADBase` to the title or sets `CADBase` if the provided value is empty.
pub(crate) fn set_title(value: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    if value.is_empty() {
        debug!("Set the default title");
        // set the default title
        document.set_title(CDBS_NAME);
        return
    }
    // trimming for long titles
    let new_title = match value.chars().count() {
        0..=20 => format!("{} | {}", value, CDBS_NAME),
        _ => format!("{:.*}... | {}", 17, value, CDBS_NAME),
    };
    if document.title() == new_title {
        debug!("The title has already been established");
        // the title has already been established
        return
    }
    document.set_title(&new_title);
}