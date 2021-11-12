//! Routes by yew_router

pub mod home;
pub mod user;
pub mod tender;
pub mod component;

pub use user::{
    login,
    register,
    settings,
    notification,
    profile,
    catalog,
};

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
    #[to = "#/components"]
    CatalogComponent,
    #[to = "#/notifications"]
    Notifications,
    #[to = "#/settings"]
    Settings,
    // #[to = "#/@{username}/favorites"]
    // ProfileFavorites(String),
    #[to = "#/users"]
    CatalogUsers,
    #[to = "#/@{username}"]
    Profile(String),
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
