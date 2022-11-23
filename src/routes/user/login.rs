use std::sync::{Arc,Mutex};
use yew::services::fetch::FetchTask;
// use yew::{agent::Bridged, Bridge};
use yew::{Component, Callback, Context, html, Html, Properties, Event, classes};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::fragments::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute::{self, Register, Profile};
use crate::services::{set_token, Auth, set_logged_user, get_logged_user, get_value_field};
use crate::types::{LoginInfo, LoginInfoWrapper, SlimUser, UserToken};
use crate::gqls::make_query;
use crate::gqls::user::{GetMySelf, get_my_self};

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
    router_agent: Arc<Mutex<Box<dyn Bridge<RouteAgent>>>>,
}

pub enum Msg {
    Request,
    Response(Result<UserToken, Error>),
    Ignore,
    UpdateUsername(String),
    UpdatePassword(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Login {
            auth: Auth::new(),
            error: None,
            request: LoginInfo::default(),
            response: ctx.link().callback(Msg::Response),
            router_agent: Arc::new(Mutex::new(RouteAgent::bridge(ctx.link().callback(|_| Msg::Ignore)))) ,
            task: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(user) = get_logged_user() {
                // route to profile page if user already logged
                self.router_agent.lock().unwrap().send(
                    ChangeRoute(Profile { username: user.username }.into())
                );
            };
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props().clone();
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
                    let username = user.username.clone();
                    props.callback.emit(user);
                    router_agent.lock().unwrap().send(ChangeRoute(Profile { username }.into()));
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

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|ev: FocusEvent| {
            ev.prevent_default(); /* Prevent event propagation */
            Msg::Request
        });
        let oninput_username = ctx.link().callback(|ev: Event| Msg::UpdateUsername(ev.value));
        let oninput_password = ctx.link().callback(|ev: Event| Msg::UpdatePassword(ev.value));

        html!{<div class="container page">
            <div class="auth-page">
                <h1 class="title">{ get_value_field(&13) }</h1>
                <h2 class="subtitle">
                    <RouterAnchor<AppRoute> route={Register}>
                        { get_value_field(&18) }
                    </RouterAnchor<AppRoute>>
                </h2>
                <ListErrors error={self.error.clone()} />
                <form onsubmit={onsubmit}>
                    <fieldset class="box">
                        <fieldset class="field">
                            <label class="label">{ get_value_field(&19) }</label>
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    id="username"
                                    class="input"
                                    type="text"
                                    placeholder={ get_value_field(&19) }
                                    value={self.request.username.clone()}
                                    oninput={oninput_username}
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
                            <label class="label">{ get_value_field(&20) }</label>
                            <div class="control has-icons-left">
                                <input
                                    id="password"
                                    class="input"
                                    type="password"
                                    placeholder={ get_value_field(&20) }
                                    value={self.request.password.clone()}
                                    oninput={oninput_password}
                                    />
                                <span class="icon is-small is-left">
                                  <i class="fas fa-lock"></i>
                                </span>
                            </div>
                        </fieldset>
                        <button
                            id="submit-button"
                            class={classes!("button", "is-fullwidth", "is-large")}
                            type="submit"
                            disabled={false}>
                            { get_value_field(&44) }
                        </button>
                    </fieldset>
                </form>
            </div>
        </div>}
    }
}
