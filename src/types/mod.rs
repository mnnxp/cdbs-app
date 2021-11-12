//! Common types

mod auth;
mod file;
mod profiles;
mod relate;
mod tags;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use chrono::NaiveDateTime;

pub use auth::{
    LoginInfo, LoginInfoWrapper, UpdatePasswordInfo, RegisterInfo, SlimUser,
    UserUpdateInfo, UserToken
};
pub use profiles::{
    SelfUserInfo, UserInfo, ShowUserShort, UserCertificate, ShowNotification, DegreeImportanceTranslateList,
};
pub use file::{
    ShowFileForDownload, DownloadFile, UploadFile,
};
pub use relate::{
    Region, Program, Certificate, TypeAccessTranslateListInfo
};
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
// pub type NaiveDateTime = NaiveDateTime;
