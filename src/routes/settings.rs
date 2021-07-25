use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_token, Auth};
use crate::types::{UserInfoWrapper, SlimUserWrapper, UserUpdateInfo, UserUpdateInfoWrapper};

/// Update settings of the author or logout
pub struct Settings {
    auth: Auth,
    error: Option<Error>,
    request: UserUpdateInfo,
    password: String,
    response: Callback<Result<UserInfoWrapper, Error>>,
    loaded: Callback<Result<UserInfoWrapper, Error>>,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<()>,
}

pub enum Msg {
    Request,
    Response(Result<UserInfoWrapper, Error>),
    Loaded(Result<UserInfoWrapper, Error>),
    Ignore,
    Logout,
    UpdateFirstname(String),
    UpdateLastname(String),
    UpdateSecondname(String),
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateIdTypeUser(String),
    UpdateIsSupplier(String),
    UpdateOrgname(String),
    UpdateShortname(String),
    UpdateInn(String),
    UpdatePhone(String),
    UpdateIdNameCad(String),
    UpdateComment(String),
    UpdateAddress(String),
    UpdateTimeZone(String),
    UpdatePosition(String),
    UpdateSiteUrl(String),
    UpdateUuidFileInfoIcon(String),
    UpdateIdRegion(String),
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Settings {
            auth: Auth::new(),
            error: None,
            request: UserUpdateInfo::default(),
            password: String::default(),
            response: link.callback(Msg::Response),
            loaded: link.callback(Msg::Loaded),
            task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && is_authenticated() {
            self.task = Some(self.auth.user_info(self.loaded.clone()));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let mut request = UserUpdateInfoWrapper {
                    user: self.request.clone(),
                };
                if !self.password.is_empty() {
                    request.user.password = Some(self.password.clone());
                }
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
            Msg::Loaded(Ok(user_info)) => {
                self.error = None;
                self.task = None;
                self.request = UserUpdateInfo {
                    // email: user_info.user.email,
                    // username: user_info.user.username,
                    // password: None,
                    // image: user_info.user.image.unwrap_or_default(),
                    // bio: user_info.user.bio.unwrap_or_default(),
                    firstname: user_info.user.firstname,
                    lastname: user_info.user.lastname,
                    secondname: user_info.user.secondname,
                    username: user_info.user.username,
                    email: user_info.user.email,
                    password: None,
                    id_type_user: user_info.user.id_type_user,
                    is_supplier: user_info.user.is_supplier,
                    orgname: user_info.user.orgname,
                    shortname: user_info.user.shortname,
                    inn: user_info.user.inn,
                    phone: user_info.user.phone,
                    id_name_cad: user_info.user.id_name_cad,
                    comment: user_info.user.comment,
                    address: user_info.user.address,
                    time_zone: user_info.user.time_zone,
                    position: user_info.user.position,
                    site_url: user_info.user.site_url,
                    uuid_file_info_icon: user_info.user.uuid_file_info_icon,
                    id_region: user_info.user.id_region,
                };
            }
            Msg::Loaded(Err(err)) => {
                self.error = Some(err);
                self.task = None;
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
            Msg::UpdateFirstname(firstname) => {
                self.request.firstname = firstname;
            }
            Msg::UpdateLastname(lastname) => {
                self.request.lastname = lastname;
            }
            Msg::UpdateSecondname(secondname) => {
                self.request.secondname = secondname;
            }
            Msg::UpdateEmail(email) => {
                self.request.email = email;
            }
            Msg::UpdatePassword(password) => {
                self.password = password;
            }
            Msg::UpdateUsername(username) => {
                self.request.username = username;
            }
            Msg::UpdateIdTypeUser(id_type_user) => {
                self.request.id_type_user = id_type_user.parse::<i32>().unwrap_or(1);
            }
            Msg::UpdateIsSupplier(is_supplier) => {
                self.request.is_supplier = is_supplier.parse::<i32>().unwrap_or(0);
            }
            Msg::UpdateOrgname(orgname) => {
                self.request.orgname = orgname;
            }
            Msg::UpdateShortname(shortname) => {
                self.request.shortname = shortname;
            }
            Msg::UpdateInn(inn) => {
                self.request.inn = inn;
            }
            Msg::UpdatePhone(phone) => {
                self.request.phone = phone;
            }
            Msg::UpdateIdNameCad(id_name_cad) => {
                self.request.id_name_cad = id_name_cad.parse::<i32>().unwrap_or(1);
            }
            Msg::UpdateComment(comment) => {
                self.request.comment = comment;
            }
            Msg::UpdateAddress(address) => {
                self.request.address = address;
            }
            Msg::UpdateTimeZone(time_zone) => {
                self.request.time_zone = time_zone.parse::<i32>().unwrap_or(3);
            }
            Msg::UpdatePosition(position) => {
                self.request.position = position;
            }
            Msg::UpdateSiteUrl(site_url) => {
                self.request.site_url = site_url;
            }
            Msg::UpdateUuidFileInfoIcon(uuid_file_info_icon) => {
                self.request.uuid_file_info_icon = uuid_file_info_icon;
            }
            Msg::UpdateIdRegion(id_region) => {
                self.request.id_region = id_region.parse::<i32>().unwrap_or(1);
            }
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
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let oninput_id_name_cad = self
            .link
            .callback(|ev: InputData| Msg::UpdateIdNameCad(ev.value));
        let oninput_id_type_user = self
            .link
            .callback(|ev: InputData| Msg::UpdateIdTypeUser(ev.value));
        let oninput_time_zone = self
            .link
            .callback(|ev: InputData| Msg::UpdateTimeZone(ev.value));
        let oninput_position = self
            .link
            .callback(|ev: InputData| Msg::UpdatePosition(ev.value));
        let oninput_uuid_file_info_icon = self
            .link
            .callback(|ev: InputData| Msg::UpdateUuidFileInfoIcon(ev.value));
        let oninput_id_region = self
            .link
            .callback(|ev: InputData| Msg::UpdateIdRegion(ev.value));
        let onclick = self.link.callback(|_| Msg::Logout);

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
                                                value={self.request.firstname.clone()}
                                                oninput=oninput_firstname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="lastname"
                                                value={self.request.lastname.clone()}
                                                oninput=oninput_lastname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="secondname"
                                                value={self.request.secondname.clone()}
                                                oninput=oninput_secondname />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="username"
                                                value={self.request.username.clone()}
                                                oninput=oninput_username />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="email"
                                                placeholder="email"
                                                value={self.request.email.clone()}
                                                oninput=oninput_email />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="password"
                                                placeholder="New Password"
                                                value={self.password.to_string()}
                                                oninput=oninput_password />
                                        </fieldset>
                                    </fieldset>

                                    <fieldset class="column">
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="time_zone"
                                                value={self.request.time_zone.to_string()}
                                                oninput=oninput_time_zone />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="position"
                                                value={self.request.position.clone()}
                                                oninput=oninput_position />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="uuid_file_info_icon"
                                                value={self.request.uuid_file_info_icon.clone()}
                                                oninput=oninput_uuid_file_info_icon />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="id_region"
                                                value={self.request.id_region.to_string()}
                                                oninput=oninput_id_region />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="id_name_cad"
                                                value={self.request.id_name_cad.to_string()}
                                                oninput=oninput_id_name_cad />
                                        </fieldset>
                                        <fieldset class="field">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="id_type_user"
                                                value={self.request.id_type_user.to_string()}
                                                oninput=oninput_id_type_user />
                                        </fieldset>
                                    </fieldset>

                                    <fieldset>
                                    {
                                        // todo!(view different data for different type user)
                                        if true {
                                            self.for_type_user_1()
                                        } else {
                                            self.for_type_user_2()
                                        }
                                    }
                                    </fieldset>
                                </fieldset>
                                <button
                                    class="button"
                                    type="submit"
                                    disabled=false>
                                    { "Update Settings" }
                                </button>
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

impl Settings {
    fn for_type_user_1(&self) -> Html {
        html! { }
    }
    fn for_type_user_2(&self) -> Html {
        let oninput_orgname = self
            .link
            .callback(|ev: InputData| Msg::UpdateOrgname(ev.value));
        let oninput_shortname = self
            .link
            .callback(|ev: InputData| Msg::UpdateShortname(ev.value));
        let oninput_inn = self
            .link
            .callback(|ev: InputData| Msg::UpdateInn(ev.value));
        let oninput_phone = self
            .link
            .callback(|ev: InputData| Msg::UpdatePhone(ev.value));
        let oninput_comment = self
            .link
            .callback(|ev: InputData| Msg::UpdateComment(ev.value));
        let oninput_address = self
            .link
            .callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_site_url = self
            .link
            .callback(|ev: InputData| Msg::UpdateSiteUrl(ev.value));
        let oninput_is_supplier = self
            .link
            .callback(|ev: InputData| Msg::UpdateIsSupplier(ev.value));

        html! {
            <fieldset class="column">
                // data user for id_type_user 2-11
                <fieldset class="field">
                    <input
                        class="input"
                        type="text"
                        placeholder="orgname"
                        value={self.request.orgname.clone()}
                        oninput=oninput_orgname />
                </fieldset>
                <fieldset class="field">
                    <input
                        class="input"
                        type="text"
                        placeholder="shortname"
                        value={self.request.shortname.clone()}
                        oninput=oninput_shortname />
                </fieldset>
                <fieldset class="field">
                    <input
                        class="input"
                        type="text"
                        placeholder="inn"
                        value={self.request.inn.clone()}
                        oninput=oninput_inn />
                </fieldset>
                <fieldset class="field">
                    <input
                        class="input"
                        type="text"
                        placeholder="phone"
                        value={self.request.phone.clone()}
                        oninput=oninput_phone />
                </fieldset>
                <fieldset class="field">
                    <input
                        class="input"
                        type="text"
                        placeholder="comment"
                        value={self.request.comment.clone()}
                        oninput=oninput_comment />
                </fieldset>
                <fieldset class="field">
                    <input
                        class="input"
                        type="text"
                        placeholder="address"
                        value={self.request.address.clone()}
                        oninput=oninput_address />
                </fieldset>
                <fieldset class="field">
                    <input
                        class="input"
                        type="text"
                        placeholder="site_url"
                        value={self.request.site_url.clone()}
                        oninput=oninput_site_url />
                </fieldset>
                <fieldset class="field">
                    <input
                        class="input"
                        type="text"
                        placeholder="is_supplier"
                        value={self.request.is_supplier.to_string()}
                        oninput=oninput_is_supplier />
                </fieldset>
            </fieldset>
        }
    }
}
