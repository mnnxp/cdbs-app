//! Routes by yew_router

pub mod home;
pub mod login;
pub mod register;
pub mod settings;
pub mod tenders;
pub mod catalog;
pub mod createTender;

use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "#/login"]
    Login,
    #[to = "#/register"]
    Register,
    #[to = "#/tenders/create"]
    CreateTender,
    #[to = "#/tenders"]
    Tenders,
    #[to = "#/catalog"]
    Catalog,
    #[to = "#/settings"]
    Settings,
    // #[to = "#/@{username}/favorites"]
    // ProfileFavorites(String),
    // #[to = "#/@{username}"]
    // Profile(String),
    #[to = "#/"]
    Home,
}

/// Fix fragment handling problem for yew_router
pub fn fix_fragment_routes(route: &mut Route) {
    let r = route.route.as_str();
    if let Some(index) = r.find('#') {
        route.route = r[index..].to_string();
    } else {
        route.route = "#/".to_string();
    }
}
