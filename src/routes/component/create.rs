use yew::{Component, Context, html, Html};
use yew::html::{Scope, TargetCast};
use yew_router::prelude::*;
use web_sys::{InputEvent, Event, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::routes::AppRoute::{Login, ComponentSettings};
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_logged_user, get_value_field};
use crate::types::{UUID, ComponentCreateData, TypeAccessInfo, ActualStatus, ComponentType};
use crate::gqls::make_query;
use crate::gqls::component::{
    GetComponentDataOpt, get_component_data_opt,
    RegisterComponent, register_component,
};

/// Component with relate data
pub struct CreateComponent {
    error: Option<Error>,
    request_component: ComponentCreateData,
    component_types: Vec<ComponentType>,
    actual_statuses: Vec<ActualStatus>,
    types_access: Vec<TypeAccessInfo>,
    disable_create_btn: bool,
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
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        CreateComponent {
            error: None,
            request_component: ComponentCreateData::new(),
            component_types: Vec::new(),
            actual_statuses: Vec::new(),
            types_access: Vec::new(),
            disable_create_btn: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if let None = get_logged_user() {
            // route to login page if not found token
            let navigator: Navigator = ctx.link().navigator().unwrap();
            navigator.replace(&Login);
        };
        if first_render {
            let link = ctx.link().clone();
            spawn_local(async move {
                let res = make_query(GetComponentDataOpt::build_query(
                    get_component_data_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::GetListOpt(res));
            })
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

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
                        parent_component_uuid,
                        name,
                        description,
                        type_access_id: type_access_id as i64,
                        component_type_id: component_type_id as i64,
                        actual_status_id: actual_status_id as i64,
                        is_base,
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
                        self.component_types =
                            serde_json::from_value(res_value.get("componentTypes").unwrap().clone()).unwrap();
                        self.actual_statuses =
                            serde_json::from_value(res_value.get("componentActualStatuses").unwrap().clone()).unwrap();
                        self.types_access =
                            serde_json::from_value(res_value.get("typesAccess").unwrap().clone()).unwrap();
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
                            // self.router_agent.send(ComponentSettings { uuid: result });
                            let navigator: Navigator = ctx.link().navigator().unwrap();
                            navigator.replace(&ComponentSettings { uuid: result });
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
                self.request_component.type_access_id = data.parse::<usize>().unwrap_or(1),
            Msg::UpdateComponentTypeId(data) =>
                self.request_component.component_type_id = data.parse::<usize>().unwrap_or(1),
            Msg::UpdateActualStatusId(data) =>
                self.request_component.actual_status_id = data.parse::<usize>().unwrap_or(1),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{
            <div class="component-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
                        <h1 class="title">{ get_value_field(&290) }</h1>
                        {self.show_main_card(ctx.link())}
                        <br/>
                        {self.show_manage_btn(ctx.link())}
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateComponent {
    fn show_main_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onchange_actual_status_id =
            link.callback(|ev: Event| Msg::UpdateActualStatusId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));
        let onchange_change_component_type =
            link.callback(|ev: Event| Msg::UpdateComponentTypeId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));
        let onchange_change_type_access =
            link.callback(|ev: Event| Msg::UpdateTypeAccessId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default()));
        let oninput_name = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateName(input.value())
        });
        let oninput_description = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateDescription(input.value())
        });

        html!{
            <div class="card">
                <div class="column">
                    <div class="columns">
                        <div class="column" style="margin-right: 1rem">
                            <label class="label">{ get_value_field(&96) }</label>
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
                            <label class="label">{ get_value_field(&97) }</label>
                            <div class="select">
                              <select
                                  id="set-component-type"
                                  select={self.request_component.component_type_id.to_string()}
                                  onchange={onchange_change_component_type}
                                >
                              { for self.component_types.iter().map(|x|
                                  html!{
                                      <option value={x.component_type_id.to_string()}
                                            selected={x.component_type_id == self.request_component.component_type_id} >
                                          {&x.component_type}
                                      </option>
                                  }
                              )}
                              </select>
                            </div>
                        </div>
                        <div class="column" style="margin-right: 1rem">
                            <label class="label">{ get_value_field(&114) }</label>
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
                <div class="column">
                  <label class="label">{ get_value_field(&110) }</label>
                  <input
                      id="update-name"
                      class="input"
                      type="text"
                      placeholder={get_value_field(&110)}
                      value={self.request_component.name.clone()}
                      oninput={oninput_name} />
                  <label class="label">{ get_value_field(&61) }</label>
                  <textarea
                      id="update-description"
                      class="textarea"
                      // rows="10"
                      type="text"
                      placeholder={get_value_field(&61)}
                      value={self.request_component.description.clone()}
                      oninput={oninput_description} />
                </div>
            </div>
        }
    }

    fn show_manage_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_create_changes = link.callback(|_| Msg::RequestManager);

        html!{
            <button
                id="create-data"
                class="button is-success is-medium is-fullwidth"
                onclick={onclick_create_changes}
                disabled={self.disable_create_btn} >
                { get_value_field(&45) } // Create
            </button>
        }
    }
}
