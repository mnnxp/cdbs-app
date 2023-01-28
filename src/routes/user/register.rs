use yew::{Component, Callback, Context, html, Html, classes};
use yew::html::{Scope, TargetCast};
use yew_router::prelude::*;
use web_sys::{InputEvent, Event, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::routes::AppRoute::{self, Login, Profile};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_from_value, get_logged_user, get_value_field, get_value_response};
use crate::types::{RegisterInfo, Region, Program, TypeAccessInfo};
use crate::gqls::make_query;
use crate::gqls::user::{
    RegisterOpt, register_opt,
    RegUser, reg_user,
};

/// Register page
pub struct Register {
    error: Option<Error>,
    request: RegisterInfo,
    regions: Vec<Region>,
    programs: Vec<Program>,
    types_access: Vec<TypeAccessInfo>,
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
    UpdateRegionId(String),
    UpdateTypeAccessId(String),
    UpdateList(String),
    GetRegister(String),
    ShowConditions,
    ResponseError(Error),
    Ignore,
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            request: RegisterInfo::default(),
            programs: Vec::new(),
            regions: Vec::new(),
            types_access: Vec::new(),
            show_conditions: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            if let Some(user) = get_logged_user() {
                // route to profile page if user already logged
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Profile { username: user.username });
            };
            spawn_local(async move {
                let res = make_query(RegisterOpt::build_query(
                    register_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::UpdateList(res))
            });
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
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
                    time_zone: Some(self.request.time_zone.clone()),
                    position: Some(self.request.position.clone()),
                    region_id: Some(self.request.region_id as i64),
                    program_id: Some(self.request.program_id as i64),
                    type_access_id: Some(self.request.type_access_id as i64),
                };
                spawn_local(async move {
                    let res = make_query(RegUser::build_query(reg_user::Variables {
                        ipt_user_data
                    })).await.unwrap();
                    link.send_message(Msg::GetRegister(res));
                })
            },
            Msg::UpdateList(res) => {
                let value: Value = get_value_response(res)
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                self.regions = get_from_value(&value, "regions").unwrap();
                self.programs = get_from_value(&value, "programs").unwrap();
                self.types_access = get_from_value(&value, "typesAccess").unwrap();
                debug!("Update: {:?}", self.programs);
            },
            Msg::GetRegister(res) => {
                let _value: Value = get_value_response(res)
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Login);
            },
            // Msg::UpdateFirstname(firstname) => self.request.firstname = firstname,
            // Msg::UpdateLastname(lastname) => self.request.lastname = lastname,
            // Msg::UpdateSecondname(secondname) => self.request.secondname = secondname,
            Msg::UpdateEmail(email) => self.request.email = email,
            Msg::UpdatePassword(password) => self.request.password = password,
            Msg::UpdateUsername(username) => self.request.username = username,
            Msg::UpdateProgramId(program_id) =>
                self.request.program_id = program_id.parse::<usize>().unwrap_or(1),
            Msg::UpdateRegionId(region_id) =>
                self.request.region_id = region_id.parse::<usize>().unwrap_or(1),
            Msg::UpdateTypeAccessId(type_access_id) =>
                self.request.type_access_id = type_access_id.parse::<usize>().unwrap_or(1),
            Msg::ShowConditions => self.show_conditions = !self.show_conditions,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::Ignore => {}
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_show_conditions = ctx.link().callback(|_| Msg::ShowConditions);
        let onclick_signup_btn = ctx.link().callback(|_| Msg::Request);

        html!{<div class="container page">
            <div class="auth-page">
                <h1 class="title">{ get_value_field(&14) }</h1>
                <h2 class="subtitle">
                    <Link<AppRoute> to={Login}>
                        { get_value_field(&21) }
                    </Link<AppRoute>>
                </h2>
                <ListErrors error={self.error.clone()} />
                {self.modal_conditions(ctx.link())}
                <div class="card column">
                    {self.fieldset_profile(ctx.link())}
                    <div class="columns">
                        <div class="column">
                            <button
                                id="signup-button"
                                class={classes!("button", "is-fullwidth", "is-large")}
                                onclick={onclick_signup_btn}
                                disabled={self.request.username.is_empty() ||
                                    self.request.email.is_empty() ||
                                    self.request.password.is_empty()}
                            >
                                { get_value_field(&45) }
                            </button>
                        </div>
                        <div class="column">
                            <div class="column is-flex is-vcentered">
                                <span>
                                    { get_value_field(&28) }
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
    fn fieldset_profile(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        // let oninput_firstname = link.callback(|ev: InputEvent| {
        //     let input: HtmlInputElement = ev.target_unchecked_into();
        //     Msg::UpdateFirstname(input.value())
        // });
        // let oninput_lastname = link.callback(|ev: InputEvent| {
        //     let input: HtmlInputElement = ev.target_unchecked_into();
        //     Msg::UpdateLastname(input.value())
        // });
        // let oninput_secondname = link.callback(|ev: InputEvent| {
        //     let input: HtmlInputElement = ev.target_unchecked_into();
        //     Msg::UpdateSecondname(input.value())
        // });
        let oninput_username = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateUsername(input.value())
        });
        let oninput_email = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateEmail(input.value())
        });
        let oninput_password = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdatePassword(input.value())
        });
        let oninput_program_id =
            link.callback(|ev: Event| Msg::UpdateProgramId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));
        let onchange_region_id =
            link.callback(|ev: Event| Msg::UpdateRegionId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));
        let onchange_type_access_id =
            link.callback(|ev: Event| Msg::UpdateTypeAccessId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));

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

            // fourth columns (program, region, access)
            <div class="columns">
                <div class="column">
                    <label class="label">{ get_value_field(&26) }</label>
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
                    <label class="label">{ get_value_field(&27) }</label>
                    <div class="control">
                        <div class="select">
                          <select
                              id="region"
                              select={self.request.region_id.to_string()}
                              onchange={onchange_region_id}
                              >
                              { for self.regions.iter().map(|x|
                                  html!{
                                      <option value={x.region_id.to_string()}
                                            selected={x.region_id == self.request.region_id} >
                                          {&x.region}
                                      </option>
                                  }
                              )}
                          </select>
                        </div>
                    </div>
                </div>
                <div class="column">
                    <label class="label">{ get_value_field(&58) }</label> // "Type Access"
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
        oninput: Callback<InputEvent>,
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

    fn modal_conditions(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_show_conditions = link.callback(|_| Msg::ShowConditions);
        let class_modal = match &self.show_conditions {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_show_conditions.clone()} />
          <div class="modal-card">
            <header class="modal-card-head">
              <p class="modal-card-title">{ get_value_field(&285) }</p>
              <button class="delete" aria-label="close" onclick={onclick_show_conditions.clone()} />
            </header>
            <section class="modal-card-body">
              <span>{ get_value_field(&251) }</span>
              <br/>
              <span class="has-text-weight-bold">{ get_value_field(&287) }</span>
              <a href="mailto:support@cadbase.rs">{"support@cadbase.rs"}</a>
            </section>
            <footer class="modal-card-foot">
              <button class="button is-fullwidth is-large" onclick={onclick_show_conditions}>
                { get_value_field(&288) }
              </button>
            </footer>
          </div>
        </div>}
    }
}
