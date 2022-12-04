mod local_en;
mod local_ru;
use local_en::LOCAL_EN;
use local_ru::LOCAL_RU;

// use log::debug;
use crate::services::get_lang;

/// Returns the field value for set language
pub fn get_value_field(row_key: &usize) -> &'static str {
    // debug!("Get translate: {}", row_key);
    if let Some(lang) = get_lang() {
        match lang.as_str() {
            "ru" => *LOCAL_RU.get(&row_key).unwrap(), // Rus
            _ => *LOCAL_EN.get(&row_key).unwrap() // Eng
        }
    } else {
        *LOCAL_EN.get(&row_key).unwrap() // Eng
    }
}
