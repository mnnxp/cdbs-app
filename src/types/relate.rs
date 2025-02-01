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

#[derive(Clone, Debug)]
pub struct PaginateSet {
    pub current_page: i64,
    pub per_page: i64,
}

impl PaginateSet {
    /// Returns with current_page 1 and per_page 5 values
    pub fn new() -> Self {
        Self {
            current_page: 1,
            per_page: 5,
        }
    }

    /// Returns with current_page 1 and per_page 5 values or with provided current_page
    pub fn set(current_page: Option<i64>, per_page: Option<i64>) -> Self {
        Self {
            current_page: current_page.unwrap_or(1),
            per_page: per_page.unwrap_or(5),
        }
    }

    /// Reduces current page by 1
    pub fn previous(&mut self) {
        self.current_page -= 1;
    }

    /// Increases current page by 1
    pub fn next(&mut self) {
        self.current_page += 1;
    }

    /// Sets provided number as current page
    pub fn to(&mut self, number: i64) {
        self.current_page = number;
    }

    /// Sets the given value as max number of elements on page
    pub fn max_on_page(&mut self, per_page: i64) {
        self.per_page = per_page;
    }

    /// Returns the result of comparing self and the provided PaginateSet
    pub fn compare(&mut self, page_set: &PaginateSet) -> bool {
        self.per_page == page_set.per_page && self.current_page == page_set.current_page
    }

    /// Returns number of items skipped on previous pages (converting to usize)
    pub fn numero_offset(&self) -> usize {
        ((self.per_page * self.current_page) - self.per_page + 1) as usize
    }
}