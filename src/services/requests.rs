use reqwest::{Client, Response, Body, RequestBuilder};
// use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::header::CONTENT_TYPE;
use dotenv_codegen::dotenv;
use log::debug;
use serde::{Deserialize, Serialize};
use yew::callback::Callback;
use wasm_bindgen_futures::spawn_local;
// use yew::format::{Json, Nothing, Text, Binary};
// use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::error::Error;
use crate::services::get_token;
use crate::types::ErrorInfo;

const API_BACKEND: &str = dotenv!("API_BACKEND");

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
        path: &str,
        body_data: Option<B>,
        body_json: bool,
        get_body: bool,
        callback: Callback<Result<Option<T>, Error>>,
    ) // -> Result<Response, reqwest::Error>
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize + Into<Body> + std::fmt::Debug,
    {
        let url = match path.get(0..4) {
            Some("http") => path,
            _ => format!("{}{}", API_BACKEND, path).as_str(),
        };
        debug!("complect url: {}", url);
        let client = Client::new();
        let mut req = match method {
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => client.get(url),
        };
        req.header(CONTENT_TYPE, "application/json");
        if let Some(token) = get_token() {
            // req.header(AUTHORIZATION, format!("Token {}", token));
            req.bearer_auth(token);
        }
        if let Some(body) = body_data {
            if body_json {
                req.json(&body);
            } else {
                req.body(body);
            }
        }
        debug!("Request: {:?}", req);
        self.handler(req, get_body, callback);

        // spawn_local(async move {
        //     let resp = req.send().await;
        //         .expect("failed to get response")
        //         .text()
        //         .await
        //         .expect("failed to get payload");
        //     self.handler(req, get_body, callback);
        // });
    }

    /// Delete request
    pub fn delete<T>(&mut self, path: String, callback: Callback<Result<Option<T>, Error>>)
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        let no_body: Option<String> = None;
        self.builder("DELETE", path.as_str(), no_body, false, true, callback);
        // self.handler(resp, true, callback);
    }

    /// Get request
    pub fn get<T>(&mut self, path: &str, callback: Callback<Result<Option<T>, Error>>)
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        let no_body: Option<String> = None;
        self.builder("GET", path, no_body, false, true, callback);
        // self.handler(resp, true, callback);
    }

    /// Post request with a body
    pub fn post<B, T>(
        &mut self,
        path: &str,
        body: B,
        callback: Callback<Result<Option<T>, Error>>,
    )
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize + Into<Body> + std::fmt::Debug,
    {
        self.builder("POST", path, Some(body), true, true, callback);
        // self.handler(resp, true, callback);
    }

    /// Put request with a body
    pub fn put<B, T>(
        &mut self,
        path: String,
        body: B,
        callback: Callback<Result<Option<T>, Error>>,
    )
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize + Into<Body> + std::fmt::Debug,
    {
        self.builder("PUT", path.as_str(), Some(body), true, true, callback);
        // self.handler(resp, true, callback);
    }

    /// Put request for send file to storage
    pub fn put_file<B, T>(
        &mut self,
        url: &str,
        body: &B,
        callback: Callback<Result<Option<T>, Error>>,
    )
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize + Into<Body> + std::fmt::Debug,
    {
        self.builder("PUT", url, Some(*body), false, false, callback);
        // self.handler(resp, false, callback)
    }

    fn handler<T>(
        &mut self,
        request: RequestBuilder,
        // response: Result<Response, reqwest::Error>,
        get_body: bool,
        callback: Callback<Result<Option<T>, Error>>,
    )
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        spawn_local(async move {
            let response: Result<Response, reqwest::Error> = request.send().await;
            if let Ok(resp) = response {
                    if resp.status().is_success() {
                        debug!("success!");
                        if get_body {
                            let data: Result<T, _> = resp.json().await;
                            if let Ok(data) = data {
                                callback.emit(Ok(Some(data)));
                            } else {
                                callback.emit(Err(Error::DeserializeError));
                            }
                        } else {
                            let data = resp.text().await;
                            debug!("Data: {:?}", data);
                            match data {
                                Ok(_d) if _d.is_empty() => callback.emit(Ok(None)),
                                Ok(_) => callback.emit(Err(Error::InternalServerError)),
                                Err(err) => {
                                    debug!("Error processing response: {:?}", err);
                                    callback.emit(Err(Error::InternalServerError));
                                },
                            }
                        }
                    } else {
                        match resp.status().as_u16() {
                            401 => callback.emit(Err(Error::Unauthorized)),
                            403 => callback.emit(Err(Error::Forbidden)),
                            404 => callback.emit(Err(Error::NotFound)),
                            422 => {
                                let data: Result<ErrorInfo, _> = resp.json().await;
                                if let Ok(data) = data {
                                    callback.emit(Err(Error::UnprocessableEntity(data)));
                                } else {
                                    callback.emit(Err(Error::DeserializeError));
                                }
                            }
                            500 => callback.emit(Err(Error::InternalServerError)),
                            _ => {
                                debug!("Something else happened. Status: {:?}", resp.status());
                                callback.emit(Err(Error::RequestError));
                            },
                        }
                    }
            } else {
                debug!("Something happened...: {:?}", response);
                callback.emit(Err(Error::RequestError));
            }
        });
    }
}
