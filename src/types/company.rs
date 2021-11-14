// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
// use super::file::{ShowFileForDownload, DownloadFile};
// use super::relate::{Region, Program, TypeAccessTranslateListInfo};
use super::UUID;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimCompany{
  pub uuid: UUID,
  pub shortname: String,
  pub is_supplier: bool
}

// for arguments users query
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompaniesQueryArg {
    pub users_uuids:  Option<Vec<UUID>>,
    pub subscribers: Option<bool>,
    pub favorite: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl CompaniesQueryArg {
    pub fn set_favorite() -> Self {
        Self {
            favorite: Some(true),
            ..Default::default()
        }
    }
    pub fn set_subscribers() -> Self {
        Self {
            subscribers: Some(true),
            ..Default::default()
        }
    }
}
