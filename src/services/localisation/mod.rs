mod local_en;
mod local_ru;
mod local_zh;
use local_en::LOCAL_EN;
use local_ru::LOCAL_RU;
use local_zh::LOCAL_ZH;

// use log::debug;

use crate::services::get_lang;

/// Returns the field value for set language
pub fn get_value_field(row_key: &usize) -> &'static str {
    // debug!("Get translate: {}", row_key);
    if let Some(lang) = get_lang() {
        match lang.as_str() {
            "zh" => *LOCAL_ZH.get(&row_key).unwrap(), // Chinese
            "ru" => *LOCAL_RU.get(&row_key).unwrap(), // Russian
            _ => *LOCAL_EN.get(&row_key).unwrap() // English
        }
    } else {
        *LOCAL_EN.get(&row_key).unwrap() // Eng
    }
}
