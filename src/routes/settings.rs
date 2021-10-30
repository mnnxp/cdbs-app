use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink,
    FocusEvent, MouseEvent, Html, InputData, ChangeData, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use chrono::NaiveDateTime;

use yew::services::ConsoleService;

use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use crate::gqls::make_query;

use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_token};
use crate::types::{
    UUID, UserUpdateInfo, SelfUserInfo, Program, Region
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetSettingDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetSelfData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct UserUpdate;

/// Get data current user
impl From<SelfUserInfo> for UserUpdateInfo {
    fn from(data: SelfUserInfo) -> Self {
        let SelfUserInfo {
            firstname,
            lastname,
            secondname,
            username,
            email,
            description,
            position,
            phone,
            address,
            region,
            program,
            ..
        } = data;

        Self {
            firstname: Some(firstname),
            lastname: Some(lastname),
            secondname: Some(secondname),
            username: Some(username),
            email: Some(email),
            description: Some(description),
            position: Some(position),
            phone: Some(phone),
            time_zone: None,
            address: Some(address),
            region_id: Some(region.region_id as i64),
            program_id: Some(program.id as i64),
        }
    }
}

pub enum Menu {
    Profile,
    Access,
    Password,
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
    current_data: Option<SelfUserInfo>,
    programs: Vec<Program>,
    regions: Vec<Region>,
    get_result: usize,
    select_menu: Menu,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<()>,
}

pub enum Msg {
    SelectMenu(Menu),
    RequestUpdateProfile,
    RequestChangeAccess,
    Response(Result<usize, Error>),
    GetData(String),
    GetResult(String),
    Ignore,
    Logout,
    UpdateFirstname(String),
    UpdateLastname(String),
    UpdateSecondname(String),
    UpdateUsername(String),
    UpdateEmail(String),
    UpdateDescription(String),
    UpdatePhone(String),
    UpdateAddress(String),
    UpdatePosition(String),
    UpdateTimeZone(String),
    UpdateProgramId(String),
    UpdateRegionId(String),
    UpdateList(String),
    GetCurrentData(),
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
            current_data: None,
            programs: Vec::new(),
            regions: Vec::new(),
            get_result: 0,
            select_menu: Menu::Profile,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render && is_authenticated() {
            spawn_local(async move {
                let res = make_query(
                    GetSettingDataOpt::build_query(get_setting_data_opt::Variables)
                ).await.unwrap();
                link.send_message(Msg::GetData(res.clone()));
                link.send_message(Msg::UpdateList(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::SelectMenu(value) => {
                self.select_menu = value;
                self.rendered(false);
            },
            Msg::RequestUpdateProfile => {
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
            Msg::RequestChangeAccess => {
                let request = self.request.clone();
                spawn_local(async move {
                    // let UserUpdateInfo {
                    //     ..
                    // } = request;
                    // let data = user_update::IptUpdateUserData {
                    //     ..
                    // };
                    // let res = make_query(UserUpdate::build_query(user_update::Variables { data })).await;
                    // link.send_message(Msg::GetResult(res.unwrap()));
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

                match res.is_null() {
                    false => {
                        let user_data: SelfUserInfo = serde_json::from_value(res.get("selfData").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("User data: {:?}", user_data).as_ref());
                        self.current_data = Some(user_data.clone());
                        self.request = user_data.into();
                    },
                    true => {
                        link.send_message(Msg::Response(Err(get_error(&data))));
                    }
                }
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
            Msg::UpdateProgramId(program_id) => {
                self.request.program_id = Some(program_id.parse::<i64>().unwrap_or_default());
                ConsoleService::info(format!("Update: {:?}", program_id).as_ref());
            },
            Msg::UpdateRegionId(region_id) => {
                self.request.region_id = Some(region_id.parse::<i64>().unwrap_or_default());
                ConsoleService::info(format!("Update: {:?}", region_id).as_ref());
            },
            Msg::UpdateList(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();
                match res_value.is_null() {
                    false => {
                        self.regions =
                            serde_json::from_value(res_value.get("regions").unwrap().clone()).unwrap();
                        self.programs =
                            serde_json::from_value(res_value.get("programs").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("Update: {:?}", self.programs).as_ref());
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
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
                        link.send_message(Msg::GetCurrentData());
                    },
                    true => {
                        link.send_message(Msg::Response(Err(get_error(&data))));
                    }
                }
            },
            Msg::GetCurrentData() => {
                spawn_local(async move {
                    let res = make_query(
                        GetSelfData::build_query(get_self_data::Variables)
                    ).await.unwrap();
                    link.send_message(Msg::GetData(res));
                })
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit_update_profile = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestUpdateProfile
        });

        let onclick_logout = self.link.callback(|_| Msg::Logout);

        html! {
            <div class="settings-page">
                <ListErrors error=self.error.clone()/>
                <div class="container page">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-one-quarter">
                                { self.view_menu() }
                            </div>
                            // <h1 class="title">{ "Your Settings" }</h1>
                            <div class="column">
                                <div class="card">
                                  <div class="card-content">
                                    <span class="tag is-info is-light">{
                                      match &self.current_data {
                                          Some(data) => format!("Last updated: {}", data.updated_at),
                                          None => "Not data".to_string(),
                                      }
                                    }</span>

                                    {match self.select_menu {
                                        Menu::Profile => html! {<>
                                            // Show interface for change profile data
                                            <span class="tag is-info is-light">
                                              { format!("Updated rows: {}", self.get_result.clone()) }
                                            </span>
                                            <form onsubmit=onsubmit_update_profile>
                                                { self.fieldset_profile() }
                                                <button
                                                    id="update-settings"
                                                    class="button"
                                                    type="submit"
                                                    disabled=false>
                                                    { "Update Settings" }
                                                </button>
                                            </form>
                                        </>},
                                        // Show interface for change access
                                        Menu::Access => html! {

                                        },
                                        // Show interface for change password
                                        Menu::Password => html! {

                                        },
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>
                    <hr />
                    <button
                        id="logout-button"
                        class="button"
                        onclick=onclick_logout >
                        { "Or click here to logout."}
                    </button>
                </div>
            </div>
          </div>
        }
    }
}

impl Settings {
    fn view_menu(
        &self
    ) -> Html {
        let onclick_profile = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Profile
            ));
        let onclick_access = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Access
            ));
        let onclick_password = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Password
            ));

        let mut active_profile = "";
        let mut active_access = "";
        let mut active_password = "";

        match self.select_menu {
            Menu::Profile => active_profile = "is-active",
            Menu::Access => active_access = "is-active",
            Menu::Password => active_password = "is-active",
        }

        html! {
            <aside class="menu">
                <p class="menu-label">
                    {"User Settings"}
                </p>
                <ul class="menu-list">
                    <li><a
                      id="profile"
                      class=active_profile
                      onclick=onclick_profile>
                        { "Profile" }
                    </a></li>
                    <li><a
                      id="access"
                      class=active_access
                      onclick=onclick_access>
                        { "Access" }
                    </a></li>
                    <li><a
                      id="password"
                      class=active_password
                      onclick=onclick_password>
                        { "Password" }
                    </a></li>
                    // <li><a>{"Notification"}</a></li>
                    // <li><a>{"Billing"}</a></li>
                    // <li><a>{"Close account"}</a></li>
                </ul>
            </aside>
        }
    }

    fn fieldset_profile(
        &self
    ) -> Html {
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
        let oninput_description = self
            .link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let oninput_position = self
            .link
            .callback(|ev: InputData| Msg::UpdatePosition(ev.value));
        let oninput_phone = self
            .link
            .callback(|ev: InputData| Msg::UpdatePhone(ev.value));
        let oninput_address = self
            .link
            .callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_program_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateProgramId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let onchange_region_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateRegionId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        html! {
            <fieldset class="columns">
                // first column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Firstname"}</label>
                        <input
                            id="firstname"
                            class="input"
                            type="text"
                            placeholder="firstname"
                            value={self.request.firstname
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_firstname />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Lastname"}</label>
                        <input
                            id="lastname"
                            class="input"
                            type="text"
                            placeholder="lastname"
                            value={self.request.lastname
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_lastname />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Secondname"}</label>
                        <input
                            id="secondname"
                            class="input"
                            type="text"
                            placeholder="secondname"
                            value={self.request.secondname
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_secondname />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Username"}</label>
                        <input
                            id="username"
                            class="input"
                            type="text"
                            placeholder="username"
                            value={self.request.username
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_username />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Email"}</label>
                        <input
                            id="email"
                            class="input"
                            type="email"
                            placeholder="email"
                            value={self.request.email
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_email />
                    </fieldset>
                </fieldset>

                // second column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Description"}</label>
                        <textarea
                            id="description"
                            class="input"
                            type="description"
                            placeholder="description"
                            value={self.request.description
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_description />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Position"}</label>
                        <input
                            id="position"
                            class="input"
                            type="text"
                            placeholder="position"
                            value={self.request.position
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_position />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Phone"}</label>
                        <input
                            id="phone"
                            class="input"
                            type="text"
                            placeholder="phone"
                            value={self.request.phone
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_phone />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Address"}</label>
                        <input
                            id="address"
                            class="input"
                            type="text"
                            placeholder="address"
                            value={self.request.address
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_address />
                    </fieldset>
                </fieldset>

                // third column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Program"}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="program"
                                  select={self.request.program_id.unwrap_or_default().to_string()}
                                  onchange=oninput_program_id
                                  >
                                { for self.programs.iter().map(|x|
                                    match self.current_data.as_ref().unwrap().program.id == x.id {
                                        true => {
                                            html!{
                                                <option value={x.id.to_string()} selected=true>{&x.name}</option>
                                            }
                                        },
                                        false => {
                                            html!{
                                                <option value={x.id.to_string()}>{&x.name}</option>
                                            }
                                        },
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Region"}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="region"
                                  select={self.request.region_id.unwrap_or_default().to_string()}
                                  onchange=onchange_region_id
                                  >
                                { for self.regions.iter().map(|x|
                                    match self.current_data.as_ref().unwrap().region.region_id == x.region_id {
                                        true => {
                                            html!{
                                                <option value={x.region_id.to_string()} selected=true>{&x.region}</option>
                                            }
                                        },
                                        false => {
                                            html!{
                                                <option value={x.region_id.to_string()}>{&x.region}</option>
                                            }
                                        },
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </fieldset>
            </fieldset>
        }
    }
}
