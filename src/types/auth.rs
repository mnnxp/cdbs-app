use serde::{Deserialize, Serialize};
use std::fmt;

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
// #[serde(rename_all = "camelCase")]
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
    pub image_file: ImageFile,  // obj
    pub region: Region,  // obj
    pub program: Program,  // obj
    pub is_email_verified: String,
    pub is_enabled: String,
    pub is_delete: String,
    pub created_at: String,
    pub updated_at: String,
    pub certificates: Certificates, // obj
    pub subscribers: i32,
    pub is_followed: bool,
    pub companies_count: i32,
    pub components_count: i32,
    pub standards_count: i32,
    pub fav_companies_count: i32,
    pub fav_components_count: i32,
    pub fav_standards_count: i32,
    pub fav_users_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Region {
    pub langId: usize,
    pub region: String,
    pub region_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub id: usize,
    pub name: String,
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
