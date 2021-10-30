use crate::types::file::ShowFileForDownload;

// use chrono::NaiveDateTime;
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    pub owner_uuid: String,
    pub file: ShowFileForDownload,
    pub description: String,
}

impl From<&super::UserCertificate> for Certificate {
    fn from(data: &super::UserCertificate) -> Self {
        let super::UserCertificate {
            user_uuid,
            file,
            description,
        } = data;

        Self {
            owner_uuid: user_uuid.to_string(),
            file: file.to_owned(),
            description: description.to_string(),
        }
    }
}

// impl From<&super::CompanyCertificate> for Certificate {
//     fn from(data: &super::CompanyCertificate) -> Self {
//         let super::CompanyCertificate {
//             company_uuid,
//             file,
//             description,
//         } = data;
//
//         Self {
//             owner_uuid: company_uuid.to_string(),
//             file: file.to_owned(),
//             description: description.to_string(),
//         }
//     }
// }
