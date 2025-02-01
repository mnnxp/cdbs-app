use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::profiles::ShowUserShort;
use super::file::DownloadFile;
use super::relate::{Region, Spec, TypeAccessInfo};
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
    pub company_represents: Vec<CompanyRepresentInfo>,
    pub company_type: CompanyType,
    // show certificates company
    pub company_certificates: Vec<CompanyCertificate>,
    pub company_specs: Vec<Spec>,
    pub type_access: TypeAccessInfo,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    // count users to folloded the company
    pub subscribers: usize,
    // for display the checkbox "favorites"
    pub is_followed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompanyCreateInfo {
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: String,
    pub region_id: i64,
    pub company_type_id: i64,
    pub type_access_id: i64,
}

impl CompanyCreateInfo {
    pub fn new() -> Self {
        Self {
            region_id: 1,
            company_type_id: 1,
            type_access_id: 1,
            ..Default::default()
        }
    }
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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
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
    pub supplier: Option<bool>,
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
pub struct CompanyRepresentInfo {
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
pub struct RegisterCompanyRepresentInfo {
    pub company_uuid: UUID,
    pub region_id: usize,
    pub representation_type_id: usize,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl Default for RegisterCompanyRepresentInfo {
    fn default() -> Self {
        Self {
            company_uuid: String::new(),
            region_id: 1,
            representation_type_id: 1,
            name: String::new(),
            address: String::new(),
            phone: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompanyRepresentUpdateInfo {
    pub region_id: Option<i64>,
    pub representation_type_id: Option<i64>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RepresentationType {
    pub representation_type_id: usize,
    pub lang_id: usize,
    pub representation_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompanyUpdateInfo {
    pub orgname: Option<String>,
    pub shortname: Option<String>,
    pub inn: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub site_url: Option<String>,
    pub time_zone: Option<String>,
    pub region_id: Option<i64>,
    pub company_type_id: Option<i64>,
}
