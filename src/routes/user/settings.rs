use yew::{Component, Callback, Context, html, Html, SubmitEvent, classes};
use yew::html::{Scope, TargetCast};
use yew_router::prelude::*;
use web_sys::{MouseEvent, InputEvent, Event, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::side_menu::{MenuItem, SideMenu};
use crate::fragments::upload_favicon::UpdateFaviconBlock;
use crate::fragments::user::{AddUserCertificateCard, UserCertificatesCard};
use crate::routes::AppRoute::{Login, Home, Profile};
use crate::services::{get_current_user, set_token, set_logged_user, get_logged_user, get_value_field, resp_parsing_item, get_value_response, get_from_value};
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
    current_data: Option<SelfUserInfo>,
    current_username: String,
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
    OpenProfile,
    RequestCurrentData,
    RequestUpdateProfile,
    RequestChangeAccess,
    RequestUpdatePassword,
    RequestRemoveProfile,
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
    ResponseError(Error),
    Ignore,
}

impl Component for Settings {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Settings {
            error: None,
            request_profile: UserUpdateInfo::default(),
            request_access: 0,
            request_password: UpdatePasswordInfo::default(),
            request_user_password: String::new(),
            // router_agent: AppRoute::bridge(ctx.link().callback(|_| Msg::Ignore)),
            current_data: None,
            current_username: String::new(),
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

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            if let None = get_logged_user() {
                // route to login page if not found token
                let navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Login);
            };

