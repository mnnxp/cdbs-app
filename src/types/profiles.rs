use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfo {
    pub bio: Option<String>,
    pub image: String,
    pub following: bool,
    pub uuid: String,
    pub email: String,
    pub email_verified: i32,
    pub id_type_user: i32,
    pub value_type_user: String,
    pub is_supplier: i32,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub id_name_cad: i32,
    pub value_name_cad: String,
    pub comment: String,
    pub address: String,
    pub time_zone: i32,
    pub position: String,
    pub site_url: String,
    pub uuid_file_info_icon: String,
    pub id_region: i32,
    pub value_region: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfoWrapper {
    pub profile: ProfileInfo,
}
