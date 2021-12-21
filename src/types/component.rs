use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::{
    UUID, SlimCompany, TypeAccessInfo, ShowStandardShort, ShowUserShort,
    ShowFileInfo, DownloadFile, LicenseInfo, Spec, Keyword, Program
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInfo {
  pub uuid: UUID,
  pub parent_component_uuid: UUID,
  pub name: String,
  pub description: String,
  pub owner_user: ShowUserShort,
  pub type_access: TypeAccessInfo,
  pub component_type: ComponentType,
  pub actual_status: ActualStatus,
  pub is_base: bool,
  pub subscribers: usize,
  pub is_followed: bool,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub licenses: Vec<LicenseInfo>,
  pub component_params: Vec<ComponentParam>,
  pub files: Vec<ShowFileInfo>,
  pub component_specs: Vec<Spec>,
  pub component_keywords: Vec<Keyword>,
  pub component_modifications: Vec<ComponentModificationInfo>,
  pub component_suppliers: Vec<Supplier>,
  pub component_standards: Vec<ShowStandardShort>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowComponentShort {
    pub uuid: UUID,
    pub name: String,
    pub description: String,
    pub owner_user: ShowUserShort,
    pub type_access: TypeAccessInfo,
    pub component_type: ComponentType,
    pub actual_status: ActualStatus,
    pub is_followed: bool,
    pub is_base: bool,
    pub updated_at: NaiveDateTime,
    pub licenses: Vec<LicenseInfo>,
    pub files: Vec<DownloadFile>, // images
    pub component_suppliers: Vec<Supplier>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Supplier{
  pub supplier:SlimCompany,
  pub component_uuid: UUID,
  pub description: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentType{
  pub component_type_id: usize,
  pub lang_id: usize,
  pub component_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActualStatus{
  pub actual_status_id: usize,
  pub lang_id: usize,
  pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentParam{
  pub component_uuid: UUID,
  pub param: Param,
  pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Param{
  pub param_id: usize,
  pub lang_id: usize,
  pub paramname: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentModificationInfo{
  pub uuid: UUID,
  pub component_uuid: UUID,
  pub parent_modification_uuid: UUID,
  pub modification_name: String,
  pub description: String,
  pub actual_status: ActualStatus,
  pub updated_at: NaiveDateTime,
  pub filesets_for_program: Vec<FilesetProgramInfo>,
  pub modification_params: Vec<ModificationParam>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModificationParam{
  pub modification_uuid: UUID,
  pub param: Param,
  pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FilesetProgramInfo{
  pub uuid: UUID,
  pub modification_uuid: UUID,
  pub program: Program,
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
            user_uuid: Some(user_uuid.clone()),
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

    pub fn set_company_uuid(company_uuid: &UUID) -> Self {
        Self {
            company_uuid: Some(company_uuid.clone()),
            ..Default::default()
        }
    }

    pub fn set_standard_uuid(standard_uuid: &UUID) -> Self {
        Self {
            standard_uuid: Some(standard_uuid.clone()),
            ..Default::default()
        }
    }
}
