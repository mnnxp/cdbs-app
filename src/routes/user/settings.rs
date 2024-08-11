use web_sys::MouseEvent;
use yew::{agent::Bridged, classes, html, Bridge, Callback, ChangeData, Component, ComponentLink, FocusEvent, Html, InputData, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::{
    buttons::{ft_delete_btn, ft_submit_btn},
    notification::show_notification,
    list_errors::ListErrors,
    side_menu::{MenuItem, SideMenu},
    upload_favicon::UpdateFaviconBlock,
    user::{AddUserCertificateCard, UserCertificatesCard},
};
use crate::routes::AppRoute;
use crate::services::content_adapter::DateDisplay;
use crate::services::{get_current_user, set_token, set_logged_user, get_logged_user, get_value_field, resp_parsing, get_value_response, get_from_value};
use crate::types::{Program, Region, SelfUserInfo, TypeAccessInfo, UpdatePasswordInfo, UserUpdateInfo};
use crate::gqls::make_query;
use crate::gqls::user::{
    GetSettingDataOpt, get_setting_data_opt,
    GetSelfData, get_self_data,
    UserUpdate, user_update,
    PutUpdatePassword, put_update_password,
    ChangeTypeAccessUser, change_type_access_user,
    DeleteUserData, delete_user_data,
};

#[derive(Clone, PartialEq)]
pub enum Menu {
    Profile,
    UpdateFavicon,
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
    current_username: String,
    programs: Vec<Program>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessInfo>,
    get_result_profile: usize,
    get_result_access: bool,
    get_result_pwd: bool,
    get_result_remove_profile: bool,
    get_confirm: String,
    select_menu: Menu,
}

#[derive(Clone)]
pub enum Msg {
    OpenProfile,
    RequestCurrentData,
    RequestUpdateProfile,
    RequestChangeAccess,
    RequestUpdatePassword,
    RequestRemoveProfile,
    ResponseError(Error),
    GetUpdateAccessResult(String),
    GetUpdatePwdResult(String),
    GetProfileDataResult(String),
    GetUpdateProfileResult(String),
    GetRemoveProfileResult(String),
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
    GetUpdateListResult(String),
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
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            // props,
            link,
            current_data: None,
            current_username: String::new(),
            programs: Vec::new(),
            regions: Vec::new(),
            types_access: Vec::new(),
            get_result_profile: 0,
            get_result_access: false,
            get_result_pwd: false,
            get_result_remove_profile: false,
            get_confirm: String::new(),
            select_menu: Menu::Profile,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let None = get_logged_user() {
                // route to login page if not found token
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
            };

            let link = self.link.clone();

            spawn_local(async move {
                let res = make_query(GetSettingDataOpt::build_query(
                    get_setting_data_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::GetProfileDataResult(res.clone()));
                link.send_message(Msg::GetUpdateListResult(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::OpenProfile => {
                // Redirect to user page
                if let Some(user_data) = &self.current_data {
                    self.router_agent.send(ChangeRoute(
                        AppRoute::Profile(user_data.username.clone()).into()
                    ));
                }
            },
            Msg::RequestCurrentData => {
                spawn_local(async move {
                    let res = make_query(GetSelfData::build_query(
                        get_self_data::Variables
                    )).await.unwrap();
                    link.send_message(Msg::GetProfileDataResult(res));
                })
            },
            Msg::RequestUpdateProfile => {
                let username =
                    match matches!(
                        &self.request_profile.username,
                        Some(username) if &self.current_username == username
                    ) {
                        true => None,
                        false => self.request_profile.username.clone(),
                    };

                let ipt_update_user_data = user_update::IptUpdateUserData {
                    email: self.request_profile.email.clone(),
                    firstname: self.request_profile.firstname.clone(),
                    lastname: self.request_profile.lastname.clone(),
                    secondname: self.request_profile.secondname.clone(),
                    username,
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
                        user_update::Variables{ipt_update_user_data}
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateProfileResult(res));
                })
            },
            Msg::RequestChangeAccess => {
                let new_type_access = self.request_access.clone();
                spawn_local(async move {
                    let res = make_query(ChangeTypeAccessUser::build_query(
                        change_type_access_user::Variables{new_type_access}
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
                if self.get_confirm == self.current_username {
                    let user_password = self.request_user_password.clone();
                    spawn_local(async move {
                        let res = make_query(DeleteUserData::build_query(delete_user_data::Variables{
                            user_password
                        })).await.unwrap();
                        link.send_message(Msg::GetRemoveProfileResult(res));
                    })
                } else {
                    self.get_confirm = self.current_username.clone();
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetUpdateAccessResult(res) => {
                match resp_parsing(res, "changeTypeAccessUser") {
                    Ok(result) => self.get_result_access = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("changeTypeAccessUser: {:?}", self.get_result_access);
            },
            Msg::GetUpdatePwdResult(res) => {
                match resp_parsing(res, "putUpdatePassword") {
                    Ok(result) => self.get_result_pwd = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("putUpdatePassword: {:?}", self.get_result_pwd);
            },
            Msg::GetProfileDataResult(res) => {
                match resp_parsing::<SelfUserInfo>(res, "selfData") {
                    Ok(user_data) => {
                        self.current_data = Some(user_data.clone());
                        self.current_username = user_data.username.clone();
                        self.request_profile = user_data.into();
                        self.rendered(false);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateListResult(res) => {
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.regions = get_from_value(value, "regions").unwrap_or_default();
                        self.programs = get_from_value(value, "programs").unwrap_or_default();
                        self.types_access = get_from_value(value, "typesAccess").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateProfileResult(res) => {
                match resp_parsing(res, "putUserUpdate") {
                    Ok(result) => {
                        self.get_result_profile = result;
                        debug!("Updated rows: {:?}", self.get_result_profile);
                        // update local data
                        set_logged_user(None);
                        spawn_local(async move {
                            let response = get_current_user().await;
                            debug!("update locale slim user: {:?}", response);
                        });
                        link.send_message(Msg::RequestCurrentData);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetRemoveProfileResult(res) => {
                match resp_parsing(res, "deleteUserData") {
                    Ok(result) => {
                        self.get_result_remove_profile = result;
                        debug!("Delete user data: {:?}", self.get_result_remove_profile);
                        if self.get_result_remove_profile {
                            // Clear global token and logged user after delete profile
                            set_token(None);
                            set_logged_user(None);
                            self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
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
            Msg::UpdateUserPassword(user_password) =>
                self.request_user_password = user_password,
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

        html!{
            <div class="settings-page">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <div class="container page">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-flex">
                                {self.view_menu()}
                                <div class="card is-flex-grow-1" >
                                  <div class="card-content">
                                    {match self.select_menu {
                                        // Show interface for change profile data
                                        Menu::Profile => html!{<>
                                            <h4 id="change-profile" class="title is-4">{get_value_field(&63)}</h4> // "Profile"
                                            {self.show_update_profile_info()}
                                            <form onsubmit={onsubmit_update_profile}>
                                                {self.change_profile_card()}
                                                {ft_submit_btn("update-settings")}
                                            </form>
                                        </>},
                                        // Show interface for change favicon user
                                        Menu::UpdateFavicon => {self.update_favicon_card()},
                                        // Show interface for add and update Certificates
                                        Menu::Certificates => html!{<>
                                            <h4 id="change-certificates" class="title is-4">{get_value_field(&64)}</h4> // "Certificates"
                                            {self.add_certificate_card()}
                                            {self.change_certificates_card()}
                                        </>},
                                        // Show interface for change access
                                        Menu::Access => html!{<>
                                            <h4 id="change-access" class="title is-4">{get_value_field(&65)}</h4> // "Access"
                                            {show_notification(
                                                &format!("{}: {}", get_value_field(&68), self.get_result_access),
                                                "is-success",
                                                self.get_result_access,
                                            )}
                                            <form onsubmit={onsubmit_update_access}>
                                                {self.change_access_card()}
                                                {ft_submit_btn("update-access")}
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
                                            <h4 id="change-password" class="title is-4">{get_value_field(&20)}</h4> // "Password"
                                            {show_notification(
                                                get_value_field(&69),
                                                "is-success",
                                                self.get_result_pwd,
                                            )}
                                            <form onsubmit={onsubmit_update_password}>
                                                {self.update_password_card()}
                                                {ft_submit_btn("update-password")}
                                            </form>
                                        </>},
                                        // Show interface for remove profile
                                        Menu::RemoveProfile => self.remove_profile_card(),
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
        // placeholder: &str,
        value: String,
        oninput: Callback<InputData>,
    ) -> Html {
        let placeholder = label;
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
                    oninput={oninput} ></@>
            </fieldset>
        }
    }

    fn show_update_profile_info(&self) -> Html {
        html!{
            <div class="columns">
                {show_notification(
                    &format!("{} {}", get_value_field(&72), self.get_result_profile),
                    "is-success",
                    self.get_result_profile > 0,
                )}
                <div id="updated-date" class="column">
                    <span class={classes!("overflow-title", "has-text-weight-bold")}>{get_value_field(&73)}</span> // "Last updated: "
                    {match &self.current_data {
                        Some(data) => html!{
                            <span class="overflow-title">
                                {data.updated_at.date_to_display()}
                            </span>
                        },
                        None => html!{<span>{get_value_field(&75)}</span>},
                    }}
                </div>
            </div>
        }
    }

    fn cb_generator(&self, cb: Menu) -> Callback<MouseEvent> {
        self.link.callback(move |_| Msg::SelectMenu(cb.clone()))
    }

    fn view_menu(&self) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            // return profile page MenuItem
            MenuItem {
                title: get_value_field(&76).to_string(),
                action: self.link.callback(|_| Msg::OpenProfile),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-angle-double-left")],
                is_active: false,
                ..Default::default()
            },
            // profile MenuItem
            MenuItem {
                title: get_value_field(&77).to_string(),
                action: self.cb_generator(Menu::Profile),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-address-card")],
                is_active: self.select_menu == Menu::Profile,
                ..Default::default()
            },
            // favicon MenuItem
            MenuItem {
                title: get_value_field(&78).to_string(),
                action: self.cb_generator(Menu::UpdateFavicon),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-image")],
                is_active: self.select_menu == Menu::UpdateFavicon,
                ..Default::default()
            },
            // certificates MenuItem
            MenuItem {
                title: get_value_field(&64).to_string(),
                action: self.cb_generator(Menu::Certificates),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-certificate")],
                is_active: self.select_menu == Menu::Certificates,
                ..Default::default()
            },
            // access MenuItem
            MenuItem {
                title: get_value_field(&80).to_string(),
                action: self.cb_generator(Menu::Access),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-low-vision")],
                is_active: self.select_menu == Menu::Access,
                ..Default::default()
            },
            // password MenuItem
            MenuItem {
                title: get_value_field(&20).to_string(),
                action: self.cb_generator(Menu::Password),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-key")],
                is_active: self.select_menu == Menu::Password,
                ..Default::default()
            },
            // remove profile MenuItem
            MenuItem {
                title: get_value_field(&82).to_string(),
                action: self.cb_generator(Menu::RemoveProfile),
                item_class: classes!("has-background-danger-light"),
                icon_classes: vec![classes!("fas", "fa-trash")],
                is_active: self.select_menu == Menu::RemoveProfile,
                ..Default::default()
            },
        ];

        html! {
          <div style="margin-right: 18px;z-index: 1;" >
              <SideMenu menu_arr={menu_arr} />
          </div>
        }
    }

    fn update_favicon_card(&self) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::RequestCurrentData);

        html! {
            <UpdateFaviconBlock
                company_uuid={None}
                callback={callback_update_favicon}
                />
        }
    }

    fn change_certificates_card(&self) -> Html {
        match &self.current_data {
            Some(current_data) => html! {
                <UserCertificatesCard
                    user_uuid={self.current_data.as_ref().map(|x| x.uuid.clone()).unwrap_or_default()}
                    certificates={current_data.certificates.clone()}
                    show_cert_btn={true}
                    download_btn={false}
                    manage_btn={true}
                />
            },
            None => html! {<span class={classes!("tag", "is-info", "is-light")}>{get_value_field(&74)}</span>}, // "Not fount certificates"
        }
    }

    fn add_certificate_card(&self) -> Html {
        let user_uuid = self
            .current_data
            .as_ref()
            .map(|user| user.uuid.to_string())
            .unwrap_or_default();

        let callback_upload_cert = self.link.callback(|_| Msg::RequestCurrentData);

        html! {
            <AddUserCertificateCard
                user_uuid={user_uuid}
                callback={callback_upload_cert}
            />
        }
    }

    fn change_access_card(&self) -> Html {
        let onchange_type_access_id = self.link.callback(|ev: ChangeData| {
            Msg::UpdateTypeAccessId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            })
        });

        html! {
            <div class="columns">
                <div class="column">
                    <label class="label">{get_value_field(&58)}</label> // "Type Access"
                </div>
                <div class="column">
                    <fieldset class="field">
                        <div class="control">
                            <div class="select">
                              <select
                                  id="types-access"
                                  select={self.request_access.to_string()}
                                  onchange={onchange_type_access_id}
                                  >
                                {for self.types_access.iter().map(|x|
                                    html!{
                                        <option value={x.type_access_id.to_string()}
                                          selected={x.type_access_id == self.current_data.as_ref().map(|x| x.type_access.type_access_id).unwrap_or_default()} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </div>
            </div>
        }
    }

    fn update_password_card(&self) -> Html {
        let oninput_old_password = self.link.callback(|ev: InputData| Msg::UpdateOldPassword(ev.value));
        let oninput_new_password = self.link.callback(|ev: InputData| Msg::UpdateNewPassword(ev.value));

        html! {
            <fieldset class="columns">
                // first column
                <fieldset class="column">
                    {self.fileset_generator(
                        "password", get_value_field(&48), // "Old password"
                        self.request_password.old_password.to_string(),
                        oninput_old_password
                    )}
                    {self.fileset_generator(
                        "password", get_value_field(&49), // "New password"
                        self.request_password.new_password.to_string(),
                        oninput_new_password
                    )}
                </fieldset>
            </fieldset>
        }
    }

    fn change_profile_card(&self) -> Html {
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
                        "username", get_value_field(&50), // "Username"
                        self.request_profile.username.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_username
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "email", get_value_field(&22), // "Email"
                        self.request_profile.email.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_email
                    )}
                </div>
            </div>

            // second columns (fio)
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "firstname", get_value_field(&52), // "Firstname"
                        self.request_profile.firstname.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_firstname
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "lastname", get_value_field(&53), // "Lastname"
                        self.request_profile.lastname.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_lastname
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "secondname", get_value_field(&54), // "Secondname"
                        self.request_profile.secondname.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_secondname
                    )}
                </div>
            </div>

            // third columns (position, phone, address)
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "position", get_value_field(&55), // "Position"
                        self.request_profile.position.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_position
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "phone", get_value_field(&56), // "Phone"
                        self.request_profile.phone.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_phone
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "address", get_value_field(&57), // "Address"
                        self.request_profile.address.as_ref().map(|x| x.clone()).unwrap_or_default(),
                        oninput_address
                    )}
                </div>
            </div>

            // fourth columns (program, region)
            <div class="columns">
                <div class="column">
                    <label class="label">{get_value_field(&26)}</label> // "Program"
                    <div class="control">
                        <div class="select">
                          <select
                              id="program"
                              select={self.request_profile.program_id.unwrap_or_default().to_string()}
                              onchange={oninput_program_id}
                              >
                            {for self.programs.iter().map(|x|
                                html!{
                                    <option value={x.id.to_string()}
                                        selected={x.id as i64 == self.request_profile.program_id.unwrap_or_default()} >
                                        {&x.name}
                                    </option>
                                }
                            )}
                          </select>
                        </div>
                    </div>
                </div>
                <div class="column">
                    <label class="label">{get_value_field(&27)}</label> // "Region"
                    <div class="control">
                        <div class="select">
                          <select
                              id="region"
                              select={self.request_profile.region_id.unwrap_or_default().to_string()}
                              onchange={onchange_region_id}
                              >
                            {for self.regions.iter().map(|x|
                                html!{
                                    <option value={x.region_id.to_string()}
                                          selected={x.region_id as i64 == self.request_profile.region_id.unwrap_or_default()} >
                                        {&x.region}
                                    </option>
                                }
                            )}
                          </select>
                        </div>
                    </div>
                </div>
            </div>

            // fifth column (only description)
            {self.fileset_generator(
                "description", get_value_field(&61), // "Description"
                self.request_profile.description.as_ref().map(|x| x.clone()).unwrap_or_default(),
                oninput_description
            )}
            <br/>
        </>}
    }

    fn remove_profile_card(&self) -> Html {
        let onclick_remove_profile = self.link.callback(|_| Msg::RequestRemoveProfile);
        let oninput_user_password = self.link.callback(|ev: InputData| Msg::UpdateUserPassword(ev.value));

        html!{<>
            <h4 id="remove-profile" class="title is-4">{get_value_field(&67)}</h4> // "Remove profile"
            <div class="content is-medium">
                <p><strong>{get_value_field(&272)}</strong> {get_value_field(&71)}</p>
            </div>
            {self.fileset_generator(
                "password", get_value_field(&62), // "your password"
                self.request_user_password.to_string(),
                oninput_user_password,
            )}
            {ft_delete_btn(
                "button-remove-profile",
                onclick_remove_profile,
                self.get_confirm == self.current_username,
                false
            )}
        </>}
    }
}
