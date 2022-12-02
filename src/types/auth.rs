use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub region_id: usize,
    pub program_id: usize,
    pub type_access_id: usize,
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
            type_access_id: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserToken {
    pub bearer: String,
}

impl fmt::Display for UserToken {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.bearer)
  }
}
