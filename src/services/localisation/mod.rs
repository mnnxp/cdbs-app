mod local_en;
mod local_ru;
mod local_zh;
mod locale_key;

use local_en::LOCAL_EN;
use local_ru::LOCAL_RU;
use local_zh::LOCAL_ZH;
pub(crate) use locale_key::LocaleKey;
use crate::services::get_lang;

const MAX_KEY: usize = LocaleKey::_Count as usize;

impl LocaleKey {
    /// Returns the field value for set language
    pub(crate) fn get_value(self) -> &'static str {
        let idx = self as usize;
        if idx == 0 || idx >= MAX_KEY {
            return "{{MISSING}}";
        }
        match get_lang().as_deref() {
            Some("zh") => LOCAL_ZH[idx], // Chinese
            Some("ru") => LOCAL_RU[idx], // Russian
            _ => LOCAL_EN[idx], // English
        }
    }
}
