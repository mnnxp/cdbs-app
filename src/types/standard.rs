use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::{
    UUID, ShowUserShort, ShowCompanyShort, Region, TypeAccessInfo,
    ShowFileInfo, DownloadFile, Spec, Keyword,
};

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
    pub type_access: TypeAccessInfo,
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
  pub standard_status_id: usize,
  pub lang_id: usize,
  pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardSpec {
    pub spec: Spec,
    pub standard_uuid: UUID,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StandardUpdatePreData {
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: Option<NaiveDateTime>,
    pub company_uuid: UUID,
    pub standard_status_id: usize,
    pub region_id: usize,
}

impl From<StandardInfo> for StandardUpdatePreData {
    fn from(data: StandardInfo) -> Self {
        Self {
            classifier: data.classifier,
            name: data.name,
            description: data.description,
            specified_tolerance: data.specified_tolerance,
            technical_committee: data.technical_committee,
            publication_at: Some(data.publication_at),
            company_uuid: data.owner_company.uuid,
            standard_status_id: data.standard_status.standard_status_id,
            region_id: data.region.region_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StandardUpdateData {
    pub classifier: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub specified_tolerance: Option<String>,
    pub technical_committee: Option<String>,
    pub publication_at: Option<NaiveDateTime>,
    pub company_uuid: Option<UUID>,
    pub standard_status_id: Option<i64>,
    pub region_id: Option<i64>,
}

impl From<&StandardUpdatePreData> for StandardUpdateData {
    fn from(new_data: &StandardUpdatePreData) -> Self {
        Self {
            classifier: Some(new_data.classifier.clone()),
            name: Some(new_data.name.clone()),
            description: Some(new_data.description.clone()),
            specified_tolerance: Some(new_data.specified_tolerance.clone()),
            technical_committee: Some(new_data.technical_committee.clone()),
            publication_at: new_data.publication_at.clone(),
            company_uuid: Some(new_data.company_uuid.clone()),
            standard_status_id: Some(new_data.standard_status_id as i64),
            region_id: Some(new_data.region_id as i64),
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
