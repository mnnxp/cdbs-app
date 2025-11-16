use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::Serialize;
// use log::debug;
use crate::services::{get_lang, get_server_locations, get_token};

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

    let gql_server_location = get_server_locations().1;
    let request = Request::new_with_str_and_init(&gql_server_location, &opts)?;

    if let Some(token) = get_token() {
        request.headers().set("Authorization", format!("Bearer {}", token).as_str()).unwrap();
    }
    if let Some(lang) = get_lang() {
        request.headers().set("Accept-Language", lang.as_str()).unwrap();
    }

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}
