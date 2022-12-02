//! Routes by yew_router

pub mod home;
pub mod user;
pub mod component;
pub mod company;
pub mod standard;

pub use user::{
    login,
    register,
    settings,
    notification,
    profile,
};

// for test, in future this routes be delete and use /search
pub use crate::fragments::{
    user as other_user,
    component as other_component
};

use yew_router::prelude::*;

/// App routes
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    // #[at("/components")]
    // CatalogComponents,
    #[at("/component/settings/:uuid")]
    ComponentSettings { uuid: String },
    #[at("/component/create")]
    CreateComponent,
    #[at("/component/:uuid")]
    ShowComponent { uuid: String },
    #[at("/notifications")]
    Notifications,
    #[at("/settings")]
    Settings,
    #[at("/company/settings/:uuid")]
    CompanySettings { uuid: String },
    #[at("/company/create")]
    CreateCompany,
    #[at("/company/:uuid")]
    ShowCompany { uuid: String },
    #[at("/standard/settings/:uuid")]
    StandardSettings { uuid: String },
    #[at("/standard/create")]
    CreateStandard,
    #[at("/standard/:uuid")]
    ShowStandard { uuid: String },
    // #[at("/users")]
    // CatalogUsers,
    #[at("/@:username")]
    Profile { username: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
