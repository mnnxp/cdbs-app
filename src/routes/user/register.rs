use yew::{agent::Bridged, html, Bridge, Component, ComponentLink, Html, InputData, ChangeData, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::fragments::form_input::InputConfig;
use crate::fragments::type_access::TypeAccessBlock;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_create_btn;
use crate::fragments::conditions::ConditionsBlock;
use crate::services::{get_logged_user, LocaleKey, get_value_response, get_from_value};
use crate::types::{RegisterInfo, Program, TypeAccessInfo};
use crate::gqls::make_query;
use crate::gqls::user::{
    RegisterOpt, register_opt,
    RegUser, reg_user,
};

/// Register page
pub struct Register {
    error: Option<Error>,
    // props: Props,
    request: RegisterInfo,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    programs: Vec<Program>,
    types_access: Vec<TypeAccessInfo>,
    link: ComponentLink<Self>,
    loading: bool,
}

pub enum Msg {
    Request,
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateProgramId(String),
    UpdateTypeAccessId(usize),
    UpdateList(String),
    GetRegister(String),
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Register {
            error: None,
            request: RegisterInfo::default(),
            // props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            programs: Vec::new(),
            types_access: Vec::new(),
            loading: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(user) = get_logged_user() {
                // route to profile page if user already logged
                self.router_agent.send(ChangeRoute(AppRoute::Profile(user.username).into()));
            };
            let link = self.link.clone();
            spawn_local(async move {
                let res = make_query(RegisterOpt::build_query(
                    register_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::UpdateList(res))
            });
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::Request => {
                self.loading = true;
                let ipt_user_data = reg_user::IptUserData {
                    email: self.request.email.clone(),
                    username: self.request.username.clone(),
                    password: self.request.password.clone(),
                    firstname: Some(self.request.firstname.clone()),
                    lastname: Some(self.request.lastname.clone()),
                    secondname: Some(self.request.secondname.clone()),
                    phone: Some(self.request.phone.clone()),
                    description: Some(self.request.description.clone()),
                    address: Some(self.request.address.clone()),
                    timeZone: Some(self.request.time_zone.clone()),
                    position: Some(self.request.position.clone()),
                    regionId: Some(8_i64), // set region "Other"
                    programId: Some(self.request.program_id as i64),
                    typeAccessId: Some(self.request.type_access_id as i64),
                };
                spawn_local(async move {
                    let res = make_query(RegUser::build_query(reg_user::Variables {
                        ipt_user_data
                    })).await.unwrap();
                    link.send_message(Msg::GetRegister(res));
                })
            },
            Msg::UpdateList(res) => {
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.programs = get_from_value(value, "programs").unwrap_or_default();
                        self.types_access = get_from_value(value, "typesAccess").unwrap_or_default();
                        debug!("Update: {:?}", self.programs);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetRegister(res) => {
                match get_value_response(res) {
                    Ok(value) => {
                        debug!("Value: {:?}", value);
                        self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.loading = false;
            },
            Msg::UpdateEmail(email) => self.request.email = email,
            Msg::UpdatePassword(password) => self.request.password = password,
            Msg::UpdateUsername(username) => self.request.username = username,
            Msg::UpdateProgramId(program_id) =>
                self.request.program_id = program_id.parse::<usize>().unwrap_or(1),
            Msg::UpdateTypeAccessId(type_access_id) => self.request.type_access_id = type_access_id,
            Msg::ResponseError(err) => self.error = Some(err),
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
        let onclick_signup_btn = self.link.callback(|_| Msg::Request);

        html!{
            <div class="container is-fluid page">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
                <div class="auth-page">
                    <h1 class="title is-spaced mb-3">{LocaleKey::SignUp.get_value()}</h1>
                    <h2 class="subtitle mt-0 mb-4">
                        <RouterAnchor<AppRoute> route={AppRoute::Login}>
                            {LocaleKey::HaveAccount.get_value()}
                        </RouterAnchor<AppRoute>>
                    </h2>
                    <div class="box p-5">
                        { self.fieldset_profile() }
                        <div class="columns is-desktop is-vcentered mt-4">
                            <div class="column is-12-mobile is-8-desktop">
                                {ft_create_btn(
                                    "signup-button",
                                    "is-large is-fullwidth".into(),
                                    onclick_signup_btn,
                                    self.loading ||
                                        self.request.username.is_empty() ||
                                        self.request.email.is_empty() ||
                                        self.request.password.is_empty(),
                                )}
                            </div>
                            <div class="column is-12-mobile is-4-desktop is-flex is-vcentered pt-2">
                                <ConditionsBlock />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl Register {
    fn fieldset_profile(&self) -> Html {
        let oninput_username = self.link.callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_email = self.link.callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self.link.callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let oninput_program_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateProgramId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let onchange_type_access = self.link.callback(|value| Msg::UpdateTypeAccessId(value));
        let current_program_id = self.request.program_id;

        html! {
            <>
                <div class="columns is-desktop mb-0">
                    <div class="column">
                        {InputConfig::profile_input(
                            "username",
                            LocaleKey::Username,
                            Some(self.request.username.clone()),
                            oninput_username,
                            Some("fas fa-user"),
                            self.loading,
                            false,
                        )}
                    </div>
                    <div class="column">
                        {InputConfig::profile_input(
                            "email",
                            LocaleKey::Email,
                            Some(self.request.email.clone()),
                            oninput_email,
                            Some("fas fa-envelope"),
                            self.loading,
                            false,
                        )}
                    </div>
                </div>
                <div class="field mb-4">
                    {InputConfig::profile_input(
                        "password",
                        LocaleKey::Password,
                        Some(self.request.password.clone()),
                        oninput_password,
                        Some("fas fa-lock"),
                        self.loading,
                        false,
                    )}
                </div>
                <div class="columns is-desktop mb-0">
                    <div class="column">
                        <div class="field">
                            <label class="label">{LocaleKey::Program.get_value()}</label>
                            <div class="control">
                                <div class="select is-fullwidth">
                                <select
                                    id="program"
                                    onchange={oninput_program_id}
                                    value={current_program_id.to_string()}
                                >
                                    { for self.programs.iter().map(|x| html! {
                                        <option
                                            value={x.id.to_string()}
                                            selected={x.id == current_program_id}
                                        >
                                            {&x.name}
                                        </option>
                                    }) }
                                </select>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="column">
                        <div class="field">
                            <label class="label">{LocaleKey::TypeAccess.get_value()}</label>
                            <TypeAccessBlock
                                change_cb={onchange_type_access}
                                types={self.types_access.clone()}
                                selected={self.request.type_access_id}
                            />
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
