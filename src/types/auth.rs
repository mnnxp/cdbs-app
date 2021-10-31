use serde::{Deserialize, Serialize};
use std::fmt;
// use chrono::NaiveDateTime;

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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePasswordInfo {
    pub old_password: String,
    pub new_password: String,
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
