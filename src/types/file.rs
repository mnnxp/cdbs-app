use super::profiles::ShowUserShort;
use super::relate::Program;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowFileForDownload {
    pub uuid: String,
    pub parent_file_uuid: String,
    pub download: DownloadFile,
    pub owner_user: ShowUserShort,
    pub content_type: String,
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
