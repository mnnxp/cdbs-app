use yew::{agent::Bridged, html, Bridge, Component, ComponentLink, Html, ShouldRender, InputData, ChangeData};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_create_btn;
use crate::services::{get_from_value, get_logged_user, get_value_field, get_value_response, resp_parsing, set_history_back};
use crate::types::{UUID, ComponentCreateData, TypeAccessInfo, ActualStatus};
use crate::gqls::make_query;
use crate::gqls::component::{
    GetComponentDataOpt, get_component_data_opt,
    RegisterComponent, register_component,
};

/// Component with relate data
pub struct CreateComponent {
    error: Option<Error>,
    request_component: ComponentCreateData,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    actual_statuses: Vec<ActualStatus>,
    types_access: Vec<TypeAccessInfo>,
    disable_create_btn: bool,
    name_empty: bool,
}

#[derive(Clone)]
pub enum Msg {
    RequestManager,
    RequestCreateComponentData,
    ResponseError(Error),
    GetListOpt(String),
    GetCreateComponentResult(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdateTypeAccessId(String),
    UpdateActualStatusId(String),
    ClearError,
    Ignore,
}

impl Component for CreateComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateComponent {
            error: None,
            request_component: ComponentCreateData::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            actual_statuses: Vec::new(),
            types_access: Vec::new(),
            disable_create_btn: false,
            name_empty: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if let None = get_logged_user() {
            set_history_back(Some(String::new()));
            // route to login page if not found token
            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
        };

        if first_render {
            let link = self.link.clone();

            spawn_local(async move {
                let res = make_query(GetComponentDataOpt::build_query(
                    get_component_data_opt::Variables
                )).await.unwrap();

                link.send_message(Msg::GetListOpt(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestManager => {
                self.disable_create_btn = true;
                // checking have data
                if self.request_component.name.is_empty() {
                    debug!("name is empty: {:?}", self.request_component.name);
                    self.name_empty = true;
                    self.disable_create_btn = false;
                }

                if self.disable_create_btn {
                    link.send_message(Msg::RequestCreateComponentData);
                }
            },
            Msg::RequestCreateComponentData => {
                let request_component: ComponentCreateData = self.request_component.clone();

                spawn_local(async move {
                    let ComponentCreateData {
                        parent_component_uuid,
                        name,
                        description,
                        type_access_id,
                        component_type_id,
                        actual_status_id,
                        is_base,
                    } = request_component;
                    let ipt_component_data = register_component::IptComponentData {
                        parentComponentUuid: parent_component_uuid,
                        name,
                        description,
                        typeAccessId: type_access_id as i64,
                        componentTypeId: component_type_id as i64,
                        actualStatusId: actual_status_id as i64,
                        isBase: is_base,
                    };
                    let res = make_query(RegisterComponent::build_query(register_component::Variables {
                        ipt_component_data
                    })).await.unwrap();
                    link.send_message(Msg::GetCreateComponentResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetListOpt(res) => {
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.actual_statuses = get_from_value(value, "componentActualStatuses").unwrap_or_default();
                        self.types_access = get_from_value(value, "typesAccess").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetCreateComponentResult(res) => {
                match resp_parsing::<UUID>(res, "registerComponent") {
                    Ok(result) => {
                        debug!("registerComponent: {:?}", result);
                        // Redirect to setting component page
                        if !result.is_empty() {
                            self.router_agent.send(
                                ChangeRoute(AppRoute::ComponentSettings(result).into())
                            );
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            // items request create main component data
            Msg::UpdateName(data) => {
                self.request_component.name = data;
                self.name_empty = false;
            },
            Msg::UpdateDescription(data) => self.request_component.description = data,
            Msg::UpdateTypeAccessId(data) =>
                self.request_component.type_access_id = data.parse::<usize>().unwrap_or_default(),
            Msg::UpdateActualStatusId(data) =>
                self.request_component.actual_status_id = data.parse::<usize>().unwrap_or_default(),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_create_component = self.link.callback(|_| Msg::RequestManager);

        html!{
            <div class="component-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                        <h1 class="title">{get_value_field(&290)}</h1>
                        {self.show_main_card()}
                        <br/>
                        {ft_create_btn(
                            "create-component",
                            "is-medium".into(),
                            onclick_create_component,
                            self.disable_create_btn,
                        )}
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateComponent {
    fn show_main_card(&self) -> Html {
        let onchange_actual_status_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let onchange_change_type_access =
            self.link.callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let oninput_name =
            self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_description =
            self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let class_name = match self.name_empty {
            true => "input is-danger",
            false => "input",
        };

        html!{
            <div class="card">
                <div class="column">
                  <label class="label">{get_value_field(&110)}</label>
                  <input
                      id="update-name"
                      class={class_name}
                      type="text"
                      placeholder={get_value_field(&110)}
                      value={self.request_component.name.clone()}
                      oninput={oninput_name} />
                  <label class="label">{get_value_field(&61)}</label>
                  <textarea
                      id="update-description"
                      class="textarea"
                      // rows="10"
                      type="text"
                      placeholder={get_value_field(&61)}
                      value={self.request_component.description.clone()}
                      oninput={oninput_description} />
                </div>
                <div class="column">
                    <div class="columns">
                        <div class="column" style="margin-right: 1rem">
                            <label class="label">{get_value_field(&96)}</label>
                            <div class="select">
                              <select
                                  id="component-status-id"
                                  select={self.request_component.actual_status_id.to_string()}
                                  onchange={onchange_actual_status_id}
                                  >
                                { for self.actual_statuses.iter().map(|x|
                                    html!{
                                        <option value={x.actual_status_id.to_string()}
                                              selected={x.actual_status_id == self.request_component.actual_status_id} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                        <div class="column" style="margin-right: 1rem">
                            <label class="label">{get_value_field(&114)}</label>
                            <div class="select">
                              <select
                                  id="set-type-access"
                                  select={self.request_component.type_access_id.to_string()}
                                  onchange={onchange_change_type_access}
                                >
                              { for self.types_access.iter().map(|x|
                                  html!{
                                      <option value={x.type_access_id.to_string()}
                                            selected={x.type_access_id == self.request_component.type_access_id} >
                                          {&x.name}
                                      </option>
                                  }
                              )}
                              </select>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
