use yew::{html, Html};
use pulldown_cmark::{Parser, Options, Event, Tag};
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};
use log::debug;

const MAX_DEPTH: usize = 100;
const ALLOWED_TAGS: &[&str] = &[
    "a", "b", "blockquote", "br", "code", "dd", "del", "div", "dl", "dt",
    "em", "h1", "h2", "h3", "h4", "h5", "h6", "hr", "i", "img", "input", "li",
    "mark", "math", "mfrac", "mi", "mn", "mo", "mover", "mroot", "mrow", "msqrt",
    "msub", "msubsup", "msup", "munder", "ol", "p", "pre", "s", "span", "strong",
    "sub", "sup", "table", "tbody", "td", "th", "thead", "tr", "u", "ul",
];
const ALLOWED_ATTRS: &[&str] = &["alt", "checked", "class", "colspan", "disabled", "href", "id", "rowspan", "src", "title", "type"];
const ALLOWED_PROTOCOLS: &[&str] = &["http", "https", "mailto", "tel"];

/// Returns converting a Markdown string into Html code wrapped in a div tag
pub(crate) fn inner_markdown(raw_text: &str) -> Html {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(raw_text, options).filter_map(|event| {
        match event {
            Event::Start(Tag::Link(link_type, url, title)) => {
                let url_str = url.to_string();
                let url_lower = url_str.to_lowercase();

                if ALLOWED_PROTOCOLS.iter().any(|&p| url_lower.starts_with(p))
                    || url_str.starts_with('/')
                    || url_str.starts_with('.')
                    || url_str.starts_with('#')
                {
                    Some(Event::Start(Tag::Link(link_type, url, title)))
                } else {
                    Some(Event::Text(url_str.into()))
                }
            }
            Event::Start(Tag::Image(link_type, url, title)) => {
                let url_str = url.to_string();
                let url_lower = url_str.to_lowercase();

                if ALLOWED_PROTOCOLS.iter().any(|&p| url_lower.starts_with(p))
                    || url_str.starts_with('/')
                    || url_str.starts_with('.')
                {
                    Some(Event::Start(Tag::Image(link_type, url, title)))
                } else {
                    None
                }
            }
            _ => Some(event),
        }
    });

    let mut html_string = String::new();
    pulldown_cmark::html::push_html(&mut html_string, parser);

    let html_output = clean_raw_html(&html_string);

    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.create_element("div").ok())
        .map(|div| {
            div.set_inner_html(&html_output);
            yew::virtual_dom::VNode::VRef(div.into())
        })
        .unwrap_or_else(|| {
            debug!("Failed to render markdown");
            html!{}
        })
}

fn clean_raw_html(html: &str) -> String {
    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            debug!("No window available for HTML sanitization");
            return String::new();
        }
    };

    let document = match window.document() {
        Some(d) => d,
        None => {
            debug!("No document available for HTML sanitization");
            return String::new();
        }
    };

    let div = match document.create_element("div") {
        Ok(el) => el,
        Err(error) => {
            debug!("Failed to create div element: {:?}", error);
            return String::new();
        }
    };

    div.set_inner_html(html);
    clean_node(&div, 0);
    div.inner_html()
}

fn clean_node(node: &Node, depth: usize) {
    if depth > MAX_DEPTH {
        debug!("Maximum depth exceeded in HTML sanitization at depth {}", depth);
        return;
    }

    // Collect all children into a vector before any modifications
    let mut children = Vec::new();
    let mut child = node.first_child();
    while let Some(child_node) = child {
        children.push(child_node.clone());
        child = child_node.next_sibling();
    }

    let mut nodes_to_remove = Vec::new();

    for child_node in children {
        if let Some(element) = child_node.dyn_ref::<Element>() {
            let tag_name = element.tag_name().to_lowercase();

            // Check if tag is in the allowed list
            if !ALLOWED_TAGS.contains(&tag_name.as_str()) {
                nodes_to_remove.push(child_node.clone());
                continue;
            }

            // Clean attributes
            let attribute_names = element.get_attribute_names();
            let mut attributes_to_remove = Vec::new();

            for attribute_name in attribute_names.iter() {
                let attribute_name_str = match attribute_name.as_string() {
                    Some(name) => name,
                    None => continue,
                };

                let attribute_name_lower = attribute_name_str.to_lowercase();

                // Remove event handlers
                if attribute_name_lower.starts_with("on") {
                    attributes_to_remove.push(attribute_name_str);
                    continue;
                }

                // Check if attribute is in the allowed list
                if !ALLOWED_ATTRS.contains(&attribute_name_lower.as_str()) {
                    attributes_to_remove.push(attribute_name_str);
                    continue;
                }

                // Validate URL protocols for href and src attributes
                if attribute_name_lower == "href" || attribute_name_lower == "src" {
                    if let Some(attribute_value) = element.get_attribute(&attribute_name_str) {
                        let attribute_value_lower = attribute_value.to_lowercase();

                        // Allow relative paths and safe protocols
                        let is_safe = ALLOWED_PROTOCOLS.iter().any(|protocol| attribute_value_lower.starts_with(protocol))
                            || attribute_value.starts_with('/')
                            || attribute_value.starts_with('.')
                            || attribute_value.starts_with('#');

                        if !is_safe {
                            let _ = element.set_attribute(&attribute_name_str, "#");
                        }
                    }
                }
            }

            // Remove unsafe attributes after collecting all names
            for attribute_name in attributes_to_remove {
                let _ = element.remove_attribute(&attribute_name);
            }
        }
        // Recursively clean child nodes
        clean_node(&child_node, depth + 1);
    }

    // Remove unsafe tags after the traversal
    for node_to_remove in nodes_to_remove {
        let _ = node.remove_child(&node_to_remove);
    }
}