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


    /// Get current user info
    pub fn user_info(&mut self, callback: Callback<Result<UserInfoWrapper, Error>>) -> FetchTask {
        self.requests
            .get::<UserInfoWrapper>("/user".to_string(), callback)
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

    /// Register a new user
    pub fn register(
        &mut self,
        register_info: RegisterInfoWrapper,
        callback: Callback<Result<SlimUserWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<RegisterInfoWrapper, SlimUserWrapper>(
            "/users".to_string(),
            register_info,
            callback,
        )
    }

    /// Query new token for logged users
    pub fn token_query() {
        unimplemented!()
    }

    /// Save info of current user
    pub fn save(
        &mut self,
        user_update_info: UserUpdateInfoWrapper,
        callback: Callback<Result<usize, Error>>,
    ) -> FetchTask {
        self.requests.put::<UserUpdateInfoWrapper, usize>(
            "/user".to_string(),
            user_update_info,
            callback,
        )
    }
}
