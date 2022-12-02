use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
// use log::debug;
use parking_lot::RwLock;
use web_sys::Storage;
use crate::types::SlimUser;

const TOKEN_KEY: &str = dotenv!("TOKEN_KEY");
const LOGGED_USER_KEY: &str = dotenv!("LOGGED_USER_KEY");
const ACCEPT_LANGUAGE: &str = dotenv!("ACCEPT_LANGUAGE");
const LIST_VIEW_TYPE: &str = dotenv!("LIST_VIEW_TYPE");

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = storage_service();
        if let Ok(token) = storage.get(TOKEN_KEY) {
            RwLock::new(token)
        } else {
            RwLock::new(None)
        }
    };

    /// Read SlimUser data from local storage.
    pub static ref LOGGED_USER: RwLock<Option<String>> = {
        let storage = storage_service();
        if let Ok(logged_user) = storage.get(LOGGED_USER_KEY) {
            RwLock::new(logged_user)
        } else {
            RwLock::new(None)
        }
    };

    /// Read accept language data from local storage.
    pub static ref LANGUAGE: RwLock<Option<String>> = {
        let storage = storage_service();
        if let Ok(accept_language) = storage.get(ACCEPT_LANGUAGE) {
            RwLock::new(accept_language)
        } else {
            RwLock::new(None)
        }
    };

    /// Read list view type from local storage.
    pub static ref LISTVIEWTYPE: RwLock<Option<String>> = {
        let storage = storage_service();
        if let Ok(list_view) = storage.get(LIST_VIEW_TYPE) {
            RwLock::new(list_view)
        } else {
            RwLock::new(None)
        }
    };
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    let storage = storage_service();
    if let Some(t) = token.clone() {
        storage.set_item(TOKEN_KEY, t.as_str())
            .expect("no access storage");
    } else {
        storage.remove_item(TOKEN_KEY)
            .expect("no access storage");
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
    let storage = storage_service();
    if let Some(u) = logged_user.clone() {
        storage.set_item(LOGGED_USER_KEY, u.as_str())
            .expect("no access storage");
    } else {
        storage.remove_item(LOGGED_USER_KEY)
            .expect("no access storage");
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
    let storage = storage_service();
    if let Some(l) = lang.clone() {
        storage.set_item(ACCEPT_LANGUAGE, l.as_str())
            .expect("no access storage");
    } else {
        storage.remove_item(ACCEPT_LANGUAGE)
            .expect("no access storage");
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
    let storage = storage_service();
    if let Some(l) = list_view.clone() {
        storage.set_item(LIST_VIEW_TYPE, l.as_str())
            .expect("no access storage");
    } else {
        storage.remove_item(LIST_VIEW_TYPE)
            .expect("no access storage");
    }
    let mut list_view_lock = LISTVIEWTYPE.write();
    *list_view_lock = list_view;
}

/// Get list view type
pub fn get_list_view() -> Option<String> {
    let list_view_lock = LISTVIEWTYPE.read();
    list_view_lock.clone()
}

/// Доступ к локальному хранилищу браузера
pub fn storage_service() -> Storage {
    web_sys::window()
        .expect("no window")
        .local_storage()
        .expect("storage was disabled")
        .expect("no session storage")
}
