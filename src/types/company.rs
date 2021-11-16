use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::file::DownloadFile;
use super::relate::Region;
use super::UUID;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowCompanyShort{
  pub uuid: UUID,
  pub shortname: String,
  pub inn: String,
  pub description: String,
  pub image_file: DownloadFile,
  pub region: Region,
  pub company_type: CompanyType,
  pub is_supplier: bool,
  pub is_followed: bool,
  pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompanyType{
  pub company_type_id: i64,
  pub lang_id: i64,
  pub name: String,
  pub shortname: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimCompany{
  pub uuid: UUID,
  pub shortname: String,
  pub is_supplier: bool
}

// for arguments users query
#[derive(PartialEq, Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompaniesQueryArg {
    pub companies_uuids:  Option<Vec<UUID>>,
    pub user_uuid:  Option<UUID>,
    pub favorite: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl CompaniesQueryArg {
    pub fn set_user_uuid(user_uuid: &UUID) -> Self {
        Self {
            user_uuid: Some(user_uuid.to_owned()),
            ..Default::default()
        }
    }
    pub fn set_favorite(user_uuid: Option<UUID>) -> Self {
        Self {
            favorite: Some(true),
            user_uuid,
            ..Default::default()
        }
    }
}
