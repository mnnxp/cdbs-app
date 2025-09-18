//! Routes by yew_router

pub mod home;
pub mod user;
pub mod component;
pub mod company;
pub mod standard;
pub mod supplier_service;

pub use user::{
    login,
    register,
    settings,
    notification,
    profile,
};

// for test, in future this routes be delete and use #/search
pub use crate::fragments::{
    user as other_user,
    component as other_component
};

use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "#/login"]
    Login,
    #[to = "#/register"]
    Register,
    // #[to = "#/components"]
    // CatalogComponents,
    #[to = "#/component/settings/{uuid}"]
    ComponentSettings(String),
    #[to = "#/component/create"]
    CreateComponent,
    #[to = "#/component/{uuid}"]
    ShowComponent(String),
    #[to = "#/notifications"]
    Notifications,
    #[to = "#/settings"]
    Settings,
    #[to = "#/company/settings/{uuid}"]
    CompanySettings(String),
    #[to = "#/company/create"]
    CreateCompany,
    #[to = "#/company/{uuid}"]
    ShowCompany(String),
    #[to = "#/supplier/{uuid}"]
    ShowSupplierCompany(String),
    #[to = "#/standard/settings/{uuid}"]
    StandardSettings(String),
    #[to = "#/standard/create"]
    CreateStandard,
    #[to = "#/standard/{uuid}"]
    ShowStandard(String),
    #[to = "#/service/create"]
    CreateService,
    #[to = "#/service/settings/{uuid}"]
    ServiceSettings(String),
    #[to = "#/service/{uuid}"]
    ShowService(String),
    // #[to = "#/users"]
    // CatalogUsers,
    #[to = "#/@{username}"]
    Profile(String),
    #[to = "#/search"]
    SearchPage,
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
