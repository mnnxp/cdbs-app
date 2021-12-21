use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;

use super::Requests;
use crate::error::{Error, get_error};
use crate::types::*;
use crate::services::{get_logged_user, set_logged_user};
use crate::gqls::make_query;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetMySelf;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct Logout;


/// Apis for authentication
#[derive(Default, Debug)]
pub struct Auth {
    requests: Requests,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Login a user
    pub fn login(
        &mut self,
        login_info: LoginInfoWrapper,
        callback: Callback<Result<UserToken, Error>>,
    ) -> FetchTask {
        self.requests.post::<LoginInfoWrapper, UserToken>(
            "/login".to_string(),
            login_info,
            callback,
        )
    }
}

/// Get slim data for current user
pub async fn get_current_user(
    // current_user: &mut Result<SlimUser, Error>,
    // error: &'static mut Option<Error>,
) -> Result<SlimUser, Error> {
    // check data in local storage
    match get_logged_user() {
        Some(x) => Ok(x),
        None => {
            let req = make_query(GetMySelf::build_query(
                get_my_self::Variables
            )).await.unwrap();

            let data: Value = serde_json::from_str(req.as_str()).unwrap();

            let res = data.as_object().unwrap().get("data").unwrap();

            match res.is_null() {
                false => {
                    let user_json = res.get("myself").unwrap().clone();
                    // save data in local storage
                    set_logged_user(Some(user_json.to_string()));

                    let slim_user: SlimUser = serde_json::from_value(user_json).unwrap();
                    debug!("SlimUser data: {:?}", slim_user);
                    // *current_user = Ok(slim_user);
                    Ok(slim_user)
                },
                true => Err(get_error(&data)),
            }
        },
    }
}

/// Logout user
pub async fn logout() -> String {
    let req = make_query(Logout::build_query(logout::Variables)).await.unwrap();
    let data: Value = serde_json::from_str(req.as_str()).unwrap();
    let res = data.as_object().unwrap().get("data").unwrap();

    match res.is_null() {
        false => serde_json::from_value(res.get("logout").unwrap().clone()).unwrap(),
        true => {
            debug!("fail logout: {:?}", res);
            String::from("fail logout")
        },
    }
}
