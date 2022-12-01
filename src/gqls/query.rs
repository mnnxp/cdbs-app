use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};
use dotenv_codegen::dotenv;
use serde::Serialize;
// use log::debug;
use crate::services::{get_token, get_lang};

const API_GPL: &str = dotenv!("API_GPL");

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

pub async fn make_query<T>(build_query: graphql_client::QueryBody<T>) -> Result<String, FetchError>
where
    T: Serialize,
{
    let query = serde_json::json!(build_query);
    // debug("Update: {:?}", query);
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(API_GPL, &opts)?;

    if let Some(token) = get_token() {
        request.headers().set("Authorization", format!("Bearer {}", token).as_str()).unwrap();
    }
    if let Some(lang) = get_lang() {
        request.headers().set("Accept-Language", lang.as_str()).unwrap();
    }

    let window = window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}
