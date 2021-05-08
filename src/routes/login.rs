use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::{set_token, Auth};
use crate::types::{LoginInfo, LoginInfoWrapper, SlimUser, SlimUserWrapper};

/// Login page
pub struct Login {
    auth: Auth,
    error: Option<Error>,
    request: LoginInfo,
    response: Callback<Result<SlimUserWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// Callback when user is logged in successfully
    pub callback: Callback<SlimUser>,
}

pub enum Msg {
    Request,
    Response(Result<SlimUserWrapper, Error>),
    Ignore,
    UpdateUsername(String),
    UpdatePassword(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            auth: Auth::new(),
            error: None,
            props,
            request: LoginInfo::default(),
            response: link.callback(Msg::Response),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let request = LoginInfoWrapper {
                    user: self.request.clone(),
                };
                self.task = Some(self.auth.login(request, self.response.clone()));
            }
            Msg::Response(Ok(user_info)) => {
                // Set global token after logged in
                // set_token(Some(user_info.user.token.clone()));
                self.props.callback.emit(user_info.user);
                self.error = None;
                self.task = None;
                // Route to home page after logged in
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateUsername(username) => {
                self.request.username = username;
            }
            Msg::UpdatePassword(password) => {
                self.request.password = password;
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default(); /* Prevent event propagation */
            Msg::Request
        });
        let oninput_username = self
            .link
            .callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));

        html! {
            <div class="auth-page">
                // <div class="container page">
                    // <div class="row">
                <h1 class="title">{ "Sign In" }</h1>
                <h2 class="subtitle">
                    <RouterAnchor<AppRoute> route=AppRoute::Register>
                        { "Need an account?" }
                    </RouterAnchor<AppRoute>>
                </h2>
                <ListErrors error=&self.error />
                <form onsubmit=onsubmit>
                    <fieldset class="box">
                        <fieldset class="field">
                            <label class="label">{"Username"}</label>
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Username"
                                    value=&self.request.username
                                    oninput=oninput_username
                                    />
                                <span class="icon is-small is-left">
                                    <i class="fas fa-user"></i>
                                </span>
                                <span class="icon is-small is-right">
                                <i class="fas fa-check"></i>
                                </span>
                            </div>
                            // <p class="help is-success">{"This username is available"}</p>
                        </fieldset>
                        <fieldset class="field">
                            <p class="control has-icons-left">
                                <label class="label">{"Password"}</label>
                                <input
                                    class="input"
                                    type="password"
                                    placeholder="Password"
                                    value=&self.request.password
                                    oninput=oninput_password
                                    />
                                <span class="icon is-small is-left">
                                  <i class="fas fa-lock"></i>
                                </span>
                            </p>
                        </fieldset>
                        <button
                            class="button"
                            type="submit"
                            disabled=false>
                            { "Sign in" }
                        </button>
                    </fieldset>
                </form>
            </div>
                // </div>
            // </div>
        }
    }
}

// <div class="field">
//   <p class="control has-icons-left has-icons-right">
//     <input class="input" type="email" placeholder="Email">
//     <span class="icon is-small is-left">
//       <i class="fas fa-envelope"></i>
//     </span>
//     <span class="icon is-small is-right">
//       <i class="fas fa-check"></i>
//     </span>
//   </p>
// </div>
// <div class="field">
//   <p class="control has-icons-left">
//     <input class="input" type="password" placeholder="Password">
//     <span class="icon is-small is-left">
//       <i class="fas fa-lock"></i>
//     </span>
//   </p>
// </div>
// <div class="field">
//   <p class="control">
//     <button class="button is-success">
//       Login
//     </button>
//   </p>
// </div>
