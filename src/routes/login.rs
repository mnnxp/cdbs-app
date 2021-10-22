use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use yew::services::ConsoleService;
use graphql_client::GraphQLQuery;

use crate::fragments::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::{set_token, Auth, get_token};
use crate::types::{LoginInfo, LoginInfoWrapper, SlimUser, UserToken};
use crate::gqls::make_query;
use wasm_bindgen_futures::spawn_local;
use std::sync::{Arc,Mutex};

/// Login page
pub struct Login {
    auth: Auth,
    error: Option<Error>,
    request: LoginInfo,
    response: Callback<Result<UserToken, Error>>,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Arc<Mutex<Box<dyn Bridge<RouteAgent>>>>,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// Callback when user is logged in successfully
    pub callback: Callback<SlimUser>,
}

pub enum Msg {
    Request,
    Response(Result<UserToken, Error>),
    Ignore,
    UpdateUsername(String),
    UpdatePassword(String),
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetMySelf;

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
            router_agent: Arc::new(Mutex::new(RouteAgent::bridge(link.callback(|_| Msg::Ignore)))) ,
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        let props = self.props.clone();
        let router_agent = self.router_agent.clone();
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
                // set_token(Some(Auth::token_query());
                // self.props.callback.emit(user_info.user);
                // self.error = None;
                // self.task = None;
                // // Route to home page after logged in
                // self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
                set_token(Some(user_info.to_string()));
                spawn_local(async move {
                    let res = make_query(GetMySelf::build_query(get_my_self::Variables)).await.unwrap();
                    ConsoleService::info(format!("{}", res).as_ref());
                    let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                    let res = data.as_object().unwrap().get("data").unwrap();
                    let user : SlimUser = serde_json::from_value(res.get("myself").unwrap().clone()).unwrap();
                    ConsoleService::info(format!("{}", user.username).as_ref());
                    props.callback.emit(user);
                    router_agent.lock().unwrap().send(ChangeRoute(AppRoute::Home.into()));
                });
                ConsoleService::info(format!("{}", get_token().unwrap()).as_ref());
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
                <ListErrors error=self.error.clone() />
                <form onsubmit=onsubmit>
                    <fieldset class="box">
                        <fieldset class="field">
                            <label class="label">{"Username"}</label>
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    id="username"
                                    class="input"
                                    type="text"
                                    placeholder="Username"
                                    value=self.request.username.clone()
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
                                    id="password"
                                    class="input"
                                    type="password"
                                    placeholder="Password"
                                    value=self.request.password.clone()
                                    oninput=oninput_password
                                    />
                                <span class="icon is-small is-left">
                                  <i class="fas fa-lock"></i>
                                </span>
                            </p>
                        </fieldset>
                        <button
                            id="submit-button"
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
