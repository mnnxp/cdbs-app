use graphql_client::{GraphQLQuery, Response};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response as Res};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::future::LinkFuture;
use serde::ser::{SerializeStruct, Serializer};
use dotenv_codegen::dotenv;
use crate::services::{get_token};
use yew::services::ConsoleService;
use serde::{Deserialize, Serialize};

// use serde

pub const API_GPL: &str = dotenv!("API_GPL");

type ObjectId = String;

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
pub struct HttpHeaders {
  Authorization: String
}

// let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6ImlvazJAYnVkc2hvbWUuY29tIiwidXNlcm5hbWUiOiLmiJHmmK9vazIiLCJleHAiOjEwMDAwMDAwMDAwfQ.Gk98TjaFPpyW2Vdunn-pVqSPizP_zzTr89psBTE6zzfLQStUnBEXA2k0yVrS0CHBt9bHLLcFgmo4zYiioRBzBg";
//     let build_query = AllUsers::build_query(all_users::Variables {
//         token: token.to_string(),
//     });

pub async fn make_query<T>(build_query: graphql_client::QueryBody<T>) -> Result<String, FetchError>
where
    T: Serialize,
{
    let query = serde_json::json!(build_query);
    // ConsoleService::info(format!("Update: {:?}", query.to_string()).as_ref());
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);
    opts.headers(&JsValue::from_serde(&serde_json::json!(serde_json::json!(HttpHeaders{
      Authorization: get_token().unwrap()
    }))).unwrap());
    let url = String::from(API_GPL);
    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Res = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

// pub async fn make_query<T, Y>(get_query: Y, query:T) -> Result<String, FetchError>
// where
//     T: Serialize,
//     Y: Fn(T) -> graphql_client::QueryBody<T>,
// {
//     let builder: graphql_client::QueryBody<T> = get_query(query);
//     let query = serde_json::json!(builder);
//     let mut opts = RequestInit::new();
//     opts.method("POST");
//     opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
//     opts.mode(RequestMode::Cors);
//     let url = String::from(API_GPL);
//     let request = Request::new_with_str_and_init(url.as_str(), &opts)?;

//     let window = yew::utils::window();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
//     let resp: Res = resp_value.dyn_into().unwrap();

//     let text = JsFuture::from(resp.text()?).await?;
//     Ok(text.as_string().unwrap())
// }