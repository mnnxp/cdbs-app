use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
// use super::file::{ShowFileForDownload, DownloadFile};
// use super::relate::{Region, Program, TypeAccessTranslateListInfo};
use super::company::SlimCompany;
use super::UUID;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowComponentShort {
  pub uuid : UUID,
  pub name: String,
  pub description: String,
  pub type_access_id: usize,
  pub is_followed: bool,
  pub is_base: bool,
  pub updated_at: NaiveDateTime,
  pub component_suppliers: Vec<Supplier>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Supplier{
  pub supplier:SlimCompany,
  pub component_uuid: UUID,
  pub description: String
}

// for arguments users query
#[derive(PartialEq, Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentsQueryArg {
    pub components_uuids: Option<Vec<UUID>>,
    pub company_uuid: Option<UUID>,
    pub standard_uuid: Option<UUID>,
    pub user_uuid: Option<UUID>,
    pub favorite: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl ComponentsQueryArg {
    pub fn set_user_uuid(user_uuid: &UUID) -> Self {
        Self {
            user_uuid: Some(user_uuid.to_owned()),
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
