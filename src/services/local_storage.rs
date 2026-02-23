use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use yew::services::storage::{Area, StorageService};

use crate::types::SlimUser;

// Environment variables
const API_BACKEND: &str = dotenv!("API_BACKEND");
const API_GQL: &str = dotenv!("API_GQL");
const TOKEN_KEY: &str = dotenv!("TOKEN_KEY");
const LOGGED_USER_KEY: &str = dotenv!("LOGGED_USER_KEY");
const ACCEPT_LANGUAGE: &str = dotenv!("ACCEPT_LANGUAGE");
const LIST_VIEW_TYPE: &str = dotenv!("LIST_VIEW_TYPE");
const HISTORY_BACK: &str = dotenv!("HISTORY_BACK");
const HISTORY_SEARCH: &str = dotenv!("HISTORY_SEARCH");

lazy_static! {
    /// REST API server location setting
    pub static ref SERVER: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(server_location) = storage.restore(API_BACKEND) {
            RwLock::new(Some(server_location))
        } else {
            RwLock::new(None)
        }
    };

    /// GraphQL API server location setting
    pub static ref SERVER_GQL: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(server_location_gql) = storage.restore(API_GQL) {
            RwLock::new(Some(server_location_gql))
        } else {
            RwLock::new(None)
        }
    };

    /// JWT authentication token
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(token) = storage.restore(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };

    /// Current logged in user data
    pub static ref LOGGED_USER: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(logged_user) = storage.restore(LOGGED_USER_KEY) {
            RwLock::new(Some(logged_user))
        } else {
            RwLock::new(None)
        }
    };

    /// User language preference
    pub static ref LANGUAGE: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(accept_language) = storage.restore(ACCEPT_LANGUAGE) {
            RwLock::new(Some(accept_language))
        } else {
            RwLock::new(None)
        }
    };

    /// List view display type preference
    pub static ref LISTVIEWTYPE: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(list_view) = storage.restore(LIST_VIEW_TYPE) {
            RwLock::new(Some(list_view))
        } else {
            RwLock::new(None)
        }
    };

    /// Flag indicating need to return to previous page after auth
    pub static ref HISTORYBACK: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(history_back) = storage.restore(HISTORY_BACK) {
            RwLock::new(Some(history_back))
        } else {
            RwLock::new(None)
        }
    };

    /// Search query history for persistence between pages
    pub static ref HISTORYSEARCH: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(history_search) = storage.restore(HISTORY_SEARCH) {
            RwLock::new(Some(history_search))
        } else {
            RwLock::new(None)
        }
    };
}

// Generic storage helper functions, saves value to storage
fn set_storage(key: &str, value: Option<String>, target: &RwLock<Option<String>>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

    match &value {
        Some(v) => storage.store(key, Ok(v.clone())),
        None => storage.remove(key),
    }

    *target.write() = value;
}

/// Retrieves value from in-memory cache
fn get_storage(source: &RwLock<Option<String>>) -> Option<String> {
    source.read().clone()
}

// Token management
/// Sets the JWT authentication token in local storage
pub fn set_token(token: Option<String>) {
    set_storage(TOKEN_KEY, token, &TOKEN);
}

/// Retrieves the current JWT authentication token
pub fn get_token() -> Option<String> {
    get_storage(&TOKEN)
}

/// Checks if user is authenticated (has valid token)
pub fn is_authenticated() -> bool {
    get_token().is_some()
}

// User management
/// Saves logged user data to local storage
pub fn set_logged_user(logged_user: Option<String>) {
    set_storage(LOGGED_USER_KEY, logged_user, &LOGGED_USER);
}

/// Retrieves the current logged user data
pub fn get_logged_user() -> Option<SlimUser> {
    get_storage(&LOGGED_USER)
        .and_then(|user_str| serde_json::from_str(&user_str).ok())
}

// Language settings
/// Sets user language preference in local storage
pub fn set_lang(lang: Option<String>) {
    set_storage(ACCEPT_LANGUAGE, lang, &LANGUAGE);
}

/// Gets current user language preference
pub fn get_lang() -> Option<String> {
    get_storage(&LANGUAGE)
}

// UI preferences
/// Saves list view type preference to local storage
pub fn set_list_view(list_view: Option<String>) {
    set_storage(LIST_VIEW_TYPE, list_view, &LISTVIEWTYPE);
}

/// Retrieves current list view type preference
pub fn get_list_view() -> Option<String> {
    get_storage(&LISTVIEWTYPE)
}

// Navigation history
/// Sets flag indicating need to return to previous page after authorization
pub fn set_history_back(history_back: Option<String>) {
    set_storage(HISTORY_BACK, history_back, &HISTORYBACK);
}

/// Gets flag indicating need to return to previous page after authorization
pub fn get_history_back() -> Option<String> {
    get_storage(&HISTORYBACK)
}

/// Saves search query to history in local storage
pub fn set_history_search(history_search: Option<String>) {
    set_storage(HISTORY_SEARCH, history_search, &HISTORYSEARCH);
}

/// Retrieves last search query from history
pub fn get_history_search() -> Option<String> {
    get_storage(&HISTORYSEARCH)
}

// Server location settings
/// Sets the REST API server location in local storage
pub fn set_server_location(server: Option<String>) {
    set_storage(API_BACKEND, server, &SERVER);
}

/// Gets the currently configured REST API server location
pub fn get_server_location() -> Option<String> {
    get_storage(&SERVER)
}

/// Sets the GraphQL API server location in local storage
pub fn set_gql_server_location(server: Option<String>) {
    set_storage(API_GQL, server, &SERVER_GQL);
}

/// Gets the currently configured GraphQL API server location
pub fn get_gql_server_location() -> Option<String> {
    get_storage(&SERVER_GQL)
}