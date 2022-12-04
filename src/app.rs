//! The root app contains initial authentication and url routes
use yew::{Component, Context, html, Html};
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::error::Error;
use crate::fragments::footer::Footer;
use crate::fragments::header::Header;
use crate::routes::AppRoute;
use crate::routes::{
    home::Home,
    login::Login,
    notification::Notifications,
    profile::Profile,
    register::Register,
    settings::Settings,
    component::{ShowComponent, ComponentSettings, CreateComponent},
    company::{ShowCompany, CompanySettings, CreateCompany},
    standard::{ShowStandard, StandardSettings, CreateStandard},
};
use crate::services::{is_authenticated, get_current_user, get_value_field};
use crate::types::SlimUser;

/// The root app component
pub struct App {
    current_user: Option<SlimUser>,
}

pub enum Msg {
    CurrentUserResponse(Result<SlimUser, Error>),
    Logout,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            current_user: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Get current user info if a token is available when mounted
        if first_render && is_authenticated() {
            let link = ctx.link().clone();
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
                <Switch<AppRoute> render={switch} />
                <Footer />
            </BrowserRouter>
        }
    }
}

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!{<Login />},
        AppRoute::Register => html!{<Register />},
        AppRoute::Home => html!{<Home />},
        AppRoute::Notifications => html!{<Notifications />},
        AppRoute::Settings => html!{<Settings />},
        AppRoute::Profile { username:_ } => html!{<Profile />},
        AppRoute::CompanySettings { uuid } => html!{
            <CompanySettings company_uuid={uuid} />
        },
        AppRoute::ShowCompany { uuid } => html!{
            <ShowCompany company_uuid={uuid} />
        },
        AppRoute::CreateCompany => html!{<CreateCompany />},
        AppRoute::StandardSettings { uuid } => html!{
            <StandardSettings standard_uuid={uuid} />
        },
        AppRoute::ShowStandard { uuid } => html!{
            <ShowStandard standard_uuid={uuid} />
        },
        AppRoute::CreateStandard => html!{<CreateStandard />},
        AppRoute::ComponentSettings { uuid } => html!{
            <ComponentSettings component_uuid={uuid} />
        },
        AppRoute::ShowComponent { uuid } => html!{
            <ShowComponent component_uuid={uuid} />
        },
        AppRoute::CreateComponent => html!{<CreateComponent />},
        AppRoute::NotFound => html! { <h1>{ get_value_field(&294) }</h1> }, // 404
    }
}
