use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::fragments::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_token, Auth};
use crate::types::{UserInfoWrapper, SlimUserWrapper, UserUpdateInfo, UserUpdateInfoWrapper};

/// Update settings of the author or logout
pub struct Settings {
    auth: Auth,
    error: Option<Error>,
    request: UserUpdateInfo,
    response: Callback<Result<usize, Error>>,
    loaded: Callback<Result<usize, Error>>,
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
    Response(Result<usize, Error>),
    Loaded(Result<usize, Error>),
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

                self.task = Some(self.auth.save(request, self.response));
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
                    email: user_info.user.email,
                    firstname: user_info.user.firstname,
                    lastname: user_info.user.lastname,
                    secondname: user_info.user.secondname,
                    username: user_info.user.username,
                    phone: user_info.user.phone,
                    description: user_info.user.description,
                    address: user_info.user.address,
                    position: user_info.user.position,
                    timeZone: user_info.user.timeZone,
                    regionId: user_info.user.region.regionId,
                    programId: user_info.user.program.id,
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
            Msg::UpdateEmail(email) => {
                self.request.email = email
            },
            Msg::UpdateFirstname(firstname) => {
                self.request.firstname = firstname
            },
            Msg::UpdateLastname(lastname) => {
                self.request.lastname = lastname
            },
            Msg::UpdateSecondname(secondname) => {
                self.request.secondname = secondname
            },
            Msg::UpdateUsername(username) => {
                self.request.username = username
            },
            Msg::UpdatePhone(phone) => {
                self.request.phone = phone
            },
            Msg::UpdateDescription(description) => {
                self.request.description = description
            },
            Msg::UpdateAddress(address) => {
                self.request.address = address
            },
            Msg::UpdatePosition(position) => {
                self.request.position = position
            },
            Msg::UpdateTimeZone(timeZone) => {
                self.request.timeZone = timeZone
            },
            Msg::UpdateRegionId(regionId) => {
                self.request.regionId = regionId
            },
            Msg::UpdateProgramId(programId) => {
                self.request.programId = programId
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
        // let oninput_password = self
        //     .link
        //     .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
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
                                        // <fieldset class="field">
                                        //     <input
                                        //         class="input"
                                        //         type="password"
                                        //         placeholder="New Password"
                                        //         value={self.password.to_string()}
                                        //         oninput=oninput_password />
                                        // </fieldset>
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
