use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::profiles::ShowUserShort;
use super::file::DownloadFile;
use super::relate::{Region, Spec};
use super::UUID;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompanyInfo {
    pub uuid: UUID,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: String,
    pub owner_user: ShowUserShort,
    pub image_file: DownloadFile,
    pub region: Region,
    pub company_represents: Vec<CompanyRepresent>,
    pub company_type: CompanyType,
    // show certificates company
    pub company_certificates: Vec<CompanyCertificate>,
    pub company_specs: Vec<CompanySpec>,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    // count users to folloded the company
    pub subscribers: usize,
    // for display the checkbox "favorites"
    pub is_followed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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
pub struct CompanyCertificate {
    pub company_uuid: UUID,
    pub file: DownloadFile,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompanySpec {
    pub spec: Spec,
    pub company_uuid: UUID,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompanyRepresent {
    pub uuid: UUID,
    pub company_uuid: UUID,
    pub region: Region,
    pub representation_type: RepresentationType,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RepresentationType {
    pub representation_type_id: usize,
    pub lang_id: usize,
    pub representation_type: String,
}
