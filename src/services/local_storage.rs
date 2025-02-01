use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
// use log::debug;
use parking_lot::RwLock;
use yew::services::storage::{Area, StorageService};

use crate::types::SlimUser;

const TOKEN_KEY: &str = dotenv!("TOKEN_KEY");
const LOGGED_USER_KEY: &str = dotenv!("LOGGED_USER_KEY");
const ACCEPT_LANGUAGE: &str = dotenv!("ACCEPT_LANGUAGE");
const LIST_VIEW_TYPE: &str = dotenv!("LIST_VIEW_TYPE");
const HISTORY_BACK: &str = dotenv!("HISTORY_BACK");

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(token) = storage.restore(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };

    /// Read SlimUser data from local storage.
    pub static ref LOGGED_USER: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(logged_user) = storage.restore(LOGGED_USER_KEY) {
            RwLock::new(Some(logged_user))
        } else {
            RwLock::new(None)
        }
    };

    /// Read accept language data from local storage.
    pub static ref LANGUAGE: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(accept_language) = storage.restore(ACCEPT_LANGUAGE) {
            RwLock::new(Some(accept_language))
        } else {
            RwLock::new(None)
        }
    };

    /// Read list view type from local storage.
    pub static ref LISTVIEWTYPE: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(list_view) = storage.restore(LIST_VIEW_TYPE) {
            RwLock::new(Some(list_view))
        } else {
            RwLock::new(None)
        }
    };

    /// Read flag of need to return to previous page
    pub static ref HISTORYBACK: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(history_back) = storage.restore(HISTORY_BACK) {
            RwLock::new(Some(history_back))
        } else {
            RwLock::new(None)
        }
    };
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(t) = token.clone() {
        storage.store(TOKEN_KEY, Ok(t));
    } else {
        storage.remove(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

/// Check if current user is authenticated.
pub fn is_authenticated() -> bool {
    get_token().is_some()
}

/// Set current user to local storage.
pub fn set_logged_user(logged_user: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(u) = logged_user.clone() {
        storage.store(LOGGED_USER_KEY, Ok(u));
    } else {
        storage.remove(LOGGED_USER_KEY);
    }
    let mut logged_user_lock = LOGGED_USER.write();
    *logged_user_lock = logged_user;
}

/// Get authenticated user from browser storage
pub fn get_logged_user() -> Option<SlimUser> {
    let logged_user_lock = LOGGED_USER.read();
    let logged_user_lock: Option<SlimUser> = serde_json::from_str(
        &logged_user_lock.clone().unwrap_or_default()
      ).unwrap_or_default();
    logged_user_lock.clone()
}

/// Set language to local storage.
pub fn set_lang(lang: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(l) = lang.clone() {
        storage.store(ACCEPT_LANGUAGE, Ok(l));
    } else {
        storage.remove(ACCEPT_LANGUAGE);
    }
    let mut lang_lock = LANGUAGE.write();
    *lang_lock = lang;
}

/// Get set language for Accept-Language
pub fn get_lang() -> Option<String> {
    let lang_lock = LANGUAGE.read();
    lang_lock.clone()
}

/// Set list view type to local storage.
pub fn set_list_view(list_view: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(l) = list_view.clone() {
        storage.store(LIST_VIEW_TYPE, Ok(l));
    } else {
        storage.remove(LIST_VIEW_TYPE);
    }
    let mut list_view_lock = LISTVIEWTYPE.write();
    *list_view_lock = list_view;
}

/// Get list view type
pub fn get_list_view() -> Option<String> {
    let list_view_lock = LISTVIEWTYPE.read();
    list_view_lock.clone()
}

/// Sets flag of need to return to previous page after authorization
pub fn set_history_back(history_back: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(h) = history_back.clone() {
        storage.store(HISTORY_BACK, Ok(h));
    } else {
        storage.remove(HISTORY_BACK);
    }
    let mut history_back_lock = HISTORYBACK.write();
    *history_back_lock = history_back;
}

/// Gets flag of need to return to previous page after authorization
pub fn get_history_back() -> Option<String> {
    let history_back_lock = HISTORYBACK.read();
    history_back_lock.clone()
}
