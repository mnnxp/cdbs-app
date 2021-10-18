use serde::{Deserialize, Serialize};
use std::fmt;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
// #[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct RegisterInfo {
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub description: String,
    pub address: String,
    pub time_zone: String,
    pub position: String,
    pub region_id: i32,
    pub program_id: i32,
}

impl Default for RegisterInfo {
    fn default() -> Self {
        Self {
            firstname: String::new(),
            lastname: String::new(),
            secondname: String::new(),
            username: String::new(),
            email: String::new(),
            password: String::new(),
            phone: String::new(),
            description: String::new(),
            address: String::new(),
            time_zone: String::new(),
            position: String::new(),
            region_id: 1,
            program_id: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct UserToken {
    pub bearer: String,
}

impl fmt::Display for UserToken {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.bearer)
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimUser {
    pub uuid: String,
    pub program_id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct SlimUserWrapper {
    pub user: SlimUser,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub uuid: String,
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
    pub image_file: ShortFile, // obj
    pub region: Region, // obj
    pub program: Program, // obj
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub certificates: Vec<Certificate>, // obj
    pub subscribers: usize,
    pub is_followed: bool,
    pub companies_count: usize,
    pub components_count: usize,
    pub standards_count: usize,
    pub fav_companies_count: usize,
    pub fav_components_count: usize,
    pub fav_standards_count: usize,
    pub fav_users_count: usize,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ShortFile {
    pub uuid: String,
    pub filename: String,
    pub filesize: usize,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Region {
    pub langId: usize,
    pub region: String,
    pub regionId: usize,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Program {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Certificate {
    pub userUuid: String,
    pub file: ShortFile,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
// #[serde(rename_all = "camelCase")]
pub struct UserUpdateInfo {
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
    pub region_id: usize,
    pub program_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}
