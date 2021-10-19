use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use chrono::NaiveDateTime;

use yew::services::ConsoleService;

use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use crate::gqls::make_query;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_token, Auth};
use crate::types::{UUID, UserUpdateInfo, UserInfo};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct UserUpdate;

/// Get data current user
impl From<UserInfo> for UserUpdateInfo {
    fn from(data: UserInfo) -> Self {
        let UserInfo {
            firstname,
            lastname,
            secondname,
            username,
            email,
            position,
            phone,
            address,
            ..
        } = data;

        Self {
            firstname: Some(firstname),
            lastname: Some(lastname),
            secondname: Some(secondname),
            username: Some(username),
            email: Some(email),
            description: None,
            position: Some(position),
            phone: Some(phone),
            time_zone: None,
            address: Some(address),
            region_id: None,
            program_id: None,
        }
    }
}

/// Update settings of the author or logout
pub struct Settings {
    // auth: Auth,
    error: Option<Error>,
    request: UserUpdateInfo,
    // response: Callback<Result<usize, Error>>,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    get_result: usize,
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
    GetResult(String),
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
    UpdateRegionId(i64),
    UpdateProgramId(i64),
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Settings {
            // auth: Auth::new(),
            error: None,
            request: UserUpdateInfo::default(),
            // response: link.callback(Msg::Response),
            task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            get_result: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render && is_authenticated() {
            spawn_local(async move {
                let res = make_query(GetSelfData::build_query(get_self_data::Variables)).await;
                link.send_message(Msg::GetData(res.unwrap()))
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::Request => {
                let request = self.request.clone();
                spawn_local(async move {
                    let UserUpdateInfo {
                        email,
                        firstname,
                        lastname,
                        secondname,
                        username,
                        phone,
                        description,
                        address,
                        position,
                        time_zone,
                        region_id,
                        program_id,
                    } = request;
                    let data = user_update::IptUpdateUserData {
                        email,
                        firstname,
                        lastname,
                        secondname,
                        username,
                        phone,
                        description,
                        address,
                        position,
                        timeZone: time_zone,
                        regionId: region_id,
                        programId: program_id,
                    };
                    let res = make_query(UserUpdate::build_query(user_update::Variables { data })).await;
                    link.send_message(Msg::GetResult(res.unwrap()));
                })
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
                let user_data: UserInfo = serde_json::from_value(res.get("selfData").unwrap().clone()).unwrap();
                ConsoleService::info(format!("User data: {:?}", user_data).as_ref());
                self.request = user_data.into();
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
            Msg::UpdateEmail(email) => {
                self.request.email = Some(email);
            },
            Msg::UpdateFirstname(firstname) => {
                self.request.firstname = Some(firstname);
            },
            Msg::UpdateLastname(lastname) => {
                self.request.lastname = Some(lastname);
            },
            Msg::UpdateSecondname(secondname) => {
                self.request.secondname = Some(secondname);
            },
            Msg::UpdateUsername(username) => {
                self.request.username = Some(username);
            },
            Msg::UpdatePhone(phone) => {
                self.request.phone = Some(phone);
            },
            Msg::UpdateDescription(description) => {
                self.request.description = Some(description);
            },
            Msg::UpdateAddress(address) => {
                self.request.address = Some(address);
            },
            Msg::UpdatePosition(position) => {
                self.request.position = Some(position);
            },
            Msg::UpdateTimeZone(time_zone) => {
                self.request.time_zone = Some(time_zone);
            },
            Msg::UpdateRegionId(region_id) => {
                self.request.region_id = Some(region_id);
            },
            Msg::UpdateProgramId(program_id) => {
                self.request.program_id = Some(program_id);
            },
            Msg::GetResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let updated_rows: usize =
                            serde_json::from_value(res.get("putUserUpdate").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("Updated rows: {:?}", updated_rows).as_ref());
                        self.get_result = updated_rows;
                    },
                    true => {
                        let val_err = data.as_object().unwrap().get("errors").unwrap();
                        let err_message: String =
                            serde_json::from_value(val_err.get(0).unwrap().get("message").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("Err update rows: {:?}", err_message).as_ref());
                        link.send_message(Msg::Response(Err(Error::BadRequest(err_message))))
                    }
                }
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
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
                                                placeholder="firstname"
                                                value={self.request.firstname.as_ref().map(|x| x.to_string()).unwrap_or_default()}
                                                oninput=oninput_firstname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="lastname"
                                                value={self.request.lastname.as_ref().map(|x| x.to_string()).unwrap_or_default()}
                                                oninput=oninput_lastname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="secondname"
                                                value={self.request.secondname.as_ref().map(|x| x.to_string()).unwrap_or_default()}
                                                oninput=oninput_secondname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="username"
                                                value={self.request.username.as_ref().map(|x| x.to_string()).unwrap_or_default()}
                                                oninput=oninput_username />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="email"
                                                placeholder="email"
                                                value={self.request.email.as_ref().map(|x| x.to_string()).unwrap_or_default()}
                                                oninput=oninput_email />
                                        </fieldset>
                                    </fieldset>

                                    <fieldset class="column">
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="position"
                                                value={self.request.position.as_ref().map(|x| x.to_string()).unwrap_or_default()}
                                                oninput=oninput_position />
                                        </fieldset>
                                    </fieldset>
                                    <fieldset class="column">
                                        // data user for id_type_user 2-11
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="phone"
                                                value={self.request.phone.as_ref().map(|x| x.to_string()).unwrap_or_default()}
                                                oninput=oninput_phone />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="address"
                                                value={self.request.address.as_ref().map(|x| x.to_string()).unwrap_or_default()}
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
                                        { self.get_result.clone() }
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
