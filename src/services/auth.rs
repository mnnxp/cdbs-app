use yew::callback::Callback;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use super::Requests;
use crate::error::Error;
use crate::types::{SlimUser, LoginInfoWrapper, UserToken};
use crate::services::{get_logged_user, resp_parsing_item, set_logged_user};
use crate::gqls::make_query;
use crate::gqls::user::{
    GetMySelf, get_my_self,
    Logout, logout,
};

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
        callback: Callback<Result<Option<UserToken>, Error>>,
    ) {
        self.requests.post::<LoginInfoWrapper, UserToken>(
            "/login",
            login_info,
            callback,
        )
    }
}

/// Get slim data for current user
pub async fn get_current_user() -> Result<SlimUser, Error> {
    // check data in local storage
    match get_logged_user() {
        Some(x) => Ok(x),
        None => {
            let res = make_query(GetMySelf::build_query(get_my_self::Variables)).await.unwrap();
            let user_json: Value = resp_parsing_item(res, "myself").unwrap();
            // save data in local storage
            set_logged_user(Some(user_json.to_string()));
            let slim_user: SlimUser = serde_json::from_value(user_json).unwrap();
            debug!("SlimUser data: {:?}", slim_user);
            // *current_user = Ok(slim_user);
            Ok(slim_user)
        },
    }
}

/// Logout user
pub async fn logout() -> String {
    let res = make_query(Logout::build_query(logout::Variables)).await.unwrap();
    resp_parsing_item(res, "logout")
        .map_err(|err| {
            debug!("fail logout: {:?}", err);
            String::from("fail logout")
        })
        .unwrap()
}
