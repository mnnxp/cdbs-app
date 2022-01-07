// use yew::services::fetch::FetchTask;
use chrono::NaiveDateTime;
use web_sys::MouseEvent;
use yew::{
    agent::Bridged, classes, html, Bridge, Callback, ChangeData, Component, ComponentLink,
    FocusEvent, Html, InputData, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::gqls::make_query;
use graphql_client::GraphQLQuery;
use log::debug;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::{
    list_errors::ListErrors,
    side_menu::{MenuItem, SideMenu},
    upload_favicon::UpdateFaviconBlock,
    user::{AddUserCertificateCard, UserCertificatesCard},
};
use crate::routes::AppRoute;
use crate::services::{get_current_user, is_authenticated, set_logged_user};
use crate::types::{
    Program, Region, SelfUserInfo, TypeAccessInfo, UpdatePasswordInfo, UserUpdateInfo, UUID,
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

#[derive(Clone, PartialEq)]
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
    error: Option<Error>,
    request_profile: UserUpdateInfo,
    request_access: i64,
    request_password: UpdatePasswordInfo,
    request_user_password: String,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    // props: Props,
    link: ComponentLink<Self>,
    current_data: Option<SelfUserInfo>,
    programs: Vec<Program>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessInfo>,
    get_result_profile: usize,
    get_result_access: bool,
    get_result_pwd: bool,
    get_result_remove_profile: bool,
    select_menu: Menu,
}

#[derive(Clone)]
pub enum Msg {
    RequestUpdateProfile,
    RequestChangeAccess,
    RequestUpdatePassword,
    RequestRemoveProfile,
    GetUpdateAccessResult(String),
    GetUpdatePwdResult(String),
    GetUpdateProfileData(String),
    GetUpdateProfileResult(String),
    GetRemoveProfileResult(String),
    GetCurrentData,
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
    SelectMenu(Menu),
    ClearError,
    Ignore,
}

impl Component for Settings {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Settings {
            error: None,
            request_profile: UserUpdateInfo::default(),
            request_access: 0,
            request_password: UpdatePasswordInfo::default(),
            request_user_password: String::new(),
            // response: link.callback(Msg::Response),
            // task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            // props,
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
                let res = make_query(GetSettingDataOpt::build_query(
                    get_setting_data_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::GetUpdateProfileData(res.clone()));
                link.send_message(Msg::UpdateList(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestUpdateProfile => {
                let ipt_update_user_data = user_update::IptUpdateUserData {
                    email: self.request_profile.email.clone(),
                    firstname: self.request_profile.firstname.clone(),
                    lastname: self.request_profile.lastname.clone(),
                    secondname: self.request_profile.secondname.clone(),
                    username: self.request_profile.username.clone(),
                    phone: self.request_profile.phone.clone(),
                    description: self.request_profile.description.clone(),
                    address: self.request_profile.address.clone(),
                    position: self.request_profile.position.clone(),
                    timeZone: self.request_profile.time_zone.clone(),
                    regionId: self.request_profile.region_id.clone(),
                    programId: self.request_profile.program_id.clone(),
                };
                spawn_local(async move {
                    let res = make_query(UserUpdate::build_query(
                        user_update::Variables { ipt_update_user_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateProfileResult(res));
                })
            },
            Msg::RequestChangeAccess => {
                let new_type_access = self.request_access.clone();
                spawn_local(async move {
                    let res = make_query(ChangeTypeAccessUser::build_query(
                        change_type_access_user::Variables{ new_type_access }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateAccessResult(res));
                })
            },
            Msg::RequestUpdatePassword => {
                let ipt_update_password = put_update_password::IptUpdatePassword {
                    oldPassword: self.request_password.old_password.clone(),
                    newPassword: self.request_password.new_password.clone(),
                };
                spawn_local(async move {
                    let res = make_query(PutUpdatePassword::build_query(put_update_password::Variables{
                        ipt_update_password
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdatePwdResult(res));
                })
            },
            Msg::RequestRemoveProfile => {
                let user_password = self.request_user_password.clone();
                spawn_local(async move {
                    let res = make_query(DeleteUserData::build_query(delete_user_data::Variables{
                        user_password
                    })).await.unwrap();
                    link.send_message(Msg::GetRemoveProfileResult(res));
                })
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
                    true => self.error = Some(get_error(&data)),
                }
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
                    true => self.error = Some(get_error(&data)),
                }
            },
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
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetRemoveProfileResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        self.get_result_remove_profile =
                            serde_json::from_value(res.get("deleteUserData").unwrap().clone()).unwrap();
                        debug!("Delete user data: {:?}", self.get_result_remove_profile);
                        if self.get_result_remove_profile {
                            self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
                        }
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateTypeAccessId(type_access_id) =>
                self.request_access = type_access_id.parse::<i64>().unwrap_or_default(),
            Msg::UpdateOldPassword(old_password) => self.request_password.old_password = old_password,
            Msg::UpdateNewPassword(new_password) => self.request_password.new_password = new_password,
            Msg::UpdateEmail(email) => self.request_profile.email = Some(email),
            Msg::UpdateFirstname(firstname) => self.request_profile.firstname = Some(firstname),
            Msg::UpdateLastname(lastname) => self.request_profile.lastname = Some(lastname),
            Msg::UpdateSecondname(secondname) => self.request_profile.secondname = Some(secondname),
            Msg::UpdateUsername(username) => self.request_profile.username = Some(username),
            Msg::UpdatePhone(phone) => self.request_profile.phone = Some(phone),
            Msg::UpdateDescription(description) => self.request_profile.description = Some(description),
            Msg::UpdateAddress(address) => self.request_profile.address = Some(address),
            Msg::UpdatePosition(position) => self.request_profile.position = Some(position),
            Msg::UpdateTimeZone(time_zone) => self.request_profile.time_zone = Some(time_zone),
            Msg::UpdateProgramId(program_id) =>
                self.request_profile.program_id = Some(program_id.parse::<i64>().unwrap_or_default()),
            Msg::UpdateRegionId(region_id) =>
                self.request_profile.region_id = Some(region_id.parse::<i64>().unwrap_or_default()),
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
                    true => self.error = Some(get_error(&data)),
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
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetCurrentData => {
                spawn_local(async move {
                    let res = make_query(GetSelfData::build_query(
                        get_self_data::Variables
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateProfileData(res));
                })
            },
            Msg::UpdateUserPassword(user_password) => {
                self.request_user_password = user_password;
            },
            Msg::SelectMenu(value) => {
                self.select_menu = value;
                self.rendered(false);
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

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

        html!{
            <div class="settings-page">
                <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
                <div class="container page">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-flex">
                                { self.view_menu() }
                                <div class="card is-flex-grow-1" >
                                  <div class="card-content">
                                    {match self.select_menu {
                                        // Show interface for change profile data
                                        Menu::Profile => html!{<>
                                            {self.show_update_profile_info()}
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
                                        Menu::UpdataFavicon => {self.fieldset_update_favicon()},
                                        // Show interface for add and update Certificates
                                        Menu::Certificates => html!{<>
                                            <div class="block">
                                                <span id="updated-certificates" class=classes!("overflow-title", "has-text-weight-bold")>
                                                    { "Certificates" }
                                                </span>
                                            </div>
                                            { self.fieldset_add_certificate() }
                                            { self.fieldset_certificates() }
                                        </>},
                                        // Show interface for change access
                                        Menu::Access => html!{<>
                                            <span id="tag-info-updated-access" class=classes!("tag", "is-info", "is-light")>
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
                                            // todo!(tasks:)
                                            // show Tokens
                                            // update Token
                                            // get new Token
                                            // remove Token
                                            // removed all Tokens
                                        </>},
                                        // Show interface for change password
                                        Menu::Password => html!{<>
                                            <span id="tag-info-updated-pwd" class=classes!("tag", "is-info", "is-light")>
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
                                        Menu::RemoveProfile => html!{<>
                                            <span id="tag-danger-remove-profile" class=classes!("tag", "is-danger", "is-light")>
                                              { "Warning: this removed all data related with profile, it cannot be canceled!" }
                                            </span>
                                            <br/>
                                            <span id="tag-info-remove-profile" class=classes!("tag", "is-info", "is-light")>
                                              { format!("Profile delete: {}", self.get_result_remove_profile) }
                                            </span>
                                            <br/>
                                            <form onsubmit=onsubmit_remove_profile>
                                                { self.fieldset_remove_profile() }
                                                <button
                                                    id="button-remove-profile"
                                                    class="button is-danger"
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
                </div>
            </div>
          </div>
        }
    }
}

impl Settings {
    fn fileset_generator(
        &self,
        id: &str,
        label: &str,
        placeholder: &str,
        value: String,
        oninput: Callback<InputData>,
    ) -> Html {
        let mut class = "input";
        let (input_tag, input_type) = match id {
            "email" => ("input", "email"),
            "description" => {
                class = "textarea";
                ("textarea", "text")
            },
            "password" => ("input", "password"),
            _ => ("input", "text"),
        };

        html!{
            <fieldset class="field">
                <label class="label">{label.to_string()}</label>
                <@{input_tag}
                    id={id.to_string()}
                    class={class}
                    type={input_type}
                    placeholder={placeholder.to_string()}
                    value={value}
                    oninput=oninput ></@>
            </fieldset>
        }
    }

    fn show_update_profile_info(&self) -> Html {
        html!{<div class="media">
            <div id="updated-rows" class="media-left">
                <span class=classes!("overflow-title", "has-text-weight-bold")>{"Updated rows: "}</span>
                <span class="overflow-title">{self.get_result_profile.clone()}</span>
            </div>
            <div id="updated-date" class="media-right">
                <span class=classes!("overflow-title", "has-text-weight-bold")>{"Last updated: "}</span>
                {match &self.current_data {
                    Some(data) => html!{<span class="overflow-title">
                        {format!("{:.*}", 19, data.updated_at.to_string())}
                    </span>},
                    None => html!{<span>{"not data"}</span>},
                }}
            </div>
        </div>}
    }

    fn cb_generator(&self, cb: Menu) -> Callback<MouseEvent> {
        self.link.callback(move |_| Msg::SelectMenu(cb.clone()))
    }

    fn view_menu(&self) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            // profile MenuItem
            MenuItem {
                title: "Profile".to_string(),
                action: self.cb_generator(Menu::Profile),
                is_active: self.select_menu == Menu::Profile,
                ..Default::default()
            },
            // favicon MenuItem
            MenuItem {
                title: "Favicon".to_string(),
                action: self.cb_generator(Menu::UpdataFavicon),
                is_active: self.select_menu == Menu::UpdataFavicon,
                ..Default::default()
            },
            // certificates MenuItem
            MenuItem {
                title: "Certificates".to_string(),
                action: self.cb_generator(Menu::Certificates),
                is_active: self.select_menu == Menu::Certificates,
                ..Default::default()
            },
            // access MenuItem
            MenuItem {
                title: "Access".to_string(),
                action: self.cb_generator(Menu::Access),
                is_active: self.select_menu == Menu::Access,
                ..Default::default()
            },
            // password MenuItem
            MenuItem {
                title: "Password".to_string(),
                action: self.cb_generator(Menu::Password),
                is_active: self.select_menu == Menu::Password,
                ..Default::default()
            },
            // remove profile MenuItem
            MenuItem {
                title: "Remove profile".to_string(),
                action: self.cb_generator(Menu::RemoveProfile),
                is_active: self.select_menu == Menu::RemoveProfile,
                ..Default::default()
            },
        ];

        html! {
          <div style="margin-right: 18px;z-index: 1;" >
              <SideMenu menu_arr={menu_arr} ></SideMenu>
          </div>
        }
    }

    fn fieldset_update_favicon(&self) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::GetCurrentData);

        html! {
            <UpdateFaviconBlock
                company_uuid = None
                callback=callback_update_favicon
                />
        }
    }

    fn fieldset_certificates(&self) -> Html {
        match &self.current_data {
            Some(current_data) => html! {
                <UserCertificatesCard
                    user_uuid = self.current_data.as_ref().map(|x| x.uuid.clone()).unwrap_or_default()
                    certificates = current_data.certificates.clone()
                    show_cert_btn = true
                    download_btn = false
                    manage_btn = true
                />
            },
            None => html! {<span class=classes!("tag", "is-info", "is-light")>{"Not fount certificates"}</span>},
        }
    }

    fn fieldset_add_certificate(&self) -> Html {
        let user_uuid = self
            .current_data
            .as_ref()
            .map(|user| user.uuid.to_string())
            .unwrap_or_default();

        let callback_upload_cert = self.link.callback(|_| Msg::GetCurrentData);

        html! {
            <AddUserCertificateCard
                user_uuid = user_uuid
                callback=callback_upload_cert
            />
        }
    }

    fn fieldset_access(&self) -> Html {
        let onchange_type_access_id = self.link.callback(|ev: ChangeData| {
            Msg::UpdateTypeAccessId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            })
        });

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
                                    html!{<option value={x.type_access_id.to_string()}>{&x.name}</option>}
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </fieldset>
            </fieldset>
        }
    }

    fn fieldset_password(&self) -> Html {
        let oninput_old_password = self.link.callback(|ev: InputData| Msg::UpdateOldPassword(ev.value));
        let oninput_new_password = self.link.callback(|ev: InputData| Msg::UpdateNewPassword(ev.value));

        html! {
            <fieldset class="columns">
                // first column
                <fieldset class="column">
                    {self.fileset_generator(
                        "password", "Old password", "old password",
                        self.request_password.old_password.to_string(),
                        oninput_old_password
                    )}
                    {self.fileset_generator(
                        "password", "New password", "new-password",
                        self.request_password.new_password.to_string(),
                        oninput_new_password
                    )}
                </fieldset>
            </fieldset>
        }
    }

    fn fieldset_profile(&self) -> Html {
        let oninput_firstname = self.link.callback(|ev: InputData| Msg::UpdateFirstname(ev.value));
        let oninput_lastname = self.link.callback(|ev: InputData| Msg::UpdateLastname(ev.value));
        let oninput_secondname = self.link.callback(|ev: InputData| Msg::UpdateSecondname(ev.value));
        let oninput_username = self.link.callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_email = self.link.callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let oninput_position = self.link.callback(|ev: InputData| Msg::UpdatePosition(ev.value));
        let oninput_phone = self.link.callback(|ev: InputData| Msg::UpdatePhone(ev.value));
        let oninput_address = self.link.callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_program_id = self.link.callback(|ev: ChangeData| {
            Msg::UpdateProgramId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            })
        });
        let onchange_region_id = self.link.callback(|ev: ChangeData| {
            Msg::UpdateRegionId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            })
        });

        html! {<>
            // first columns (username, email)
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "username", "Username", "Username",
                        self.request_profile.username.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_username
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "email", "Email", "Email",
                        self.request_profile.email.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_email
                    )}
                </div>
            </div>

            // second columns (fio)
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "firstname", "Firstname", "Firstname",
                        self.request_profile.firstname.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_firstname
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "lastname", "Lastname", "Lastname",
                        self.request_profile.lastname.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_lastname
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "secondname", "Secondname", "Secondname",
                        self.request_profile.secondname.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_secondname
                    )}
                </div>
            </div>

            // third columns (position, phone, address)
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "position", "Position", "Position",
                        self.request_profile.position.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_position
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "phone", "Phone", "Phone",
                        self.request_profile.phone.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_phone
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "address", "Address", "Address",
                        self.request_profile.address.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_address
                    )}
                </div>
            </div>

            // fourth columns (program, region)
            <div class="columns">
                <div class="column">
                    <label class="label">{"Program"}</label>
                    <div class="control">
                        <div class="select">
                          <select
                              id="program"
                              select={self.request_profile.program_id.unwrap_or_default().to_string()}
                              onchange=oninput_program_id
                              >
                            { for self.programs.iter().map(|x|
                                html!{<option value={x.id.to_string()}>{&x.name}</option>}
                            )}
                          </select>
                        </div>
                    </div>
                </div>
                <div class="column">
                    <label class="label">{"Region"}</label>
                    <div class="control">
                        <div class="select">
                          <select
                              id="region"
                              select={self.request_profile.region_id.unwrap_or_default().to_string()}
                              onchange=onchange_region_id
                              >
                            { for self.regions.iter().map(|x|
                                html!{<option value={x.region_id.to_string()}>{&x.region}</option>}
                            )}
                          </select>
                        </div>
                    </div>
                </div>
            </div>

            // fifth column (only description)
            {self.fileset_generator(
                "description", "Description", "Description",
                self.request_profile.description.as_ref().map(|x| x.clone()).unwrap_or_default(),
                oninput_description
            )}
            <br/>
        </>}
    }

    fn fieldset_remove_profile(&self) -> Html {
        let oninput_user_password = self.link.callback(|ev: InputData| Msg::UpdateUserPassword(ev.value));

        self.fileset_generator(
            "password",
            "Input your password for confirm delete profile",
            "your password",
            self.request_user_password.to_string(),
            oninput_user_password,
        )
    }
}
