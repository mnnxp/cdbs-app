//! Api requests via yew FetchService

mod auth;
mod requests;
mod tags;
mod upload_file;
mod util;

pub use auth::{Auth, get_current_user, logout};
pub use requests::{
    get_logged_user, set_logged_user, get_token, is_authenticated, 
    set_lang, get_lang, limit, set_token, Requests
};
pub use tags::Tags;
pub use upload_file::{PutUploadFile, UploadData};
pub(crate) use util::image_detector;
