mod auth;
mod localisation;
mod filesize;
mod local_storage;
mod preview_model;
mod requests;
mod set_classes;
mod upload_file;
mod util;
mod response_parsing;
mod subscribers;
mod clipboard;

pub(crate) mod content_adapter;
pub(crate) mod title_changer;

pub use auth::{Auth, get_current_user, logout};
pub use localisation::get_value_field;
pub use filesize::Size;
pub use local_storage::{
    get_logged_user, set_logged_user, get_token, is_authenticated, set_lang, get_lang,
    set_token, set_list_view, get_list_view, set_history_back, get_history_back, set_history_search, get_history_search,
};
pub(crate) use preview_model::preview_model;
pub use requests::Requests;
pub use set_classes::get_classes_table;
pub use upload_file::{PutUploadFile, UploadData};
pub(crate) use util::{ext_str, image_detector, three_detector, prepare_username, compare_op_uuid, wraps_text};
pub(crate) use response_parsing::{
    resp_parsing, resp_parsing_two_level, get_value_response, get_from_value
};
pub(crate) use subscribers::Counter;
pub(crate) use clipboard::set_clipboard;