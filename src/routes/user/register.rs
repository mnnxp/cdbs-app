use yew::{agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, InputData, ChangeData, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_create_btn;
use crate::services::{get_logged_user, get_value_field, get_value_response, get_from_value};
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
    show_conditions: bool,
}

pub enum Msg {
    Request,
    // UpdateFirstname(String),
    // UpdateLastname(String),
    // UpdateSecondname(String),
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateProgramId(String),
    UpdateTypeAccessId(String),
    UpdateList(String),
    GetRegister(String),
    ShowConditions,
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
            show_conditions: false,
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
            },
            // Msg::UpdateFirstname(firstname) => self.request.firstname = firstname,
            // Msg::UpdateLastname(lastname) => self.request.lastname = lastname,
            // Msg::UpdateSecondname(secondname) => self.request.secondname = secondname,
            Msg::UpdateEmail(email) => self.request.email = email,
            Msg::UpdatePassword(password) => self.request.password = password,
            Msg::UpdateUsername(username) => self.request.username = username,
            Msg::UpdateProgramId(program_id) =>
                self.request.program_id = program_id.parse::<usize>().unwrap_or(1),
            Msg::UpdateTypeAccessId(type_access_id) =>
                self.request.type_access_id = type_access_id.parse::<usize>().unwrap_or(1),
            Msg::ShowConditions => self.show_conditions = !self.show_conditions,
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
        let onclick_show_conditions = self.link.callback(|_| Msg::ShowConditions);
        let onclick_signup_btn = self.link.callback(|_| Msg::Request);

        html!{<div class="container page">
            <div class="auth-page">
                <h1 class="title">{get_value_field(&14)}</h1>
                <h2 class="subtitle">
                    <RouterAnchor<AppRoute> route={AppRoute::Login}>
                        {get_value_field(&21)}
                    </RouterAnchor<AppRoute>>
                </h2>
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
                {self.modal_conditions()}
                <div class="card column">
                    {self.fieldset_profile()}
                    <div class="columns">
                        <div class="column">
                            {ft_create_btn(
                                "signup-button",
                                "is-large".into(),
                                onclick_signup_btn,
                                self.request.username.is_empty() ||
                                    self.request.email.is_empty() ||
                                    self.request.password.is_empty(),
                            )}
                        </div>
                        <div class="column">
                            <div class="column is-flex is-vcentered">
                                <span>
                                    {get_value_field(&28)}
                                    {" ["}<a onclick={onclick_show_conditions}>{ get_value_field(&29)}</a>{"]"}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>}
    }
}

impl Register {
    fn fieldset_profile(&self) -> Html {
        // let oninput_firstname = self.link.callback(|ev: InputData| Msg::UpdateFirstname(ev.value));
        // let oninput_lastname = self.link.callback(|ev: InputData| Msg::UpdateLastname(ev.value));
        // let oninput_secondname = self.link.callback(|ev: InputData| Msg::UpdateSecondname(ev.value));
        let oninput_username = self.link.callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_email = self.link.callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self.link.callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let oninput_program_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateProgramId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let onchange_type_access_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            }));

        html! {<>
            // first columns (username, email)
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "username", get_value_field(&19), // "Username",
                        "fas fa-user",
                        self.request.username.clone(),
                        oninput_username
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "email", get_value_field(&22), // "Email",
                        "fas fa-envelope",
                        self.request.email.clone(),
                        oninput_email
                    )}
                </div>
            </div>

            // second columns (fio)
            // <div class="columns">
            //     <div class="column">
            //         {self.fileset_generator(
            //             "firstname", get_value_field(&23), // "Firstname (not required)",
            //             "", // no set icon for input
            //             self.request.firstname.clone(),
            //             oninput_firstname
            //         )}
            //     </div>
            //     <div class="column">
            //         {self.fileset_generator(
            //             "lastname", get_value_field(&24), // "Lastname (not required)",
            //             "", // no set icon for input
            //             self.request.lastname.clone(),
            //             oninput_lastname
            //         )}
            //     </div>
            //     <div class="column">
            //         {self.fileset_generator(
            //             "secondname", get_value_field(&25), // "Secondname (not required)",
            //             "", // no set icon for input
            //             self.request.secondname.clone(),
            //             oninput_secondname
            //         )}
            //     </div>
            // </div>

            // third columns (password)
            {self.fileset_generator(
                "password", get_value_field(&20), // "Password",
                "fas fa-lock",
                self.request.password.clone(),
                oninput_password
            )}

            // fourth columns (program, access)
            <div class="columns">
                <div class="column">
                    <label class="label">{get_value_field(&26)}</label>
                    <div class="control">
                        <div class="select">
                          <select
                              id="program"
                              select={self.request.program_id.to_string()}
                              onchange={oninput_program_id}
                              >
                            { for self.programs.iter().map(|x| html!{
                              <option value={x.id.to_string()} >{&x.name}</option>
                            }) }
                          </select>
                        </div>
                    </div>
                </div>
                <div class="column">
                    <label class="label">{get_value_field(&58)}</label> // "Type Access"
                    <div class="control">
                        <div class="select">
                          <select
                              id="types-access"
                              select={self.request.type_access_id.to_string()}
                              onchange={onchange_type_access_id}
                              >
                              { for self.types_access.iter().map(|x|
                                  html!{
                                      <option value={x.type_access_id.to_string()}
                                        selected={x.type_access_id == self.request.type_access_id} >
                                          {&x.name}
                                      </option>
                                  }
                              )}
                          </select>
                        </div>
                    </div>
                </div>
            </div>
        </>}
    }

    fn fileset_generator(
        &self,
        id: &str,
        label: &str,
        // placeholder: &str,
        icon_left: &str,
        value: String,
        oninput: Callback<InputData>,
    ) -> Html {
        let placeholder = label;
        let input_type = match id {
            "email" => "email",
            "password" => "password",
            _ => "text",
        };

        html!{
            <fieldset class="field">
                <label class="label">{label.to_string()}</label>
                {match icon_left.is_empty() {
                    true => html!{
                        <input
                            id={id.to_string()}
                            class="input"
                            type={input_type}
                            placeholder={placeholder.to_string()}
                            value={value}
                            oninput={oninput} />
                    },
                    false => html!{
                        <div class="control has-icons-left">
                            <input
                                id={id.to_string()}
                                class="input"
                                type={input_type}
                                placeholder={placeholder.to_string()}
                                value={value}
                                oninput={oninput} />
                            <span class="icon is-small is-left">
                              <i class={icon_left.to_string()}></i>
                            </span>
                        </div>
                    },
                }}
            </fieldset>
        }
    }

    fn modal_conditions(&self) -> Html {
        let onclick_show_conditions = self.link.callback(|_| Msg::ShowConditions);

        let class_modal = match &self.show_conditions {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_show_conditions.clone()} />
          <div class="modal-card">
            <header class="modal-card-head">
              <p class="modal-card-title">{get_value_field(&285)}</p>
              <button class="delete" aria-label="close" onclick={onclick_show_conditions.clone()} />
            </header>
            <section class="modal-card-body">
              <span>{get_value_field(&251)}</span>
              <br/>
              <span class="has-text-weight-bold">{get_value_field(&287)}</span>
              <a href="mailto:support@cadbase.rs">{"support@cadbase.rs"}</a>
            </section>
            <footer class="modal-card-foot">
              <button class="button is-fullwidth is-large" onclick={onclick_show_conditions}>
                {get_value_field(&288)}
              </button>
            </footer>
          </div>
        </div>}
    }
}
