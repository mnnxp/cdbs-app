use web_sys::window;
use log::debug;
use crate::services::{get_server_location, set_server_location, get_gql_server_location, set_gql_server_location};

/// Available server locations with REST API, GraphQL endpoints and hostnames for matching
const SERVER_LOCATIONS: &[(&str, &str, &str)] = &[
    ("https://api.cadbase.rs", "https://api.cadbase.rs/graphql", "app.cadbase.rs"),
    ("https://api.cadbase.ru", "https://api.cadbase.ru/graphql", "app.cadbase.ru"),
    ("https://api.cadbase.org", "https://api.cadbase.org/graphql", "app.cadbase.org"),
    ("http://127.0.0.1:3000", "http://127.0.0.1:3000/graphql", "127.0.0.1"),
];

/// Default server (first in the list)
const DEFAULT_SERVER: (&str, &str, &str) = SERVER_LOCATIONS[0];

/// Sets server locations based on browser URL auto-detection or provided ID
///
/// # Arguments
/// * `server_location_id` - 1-based index into SERVER_LOCATIONS (None for auto-detection)
pub(crate) fn set_server_locations(server_location_id: Option<usize>) {
    let (server_url, gql_url, _) = match server_location_id {
        Some(id) => {
            // Convert 1-based index to 0-based, with bounds checking
            let index = id.saturating_sub(1);
            SERVER_LOCATIONS.get(index).copied().unwrap_or(DEFAULT_SERVER)
        },
        None => {
            // Auto-detect based on current URL
            let current_hostname = window()
                .and_then(|w| w.location().hostname().ok())
                .unwrap_or_default();
            debug!("Current hostname={:?}", current_hostname);

            SERVER_LOCATIONS
                .iter()
                .find(|(_, _, hostname)| current_hostname == *hostname)
                .copied()
                .unwrap_or(DEFAULT_SERVER)
        },
    };

    debug!("Server location id={:?}. Setting server locations: REST={}, GraphQL={}", server_location_id, server_url, gql_url);
    set_server_location(Some(server_url.to_string()));
    set_gql_server_location(Some(gql_url.to_string()));
}

/// Returns (REST_URL, GraphQL_URL) from storage or defaults
pub(crate) fn get_server_locations() -> (String, String) {
    let server_url = get_server_location().unwrap_or_else(|| DEFAULT_SERVER.0.to_string());
    let gql_url = get_gql_server_location().unwrap_or_else(|| DEFAULT_SERVER.1.to_string());

    (server_url, gql_url)
}

/// Returns current server location ID (index in SERVER_LOCATIONS + 1), return 0 if not found
pub(crate) fn get_server_location_id() -> usize {
    let current_server = get_server_location().unwrap_or_default();

    SERVER_LOCATIONS
        .iter()
        .position(|(server_url, _, _)| *server_url == current_server)
        .map(|x| x + 1) // Convert to 1-based index
        .unwrap_or(0)
}