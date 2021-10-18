use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use chrono::NaiveDateTime;

use yew::services::ConsoleService;

use graphql_client::{GraphQLQuery, Response};
use serde_json::Value;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use crate::gqls::make_query;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_token, Auth};
use crate::types::{UUID, SlimUserWrapper, UserInfoWrapper, UserUpdateInfo, UserUpdateInfoWrapper, UserInfo};

/// Update settings of the author or logout
pub struct Settings {
    auth: Auth,
    error: Option<Error>,
    request: UserUpdateInfo,
    response: Callback<Result<usize, Error>>,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    user_data: Option<UserInfo>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<()>,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetSelfData;

pub enum Msg {
    Request,
    Response(Result<usize, Error>),
    GetData(String),
    Ignore,
    Logout,
    UpdateEmail(String),
    UpdateFirstname(String),
    UpdateLastname(String),
    UpdateSecondname(String),
    UpdateUsername(String),
    UpdatePhone(String),
    UpdateDescription(String),
    UpdateAddress(String),
    UpdatePosition(String),
    UpdateTimeZone(String),
    UpdateRegionId(usize),
    UpdateProgramId(usize),
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Settings {
            auth: Auth::new(),
            error: None,
            request: UserUpdateInfo::default(),
            response: link.callback(Msg::Response),
            task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            user_data: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render && is_authenticated() {
            // self.task = Some(self.auth.user_info(self.loaded.clone()));

            spawn_local(async move {
                let res = make_query(GetSelfData::build_query(get_self_data::Variables)).await;
                link.send_message(Msg::GetData(res.unwrap()))
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let mut request = UserUpdateInfoWrapper {
                    user: self.request.clone(),
                };

                self.task = Some(self.auth.save(request, self.response.clone()));
            }
            Msg::Response(Ok(_)) => {
                self.error = None;
                self.task = None;
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::GetData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();
                self.user_data = Some(serde_json::from_value(res.get("selfData").unwrap().clone()).unwrap());
                ConsoleService::info(format!("User data: {:?}", self.user_data).as_ref());
            }
            Msg::Ignore => {}
            Msg::Logout => {
                // Clear global token after logged out
                set_token(None);
                // Notify app to clear current user info
                self.props.callback.emit(());
                // Redirect to home page
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::UpdateEmail(email) => self.request.email = email,
            Msg::UpdateFirstname(firstname) => self.request.firstname = firstname,
            Msg::UpdateLastname(lastname) => self.request.lastname = lastname,
            Msg::UpdateSecondname(secondname) => self.request.secondname = secondname,
            Msg::UpdateUsername(username) => self.request.username = username,
            Msg::UpdatePhone(phone) => self.request.phone = phone,
            Msg::UpdateDescription(description) => self.request.description = description,
            Msg::UpdateAddress(address) => self.request.address = address,
            Msg::UpdatePosition(position) => self.request.position = position,
            Msg::UpdateTimeZone(time_zone) => self.request.time_zone = time_zone,
            Msg::UpdateRegionId(region_id) => self.request.region_id = region_id,
            Msg::UpdateProgramId(program_id) => self.request.program_id = program_id,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let preload_firstname = match &self.user_data {
            Some(x) => x.firstname.clone(),
            None => "firstname".to_string(),
        };
        let preload_lastname = match &self.user_data {
            Some(x) => x.lastname.clone(),
            None => "lastname".to_string(),
        };
        let preload_secondname = match &self.user_data {
            Some(x) => x.secondname.clone(),
            None => "secondname".to_string(),
        };
        let preload_username = match &self.user_data {
            Some(x) => x.username.clone(),
            None => "username".to_string(),
        };
        let preload_email = match &self.user_data {
            Some(x) => x.email.clone(),
            None => "email".to_string(),
        };
        let preload_position = match &self.user_data {
            Some(x) => x.position.clone(),
            None => "position".to_string(),
        };
        let preload_phone = match &self.user_data {
            Some(x) => x.phone.clone(),
            None => "phone".to_string(),
        };
        let preload_address = match &self.user_data {
            Some(x) => x.address.clone(),
            None => "address".to_string(),
        };

        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Request
        });

        let oninput_firstname = self
            .link
            .callback(|ev: InputData| Msg::UpdateFirstname(ev.value));
        let oninput_lastname = self
            .link
            .callback(|ev: InputData| Msg::UpdateLastname(ev.value));
        let oninput_secondname = self
            .link
            .callback(|ev: InputData| Msg::UpdateSecondname(ev.value));
        let oninput_username = self
            .link
            .callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_email = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_position = self
            .link
            .callback(|ev: InputData| Msg::UpdatePosition(ev.value));
        let onclick = self.link.callback(|_| Msg::Logout);
        let oninput_phone = self
            .link
            .callback(|ev: InputData| Msg::UpdatePhone(ev.value));
        let oninput_address = self
            .link
            .callback(|ev: InputData| Msg::UpdateAddress(ev.value));

        html! {
            <div class="settings-page">
                <div class="container page">
                    <div class="row">
                        <div>
                            <h1 class="title">{ "Your Settings" }</h1>
                            <ListErrors error=self.error.clone()/>
                            <form onsubmit=onsubmit>
                                <fieldset class="columns">
                                    // main data of username
                                    <fieldset class="column">
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder={preload_firstname}
                                                value={self.request.firstname.clone()}
                                                oninput=oninput_firstname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder={preload_lastname}
                                                value={self.request.lastname.clone()}
                                                oninput=oninput_lastname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder = {preload_secondname}
                                                value={self.request.secondname.clone()}
                                                oninput=oninput_secondname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder={preload_username}
                                                value={self.request.username.clone()}
                                                oninput=oninput_username />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="email"
                                                placeholder={preload_email}
                                                value={self.request.email.clone()}
                                                oninput=oninput_email />
                                        </fieldset>
                                    </fieldset>

                                    <fieldset class="column">
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder={preload_position}
                                                value={self.request.position.clone()}
                                                oninput=oninput_position />
                                        </fieldset>
                                    </fieldset>
                                    <fieldset class="column">
                                        // data user for id_type_user 2-11
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder={preload_phone}
                                                value={self.request.phone.clone()}
                                                oninput=oninput_phone />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder={preload_address}
                                                value={self.request.address.clone()}
                                                oninput=oninput_address />
                                        </fieldset>
                                    </fieldset>
                                </fieldset>
                                <button
                                    class="button"
                                    type="submit"
                                    disabled=false>
                                    { "Update Settings" }
                                </button>
                                <fieldset class="field">
                                    <span class="tag is-info is-light">
                                        {0}
                                    </span>
                                </fieldset>
                            </form>
                            <hr />
                            <button
                                class="button"
                                onclick=onclick >
                                { "Or click here to logout."}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
