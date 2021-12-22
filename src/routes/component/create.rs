use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
// use chrono::NaiveDateTime;
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::services::is_authenticated;
use crate::types::{
    UUID, ComponentCreateData, TypeAccessInfo,
    ActualStatus, ComponentType
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetComponentDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct RegisterComponent;

/// Component with relate data
pub struct CreateComponent {
    error: Option<Error>,
    request_component: ComponentCreateData,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    component_types: Vec<ComponentType>,
    actual_statuses: Vec<ActualStatus>,
    types_access: Vec<TypeAccessInfo>,
    disable_create_btn: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    // pub current_user: Option<SlimUser>,
}

#[derive(Clone)]
pub enum Msg {
    RequestManager,
    RequestCreateComponentData,
    GetListOpt(String),
    GetCreateComponentResult(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdateTypeAccessId(String),
    UpdateComponentTypeId(String),
    UpdateActualStatusId(String),
    ClearError,
    Ignore,
}

impl Component for CreateComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateComponent {
            error: None,
            request_component: ComponentCreateData::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            component_types: Vec::new(),
            actual_statuses: Vec::new(),
            types_access: Vec::new(),
            disable_create_btn: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if first_render && is_authenticated() {
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
                let mut flag = true;
                // checking have data
                if self.request_component.name.is_empty() {
                    debug!("name is empty: {:?}", self.request_component.name);
                    flag = false;
                }
                if self.request_component.description.is_empty() {
                    debug!("description is empty: {:?}", self.request_component.description);
                    flag = false;
                }

                if flag {
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
            Msg::GetListOpt(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.component_types = serde_json::from_value(
                            res_value.get("componentTypes").unwrap().clone()
                        ).unwrap();
                        self.actual_statuses = serde_json::from_value(
                            res_value.get("componentActualStatuses").unwrap().clone()
                        ).unwrap();
                        self.types_access = serde_json::from_value(
                            res_value.get("typesAccess").unwrap().clone()
                        ).unwrap();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetCreateComponentResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UUID = serde_json::from_value(
                            res_value.get("registerComponent").unwrap().clone()
                        ).unwrap();
                        debug!("registerComponent: {:?}", result);
                        // Redirect to setting component page
                        if !result.is_empty() {
                            self.router_agent.send(ChangeRoute(
                                AppRoute::ComponentSettings(result).into()
                            ));
                        }

                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            // items request create main component data
            Msg::UpdateName(data) =>
                self.request_component.name = data,
            Msg::UpdateDescription(data) =>
                self.request_component.description = data,
            Msg::UpdateTypeAccessId(data) =>
                self.request_component.type_access_id = data.parse::<usize>().unwrap_or_default(),
            Msg::UpdateComponentTypeId(data) =>
                self.request_component.component_type_id = data.parse::<usize>().unwrap_or_default(),
            Msg::UpdateActualStatusId(data) =>
                self.request_component.actual_status_id = data.parse::<usize>().unwrap_or_default(),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link
            .callback(|_| Msg::ClearError);

        html! {
            <div class="component-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
                        // <br/>
                        {self.show_manage_btn()}
                        <br/>
                        {self.show_main_card()}
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateComponent {
    fn show_main_card(&self) -> Html {
        let onchange_actual_status_id = self.link
            .callback(|ev: ChangeData| Msg::UpdateActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let onchange_change_component_type = self.link
            .callback(|ev: ChangeData| Msg::UpdateComponentTypeId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let onchange_change_type_access = self.link
            .callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let oninput_name = self.link
            .callback(|ev: InputData| Msg::UpdateName(ev.value));

        let oninput_description = self.link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        html!{
            <div class="card">
                <div class="column">
                    <div class="columns">
                        <div class="column" style="margin-right: 1rem">
                            <label class="label">{"Actual status"}</label>
                            <div class="select">
                              <select
                                  id="component-status-id"
                                  select={self.request_component.actual_status_id.to_string()}
                                  onchange=onchange_actual_status_id
                                  >
                                { for self.actual_statuses.iter().map(|x|
                                    match self.request_component.actual_status_id == x.actual_status_id {
                                        true => html!{<option value={x.actual_status_id.to_string()} selected=true>{&x.name}</option>},
                                        false => html!{<option value={x.actual_status_id.to_string()}>{&x.name}</option>},
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                        <div class="column" style="margin-right: 1rem">
                            <label class="label">{"Component type "}</label>
                            <div class="select">
                              <select
                                  id="set-component-type"
                                  select={self.request_component.component_type_id.to_string()}
                                  onchange=onchange_change_component_type
                                >
                              { for self.component_types.iter().map(|x|
                                  match self.request_component.component_type_id == x.component_type_id {
                                      true => html!{ <option value={x.component_type_id.to_string()} selected=true>{&x.component_type}</option> },
                                      false => html!{ <option value={x.component_type_id.to_string()}>{&x.component_type}</option> },
                                  }
                              )}
                              </select>
                            </div>
                        </div>
                        <div class="column" style="margin-right: 1rem">
                            <label class="label">{"Type access "}</label>
                            <div class="select">
                              <select
                                  id="set-type-access"
                                  select={self.request_component.type_access_id.to_string()}
                                  onchange=onchange_change_type_access
                                >
                              { for self.types_access.iter().map(|x|
                                  match self.request_component.type_access_id == x.type_access_id {
                                      true => html!{ <option value={x.type_access_id.to_string()} selected=true>{&x.name}</option> },
                                      false => html!{ <option value={x.type_access_id.to_string()}>{&x.name}</option> },
                                  }
                              )}
                              </select>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="column">
                  <label class="label">{"Name"}</label>
                  <input
                      id="update-name"
                      class="input"
                      type="text"
                      placeholder="component name"
                      value={self.request_component.name.clone()}
                      oninput=oninput_name />
                  <label class="label">{"Description"}</label>
                  <textarea
                      id="update-description"
                      class="textarea"
                      // rows="10"
                      type="text"
                      placeholder="component description"
                      value={self.request_component.description.clone()}
                      oninput=oninput_description />
                </div>
            </div>
        }
    }

    fn show_manage_btn(&self) -> Html {
        let onclick_create_changes = self.link
            .callback(|_| Msg::RequestManager);

        html!{
            <div class="media">
                <div class="media-content">
                    // html!{"Data updated"}
                </div>
                <div class="media-right">
                    <button
                        id="create-data"
                        class="button"
                        onclick={onclick_create_changes}
                        disabled={self.disable_create_btn} >
                        {"Create"}
                    </button>
                </div>
            </div>
        }
    }
}
