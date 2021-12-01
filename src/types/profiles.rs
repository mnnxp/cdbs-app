use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::file::DownloadFile;
use super::relate::{Region, Program, TypeAccessTranslateListInfo};
use super::UUID;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelfUserInfo {
    pub uuid: UUID,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub phone: String,
    pub description: String,
    pub address: String,
    pub position: String,
    pub time_zone: String,
    pub image_file: DownloadFile, // obj
    pub region: Region, // obj
    pub program: Program, // obj
    pub type_access: TypeAccessTranslateListInfo, // obj
    pub is_email_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub certificates: Vec<UserCertificate>, // obj
    pub subscribers: usize,
    pub companies_count: usize,
    pub components_count: usize,
    pub standards_count: usize,
    pub fav_companies_count: usize,
    pub fav_components_count: usize,
    pub fav_standards_count: usize,
    pub fav_users_count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub uuid: UUID,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub description: String,
    pub position: String,
    pub image_file: DownloadFile, // obj
    pub region: Region, // obj
    pub program: Program, // obj
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub certificates: Vec<UserCertificate>, // obj
    pub subscribers: usize,
    pub is_followed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShowUserShort {
    pub uuid: UUID,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub image_file: DownloadFile,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimUser {
    pub uuid: String,
    pub program_id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateInfo {
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub secondname: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub position: Option<String>,
    pub time_zone: Option<String>,
    pub region_id: Option<i64>,
    pub program_id: Option<i64>,
}

/// Get data current user
impl From<SelfUserInfo> for UserUpdateInfo {
    fn from(data: SelfUserInfo) -> Self {
        let SelfUserInfo {
            firstname,
            lastname,
            secondname,
            username,
            email,
            description,
            position,
            phone,
            address,
            region,
            program,
            ..
        } = data;

        Self {
            firstname: Some(firstname),
            lastname: Some(lastname),
            secondname: Some(secondname),
            username: Some(username),
            email: Some(email),
            description: Some(description),
            position: Some(position),
            phone: Some(phone),
            time_zone: None,
            address: Some(address),
            region_id: Some(region.region_id as i64),
            program_id: Some(program.id as i64),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserCertificate {
    pub user_uuid: UUID,
    pub file: DownloadFile,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowNotification {
    pub id: usize,
    pub notification: String,
    pub degree_importance: DegreeImportanceTranslateList,
    pub created_at: NaiveDateTime,
    pub is_read: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DegreeImportanceTranslateList {
    pub degree_importance_id: usize,
    pub lang_id: usize,
    pub degree: String,
}

// for arguments users query
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UsersQueryArg {
    pub users_uuids:  Option<Vec<UUID>>,
    pub subscribers: Option<bool>,
    pub favorite: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl UsersQueryArg {
    pub fn set_favorite() -> Self {
        Self {
            favorite: Some(true),
            ..Default::default()
        }
    }
    pub fn set_subscribers() -> Self {
        Self {
            subscribers: Some(true),
            ..Default::default()
        }
    }
}
