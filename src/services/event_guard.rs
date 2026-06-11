use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// A RAII guard that automatically removes a global keyboard event listener 
/// from the browser window when it goes out of scope.
pub(crate) struct KeyboardGuard {
    closure: Closure<dyn FnMut(web_sys::KeyboardEvent)>,
}

impl KeyboardGuard {
    /// Creates a new guard and registers the provided closure as a window event listener.
    pub(crate) fn new(closure: Closure<dyn FnMut(web_sys::KeyboardEvent)>) -> Self {
        let window = web_sys::window().expect("No global window found");
        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("Failed to register keydown listener");
        Self { closure }
    }
}

impl Drop for KeyboardGuard {
    fn drop(&mut self) {
        if let Some(window) = web_sys::window() {
            let _ = window.remove_event_listener_with_callback(
                "keydown",
                self.closure.as_ref().unchecked_ref(),
            );
            web_sys::console::debug_1(&"Global hotkeys listener dropped automatically by RAII guard".into());
        }
    }
}