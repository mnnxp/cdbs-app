use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::{
    UUID, ShowUserShort, ShowCompanyShort, TypeAccessInfo,
    ShowFileInfo, DownloadFile, Spec, Keyword,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardInfo {
    pub uuid: UUID,
    pub parent_standard_uuid: UUID,
    pub name: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
    pub image_file: DownloadFile,
    pub owner_user: ShowUserShort,
    pub owner_company: ShowCompanyShort,
    pub type_access: TypeAccessInfo,
    pub standard_status: StandardStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub standard_files: Vec<ShowFileInfo>, // <-- documentation files, etc.
    pub standard_specs: Vec<Spec>,
    pub standard_keywords: Vec<Keyword>,
    // count users to folloded the standard
    pub subscribers: usize,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardCreateData {
    pub parent_standard_uuid: Option<UUID>,
    pub name: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
    pub company_uuid: UUID,
    pub type_access_id: usize,
    pub standard_status_id: usize,
}

impl StandardCreateData {
    pub fn new() -> Self {
        Self{
            parent_standard_uuid: None,
            name: String::default(),
            description: String::default(),
            publication_at: NaiveDateTime::from_timestamp(1_000_000_000, 0),
            company_uuid: String::default(),
            type_access_id: 1,
            standard_status_id: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowStandardShort {
    pub uuid: UUID,
    pub name: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
    pub image_file: DownloadFile,
    pub owner_company: ShowCompanyShort,
    pub standard_status: StandardStatus,
    pub updated_at: NaiveDateTime,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardStatus{
  pub standard_status_id: usize,
  pub lang_id: usize,
  pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StandardUpdatePreData {
    pub name: String,
    pub description: String,
    pub publication_at: Option<NaiveDateTime>,
    pub company_uuid: UUID,
    pub standard_status_id: usize,
}

impl From<StandardInfo> for StandardUpdatePreData {
    fn from(data: StandardInfo) -> Self {
        Self {
            name: data.name,
            description: data.description,
            publication_at: Some(data.publication_at),
            company_uuid: data.owner_company.uuid,
            standard_status_id: data.standard_status.standard_status_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StandardUpdateData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub publication_at: Option<NaiveDateTime>,
    pub company_uuid: Option<UUID>,
    pub standard_status_id: Option<i64>,
}

impl From<&StandardUpdatePreData> for StandardUpdateData {
    fn from(new_data: &StandardUpdatePreData) -> Self {
        Self {
            name: Some(new_data.name.clone()),
            description: Some(new_data.description.clone()),
            publication_at: new_data.publication_at.clone(),
            company_uuid: Some(new_data.company_uuid.clone()),
            standard_status_id: Some(new_data.standard_status_id as i64),
        }
    }
}

// for arguments users query
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardsQueryArg {
    pub standards_uuids:  Option<Vec<UUID>>,
    pub company_uuid:  Option<UUID>,
    pub favorite: Option<bool>,
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
