use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ShortFile {
    pub uuid: String,
    pub filename: String,
    pub filesize: usize,
}
