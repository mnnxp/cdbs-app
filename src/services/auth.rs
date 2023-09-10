use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use graphql_client::GraphQLQuery;
use log::debug;

use super::{Requests, resp_parsing};
use crate::error::Error;
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
pub async fn get_current_user() -> Result<SlimUser, Error> {
    // check data in local storage
    match get_logged_user() {
        Some(x) => Ok(x),
        None => {
            let res = make_query(GetMySelf::build_query(get_my_self::Variables)).await.unwrap();
            match resp_parsing::<SlimUser>(res, "myself") {
                Ok(slim_user) => {
                    debug!("SlimUser data: {:?}", slim_user);
                    // save data in local storage
                    set_logged_user(Some(serde_json::to_string(&slim_user).unwrap()));
                    // *current_user = Ok(slim_user);
                    Ok(slim_user)
                },
                Err(err) => {
                    debug!("Logged error: {:?}", err);
                    Err(err)
                },
            }
        },
    }
}

/// Logout user
pub async fn logout() -> String {
    let res = make_query(Logout::build_query(logout::Variables)).await.unwrap();
    match resp_parsing(res, "logout") {
        Ok(result) => result,
        Err(err) => {
            debug!("fail logout: {:?}", err);
            String::from("fail logout")
        },
    }
}
