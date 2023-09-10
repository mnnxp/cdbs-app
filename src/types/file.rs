use super::profiles::ShowUserShort;
use super::relate::Program;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowFileInfo {
    pub uuid: String,
    pub filename: String,
    pub revision: usize,
    pub parent_file_uuid: String,
    pub owner_user: ShowUserShort,
    pub content_type: String,
    pub filesize: usize,
    pub program: Program,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DownloadFile {
    pub uuid: String,
    pub filename: String,
    pub filesize: usize,
    pub download_url: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadFile {
    pub file_uuid: String,
    pub filename: String,
    pub upload_url: String,
}
