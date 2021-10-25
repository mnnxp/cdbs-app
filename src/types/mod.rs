//! Common types

mod auth;
mod file;
mod profiles;
mod relate;
mod tags;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use auth::{
    LoginInfo, LoginInfoWrapper, RegisterInfo, SlimUser,
    UserUpdateInfo, UserToken
};
pub use profiles::{SelfUserInfo, UserInfo};
pub use file::ShowFileForDownload;
pub use relate::{Region, Program};
pub use tags::TagListInfo;

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;

/// For GraphQLQuery
pub type UUID = String;
