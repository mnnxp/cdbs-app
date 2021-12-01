// use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink,
    FocusEvent, Html, InputData, ChangeData, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use chrono::NaiveDateTime;

use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use crate::gqls::make_query;

use crate::error::{Error, get_error};
use crate::fragments::{
    list_errors::ListErrors,
    user_certificate::CertificateCard,
    user_add_certificate::AddCertificateCard,
    upload_favicon::UpdateFaviconCard,
};
use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_logged_user, get_current_user};
use crate::types::{
    UUID, UserUpdateInfo, SelfUserInfo, Program, Region,
    UpdatePasswordInfo, TypeAccessTranslateListInfo,
    Certificate, UserCertificate,
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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct PutUpdatePassword;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct ChangeTypeAccessUser;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct DeleteUserData;

pub enum Menu {
    Profile,
    UpdataFavicon,
    Certificates,
    Access,
    Password,
    RemoveProfile,
}

/// Update settings of the author or logout
pub struct Settings {
    // auth: Auth,
    error: Option<Error>,
    request_profile: UserUpdateInfo,
    request_access: i64,
    request_password: UpdatePasswordInfo,
    request_user_password: String,
    // response: Callback<Result<usize, Error>>,
    // task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    current_data: Option<SelfUserInfo>,
    programs: Vec<Program>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessTranslateListInfo>,
    get_result_profile: usize,
    get_result_access: bool,
    get_result_pwd: bool,
    get_result_remove_profile: bool,
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
    RequestUpdatePassword,
    RequestRemoveProfile,
    ResponseError(Error),
    GetUpdateAccessResult(String),
    GetUpdatePwdResult(String),
    GetUpdateProfileData(String),
    GetUpdateProfileResult(String),
    GetRemoveProfileResult(String),
    Ignore,
    UpdateUserPassword(String),
    UpdateTypeAccessId(String),
    UpdateOldPassword(String),
    UpdateNewPassword(String),
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
    GetCurrentData,
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Settings {
            // auth: Auth::new(),
            error: None,
            request_profile: UserUpdateInfo::default(),
            request_access: 0,
            request_password: UpdatePasswordInfo::default(),
            request_user_password: String::new(),
            // response: link.callback(Msg::Response),
            // task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            current_data: None,
            programs: Vec::new(),
            regions: Vec::new(),
            types_access: Vec::new(),
            get_result_profile: 0,
            get_result_access: false,
            get_result_pwd: false,
            get_result_remove_profile: false,
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
                link.send_message(Msg::GetUpdateProfileData(res.clone()));
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
                let request_profile = self.request_profile.clone();
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
                    } = request_profile;
                    let data_user_update = user_update::IptUpdateUserData {
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
                    let res = make_query(UserUpdate::build_query(user_update::Variables {
                        data_user_update
                    })).await;
                    link.send_message(Msg::GetUpdateProfileResult(res.unwrap()));
                })
            },
            Msg::RequestChangeAccess => {
                let new_type_access = self.request_access.clone();
                spawn_local(async move {
                    let res = make_query(ChangeTypeAccessUser::build_query(
                        change_type_access_user::Variables {
                            new_type_access,
                        }
                    )).await;
                    link.send_message(Msg::GetUpdateAccessResult(res.unwrap()));
                })
            },
            Msg::UpdateTypeAccessId(type_access_id) => {
                self.request_access = type_access_id.parse::<i64>().unwrap_or_default();
                debug!("Update: {:?}", type_access_id);
            },
            Msg::GetUpdateAccessResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res.get("changeTypeAccessUser").unwrap().clone()).unwrap();
                        debug!("changeTypeAccessUser: {:?}", result);
                        self.get_result_access = result;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::RequestUpdatePassword => {
                let request_password = self.request_password.clone();
                spawn_local(async move {
                    let UpdatePasswordInfo {
                        old_password,
                        new_password,
                    } = request_password;
                    let data_update_pwd = put_update_password::IptUpdatePassword {
                        oldPassword: old_password,
                        newPassword: new_password,
                    };
                    let res = make_query(PutUpdatePassword::build_query(
                        put_update_password::Variables { data_update_pwd })
                    ).await;
                    link.send_message(Msg::GetUpdatePwdResult(res.unwrap()));
                })
            },
            Msg::UpdateOldPassword(old_password) => {
                self.request_password.old_password = old_password;
            },
            Msg::UpdateNewPassword(new_password) => {
                self.request_password.new_password = new_password;
            },
            Msg::GetUpdatePwdResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res.get("putUpdatePassword").unwrap().clone()).unwrap();
                        debug!("putUpdatePassword: {:?}", result);
                        self.get_result_pwd = result;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
                // self.task = None;
            }
            Msg::GetUpdateProfileData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let user_data: SelfUserInfo = serde_json::from_value(res.get("selfData").unwrap().clone()).unwrap();
                        debug!("User data: {:?}", user_data);
                        self.current_data = Some(user_data.clone());
                        self.request_profile = user_data.into();
                        self.rendered(false);
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            }
            Msg::Ignore => {}
            Msg::UpdateEmail(email) => {
                self.request_profile.email = Some(email);
            },
            Msg::UpdateFirstname(firstname) => {
                self.request_profile.firstname = Some(firstname);
            },
            Msg::UpdateLastname(lastname) => {
                self.request_profile.lastname = Some(lastname);
            },
            Msg::UpdateSecondname(secondname) => {
                self.request_profile.secondname = Some(secondname);
            },
            Msg::UpdateUsername(username) => {
                self.request_profile.username = Some(username);
            },
            Msg::UpdatePhone(phone) => {
                self.request_profile.phone = Some(phone);
            },
            Msg::UpdateDescription(description) => {
                self.request_profile.description = Some(description);
            },
            Msg::UpdateAddress(address) => {
                self.request_profile.address = Some(address);
            },
            Msg::UpdatePosition(position) => {
                self.request_profile.position = Some(position);
            },
            Msg::UpdateTimeZone(time_zone) => {
                self.request_profile.time_zone = Some(time_zone);
            },
            Msg::UpdateProgramId(program_id) => {
                self.request_profile.program_id = Some(program_id.parse::<i64>().unwrap_or_default());
                debug!("Update: {:?}", program_id);
            },
            Msg::UpdateRegionId(region_id) => {
                self.request_profile.region_id = Some(region_id.parse::<i64>().unwrap_or_default());
                debug!("Update: {:?}", region_id);
            },
            Msg::UpdateList(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();
                match res_value.is_null() {
                    false => {
                        // debug!("Result: {:#?}", res_value.clone());
                        self.regions =
                            serde_json::from_value(res_value.get("regions").unwrap().clone()).unwrap();
                        self.programs =
                            serde_json::from_value(res_value.get("programs").unwrap().clone()).unwrap();
                        self.types_access =
                            serde_json::from_value(res_value.get("typesAccess").unwrap().clone()).unwrap();
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
            },
            Msg::GetUpdateProfileResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let updated_rows: usize =
                            serde_json::from_value(res.get("putUserUpdate").unwrap().clone()).unwrap();
                        debug!("Updated rows: {:?}", updated_rows);
                        // update local data
                        set_logged_user(None);
                        spawn_local(async move {
                            let res = get_current_user().await;
                            debug!("update locale slim user: {:?}", res);
                        });
                        self.get_result_profile = updated_rows;
                        link.send_message(Msg::GetCurrentData);
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::GetCurrentData => {
                spawn_local(async move {
                    let res = make_query(
                        GetSelfData::build_query(get_self_data::Variables)
                    ).await.unwrap();
                    link.send_message(Msg::GetUpdateProfileData(res));
                })
            },
            Msg::RequestRemoveProfile => {
                let user_password = self.request_user_password.clone();
                spawn_local(async move {
                    let res = make_query(DeleteUserData::build_query(
                        delete_user_data::Variables { user_password })
                    ).await;
                    link.send_message(Msg::GetRemoveProfileResult(res.unwrap()));
                })
            },
            Msg::UpdateUserPassword(user_password) => {
                self.request_user_password = user_password;
            },
            Msg::GetRemoveProfileResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let delete_user: bool =
                            serde_json::from_value(res.get("deleteUserData").unwrap().clone()).unwrap();
                        debug!("Delete user data: {:?}", delete_user);
                        self.get_result_remove_profile = delete_user;
                        if delete_user {
                            self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
                        }
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
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
        let onsubmit_update_profile = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestUpdateProfile
        });

        let onsubmit_update_access = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestChangeAccess
        });

        let onsubmit_update_password = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestUpdatePassword
        });

        let onsubmit_remove_profile = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestRemoveProfile
        });

        // let onclick_logout = self.link.callback(|_| Msg::Logout);

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
                                    {match self.select_menu {
                                        // Show interface for change profile data
                                        Menu::Profile => html! {<>
                                            <span id="tag-info-updated-date" class="tag is-info is-light">{
                                                match &self.current_data {
                                                    Some(data) => format!("Last updated: {}", data.updated_at),
                                                    None => "Not data".to_string(),
                                                }
                                            }</span>
                                            <span id="tag-info-updated-rows" class="tag is-info is-light">
                                                { format!("Updated rows: {}", self.get_result_profile.clone()) }
                                            </span>
                                            <form onsubmit=onsubmit_update_profile>
                                                { self.fieldset_profile() }
                                                <button
                                                    id="update-settings"
                                                    class="button"
                                                    type="submit"
                                                    disabled=false>
                                                    { "Update Profile" }
                                                </button>
                                            </form>
                                        </>},
                                        // Show interface for change favicon user
                                        Menu::UpdataFavicon => html! {<>
                                            <span id="tag-info-updated-favicon-user" class="tag is-info is-light">
                                                // { format!("Updated certificates: {}", self.get_result_certificates.clone()) }
                                                { "Update favicon" }
                                            </span>
                                            { self.fieldset_update_favicon() }
                                        </>},
                                        // Show interface for add and update Certificates
                                        Menu::Certificates => html! {<>
                                            <span id="tag-info-updated-certificates" class="tag is-info is-light">
                                                // { format!("Updated certificates: {}", self.get_result_certificates.clone()) }
                                                { "Certificates" }
                                            </span>
                                            { self.fieldset_certificates() }
                                            <br/>
                                            { self.fieldset_add_certificate() }
                                        </>},
                                        // Show interface for change access
                                        Menu::Access => html! {<>
                                            <span id="tag-info-updated-access" class="tag is-info is-light">
                                                { format!("Updated access: {}", self.get_result_access.clone()) }
                                            </span>
                                            <form onsubmit=onsubmit_update_access>
                                                { self.fieldset_access() }
                                                <button
                                                    id="update-access"
                                                    class="button"
                                                    type="submit"
                                                    disabled=false>
                                                    { "Update access" }
                                                </button>
                                            </form>

                                            // show Tokens
                                            // update Token
                                            // get new Token
                                            // remove Token
                                            // removed all Tokens
                                        </>},
                                        // Show interface for change password
                                        Menu::Password => html! {<>
                                            <span id="tag-info-updated-pwd" class="tag is-info is-light">
                                              { format!("Updated password: {}", self.get_result_pwd.clone()) }
                                            </span>
                                            <form onsubmit=onsubmit_update_password>
                                                { self.fieldset_password() }
                                                <button
                                                    id="update-password"
                                                    class="button"
                                                    type="submit"
                                                    disabled=false>
                                                    { "Update Password" }
                                                </button>
                                            </form>
                                        </>},
                                        // Show interface for remove profile
                                        Menu::RemoveProfile => html! {<>
                                            <span id="tag-danger-remove-profile" class="tag is-danger is-light">
                                              { "Warning: this removed all data related with profile, it cannot be canceled!" }
                                            </span>
                                            <br/>
                                            <span id="tag-info-remove-profile" class="tag is-info is-light">
                                              { format!("Profile delete: {}", self.get_result_remove_profile) }
                                            </span>
                                            <br/>
                                            <form onsubmit=onsubmit_remove_profile>
                                                { self.fieldset_remove_profile() }
                                                <button
                                                    id="button-remove-profile"
                                                    class="button"
                                                    type="submit"
                                                    disabled=false>
                                                    { "Delete profile data" }
                                                </button>
                                            </form>
                                        </>},
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>
                    // <hr />
                    // <button
                    //     id="logout-button"
                    //     class="button"
                    //     onclick=onclick_logout >
                    //     { "Or click here to logout."}
                    // </button>
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
        let onclick_favicon = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::UpdataFavicon
            ));
        let onclick_certificates = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Certificates
            ));
        let onclick_access = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Access
            ));
        let onclick_password = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Password
            ));
        let onclick_remove_profile = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::RemoveProfile
            ));

        let mut active_profile = "";
        let mut active_favicon = "";
        let mut active_certificates = "";
        let mut active_access = "";
        let mut active_password = "";
        let mut active_remove_profile = "";

        match self.select_menu {
            Menu::Profile => active_profile = "is-active",
            Menu::UpdataFavicon => active_favicon = "is-active",
            Menu::Certificates => active_certificates = "is-active",
            Menu::Access => active_access = "is-active",
            Menu::Password => active_password = "is-active",
            Menu::RemoveProfile => active_remove_profile = "is-active",
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
                      id="profile"
                      class=active_favicon
                      onclick=onclick_favicon>
                        { "Favicon" }
                    </a></li>
                    <li><a
                      id="certificates"
                      class=active_certificates
                      onclick=onclick_certificates>
                        { "Certificates" }
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
                    <li><a
                      id="remove-profile"
                      class=active_remove_profile
                      onclick=onclick_remove_profile>
                        { "Remove profile" }
                    </a></li>
                    // <li><a>{"Notification"}</a></li>
                    // <li><a>{"Billing"}</a></li>
                    // <li><a>{"Close account"}</a></li>
                </ul>
            </aside>
        }
    }

    fn fieldset_update_favicon(
        &self
    ) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::GetCurrentData);

        html! {
            <UpdateFaviconCard
                company_uuid = None
                callback=callback_update_favicon
                />
        }
    }

    fn fieldset_certificates(
        &self
    ) -> Html {
        let mut certificates: &[UserCertificate] = &Vec::new();
        if let Some(ref data) = self.current_data {
            certificates = data.certificates.as_ref();
        };

        match certificates.is_empty() {
            true => html!{
                <div>
                    <span id="tag-info-no-certificates" class="tag is-info is-light">
                        // { format!("Updated certificates: {}", self.get_result_certificates.clone()) }
                        { "You don't have Certificates" }
                    </span>
                </div>
            },
            false => {
                html!{
                    // <p class="card-footer-item">
                    <>{
                        for certificates.iter().map(|cert| {
                            let view_cert: Certificate = cert.into();
                            html! {
                                <CertificateCard
                                    certificate = view_cert
                                    show_cert_btn = true
                                    download_btn = false
                                    change_btn = true
                                    />
                            }
                        })
                    }</>
                    // </p>
                }
            },
        }
    }

    fn fieldset_add_certificate(
        &self
    ) -> Html {
        let user_uuid = self.current_data
            .as_ref()
            .map(|user| user.uuid.to_string())
            .unwrap_or_default();

        let callback_upload_cert = self.link.callback(|_| Msg::GetCurrentData);

        html! {
            <AddCertificateCard
                user_uuid = user_uuid
                callback=callback_upload_cert
                />
        }
    }

    fn fieldset_access(
        &self
    ) -> Html {
      let onchange_type_access_id = self
          .link
          .callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
            ChangeData::Select(el) => el.value(),
            _ => "1".to_string(),
        }));

        html! {
            <fieldset class="columns">
                // first column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Type Access"}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="types-access"
                                  select={self.request_access.to_string()}
                                  onchange=onchange_type_access_id
                                  >
                                { for self.types_access.iter().map(|x|
                                    match self.current_data.as_ref().unwrap().type_access.type_access_id == x.type_access_id {
                                        true => {
                                            html!{
                                                <option value={x.type_access_id.to_string()} selected=true>{&x.name}</option>
                                            }
                                        },
                                        false => {
                                            html!{
                                                <option value={x.type_access_id.to_string()}>{&x.name}</option>
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

    fn fieldset_password(
        &self
    ) -> Html {
        let oninput_old_password = self
            .link
            .callback(|ev: InputData| Msg::UpdateOldPassword(ev.value));
        let oninput_new_password = self
            .link
            .callback(|ev: InputData| Msg::UpdateNewPassword(ev.value));

        html! {
            <fieldset class="columns">
                // first column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Old password"}</label>
                        <input
                            id="old-password"
                            class="input"
                            type="password"
                            placeholder="old password"
                            value={self.request_password.old_password.to_string()}
                            oninput=oninput_old_password />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"New password"}</label>
                        <input
                            id="new-password"
                            class="input"
                            type="password"
                            placeholder="new password"
                            value={self.request_password.new_password.to_string()}
                            oninput=oninput_new_password />
                    </fieldset>
                </fieldset>
            </fieldset>
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
                            value={self.request_profile.firstname
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
                            value={self.request_profile.lastname
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
                            value={self.request_profile.secondname
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
                            value={self.request_profile.username
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
                            value={self.request_profile.email
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
                            value={self.request_profile.description
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
                            value={self.request_profile.position
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
                            value={self.request_profile.phone
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
                            value={self.request_profile.address
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
                                  select={self.request_profile.program_id.unwrap_or_default().to_string()}
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
                                  select={self.request_profile.region_id.unwrap_or_default().to_string()}
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

    fn fieldset_remove_profile(
        &self
    ) -> Html {
        let oninput_user_password = self
            .link
            .callback(|ev: InputData| Msg::UpdateUserPassword(ev.value));

        html! {
            <fieldset class="columns">
                // first column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Input your password for confirm delete profile"}</label>
                        <input
                            id="user-password"
                            class="input"
                            type="password"
                            placeholder="your password"
                            value={self.request_user_password.to_string()}
                            oninput=oninput_user_password />
                    </fieldset>
                </fieldset>
            </fieldset>
        }
    }
}
