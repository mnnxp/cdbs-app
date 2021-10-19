use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,ChangeData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::routes::AppRoute;
use crate::services::Auth;
use crate::types::{RegisterInfo, SlimUser, SlimUserWrapper};

/// Register page
pub struct Register {
    auth: Auth,
    error: Option<Error>,
    props: Props,
    request: RegisterInfo,
    response: Callback<Result<SlimUserWrapper, Error>>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    task: Option<FetchTask>,
    regions: Vec<Region>,
    programs: Vec<Program>,
    link: ComponentLink<Self>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Region {
    langId: usize,
    region: String,
    regionId: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Program {
    id: usize,
    name: String,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/register.graphql",
    response_derives = "Debug"
)]
struct RegisterOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/register.graphql",
    response_derives = "Debug"
)]
struct RegUser;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// Callback when user is registered in successfully
    pub callback: Callback<SlimUser>,
}

pub enum Msg {
    Request,
    Response(Result<SlimUserWrapper, Error>),
    Ignore,
    UpdateFirstname(String),
    UpdateLastname(String),
    UpdateSecondname(String),
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateProgramId(String),
    UpdateRegionId(String),
    UpdateList(String),
    GetRegister(String),
}

impl Component for Register {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Register {
            auth: Auth::new(),
            error: None,
            request: RegisterInfo::default(),
            response: link.callback(Msg::Response),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            programs: Vec::new(),
            regions: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render {
            spawn_local(async move {
                let res = make_query(RegisterOpt::build_query(register_opt::Variables)).await;
                link.send_message(Msg::UpdateList(res.unwrap()))
            });
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::Request => {
                let request = self.request.clone();
                spawn_local(async move {
                    let RegisterInfo {
                        firstname,
                        lastname,
                        secondname,
                        username,
                        email,
                        password,
                        phone,
                        description,
                        address,
                        time_zone,
                        position,
                        region_id,
                        program_id,
                    } = request;
                    let data = reg_user::IptUserData {
                        email,
                        username,
                        password,
                        firstname: Some(firstname),
                        lastname: Some(lastname),
                        secondname: Some(secondname),
                        phone: Some(phone),
                        description: Some(description),
                        address: Some(address),
                        timeZone: Some(time_zone.to_string()),
                        position: Some(position),
                        regionId: Some(region_id.into()),
                        programId: Some(program_id.into()),
                        typeAccessId: Some(3), // todo!(make change in future)
                    };
                    let res = make_query(RegUser::build_query(reg_user::Variables { data })).await;
                    link.send_message(Msg::GetRegister(res.unwrap()));
                })
            }
            Msg::Response(Ok(user_info)) => {
                self.props.callback.emit(user_info.user);
                self.error = None;
                self.task = None;
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
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
                self.request.password = password;
            }
            Msg::UpdateUsername(username) => {
                self.request.username = username;
            }
            Msg::UpdateProgramId(program_id) => {
                self.request.program_id = program_id.parse::<i32>().unwrap_or(1);
                ConsoleService::info(format!("Update: {:?}", program_id).as_ref());
            }
            Msg::UpdateRegionId(region_id) => {
                self.request.region_id = region_id.parse::<i32>().unwrap_or(1);
                ConsoleService::info(format!("Update: {:?}", region_id).as_ref());
            }
            Msg::UpdateList(res) => {
                // self.list = res;
                // true
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();
                let programs = res.get("programs").unwrap();
                self.regions = serde_json::from_value(res.get("regions").unwrap().clone()).unwrap();
                self.programs =
                    serde_json::from_value(res.get("programs").unwrap().clone()).unwrap();
                ConsoleService::info(format!("Update: {:?}", self.programs).as_ref());
            }
            Msg::GetRegister(res) => {
              self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
            }
            Msg::Ignore => {}
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
        let oninput_program_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateProgramId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let onchange_region_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateProgramId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        html! {
            <div class="auth-page">
                <h1 class="title">{ "Sign Up" }</h1>
                <h2 class="subtitle">
                    <RouterAnchor<AppRoute> route=AppRoute::Login>
                        { "Have an account?" }
                    </RouterAnchor<AppRoute>>
                </h2>
                <ListErrors error=self.error.clone() />
                <form onsubmit=onsubmit>
                    <fieldset>
                        <fieldset class="field">
                            <label class="label">{"Firstname"}</label>
                            <div class="control">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Text input"
                                    value=self.request.firstname.clone()
                                    oninput=oninput_firstname
                                    />
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Lastname"}</label>
                            <div class="control">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Text input"
                                    value=self.request.lastname.clone()
                                    oninput=oninput_lastname
                                    />
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Secondname"}</label>
                            <div class="control">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Text input"
                                    value=self.request.secondname.clone()
                                    oninput=oninput_secondname
                                    />
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Username"}</label>
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Username"
                                    value=self.request.username.clone()
                                    oninput=oninput_username
                                    />
                                <span class="icon is-small is-left">
                                    <i class="fas fa-user"></i>
                                </span>
                                <span class="icon is-small is-right">
                                <i class="fas fa-check"></i>
                                </span>
                            </div>
                            // <p class="help is-success">{"This username is available"}</p>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Email"}</label>
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    class="input"
                                    type="email"
                                    placeholder="Email"
                                    value=self.request.email.clone()
                                    oninput=oninput_email
                                    />
                                <span class="icon is-small is-left">
                                <i class="fas fa-envelope"></i>
                                </span>
                                <span class="icon is-small is-right">
                                <i class="fas fa-exclamation-triangle"></i>
                                </span>
                            </div>
                            // <p class="help is-danger">{"This email is invalid"}</p>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Password"}</label>
                            <div class="control has-icons-left">
                                <input
                                    class="input"
                                    type="password"
                                    placeholder="Password"
                                    value=self.request.password.clone()
                                    oninput=oninput_password
                                    />
                                <span class="icon is-small is-left">
                                  <i class="fas fa-lock"></i>
                                </span>
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Select a program:"}</label>
                            <div class="control">
                                <div class="select">
                                  <select
                                      select=self.request.program_id.to_string()
                                      onchange=oninput_program_id
                                      >
                                    { for self.programs.iter().map(|x| html!{
                                      <option value={x.id.to_string()} >{&x.name}</option>
                                    }) }
                                  </select>
                                </div>
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"What's your region?"}</label>
                            <div class="control">
                                <div class="select">
                                  <select
                                      select=self.request.region_id.to_string()
                                      onchange=onchange_region_id
                                      >
                                      { for self.regions.iter().map(|x| html!{
                                        <option value={x.regionId.to_string()} >{&x.region}</option>
                                      }) }
                                  </select>
                                </div>
                            </div>
                        </fieldset>
                        <div class="field">
                          <div class="control">
                            <label class="checkbox">
                              <input type="checkbox"/>
                              {" I agree to the "}<a href="#">{"terms and conditions"}</a>
                            </label>
                          </div>
                        </div>
                        <button
                            class="button"
                            type="submit"
                            disabled=false
                        >
                            { "Sign up" }
                        </button>
                    </fieldset>
                </form>
            </div>
        }
    }
}
