use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::company::ShowCompanyShort;
use super::UUID;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowStandardShort {
    pub uuid: UUID,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub publication_at: NaiveDateTime,
    pub owner_company: ShowCompanyShort,
    pub standard_status: StandardStatusTranslateList,
    pub updated_at: NaiveDateTime,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardStatusTranslateList{
  pub standard_status_id: i64,
  pub lang_id: i64,
  pub name: String,
}

// for arguments users query
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardsQueryArg {
    pub standards_uuids:  Option<Vec<UUID>>,
    pub company_uuid:  Option<UUID>,
    pub favorite: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl StandardsQueryArg {
    pub fn set_company_uuid(company_uuid: &UUID) -> Self {
        Self {
            company_uuid: Some(company_uuid.to_owned()),
            ..Default::default()
        }
    }
    pub fn set_favorite() -> Self {
        Self {
            favorite: Some(true),
            ..Default::default()
        }
    }
}
