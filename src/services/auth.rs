use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use super::Requests;
use crate::error::Error;
use crate::types::*;

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

    /// Get current user slim data
    pub fn current(&mut self, callback: Callback<Result<SlimUserWrapper, Error>>) -> FetchTask {
        self.requests
            .get::<SlimUserWrapper>("/me".to_string(), callback)
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
