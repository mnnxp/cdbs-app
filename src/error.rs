//! Error type for error handling

use thiserror::Error as ThisError;
use serde_json::Value;
use log::debug;
use crate::types::ErrorInfo;
use crate::services::{set_token, set_logged_user};

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum Error {
    /// 400
    #[error("{0}")]
    BadRequest(String),

    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 422
    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Http Request Error")]
    RequestError,
}

/// Get error message from response
pub(crate) fn get_error(data: &Value) -> Error {
    let val_err = data.as_object().unwrap().get("errors").unwrap();

    let err_message: String =
        serde_json::from_value(val_err.get(0).unwrap().get("message").unwrap().clone()).unwrap();

    debug!("Err message: {:?}", err_message);

    match err_message.as_str() {
        "Unauthorized" => {
            // clean storage if the token has expired
            set_token(None);
            set_logged_user(None);
            Error::Unauthorized
        },
        // "Not Found" => Error::NotFound,
        _ => Error::BadRequest(err_message),

    }
}
