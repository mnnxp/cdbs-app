//! Api requests via yew FetchService

mod auth;
mod localisation;
mod local_storage;
mod preview_model;
mod requests;
mod tags;
mod upload_file;
mod util;
mod response_parsing;

pub use auth::{Auth, get_current_user, logout};
pub use localisation::get_value_field;
pub use local_storage::{
    get_logged_user, set_logged_user, get_token, is_authenticated,
    set_lang, get_lang, set_token, set_list_view, get_list_view
};
pub(crate) use preview_model::preview_model;
pub use requests::Requests;
pub use tags::Tags;
pub use upload_file::{PutUploadFile, UploadData};
pub(crate) use util::{image_detector, three_detector, url_decode};
pub(crate) use response_parsing::{
    resp_parsing, resp_parsing_item, resp_parsing_two_level, get_value_response, get_from_value
};
