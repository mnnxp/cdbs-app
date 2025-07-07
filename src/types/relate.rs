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
pub struct SpecWithParent {
    pub spec_id: usize,
    pub lang_id: usize,
    pub spec: String,
    pub parent_spec: Spec,
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
/// Represents a single keyword with a ID
pub struct Keyword {
    pub id: usize,
    pub keyword: String,
}

impl Keyword {
    /// Parses a given string, potentially containing multiple keywords separated by spaces or commas,
    /// into individual `Keyword` entries. Each valid keyword is then added to the `add_keywords` vector.
    ///
    /// This function handles the splitting of the input string and delegates the processing of
    /// individual keywords to the `prepare_keyword` method. It also tracks whether any
    /// "bad" keywords (e.g., exceeding length limits) were encountered during parsing.
    pub(crate) fn parse_keywords(
        keyword: String,
        ipt_index: &mut usize,
        ipt_keyword: &mut String,
        add_keywords: &mut Vec<Keyword>,
        bad_keyword: &mut bool,
    ) {
        *bad_keyword = false; // Clear errors before parsing
        match keyword.find(|c| (c == ' ') || (c == ',')) {
            None => (), // No spaces or commas, the keyword is not complete
            Some(1) => Keyword::prepare_keyword(&keyword, ipt_index, ipt_keyword, add_keywords, bad_keyword),
            Some(_) => for k in keyword.split(|c| c == ' ' || c == ',') {
                // Split by spaces or commas and process each segment
                Keyword::prepare_keyword(&k, ipt_index, ipt_keyword, add_keywords, bad_keyword);
            },
        }
    }

    /// Prepares a single keyword for storage by performing validation and adding it to the list.
    ///
    /// This function checks the length of the input `keyword`. If it exceeds 30 bytes/characters,
    /// it marks `bad_keyword` as `true` and returns without adding the keyword.
    /// Otherwise, it creates a `Keyword` struct, assigns it the current `ipt_index` as its ID,
    /// trims any leading/trailing whitespace from the keyword, and adds it to the `add_keywords` vector.
    /// It then clears `ipt_keyword` and increments `ipt_index`.
    pub(crate) fn prepare_keyword(
        keyword: &str,
        ipt_index: &mut usize,
        ipt_keyword: &mut String,
        add_keywords: &mut Vec<Keyword>,
        bad_keyword: &mut bool,
    ) {
        if keyword.len() > 30 {
            *bad_keyword = true;
            return;
        }
        add_keywords.push(Keyword {
            id: *ipt_index,
            keyword: keyword.trim().to_string()
        });
        ipt_keyword.clear();
        *ipt_index += 1;
    }
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

#[derive(Clone, Debug)]
pub struct SpecNode {
    pub spec_id: usize,
    pub spec: String,
    pub children: Vec<SpecNode>,
    pub parent_spec: SpecWithParent,
    pub expanded: bool,
    pub loading: bool,
}

impl Default for SpecNode {
    fn default() -> Self {
        SpecNode {
            spec_id: 1,
            spec: "ROOT".to_string(),
            children: Vec::new(),
            parent_spec: SpecWithParent::default(),
            expanded: false,
            loading: false,
        }
    }
}