use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component,
    ComponentLink, FocusEvent, Html, InputData, Properties, ShouldRender
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use graphql_client::GraphQLQuery;
use log::debug;

use crate::fragments::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::{set_token, Auth, set_logged_user};
use crate::types::{UUID, LoginInfo, LoginInfoWrapper, SlimUser, UserToken};
use crate::gqls::make_query;
use wasm_bindgen_futures::spawn_local;
use std::sync::{Arc,Mutex};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// Callback when user is logged in successfully
    pub callback: Callback<SlimUser>,
}

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
        let props = self.props.clone();
        let router_agent = self.router_agent.clone();
        match msg {
            Msg::Request => {
                let request = LoginInfoWrapper {
                    user: self.request.clone(),
                };
                self.task = Some(self.auth.login(request, self.response.clone()));
            },
            Msg::Response(Ok(user_info)) => {
                set_token(Some(user_info.to_string()));
                spawn_local(async move {
                    let res = make_query(GetMySelf::build_query(get_my_self::Variables)).await.unwrap();
                    debug!("res: {}", res);
                    let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                    let res = data.as_object().unwrap().get("data").unwrap();
                    let user_json = res.get("myself").unwrap().clone();
                    set_logged_user(Some(user_json.to_string()));
                    let user : SlimUser = serde_json::from_value(user_json).unwrap();
                    debug!("user.username: {}", user.username);
                    props.callback.emit(user);
                    router_agent.lock().unwrap().send(ChangeRoute(AppRoute::Home.into()));
                });
                // debug!("get_token().unwrap(): {:?}", get_token().unwrap());
            },
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            },
            Msg::UpdateUsername(username) => self.request.username = username,
            Msg::UpdatePassword(password) => self.request.password = password,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
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

        html!{<div class="container page">
            <div class="auth-page">
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
                            <label class="label">{"Password"}</label>
                            <div class="control has-icons-left">
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
                            </div>
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
        </div>}
    }
}
