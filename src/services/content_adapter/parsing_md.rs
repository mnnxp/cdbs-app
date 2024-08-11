use yew::{Html, html};
use pulldown_cmark;
use log::debug;

/// Returns converting a Markdown string into Html code wrapped in a div tag
pub(crate) fn inner_markdown(raw_text: &str) -> Html {
    // create parser with example Markdown text
    let parser = pulldown_cmark::Parser::new(raw_text);

    // write to a new String buffer
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    web_sys::window()
        .and_then(|window| window.document())
        .map_or_else(
            || {
                debug!("Failed to resolve `document`.");
                html!{}
            },
            |document| match document.create_element("div") {
                Ok(div) => {
                    div.set_inner_html(&html_output);
                    yew::virtual_dom::VNode::VRef(div.into())
                },
                Err(e) => {
                    debug!("{:?}", &e);
                    html!{}
                },
            },
        )
}