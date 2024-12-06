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
  pub image_file: DownloadFile,
  pub owner_user: ShowUserShort,
  pub type_access: TypeAccessInfo,
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
//   pub component_modifications: Vec<ComponentModificationInfo>,
  pub component_suppliers: Vec<Supplier>,
  pub component_standards: Vec<ShowStandardShort>,
  pub params_count: i64,
  pub files_count: i64,
  pub modifications_count: i64,
  pub suppliers_count: i64,
  pub standards_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCreateData {
    pub parent_component_uuid: Option<UUID>,
    pub name: String,
    pub description: String,
    pub type_access_id: usize,
    pub component_type_id: usize,
    pub actual_status_id: usize,
    pub is_base: bool,
}

impl ComponentCreateData {
    pub fn new() -> Self {
        Self{
            parent_component_uuid: None,
            name: String::default(),
            description: String::default(),
            type_access_id: 1,
            component_type_id: 1,
            actual_status_id: 1,
            is_base: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowComponentShort {
    pub uuid: UUID,
    pub name: String,
    pub description: String,
    pub image_file: DownloadFile,
    pub owner_user: ShowUserShort,
    pub type_access: TypeAccessInfo,
    pub actual_status: ActualStatus,
    pub is_followed: bool,
    pub is_base: bool,
    pub updated_at: NaiveDateTime,
    pub licenses: Vec<LicenseInfo>,
    pub files: Vec<DownloadFile>, // images
    pub component_suppliers: Vec<Supplier>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentUpdatePreData {
    pub parent_component_uuid: UUID,
    pub name: String,
    pub description: String,
    pub component_type_id: usize,
    pub actual_status_id: usize,
}

impl From<ComponentInfo> for ComponentUpdatePreData {
    fn from(data: ComponentInfo) -> Self {
        Self {
            parent_component_uuid: data.parent_component_uuid,
            name: data.name,
            description: data.description,
            component_type_id: 1,
            actual_status_id: data.actual_status.actual_status_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentUpdateData {
    pub parent_component_uuid: Option<UUID>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub component_type_id: Option<i64>,
    pub actual_status_id: Option<i64>,
}

impl From<&ComponentUpdatePreData> for ComponentUpdateData {
    fn from(new_data: &ComponentUpdatePreData) -> Self {
        Self {
            parent_component_uuid: Some(new_data.parent_component_uuid.clone()),
            name: Some(new_data.name.clone()),
            description: Some(new_data.description.clone()),
            component_type_id: None,
            actual_status_id: Some(new_data.actual_status_id as i64),
        }
    }
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Param{
  pub param_id: usize,
  pub lang_id: usize,
  pub paramname: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamValue{
  pub param_id: usize,
  pub value: String,
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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModificationUpdatePreData{
  pub modification_name: String,
  pub description: String,
  pub actual_status_id: usize,
}

impl ModificationUpdatePreData {
    pub(crate) fn new() -> Self {
        Self {
            modification_name: String::new(),
            description: String::new(),
            actual_status_id: 1,
        }
    }
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
