use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use log::debug;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use yew::callback::Callback;
use yew::format::{Json, Nothing, Text, Binary};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

use crate::error::Error;
use crate::types::{SlimUser, ErrorInfo};

const API_BACKEND: &str = dotenv!("API_BACKEND");

const TOKEN_KEY: &str = dotenv!("TOKEN_KEY");
const LOGGED_USER_KEY: &str = dotenv!("LOGGED_USER_KEY");

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(token) = storage.restore(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };

    /// Read SlimUser data from local storage.
    pub static ref LOGGED_USER: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(logged_user) = storage.restore(LOGGED_USER_KEY) {
            RwLock::new(Some(logged_user))
        } else {
            RwLock::new(None)
        }
    };
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(t) = token.clone() {
        storage.store(TOKEN_KEY, Ok(t));
    } else {
        storage.remove(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

/// Check if current user is authenticated.
pub fn is_authenticated() -> bool {
    get_token().is_some()
}

/// Set current user to local storage.
pub fn set_logged_user(logged_user: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(u) = logged_user.clone() {
        storage.store(LOGGED_USER_KEY, Ok(u));
    } else {
        storage.remove(LOGGED_USER_KEY);
    }
    let mut logged_user_lock = LOGGED_USER.write();
    *logged_user_lock = logged_user;
}

/// Get authenticated user from lazy static.
pub fn get_logged_user() -> Option<SlimUser> {
    let logged_user_lock = LOGGED_USER.read();
    let logged_user_lock: Option<SlimUser> = serde_json::from_str(
        &logged_user_lock.clone().unwrap_or_default()
      ).unwrap_or_default();
    logged_user_lock.clone()
}

/// Http request
#[derive(Default, Debug, Clone)]
pub struct Requests {}

impl Requests {
    pub fn new() -> Self {
        Self {}
    }

    /// build all kinds of http request: post/get/delete etc.
    pub fn builder<B, T>(
        &mut self,
        method: &str,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Into<Text> + std::fmt::Debug,
    {
        let handler = move |response: Response<Text>| {
            if let (meta, Ok(data)) = response.into_parts() {
                debug!("Response: {:?}", data);
                if meta.status.is_success() {
                    let data: Result<T, _> = serde_json::from_str(&data);
                    if let Ok(data) = data {
                        callback.emit(Ok(data))
                    } else {
                        callback.emit(Err(Error::DeserializeError))
                    }
                } else {
                    match meta.status.as_u16() {
                        401 => callback.emit(Err(Error::Unauthorized)),
                        403 => callback.emit(Err(Error::Forbidden)),
                        404 => callback.emit(Err(Error::NotFound)),
                        500 => callback.emit(Err(Error::InternalServerError)),
                        422 => {
                            let data: Result<ErrorInfo, _> = serde_json::from_str(&data);
                            if let Ok(data) = data {
                                callback.emit(Err(Error::UnprocessableEntity(data)))
                            } else {
                                callback.emit(Err(Error::DeserializeError))
                            }
                        }
                        _ => callback.emit(Err(Error::RequestError)),
                    }
                }
            } else {
                callback.emit(Err(Error::RequestError))
            }
        };

        let url = format!("{}{}", API_BACKEND, url);
        debug!("complect url: {}", url);
        let mut builder = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("Content-Type", "application/json");
        if let Some(token) = get_token() {
            builder = builder.header("Authorization", format!("Token {}", token));
        }
        let request = builder.body(body).unwrap();
        debug!("Request: {:?}", request);

        FetchService::fetch(request, handler.into()).unwrap()
    }

    /// Delete request
    pub fn delete<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("DELETE", url, Nothing, callback)
    }

    /// Get request
    pub fn get<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("GET", url, Nothing, callback)
    }

    /// Post request with a body
    pub fn post<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("POST", url, body, callback)
    }

    /// Put request with a body
    pub fn put<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("PUT", url, body, callback)
    }

    /// Put request for send file to storage
    pub fn put_f<T>(
        &mut self,
        url: String,
        body: Vec<u8>,
        callback: Callback<Result<Option<T>, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        let handler = move |response: Response<Binary>| {
            if let (meta, Ok(data)) = response.into_parts() {
                debug!("Response: {:?}", data);
                debug!("Meta status: {:?}", meta.status.is_success());
                if meta.status.is_success() {
                    debug!("Data: {:?}", data);
                    if data.is_empty() {
                        callback.emit(Ok(None))
                    } else {
                        callback.emit(Err(Error::InternalServerError))
                    }
                } else {
                    match meta.status.as_u16() {
                        401 => callback.emit(Err(Error::Unauthorized)),
                        403 => callback.emit(Err(Error::Forbidden)),
                        404 => callback.emit(Err(Error::NotFound)),
                        500 => callback.emit(Err(Error::InternalServerError)),
                        422 => {
                            let data: Result<ErrorInfo, _> = serde_json::from_slice(&data);
                            if let Ok(data) = data {
                                callback.emit(Err(Error::UnprocessableEntity(data)))
                            } else {
                                callback.emit(Err(Error::DeserializeError))
                            }
                        }
                        _ => callback.emit(Err(Error::RequestError)),
                    }
                }
            } else {
                callback.emit(Err(Error::RequestError))
            }
        };

        let body: Binary = Ok(body);

        let builder = Request::builder()
            .method("PUT")
            .uri(url.as_str());

        let request = builder.body(body).unwrap();
        debug!("Request: {:?}", request);

        FetchService::fetch_binary(request, handler.into()).unwrap()
    }
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
