use serde::{Deserialize, Serialize};

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
    // pub id_type_user: i32,
    // pub is_supplier: i32,
    // pub orgname: String,
    // pub shortname: String,
    // pub inn: String,
    pub phone: String,
    pub description: String,
    // pub id_name_cad: i32,
    // pub comment: String,
    pub address: String,
    pub time_zone: i32,
    pub position: String,
    pub regionId: String,
    pub programId: String,
    // pub site_url: String,
    // pub uuid_file_info_icon: String,
    // pub id_region: i32,
}

impl Default for RegisterInfo {
    fn default() -> Self {
        Self {
            // id_region: (1),
            // uuid_file_info_icon: ("".to_owned()),
            // site_url: ("".to_owned()),
            position: ("".to_owned()),
            time_zone: (3),
            address: ("".to_owned()),
            // comment: ("".to_owned()),
            // id_name_cad: (1),
            phone: ("".to_owned()),
            // inn: ("".to_owned()),
            // shortname: ("".to_owned()),
            // orgname: ("".to_owned()),
            // is_supplier: (0),
            // id_type_user: (1),
            password: ("".to_owned()),
            email: ("".to_owned()),
            username: ("".to_owned()),
            secondname: ("".to_owned()),
            lastname: ("".to_owned()),
            firstname: ("".to_owned()),
            description: ("".to_owned()),
            regionId: ("".to_owned()),
            programId: ("".to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlimUser {
    pub uuid: String,
    pub is_supplier: i32,
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
    // pub email: String,
    // pub token: String,
    // pub username: String,
    // pub bio: Option<String>,
    // pub image: Option<String>,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub email: String,
    pub id_type_user: i32,
    pub is_supplier: i32,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub address: String,
    pub time_zone: i32,
    pub position: String,
    pub site_url: String,
    pub uuid_file_info_icon: String,
    pub id_region: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
// #[serde(rename_all = "camelCase")]
pub struct UserUpdateInfo {
    // pub email: String,
    // pub username: String,
    // pub password: Option<String>,
    // pub image: String,
    // pub bio: String,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub id_type_user: i32,
    pub is_supplier: i32,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub address: String,
    pub time_zone: i32,
    pub position: String,
    pub site_url: String,
    pub uuid_file_info_icon: String,
    pub id_region: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}
