use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::{
    ShowUserShort, ShowCompanyShort, Region,
    ShowFileInfo, DownloadFile, Spec, Keyword
};
use super::UUID;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardInfo {
    pub uuid: UUID,
    pub parent_standard_uuid: UUID,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub image_file: DownloadFile,
    pub owner_user: ShowUserShort,
    pub owner_company: ShowCompanyShort,
    pub type_access_id: usize,
    pub standard_status: StandardStatus,
    pub region: Region,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub standard_files: Vec<ShowFileInfo>, // <-- documentation files, etc.
    pub standard_specs: Vec<StandardSpec>,
    pub standard_keywords: Vec<Keyword>,
    // count users to folloded the standard
    pub subscribers: usize,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

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
    pub standard_status: StandardStatus,
    pub updated_at: NaiveDateTime,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardStatus{
  pub standard_status_id: i64,
  pub lang_id: i64,
  pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardSpec {
    pub spec: Spec,
    pub standard_uuid: UUID,
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