            spawn_local(async move {
                let res = make_query(GetSettingDataOpt::build_query(
                    get_setting_data_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::GetProfileDataResult(res.clone()));
                link.send_message(Msg::GetUpdateListResult(res));
            })
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let navigator: Navigator = ctx.link().navigator().unwrap();

        match msg {
            Msg::OpenProfile => {
                // Redirect to user page
                if let Some(user_data) = &self.current_data {
                    navigator.clone().replace(&Profile { username: user_data.username.clone() });
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
                    time_zone: self.request_profile.time_zone.clone(),
                    region_id: self.request_profile.region_id.clone(),
                    program_id: self.request_profile.program_id.clone(),
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
                    old_password: self.request_password.old_password.clone(),
                    new_password: self.request_password.new_password.clone(),
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
                self.get_result_access = resp_parsing_item(res, "changeTypeAccessUser")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
            },
            Msg::GetUpdatePwdResult(res) => {
                self.get_result_pwd = resp_parsing_item(res, "putUpdatePassword")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
            },
            Msg::GetProfileDataResult(res) => {
                let user_data: SelfUserInfo = resp_parsing_item(res, "selfData")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                self.current_data = Some(user_data.clone());
                self.current_username = user_data.username.clone();
                self.request_profile = user_data.into();
                self.rendered(ctx, false);
            },
            Msg::GetUpdateListResult(res) => {
                let value: Value = get_value_response(res)
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                self.regions = get_from_value(&value, "regions").unwrap();
                self.programs = get_from_value(&value, "programs").unwrap();
                self.types_access = get_from_value(&value, "typesAccess").unwrap();
            },
            Msg::GetUpdateProfileResult(res) => {
                self.get_result_profile = resp_parsing_item(res, "putUserUpdate")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                // update local data
                set_logged_user(None);
                spawn_local(async move {
                    let res = get_current_user().await;
                    debug!("update locale slim user: {:?}", res);
                });
                link.send_message(Msg::RequestCurrentData);
            },
            Msg::GetRemoveProfileResult(res) => {
                self.get_result_profile = resp_parsing_item(res, "deleteUserData")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if self.get_result_remove_profile {
                    // Clear global token and logged user after delete profile
                    set_token(None);
                    set_logged_user(None);
                    // self.router_agent.send(Home);
                    navigator.replace(&Home);
                }
            },
            Msg::UpdateTypeAccessId(type_access_id) =>
                self.request_access = type_access_id.parse::<i64>().unwrap_or(1),
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
                self.request_profile.program_id = Some(program_id.parse::<i64>().unwrap_or(1)),
            Msg::UpdateRegionId(region_id) =>
                self.request_profile.region_id = Some(region_id.parse::<i64>().unwrap_or(1)),
            Msg::UpdateUserPassword(user_password) => self.request_user_password = user_password,
            Msg::SelectMenu(value) => {
                self.select_menu = value;
                self.rendered(ctx, false);
            },
            Msg::ClearError => self.error = None,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::Ignore => {}
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);
        let onsubmit_update_profile = ctx.link().callback(|ev: SubmitEvent| {
            ev.prevent_default();
            Msg::RequestUpdateProfile
        });
        let onsubmit_update_access = ctx.link().callback(|ev: SubmitEvent| {
            ev.prevent_default();
            Msg::RequestChangeAccess
        });
        let onsubmit_update_password = ctx.link().callback(|ev: SubmitEvent| {
            ev.prevent_default();
            Msg::RequestUpdatePassword
        });
        let onsubmit_remove_profile = ctx.link().callback(|ev: SubmitEvent| {
            ev.prevent_default();
            Msg::RequestRemoveProfile
        });

        html!{
            <div class="settings-page">
                <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
                <div class="container page">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-flex">
                                { self.view_menu(ctx.link()) }
                                <div class="card is-flex-grow-1" >
                                  <div class="card-content">
                                    {match self.select_menu {
                                        // Show interface for change profile data
                                        Menu::Profile => html!{<>
                                            <h4 id="change-profile" class="title is-4">
                                                { get_value_field(&63) } // "Profile"
                                            </h4>
                                            {self.show_update_profile_info()}
                                            <form onsubmit={onsubmit_update_profile}>
                                                { self.change_profile_card(ctx.link()) }
                                                <button
                                                    id="update-settings"
                                                    class={classes!("button", "is-fullwidth")}
                                                    type="submit"
                                                    disabled={false}>
                                                    { get_value_field(&46) }
                                                </button>
                                            </form>
                                        </>},
                                        // Show interface for change favicon user
                                        Menu::UpdateFavicon => {self.update_favicon_card(ctx.link())},
                                        // Show interface for add and update Certificates
                                        Menu::Certificates => html!{<>
                                            <h4 id="change-certificates" class="title is-4">
                                                { get_value_field(&64) } // "Certificates"
                                            </h4>
                                            { self.add_certificate_card(ctx.link()) }
                                            { self.change_certificates_card() }
                                        </>},
                                        // Show interface for change access
                                        Menu::Access => html!{<>
                                            <h4 id="change-access" class="title is-4">
                                                { get_value_field(&65) } // "Access"
                                            </h4>
                                            <span id="tag-info-updated-access" class={classes!("tag", "is-info", "is-light")}>
                                                { format!("{}: {}", get_value_field(&68), self.get_result_access.clone()) } // Updated access
                                            </span>
                                            <form onsubmit={onsubmit_update_access}>
                                                { self.change_access_card(ctx.link()) }
                                                <button
                                                    id="update-access"
                                                    class={classes!("button", "is-fullwidth")}
                                                    type="submit"
                                                    disabled={false}>
                                                    { get_value_field(&46) }
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
                                            <h4 id="change-password" class="title is-4">
                                                { get_value_field(&20) } // "Password"
                                            </h4>
                                            <span id="tag-info-updated-pwd" class={classes!("tag", "is-info", "is-light")}>
                                              { format!("{}: {}", get_value_field(&69), self.get_result_pwd.clone()) } // Updated password
                                            </span>
                                            <form onsubmit={onsubmit_update_password}>
                                                { self.update_password_card(ctx.link()) }
                                                <button
                                                    id="update-password"
                                                    class={classes!("button", "is-fullwidth")}
                                                    type="submit"
                                                    disabled={false}>
                                                    { get_value_field(&46) }
                                                </button>
                                            </form>
                                        </>},
                                        // Show interface for remove profile
                                        Menu::RemoveProfile => html!{<>
                                            <h4 id="remove-profile" class="title is-4">
                                                { get_value_field(&67) } // "Remove profile"
                                            </h4>
                                            <div id="tag-danger-remove-profile" class={classes!("notification", "is-danger", "is-light")}>
                                              { get_value_field(&71) }
                                            </div>
                                            <form onsubmit={onsubmit_remove_profile}>
                                                { self.remove_profile_card(ctx.link()) }
                                                <button
                                                    id="button-remove-profile"
                                                    class={classes!("button", "is-fullwidth", "is-danger")}
                                                    type="submit"
                                                    disabled={false}>
                                                    { get_value_field(&47) }
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
        // placeholder: &str,
        value: String,
        oninput: Callback<InputEvent>,
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
                <div id="updated-rows" class="column">
                    <span class={classes!("overflow-title", "has-text-weight-bold")}>
                        { get_value_field(&72) } // "Updated rows: "
                    </span>
                    <span class="overflow-title">{self.get_result_profile.clone()}</span>
                </div>
                <div id="updated-date" class="column">
                    <span class={classes!("overflow-title", "has-text-weight-bold")}>
                        { get_value_field(&73) } // "Last updated: "
                    </span>
                    {match &self.current_data {
                        Some(data) => html!{
                            <span class="overflow-title">
                                {format!("{:.*}", 19, data.updated_at.to_string())}
                            </span>
                        },
                        None => html!{<span>{ get_value_field(&75) }</span>},
                    }}
                </div>
            </div>
        }
    }

    fn cb_generator(
        &self,
        link: &Scope<Self>,
        cb: Menu
    ) -> Callback<MouseEvent> {
        link.callback(move |_| Msg::SelectMenu(cb.clone()))
    }

    fn view_menu(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            // return profile page MenuItem
            MenuItem {
                title: get_value_field(&76).to_string(),
                action: link.callback(|_| Msg::OpenProfile),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-angle-double-left")],
                is_active: false,
                ..Default::default()
            },
            // profile MenuItem
            MenuItem {
                title: get_value_field(&77).to_string(),
                action: self.cb_generator(link, Menu::Profile),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-address-card")],
                is_active: self.select_menu == Menu::Profile,
                ..Default::default()
            },
            // favicon MenuItem
            MenuItem {
                title: get_value_field(&78).to_string(),
                action: self.cb_generator(link, Menu::UpdateFavicon),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-image")],
                is_active: self.select_menu == Menu::UpdateFavicon,
                ..Default::default()
            },
            // certificates MenuItem
            MenuItem {
                title: get_value_field(&64).to_string(),
                action: self.cb_generator(link, Menu::Certificates),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-certificate")],
                is_active: self.select_menu == Menu::Certificates,
                ..Default::default()
            },
            // access MenuItem
            MenuItem {
                title: get_value_field(&80).to_string(),
                action: self.cb_generator(link, Menu::Access),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-low-vision")],
                is_active: self.select_menu == Menu::Access,
                ..Default::default()
            },
            // password MenuItem
            MenuItem {
                title: get_value_field(&20).to_string(),
                action: self.cb_generator(link, Menu::Password),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-key")],
                is_active: self.select_menu == Menu::Password,
                ..Default::default()
            },
            // remove profile MenuItem
            MenuItem {
                title: get_value_field(&82).to_string(),
                action: self.cb_generator(link, Menu::RemoveProfile),
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

    fn update_favicon_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let callback_update_favicon = link.callback(|_| Msg::RequestCurrentData);
        html! {
            <UpdateFaviconBlock
                callback={callback_update_favicon}
                // company_uuid={None}
                />
        }
    }

    fn change_certificates_card(&self) -> Html {
        match &self.current_data {
            Some(current_data) => html! {
                <UserCertificatesCard
                    user_uuid = {self.current_data.as_ref().map(|x| x.uuid.clone()).unwrap_or_default()}
                    certificates = {current_data.certificates.clone()}
                    show_cert_btn = {true}
                    download_btn = {false}
                    manage_btn = {true}
                />
            },
            None => html! {
                <span class={classes!("tag", "is-info", "is-light")}>
                    {get_value_field(&74)} // "Not fount certificates"
                </span>
            },
        }
    }

    fn add_certificate_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let user_uuid = self
            .current_data
            .as_ref()
            .map(|user| user.uuid.to_string())
            .unwrap_or_default();
        let callback_upload_cert = link.callback(|_| Msg::RequestCurrentData);

        html! {
            <AddUserCertificateCard
                user_uuid={user_uuid}
                callback={callback_upload_cert}
            />
        }
    }

    fn change_access_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onchange_type_access_id =
            link.callback(|ev: Event| Msg::UpdateTypeAccessId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));

        html! {
            <div class="columns">
                <div class="column">
                    <label class="label">{ get_value_field(&58) }</label> // "Type Access"
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
                                { for self.types_access.iter().map(|x|
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

    fn update_password_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_old_password = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateOldPassword(input.value())
        });
        let oninput_new_password = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateNewPassword(input.value())
        });

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

    fn change_profile_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_firstname = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateFirstname(input.value())
        });
        let oninput_lastname = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateLastname(input.value())
        });
        let oninput_secondname = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateSecondname(input.value())
        });
        let oninput_username = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateUsername(input.value())
        });
        let oninput_email = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateEmail(input.value())
        });
        let oninput_description = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateDescription(input.value())
        });
        let oninput_position = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdatePosition(input.value())
        });
        let oninput_phone = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdatePhone(input.value())
        });
        let oninput_address = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateAddress(input.value())
        });
        let onchange_program_id =
            link.callback(|ev: Event| Msg::UpdateProgramId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));
        let onchange_region_id =
            link.callback(|ev: Event| Msg::UpdateRegionId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));

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
                    <label class="label">{ get_value_field(&26) }</label> // "Program"
                    <div class="control">
                        <div class="select">
                          <select
                              id="program"
                              select={self.request_profile.program_id.unwrap_or_default().to_string()}
                              onchange={onchange_program_id}
                              >
                            { for self.programs.iter().map(|x|
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
                    <label class="label">{ get_value_field(&27) }</label> // "Region"
                    <div class="control">
                        <div class="select">
                          <select
                              id="region"
                              select={self.request_profile.region_id.unwrap_or_default().to_string()}
                              onchange={onchange_region_id}
                              >
                            { for self.regions.iter().map(|x|
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

    fn remove_profile_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_user_password = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateUserPassword(input.value())
        });

        self.fileset_generator(
            "password", get_value_field(&62), // "your password"
            self.request_user_password.to_string(),
            oninput_user_password,
        )
    }
}
