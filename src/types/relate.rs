use crate::types::file::DownloadFile;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub lang_id: usize,
    pub region: String,
    pub region_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Program {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub spec_id: usize,
    pub lang_id: usize,
    pub spec: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpecPathInfo {
    pub spec_id: usize,
    pub lang_id: usize,
    pub path: String,
}

impl From<&SpecPathInfo> for Spec {
    fn from(data: &SpecPathInfo) -> Self {
        let SpecPathInfo {
            spec_id,
            path,
            lang_id,
        } = data;

        Self {
            spec_id: *spec_id, // as usize,
            lang_id: *lang_id, // as usize,
            spec: path.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Keyword {
    pub id: usize,
    pub keyword: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    pub file: DownloadFile,
    pub description: String,
}

impl From<super::CompanyCertificate> for Certificate {
    fn from(item: super::CompanyCertificate) -> Self {
        Self {
            file: item.file,
            description: item.description,
        }
    }
}

impl From<super::UserCertificate> for Certificate {
    fn from(item: super::UserCertificate) -> Self {
        Self {
            file: item.file,
            description: item.description,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeAccessInfo {
    pub lang_id: usize,
    pub name: String,
    pub type_access_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LicenseInfo {
    pub id: usize,
    pub name: String,
    pub keyword: String,
    pub publication_at: NaiveDateTime,
}
