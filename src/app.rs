//! The root app contains initial authentication and url routes

use yew::services::fetch::FetchTask;
use yew::{agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use crate::fragments::{footer::Footer, header::Header};
use crate::error::Error;
use crate::routes::{
    article::Article,
    editor::Editor,
    fix_fragment_routes,
    home::Home,
    login::Login,
    tenders::Tenders,
    profile::{Profile, ProfileTab},
    register::Register,
    settings::Settings,
    AppRoute,
};
use crate::services::{is_authenticated, Auth};
use crate::types::{SlimUser, SlimUserWrapper};

/// The root app component
pub struct App {
    auth: Auth,
    current_route: Option<AppRoute>,
    current_user: Option<SlimUser>,
    current_user_response: Callback<Result<SlimUserWrapper, Error>>,
    current_user_task: Option<FetchTask>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    CurrentUserResponse(Result<SlimUserWrapper, Error>),
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
            auth: Auth::new(),
            current_route: AppRoute::switch(route),
            router_agent,
            current_user: None,
            current_user_response: link.callback(Msg::CurrentUserResponse),
            current_user_task: None,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // Get current user info if a token is available when mounted
        if first_render && is_authenticated() {
            let task = self.auth.current(self.current_user_response.clone());
            self.current_user_task = Some(task);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CurrentUserResponse(Ok(slim_user)) => {
                self.current_user = Some(slim_user.user);
                self.current_user_task = None;
            }
            Msg::CurrentUserResponse(Err(_)) => {
                self.current_user_task = None;
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
                <Header current_user=self.current_user.clone()/>
                {
                    // Routes to render sub components
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Login => html!{<Login callback=callback_login />},
                            AppRoute::Register => html!{<Register callback=callback_register />},
                            AppRoute::Home => html!{<Home />},
                            AppRoute::Editor(slug) => html!{<Editor slug=Some(slug.clone())/>},
                            AppRoute::EditorCreate => html!{<Editor />},
                            AppRoute::Article(slug) => html!{<Article slug=slug.clone() current_user=self.current_user.clone() />},
                            AppRoute::Settings => html!{<Settings callback=callback_logout />},
                            AppRoute::ProfileFavorites(username) => html!{
                                <Profile username=username.clone() current_user=self.current_user.clone() tab=ProfileTab::FavoritedBy />
                            },
                            AppRoute::Profile(username) => html!{
                                <Profile username=username.clone() current_user=self.current_user.clone() tab=ProfileTab::ByAuthor />
                            },
                            AppRoute::Tenders => html!{<Tenders />},
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
