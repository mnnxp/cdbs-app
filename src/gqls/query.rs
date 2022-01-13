use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response as Res};
use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};
// use log::debug;
use crate::services::{get_token};

// use serde

const BACKEND_HOST: &str = dotenv!("BACKEND_HOST");
const BACKEND_PORT: &str = dotenv!("BACKEND_PORT");
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HttpHeaders {
  authorization: String
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
    if get_token().is_some() {
      opts.headers(&JsValue::from_serde(&serde_json::json!(serde_json::json!(HttpHeaders{
        authorization: format!("Bearer {}", get_token().unwrap())
      }))).unwrap());
    }

    let url = format!(
        "http://{}:{}/{}",
        BACKEND_HOST,
        BACKEND_PORT,
        API_GPL
    );
    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Res = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}
