use yew::{Classes, classes};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::{
    Keyword, Param, Region, ShowCompanyShort, ShowFileInfo,
    DownloadFile, ShowUserShort, Spec, UUID
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    pub uuid: UUID,
    pub name: String,
    pub description: String,
    pub owner_user: ShowUserShort,
    pub owner_company: ShowCompanyShort,
    // pub type_access: TypeAccessInfo,
    pub service_status: ServiceStatus,
    pub region: Region,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub params_count: i64,
    pub files_count: i64,
    // related data
    // pub service_params: Vec<ParamValue>,
    pub files: Vec<ShowFileInfo>,
    pub service_specs: Vec<Spec>,
    pub service_keywords: Vec<Keyword>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCreateData {
    pub name: String,
    pub description: String,
    pub company_uuid: UUID,
    pub region_id: usize,
}

impl ServiceCreateData {
    pub fn new() -> Self {
        Self{
            name: String::default(),
            description: String::default(),
            company_uuid: String::default(),
            region_id: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowServiceShort {
    pub uuid: UUID,
    pub name: String,
    pub description: String,
    pub owner_user: ShowUserShort,
    pub owner_company: ShowCompanyShort,
    pub service_status: ServiceStatus,
    pub files: Vec<DownloadFile>,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus{
  pub service_status_id: usize,
  pub lang_id: usize,
  pub name: String,
}

impl ServiceStatus {
    /// Returns class for CSS highlighting according to status
    pub(crate) fn get_class_color(&self) -> Classes {
        match self.service_status_id {
            1 => classes!(""),
            2 => classes!("has-background-warning-light"),
            3 => classes!("has-background-success-light"),
            4 => classes!("has-background-link-light"),
            _ => classes!("has-background-danger-light"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUpdatePreData {
    pub name: String,
    pub description: String,
    pub region_id: usize,
}

impl From<ServiceInfo> for ServiceUpdatePreData {
    fn from(data: ServiceInfo) -> Self {
        Self {
            name: data.name,
            description: data.description,
            region_id: data.region.region_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUpdateData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub region_id: Option<i64>,
}

impl From<&ServiceUpdatePreData> for ServiceUpdateData {
    fn from(new_data: &ServiceUpdatePreData) -> Self {
        Self {
            name: Some(new_data.name.clone()),
            description: Some(new_data.description.clone()),
            region_id: Some(new_data.region_id as i64),
        }
    }
}

// for arguments users query
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServicesQueryArg {
    pub services_uuids:  Option<Vec<UUID>>,
    pub company_uuid:  Option<UUID>,
    pub user_uuid:  Option<UUID>,
}

impl ServicesQueryArg {
    pub fn set_company_uuid(company_uuid: &UUID) -> Self {
        Self {
            company_uuid: Some(company_uuid.to_owned()),
            ..Default::default()
        }
    }

    pub fn set_user_uuid(user_uuid: &UUID) -> Self {
        Self {
            user_uuid: Some(user_uuid.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceParam{
  pub service_uuid: UUID,
  pub param: Param,
  pub value: String,
}

#[derive(Clone, PartialEq)]
pub struct PreServiceRequestData {
    pub company_uuid: UUID,
    pub calc_params: Vec<(String, String)>,
    pub cost: f64,
}