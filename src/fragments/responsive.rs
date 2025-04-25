/// Determines the width of the specified block by ID and returns a string with CSS in which the value of the `trim` variable in rem is subtracted from the resulting width.
/// Example of returned string: `width: calc(920px - 5rem)`
pub(crate) fn resizer(p_element: &str, trim: i32) -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let card_list_width = document.get_element_by_id(p_element).map(|c| c.client_width()).unwrap_or_default();
    format!("width: calc({}px - {}rem)", card_list_width, trim)
}