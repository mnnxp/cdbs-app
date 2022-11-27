//! Api requests via yew FetchService

mod auth;
mod localisation;
mod local_storage;
mod requests;
mod tags;
mod util;
pub mod storage_upload;
// pub mod storage_upload::StorageUpload;
// pub mod storage_upload::storage_upload;

pub use auth::{Auth, get_current_user, logout};
pub use localisation::get_value_field;
pub use local_storage::{
    get_logged_user, set_logged_user, get_token, is_authenticated,
    set_lang, get_lang, set_token, set_list_view, get_list_view
};
pub use requests::Requests;
pub use tags::Tags;
// pub use storage_upload::StorageUpload;
pub(crate) use util::{image_detector, url_decode};
