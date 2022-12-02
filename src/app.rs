//! The root app contains initial authentication and url routes
use yew::{Component, Context, html, html::Scope, Html};
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::error::Error;
use crate::fragments::footer::Footer;
use crate::fragments::header::Header;
use crate::routes::home::Home;
use crate::routes::login::Login;
use crate::routes::notification::Notifications;
use crate::routes::profile::Profile;
use crate::routes::register::Register;
use crate::routes::settings::Settings;
use crate::routes::component::{ShowComponent, ComponentSettings, CreateComponent};
use crate::routes::company::{ShowCompany, CompanySettings, CreateCompany};
use crate::routes::standard::{ShowStandard, StandardSettings, CreateStandard};
use crate::routes::AppRoute;
use crate::services::{is_authenticated, get_current_user, get_value_field};
use crate::types::SlimUser;

/// The root app component
pub struct App {
    // auth: Auth,
    current_route: Option<AppRoute>,
    current_user: Option<SlimUser>,
}

pub enum Msg {
    CurrentUserResponse(Result<SlimUser, Error>),
    Authenticated(SlimUser),
    Logout,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let current_route =
            AppRoute::recognize(
                ctx.link()
                .location()
                .map(|r| r.path().to_string())
                .unwrap_or_default()
                .as_str()
            );
        App {
            current_route,
            current_user: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Get current user info if a token is available when mounted
        if first_render && is_authenticated() {
            let link = ctx.link().clone();
            // let task = self.auth.current(self.current_user_response.clone());
            spawn_local(async move {
                let res = get_current_user().await;
                link.send_message(Msg::CurrentUserResponse(res));
            })
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CurrentUserResponse(res) => {
                res.map_or_else(
                    |err| debug!("Error with CurrentUserResponse: {:#?}", err),
                    |slim_user| self.current_user = Some(slim_user)
                )
            },
            Msg::Authenticated(slim_user) => self.current_user = Some(slim_user),
            Msg::Logout => self.current_user = None,
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback_logout = ctx.link().callback(|_| Msg::Logout);
        html!{
            <BrowserRouter>
                <Header current_user={self.current_user.clone()} callback={callback_logout} />
                {
                    if let Some(route) = &self.current_route {
                        // Routes to render sub components
                        html!{ self.switch(ctx.link(), route) }
                    } else {
                        // 404 when route matches no component
                        html!{ get_value_field(&294) }
                    }
                }
                <Footer />
            </BrowserRouter>
        }
    }
}

impl App {
    fn switch(
        &self,
        link: &Scope<Self>,
        route: &AppRoute,
    ) -> Html {
        let callback_login = link.callback(Msg::Authenticated);
        // let callback_register = link.callback(Msg::Authenticated);
        // let callback_logout = link.callback(Msg::Logout);

        match route {
            AppRoute::Login => html!{
                <Login callback={callback_login} />
            },
            AppRoute::Register => html!{<Register />},
            AppRoute::Home => html!{<Home />},
            AppRoute::Notifications => html!{<Notifications />},
            AppRoute::Settings => html!{<Settings />},
            AppRoute::Profile { username:_ } => html!{
                <Profile current_user={self.current_user.clone()} />
            },
            AppRoute::CompanySettings { uuid } => html!{
                <CompanySettings
                    current_user={self.current_user.clone()}
                    company_uuid={ uuid.clone() }
                />
            },
            AppRoute::ShowCompany { uuid } => html!{
                <ShowCompany
                    current_user={self.current_user.clone()}
                    company_uuid={ uuid.clone() }
                />
            },
            AppRoute::CreateCompany => html!{<CreateCompany />},
            AppRoute::StandardSettings { uuid } => html!{
                <StandardSettings
                    current_user={self.current_user.clone()}
                    standard_uuid={ uuid.clone() }
                />
            },
            AppRoute::ShowStandard { uuid } => html!{
                <ShowStandard
                    current_user={self.current_user.clone()}
                    standard_uuid={ uuid.clone() }
                />
            },
            AppRoute::CreateStandard => html!{<CreateStandard />},
            AppRoute::ComponentSettings { uuid } => html!{
                <ComponentSettings
                    current_user={self.current_user.clone()}
                    component_uuid={ uuid.clone()}
                />
            },
            AppRoute::ShowComponent { uuid } => html!{
                <ShowComponent
                    current_user={self.current_user.clone()}
                    component_uuid={ uuid.clone()}
                />
            },
            AppRoute::CreateComponent => html!{<CreateComponent />},
            AppRoute::NotFound => html! { <h1>{ get_value_field(&294) }</h1> }, // 404
        }
    }
}
