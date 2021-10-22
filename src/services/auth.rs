use yew::callback::Callback;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

use graphql_client::GraphQLQuery;
use serde_json::Value;

use super::Requests;
use crate::error::{Error, get_error};
use crate::types::*;
use crate::gqls::make_query;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetMySelf;

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
    let req = make_query(
        GetMySelf::build_query(get_my_self::Variables)
    ).await.unwrap();

    let data: Value = serde_json::from_str(req.as_str()).unwrap();

    let res = data.as_object().unwrap().get("data").unwrap();

    match res.is_null() {
        false => {
            let slim_user: SlimUser = serde_json::from_value(res.get("myself").unwrap().clone()).unwrap();
            ConsoleService::info(format!("SlimUser data: {:?}", slim_user).as_ref());
            // *current_user = Ok(slim_user);
            Ok(slim_user)
        },
        true => {
            // *current_user = Err(get_error(&data));
            Err(get_error(&data))
        },
    }
}
