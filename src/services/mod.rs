//! Api requests via yew FetchService

mod auth;
mod localisation;
mod filesize;
mod local_storage;
mod content_adapter;
mod parsing_md;
mod preview_model;
mod requests;
mod tags;
mod upload_file;
mod util;
mod response_parsing;
mod subscribers;
mod clipboard;
pub(crate) mod title_changer;

pub use auth::{Auth, get_current_user, logout};
pub use localisation::get_value_field;
pub use filesize::Size;
pub use local_storage::{
    get_logged_user, set_logged_user, get_token, is_authenticated,
    set_lang, get_lang, set_token, set_list_view, get_list_view
};
pub(crate) use content_adapter::ContentAdapter;
pub(crate) use parsing_md::inner_markdown;
pub(crate) use preview_model::preview_model;
pub use requests::Requests;
pub use tags::Tags;
pub use upload_file::{PutUploadFile, UploadData};
pub(crate) use util::{image_detector, three_detector, url_decode};
pub(crate) use response_parsing::{
    resp_parsing, resp_parsing_two_level, get_value_response, get_from_value
};
pub(crate) use subscribers::Counter;
pub(crate) use clipboard::set_clipboard;