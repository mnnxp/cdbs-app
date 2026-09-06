use dotenv_codegen::dotenv;
use std::sync::{OnceLock, RwLock};
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

/// Helper function to load a value from local storage
fn load_from_storage(key: &str) -> RwLock<Option<String>> {
    let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    match storage.restore(key) {
        Ok(value) => RwLock::new(Some(value)),
        Err(_) => RwLock::new(None),
    }
}

/// Creates a static OnceLock with a getter function
macro_rules! storage_static {
    ($name:ident, $key:expr, $doc:expr) => {
        #[doc = $doc]
        #[allow(non_snake_case)]
        fn $name() -> &'static RwLock<Option<String>> {
            static CACHE: OnceLock<RwLock<Option<String>>> = OnceLock::new();
            CACHE.get_or_init(|| load_from_storage($key))
        }
    };
}

// Define all storage statics
storage_static!(SERVER, API_BACKEND, "REST API server location setting");
storage_static!(SERVER_GQL, API_GQL, "GraphQL API server location setting");
storage_static!(TOKEN, TOKEN_KEY, "JWT authentication token");
storage_static!(LOGGED_USER, LOGGED_USER_KEY, "Current logged in user data");
storage_static!(LANGUAGE, ACCEPT_LANGUAGE, "User language preference");
storage_static!(LISTVIEWTYPE, LIST_VIEW_TYPE, "List view display type preference");
storage_static!(HISTORYBACK, HISTORY_BACK, "Flag indicating need to return to previous page after auth");
storage_static!(HISTORYSEARCH, HISTORY_SEARCH, "Search query history for persistence between pages");

/// Generic storage helper function, saves value to storage
fn set_storage(key: &str, value: Option<String>, target: &RwLock<Option<String>>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

    match &value {
        Some(v) => storage.store(key, Ok(v.clone())),
        None => storage.remove(key),
    }

    *target.write().expect("Failed to acquire write lock") = value;
}

/// Retrieves value from in-memory cache
fn get_storage(source: &RwLock<Option<String>>) -> Option<String> {
    source.read().expect("Failed to acquire read lock").clone()
}

// Token management
/// Sets the JWT authentication token in local storage
pub(crate) fn set_token(token: Option<String>) {
    set_storage(TOKEN_KEY, token, TOKEN());
}

/// Retrieves the current JWT authentication token
pub(crate) fn get_token() -> Option<String> {
    get_storage(TOKEN())
}

/// Checks if user is authenticated (has valid token)
pub(crate) fn is_authenticated() -> bool {
    get_token().is_some()
}

// User management
/// Saves logged user data to local storage
pub(crate) fn set_logged_user(logged_user: Option<String>) {
    set_storage(LOGGED_USER_KEY, logged_user, LOGGED_USER());
}

/// Retrieves the current logged user data
pub(crate) fn get_logged_user() -> Option<SlimUser> {
    get_storage(LOGGED_USER())
        .and_then(|user_str| serde_json::from_str(&user_str).ok())
}

// Language settings
/// Sets user language preference in local storage
pub(crate) fn set_lang(lang: Option<String>) {
    set_storage(ACCEPT_LANGUAGE, lang, LANGUAGE());
}

/// Gets current user language preference
pub(crate) fn get_lang() -> Option<String> {
    get_storage(LANGUAGE())
}

// UI preferences
/// Saves list view type preference to local storage
pub(crate) fn set_list_view(list_view: Option<String>) {
    set_storage(LIST_VIEW_TYPE, list_view, LISTVIEWTYPE());
}

/// Retrieves current list view type preference
pub(crate) fn get_list_view() -> Option<String> {
    get_storage(LISTVIEWTYPE())
}

// Navigation history
/// Sets flag indicating need to return to previous page after authorization
pub(crate) fn set_history_back(history_back: Option<String>) {
    set_storage(HISTORY_BACK, history_back, HISTORYBACK());
}

/// Gets flag indicating need to return to previous page after authorization
pub(crate) fn get_history_back() -> Option<String> {
    get_storage(HISTORYBACK())
}

/// Saves search query to history in local storage
pub(crate) fn set_history_search(history_search: Option<String>) {
    set_storage(HISTORY_SEARCH, history_search, HISTORYSEARCH());
}

/// Retrieves last search query from history
pub(crate) fn get_history_search() -> Option<String> {
    get_storage(HISTORYSEARCH())
}

// Server location settings
/// Sets the REST API server location in local storage
pub(crate) fn set_server_location(server: Option<String>) {
    set_storage(API_BACKEND, server, SERVER());
}

/// Gets the currently configured REST API server location
pub(crate) fn get_server_location() -> Option<String> {
    get_storage(SERVER())
}

/// Sets the GraphQL API server location in local storage
pub(crate) fn set_gql_server_location(server: Option<String>) {
    set_storage(API_GQL, server, SERVER_GQL());
}

/// Gets the currently configured GraphQL API server location
pub(crate) fn get_gql_server_location() -> Option<String> {
    get_storage(SERVER_GQL())
}