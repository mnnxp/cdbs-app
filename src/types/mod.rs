//! Common types

mod auth;
mod company;
mod component;
mod file;
mod profiles;
mod relate;
mod standard;
mod supplier_service;
mod tags;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use chrono::NaiveDateTime;

pub use auth::*;
pub use company::*;
pub use component::*;
pub use profiles::*;
pub use file::*;
pub use relate::*;
pub use standard::*;
pub use supplier_service::*;
pub use tags::*;

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

pub enum Pathname {
    Component(UUID),
    ComponentSetting(UUID),
    Company(UUID),
    CompanySetting(UUID),
    Standard(UUID),
    StandardSetting(UUID),
    Service(UUID),
    ServiceSetting(UUID),
    User(UUID),
    UserSetting,
}

impl Pathname {
    /// Returns pathname to specified object
    pub fn get_pathname(&self) -> String {
        match self {
            Self::Component(uuid) => format!("#/component/{}", uuid),
            Self::ComponentSetting(uuid) => format!("#/component/settings/{}", uuid),
            Self::Company(uuid) => format!("#/company/{}", uuid),
            Self::CompanySetting(uuid) => format!("#/company/settings/{}", uuid),
            Self::Standard(uuid) => format!("#/standard/{}", uuid),
            Self::StandardSetting(uuid) => format!("#/standard/settings/{}", uuid),
            Self::Service(uuid) => format!("#/service/{}", uuid),
            Self::ServiceSetting(uuid) => format!("#/service/settings/{}", uuid),
            Self::User(username) => format!("#/@{}", username),
            Self::UserSetting => format!("#/settings"),
        }
    }
}