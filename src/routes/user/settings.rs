use web_sys::MouseEvent;
use yew::{agent::Bridged, classes, html, Bridge, Callback, ChangeData, Component, ComponentLink, FocusEvent, Html, InputData, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::form_input::{render_form_input, InputConfig};
use crate::fragments::type_access::TypeAccessBlock;
use crate::fragments::{
    buttons::{ft_delete_class_btn, ft_submit_btn},
    notification::show_notification,
    list_errors::ListErrors,
    side_menu::{MenuBuilder, MenuItemTemplate},
    upload_favicon::UpdateFaviconBlock,
    user::{AddUserCertificateCard, UserCertificatesCard},
};
use crate::routes::AppRoute;
use crate::services::content_adapter::DateDisplay;
use crate::services::{get_current_user, get_from_value, get_logged_user, LocaleKey, get_value_response, resp_parsing, set_history_back, set_logged_user, set_token};
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

impl MenuBuilder for Settings {
    type TabType = Menu;

    fn menu_config() -> &'static [MenuItemTemplate<Menu>] {
        use Menu::*;
        &[
            MenuItemTemplate { lk_title: LocaleKey::OpenProfile, icon_classes: &[&["fas", "fa-angle-double-left"]], tab: OpenProfile, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::ProfileTitle, icon_classes: &[&["fas", "fa-address-card"]], tab: Profile, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::ProfilePicture, icon_classes: &[&["fas", "fa-image"]], tab: UpdateFavicon, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::CertificatesLabel, icon_classes: &[&["fas", "fa-certificate"]], tab: Certificates, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::AccessPolicy, icon_classes: &[&["fas", "fa-low-vision"]], tab: Access, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::Password, icon_classes: &[&["fas", "fa-key"]], tab: Password, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::RemoveProfileTitle, icon_classes: &[&["fas", "fa-trash"]], tab: RemoveProfile, custom_class: Some("has-background-danger-light") },
        ]
    }

    fn is_active(&self, tab: &Menu) -> bool { self.select_menu == *tab }
    fn get_count(&self, _tab: &Menu) -> usize { 0 }
    fn is_extend(&self, _tab: &Menu) -> bool { false }
    fn get_action(&self, tab: &Menu) -> Callback<MouseEvent> {
        if *tab == Menu::OpenProfile {
            self.link.callback(|_| Msg::OpenProfile)
        } else {
            self.cb_generator(tab.clone())
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Menu {
    OpenProfile,
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
    loading: bool,
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
    UpdateTypeAccessId(usize),
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
            loading: false,
            get_confirm: String::new(),
            select_menu: Menu::Profile,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let None = get_logged_user() {
                set_history_back(Some(String::new()));
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
                self.loading = true;
                spawn_local(async move {
                    let res = make_query(GetSelfData::build_query(
                        get_self_data::Variables
                    )).await.unwrap();
                    link.send_message(Msg::GetProfileDataResult(res));
                })
            },
            Msg::RequestUpdateProfile => {
                self.loading = true;
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
                self.loading = true;
                let new_type_access = self.request_access.clone();
                spawn_local(async move {
                    let res = make_query(ChangeTypeAccessUser::build_query(
                        change_type_access_user::Variables{new_type_access}
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateAccessResult(res));
                })
            },
            Msg::RequestUpdatePassword => {
                self.loading = true;
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
                    self.loading = true;
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
                self.loading = false;
                match resp_parsing(res, "changeTypeAccessUser") {
                    Ok(result) => self.get_result_access = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("changeTypeAccessUser: {:?}", self.get_result_access);
            },
            Msg::GetUpdatePwdResult(res) => {
                self.loading = false;
                match resp_parsing(res, "putUpdatePassword") {
                    Ok(result) => self.get_result_pwd = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("putUpdatePassword: {:?}", self.get_result_pwd);
            },
            Msg::GetProfileDataResult(res) => {
                self.loading = false;
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
                self.loading = false;
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
                self.loading = false;
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
                self.loading = false;
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
            Msg::UpdateTypeAccessId(type_access_id) => self.request_access = type_access_id as i64,
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
                <div class="container is-fluid page pl-0">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-flex side-menu-content-fix">
                                {self.render_menu()}
                                <div class="card is-flex-grow-1 side-menu-content-fix">
                                  <div class="card-content">
                                    {match self.select_menu {
                                        Menu::OpenProfile => html!{},
                                        // Show interface for change profile data
                                        Menu::Profile => html!{<>
                                            <h4 id="change-profile" class="title is-4">{LocaleKey::ProfileLabel.get_value()}</h4>
                                            {self.show_update_profile_info()}
                                            <form onsubmit={onsubmit_update_profile}>
                                                {self.change_profile_card()}
                                                <div class="mt-5">{ft_submit_btn("update-settings")}</div>
                                            </form>
                                        </>},
                                        // Show interface for change favicon user
                                        Menu::UpdateFavicon => {self.update_favicon_card()},
                                        // Show interface for add and update Certificates
                                        Menu::Certificates => html!{<>
                                            <h4 id="change-certificates" class="title is-4">{LocaleKey::CertificatesLabel.get_value()}</h4>
                                            {self.add_certificate_card()}
                                            {self.change_certificates_card()}
                                        </>},
                                        // Show interface for change access
                                        Menu::Access => html!{<>
                                            <h4 id="change-access" class="title is-4">{LocaleKey::Access.get_value()}</h4>
                                            {show_notification(
                                                &format!("{}: {}", LocaleKey::UpdatedAccess.get_value(), self.get_result_access),
                                                "is-success",
                                                self.get_result_access,
                                            )}
                                            <form onsubmit={onsubmit_update_access}>
                                                {self.change_access_card()}
                                                <div class="mt-5">{ft_submit_btn("update-access")}</div>
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
                                            <h4 id="change-password" class="title is-4">{LocaleKey::Password.get_value()}</h4>
                                            {show_notification(
                                                LocaleKey::UpdatedPassword.get_value(),
                                                "is-success",
                                                self.get_result_pwd,
                                            )}
                                            <form onsubmit={onsubmit_update_password}>
                                                {self.update_password_card()}
                                                <div class="mt-5">{ft_submit_btn("update-password")}</div>
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
    fn show_update_profile_info(&self) -> Html {
        html!{
            <div class="columns">
                {show_notification(
                    &format!("{} {}", LocaleKey::UpdatedRows.get_value(), self.get_result_profile),
                    "is-success",
                    self.get_result_profile > 0,
                )}
                <div id="updated-date" class="column">
                    <span class={classes!("overflow-title", "has-text-weight-bold")}>{LocaleKey::LastUpdated.get_value()}</span>
                    {match &self.current_data {
                        Some(data) => html!{
                            <span class="overflow-title">
                                {data.updated_at.date_to_display()}
                            </span>
                        },
                        None => html!{<span>{LocaleKey::NoData.get_value()}</span>},
                    }}
                </div>
            </div>
        }
    }

    fn cb_generator(&self, cb: Menu) -> Callback<MouseEvent> {
        self.link.callback(move |_| Msg::SelectMenu(cb.clone()))
    }

    fn update_favicon_card(&self) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::RequestCurrentData);

        html!{
            <UpdateFaviconBlock
                company_uuid={None}
                callback={callback_update_favicon}
                />
        }
    }

    fn change_certificates_card(&self) -> Html {
        match &self.current_data {
            Some(current_data) => html!{
                <UserCertificatesCard
                    user_uuid={self.current_data.as_ref().map(|x| x.uuid.clone()).unwrap_or_default()}
                    certificates={current_data.certificates.clone()}
                    show_cert_btn={true}
                    manage_btn={true}
                />
            },
            None => html!{<span class={classes!("tag", "is-info", "is-light")}>{LocaleKey::NoCertificates.get_value()}</span>},
        }
    }

    fn add_certificate_card(&self) -> Html {
        let user_uuid = self
            .current_data
            .as_ref()
            .map(|user| user.uuid.to_string())
            .unwrap_or_default();

        let callback_upload_cert = self.link.callback(|_| Msg::RequestCurrentData);

        html!{
            <AddUserCertificateCard
                user_uuid={user_uuid}
                callback={callback_upload_cert}
            />
        }
    }

    fn change_access_card(&self) -> Html {
        let onchange_type_access = self.link.callback(|value| Msg::UpdateTypeAccessId(value));
        html!{
            <div class="column">
                <label class="label">{LocaleKey::TypeAccess.get_value()}</label>
                <TypeAccessBlock
                    change_cb={onchange_type_access}
                    types={self.types_access.clone()}
                    selected={self.request_access as usize}
                    preset={self.current_data.as_ref().map(|data| data.type_access.type_access_id)}
                />
            </div>
        }
    }

    fn update_password_card(&self) -> Html {
        let oninput_old_password = self.link.callback(|ev: InputData| Msg::UpdateOldPassword(ev.value));
        let oninput_new_password = self.link.callback(|ev: InputData| Msg::UpdateNewPassword(ev.value));

        html!{
            <div class="columns is-desktop">
                <div class="column">
                    {render_form_input(InputConfig {
                        id: "password",
                        label: LocaleKey::OldPassword.get_value(),
                        value: self.request_password.old_password.to_string(),
                        oninput: oninput_old_password,
                        is_disabled: self.loading,
                        icon: Some("fas fa-lock"),
                        add_classes: classes!(""),
                        is_danger: false,
                    })}
                    {render_form_input(InputConfig {
                        id: "password",
                        label: LocaleKey::NewPassword.get_value(),
                        value: self.request_password.new_password.to_string(),
                        oninput: oninput_new_password,
                        is_disabled: self.loading,
                        icon: Some("fas fa-key"),
                        add_classes: classes!(""),
                        is_danger: false,
                    })}
                </div>
            </div>
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

        html!{
            <div class="settings-profile-fields">
                <div class="box mb-5">
                    <h5 class="title is-6 has-text-grey mb-4 is-uppercase has-text-weight-bold">
                        <span class="icon mr-2"><i class="fas fa-id-card"></i></span>
                    </h5>
                    <div class="columns is-desktop">
                        <div class="column">
                            {InputConfig::profile_input("username", LocaleKey::Username, self.request_profile.username.clone(), oninput_username, Some("fas fa-user"), self.loading, false)}
                        </div>
                        <div class="column">
                            {InputConfig::profile_input("email", LocaleKey::Email, self.request_profile.email.clone(), oninput_email, Some("fas fa-envelope"), self.loading, false)}
                        </div>
                    </div>
                </div>
                <div class="box mb-5">
                    <h5 class="title is-6 has-text-grey mb-4 is-uppercase has-text-weight-bold">
                        <span class="icon mr-2"><i class="fas fa-user-circle"></i></span>
                    </h5>
                    <div class="columns is-desktop">
                        <div class="column">
                            {InputConfig::profile_input("lastname", LocaleKey::Lastname, self.request_profile.lastname.clone(), oninput_lastname, None, self.loading, false)}
                        </div>
                        <div class="column">
                            {InputConfig::profile_input("firstname", LocaleKey::Firstname, self.request_profile.firstname.clone(), oninput_firstname, None, self.loading, false)}
                        </div>
                        <div class="column">
                            {InputConfig::profile_input("secondname", LocaleKey::Secondname, self.request_profile.secondname.clone(), oninput_secondname, None, self.loading, false)}
                        </div>
                    </div>
                </div>
                <div class="box mb-5">
                    <h5 class="title is-6 has-text-grey mb-4 is-uppercase has-text-weight-bold">
                        <span class="icon mr-2"><i class="fas fa-briefcase"></i></span>
                    </h5>
                    <div class="columns is-desktop">
                        <div class="column">
                            {InputConfig::profile_input("position", LocaleKey::Position, self.request_profile.position.clone(), oninput_position, Some("fas fa-id-badge"), self.loading, false)}
                        </div>
                        <div class="column">
                            <div class="field">
                                <label class="label">{LocaleKey::Program.get_value()}</label>
                                <div class="control is-expanded">
                                    <div class="select is-fullwidth">
                                        <select
                                            id="program"
                                            disabled={self.loading}
                                            onchange={oninput_program_id}
                                            value={self.request_profile.program_id.unwrap_or_default().to_string()}
                                        >
                                            {for self.programs.iter().map(|x| html!{
                                                <option value={x.id.to_string()}
                                                    selected={x.id as i64 == self.request_profile.program_id.unwrap_or_default()}
                                                >
                                                    {&x.name}
                                                </option>
                                            })}
                                        </select>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="column">
                            <div class="field">
                                <label class="label">{LocaleKey::Region.get_value()}</label>
                                <div class="control is-expanded">
                                    <div class="select is-fullwidth">
                                        <select
                                            id="region"
                                            disabled={self.loading}
                                            onchange={onchange_region_id}
                                            value={self.request_profile.region_id.unwrap_or_default().to_string()}
                                        >
                                            {for self.regions.iter().map(|x| html!{
                                                <option value={x.region_id.to_string()}
                                                    selected={x.region_id as i64 == self.request_profile.region_id.unwrap_or_default()}
                                                >
                                                    {&x.region}
                                                </option>
                                            })}
                                        </select>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="box mb-5">
                    <h5 class="title is-6 has-text-grey mb-4 is-uppercase has-text-weight-bold">
                        <span class="icon mr-2"><i class="fas fa-address-book"></i></span>
                    </h5>
                    <div class="columns is-desktop">
                        <div class="column is-3-desktop">
                            {InputConfig::profile_input("tel", LocaleKey::Phone, self.request_profile.phone.clone(), oninput_phone, Some("fas fa-phone"), self.loading, false)}
                        </div>
                        <div class="column">
                            {InputConfig::profile_input("address", LocaleKey::Address, self.request_profile.address.clone(), oninput_address, Some("fas fa-map-marker-alt"), self.loading, false)}
                        </div>
                    </div>
                </div>
                <div class="box">
                    <h5 class="title is-6 has-text-grey mb-4 is-uppercase has-text-weight-bold">
                        <span class="icon mr-2"><i class="fas fa-align-left"></i></span>
                    </h5>
                    {InputConfig::profile_input("description", LocaleKey::Description, self.request_profile.description.clone(), oninput_description, None, self.loading, false)}
                </div>
            </div>
        }
    }

    fn remove_profile_card(&self) -> Html {
        let onclick_remove_profile = self.link.callback(|_| Msg::RequestRemoveProfile);
        let oninput_user_password = self.link.callback(|ev: InputData| Msg::UpdateUserPassword(ev.value));

        html!{<>
            <h4 id="remove-profile" class="title is-4">{LocaleKey::RemoveProfile.get_value()}</h4>
            <div class="content is-medium">
                <p><strong>{LocaleKey::Warning.get_value()}</strong> {LocaleKey::ProfileDeleteWarning.get_value()}</p>
            </div>
            {InputConfig::profile_input(
                "password",
                LocaleKey::ConfirmDeleteProfile,
                Some(self.request_user_password.clone()),
                oninput_user_password,
                Some("fas fa-lock"),
                self.loading,
                false
            )}
            <div class="column is-half right-side">
            {ft_delete_class_btn(
                "button-remove-profile",
                onclick_remove_profile,
                self.get_confirm == self.current_username,
                false,
                classes!("is-fullwidth")
            )}
            </div>
        </>}
    }
}
