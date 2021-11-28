//! The root app contains initial authentication and url routes

use yew::services::fetch::FetchTask;
use yew::{agent::Bridged, html, Bridge, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::{
    footer::Footer,
    header::Header,
    catalog_user::CatalogUsers, // for test
    catalog_component::CatalogComponents, // for test
};
use crate::routes::{
    // article::Article,
    // component::CatalogComponents,
    tender::{Tenders, CreateTender},
    // editor::Editor,
    fix_fragment_routes,
    home::Home,
    login::Login,
    notification::Notifications,
    profile::Profile,
    register::Register,
    settings::Settings,
    company::{ShowCompany, CompanySettings, CreateCompany},
    standard::ShowStandard,
    AppRoute,
};
use crate::services::{is_authenticated, get_current_user};
use crate::types::SlimUser;

/// The root app component
pub struct App {
    // auth: Auth,
    current_route: Option<AppRoute>,
    current_user: Option<SlimUser>,
    current_user_task: Option<FetchTask>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    CurrentUserResponse(Result<SlimUser, Error>),
    Route(Route),
    Authenticated(SlimUser),
    Logout,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let mut route = route_service.get_route();
        fix_fragment_routes(&mut route);
        App {
            // auth: Auth::new(),
            current_route: AppRoute::switch(route),
            router_agent,
            current_user: None,
            current_user_task: None,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // Get current user info if a token is available when mounted
        if first_render && is_authenticated() {
            let link = self.link.clone();
            // let task = self.auth.current(self.current_user_response.clone());
            // self.current_user_task = Some(task);
            spawn_local(async move {
                let res = get_current_user().await;
                link.send_message(Msg::CurrentUserResponse(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CurrentUserResponse(res) => {
                match res {
                    Ok(slim_user) => {
                        self.current_user = Some(slim_user);
                    },
                    Err(err) => {
                        debug!("Error with CurrentUserResponse: {:#?}", err);
                        self.current_user_task = None;
                    },
                }
            }
            Msg::Route(mut route) => {
                fix_fragment_routes(&mut route);
                self.current_route = AppRoute::switch(route)
            }
            Msg::Authenticated(slim_user) => {
                self.current_user = Some(slim_user);
            }
            Msg::Logout => {
                self.current_user = None;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback_login = self.link.callback(Msg::Authenticated);
        let callback_register = self.link.callback(Msg::Authenticated);
        let callback_logout = self.link.callback(|_| Msg::Logout);

        html! {
            <>
                <Header current_user=self.current_user.clone() callback=callback_logout.clone() />
                {
                    // Routes to render sub components
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Login => html!{<Login callback=callback_login />},
                            AppRoute::Register => html!{<Register callback=callback_register />},
                            AppRoute::Home => html!{<Home />},
                            // AppRoute::Editor(slug) => html!{<Editor slug=Some(slug.clone())/>},
                            // AppRoute::EditorCreate => html!{<Editor />},
                            // AppRoute::Article(slug) => html!{<Article slug=slug.clone() current_user=self.current_user.clone() />},
                            AppRoute::Notifications => html!{<Notifications callback=callback_logout />},
                            AppRoute::Settings => html!{<Settings callback=callback_logout />},
                            AppRoute::CatalogUsers => html!{<CatalogUsers />},
                            AppRoute::Profile(_username) => html!{<Profile current_user=self.current_user.clone()/>},
                            AppRoute::CompanySettings(company_uuid) => html!{<CompanySettings
                                current_user=self.current_user.clone()
                                company_uuid=company_uuid.to_string()
                            />},
                            AppRoute::ShowCompany(company_uuid) => html!{<ShowCompany
                                current_user=self.current_user.clone()
                                company_uuid=company_uuid.to_string()
                            />},
                            AppRoute::CreateCompany => html!{<CreateCompany />},
                            AppRoute::ShowStandard(standard_uuid) => html!{<ShowStandard
                                current_user=self.current_user.clone()
                                standard_uuid=standard_uuid.to_string()
                                />},
                            AppRoute::Tenders => html!{<Tenders />},
                            AppRoute::CatalogComponents => html!{<CatalogComponents show_create_btn = true />},
                            AppRoute::CreateTender => html!{<CreateTender />},
                        }
                    } else {
                        // 404 when route matches no component
                        html! { "No child component available" }
                    }
                }
                <Footer />
            </>
        }
    }
}
