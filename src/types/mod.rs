//! Common types

mod auth;
mod company;
mod component;
mod file;
mod profiles;
mod relate;
mod standard;
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
