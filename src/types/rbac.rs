use crate::types::UUID;
use crate::types::relate::PermissionLevel;
use crate::types::file::DownloadFile;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::ShowUserShort;

/// Access level identifiers for RBAC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessLevel {
    Manage = 1,
    Write = 2,
    Read = 3,
}

impl AccessLevel {
    /// Convert from type_access_id to AccessLevel
    pub fn from_id(id: i64) -> Option<Self> {
        match id {
            1 => Some(AccessLevel::Manage),
            2 => Some(AccessLevel::Write),
            3 => Some(AccessLevel::Read),
            _ => None,
        }
    }

    /// Convert to type_access_id
    pub fn to_id(&self) -> i64 {
        *self as i64
    }

    /// Get display name for the access level
    pub fn display_name(&self) -> &'static str {
        match self {
            AccessLevel::Manage => "Manage",
            AccessLevel::Write => "Write",
            AccessLevel::Read => "Read",
        }
    }
}

/// User access entry
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAccess {
    pub user: ShowUserShort,
    pub permission: PermissionLevel,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Company access entry
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompanyAccess {
    pub company: CompanySearchResult,
    pub permission: PermissionLevel,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Owner information for an object
pub(crate) type OwnerInfo = ShowUserShort;

/// User search result for access management
pub(crate) type UserSearchResult = ShowUserShort;

/// Company search result for access management
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompanySearchResult{
  pub uuid: UUID,
  pub shortname: String,
  pub inn: String,
  pub image_file: DownloadFile,
}

/// User info for company member addition
pub(crate) type UserInfoForMember = ShowUserShort;

/// Data for adding a company member
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddCompanyMemberData {
    pub company_uuid: UUID,
    pub user_uuid: UUID,
    pub role_id: i64,
}

/// Data for deleting a company member
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCompanyMemberData {
    pub company_uuid: UUID,
    pub user_uuid: UUID,
}

/// Data for setting user access on component
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAccessComponentData {
    pub component_uuid: UUID,
    pub user_uuid: UUID,
    pub type_access_id: i64,
}

/// Data for deleting user access on component
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserAccessComponentData {
    pub component_uuid: UUID,
    pub user_uuid: UUID,
}

/// Data for setting company access on component
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetCompanyAccessComponentData {
    pub component_uuid: UUID,
    pub company_uuid: UUID,
    pub type_access_id: i64,
}

/// Data for deleting company access on component
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCompanyAccessComponentData {
    pub component_uuid: UUID,
    pub company_uuid: UUID,
}