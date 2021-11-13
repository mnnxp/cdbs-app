use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::file::{ShowFileForDownload, DownloadFile};
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
pub struct UserCertificate {
    pub user_uuid: UUID,
    pub file: ShowFileForDownload,
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
