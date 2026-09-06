use log::{debug, warn};

pub(crate) fn set_clipboard(text: &str) {
    debug!("set_clipboard");
    if let Some(window) = web_sys::window() {
        if let Some(clipboard) = window.navigator().clipboard() {
            let _ = clipboard.write_text(text);
        } else {
            warn!("Clipboard API is not available");
        }
    }
}