mod item;
mod add;

pub use item::ComponentParamTag;
pub use add::RegisterParamnameBlock;

use std::collections::BTreeSet;
// use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, ComponentParam, Param};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct PutComponentParams;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetComponentParams;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct GetParams;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub component_uuid: UUID,
    pub component_params: Vec<ComponentParam>,
}

pub struct ComponentParamsTags {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    param_ids: BTreeSet<usize>,
    component_params: Vec<ComponentParam>,
    param_list: Vec<Param>,
    request_add_param_id: usize,
    request_set_param_value: String,
    hide_add_param_modal: bool,
}

#[derive(Clone)]
pub enum Msg {
    DeleteComponentParam(usize),
    RequestParamsList,
    RequestAddParam(usize, String),
    RequestComponentParams,
    GetParamsListResult(String),
    GetComponentParamsResult(String),
    GetAddParamResult(String),
    UpdateParamValue(String),
    ChangeHideAddParam,
    SetSelectParam,
    ClearError,
    Ignore,
}

impl Component for ComponentParamsTags {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut param_ids: BTreeSet<usize> = BTreeSet::new();

        for param in props.component_params.clone() {
            param_ids.insert(param.param.param_id);
        };

        let component_params = props.component_params.clone();

        Self {
            error: None,
            props,
            link,
            param_ids,
            component_params,
            param_list: Vec::new(),
            request_add_param_id: 0,
            request_set_param_value: String::new(),
            hide_add_param_modal: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::DeleteComponentParam(param_id) => {
                self.param_ids.remove(&param_id);
                link.send_message(Msg::SetSelectParam);
            },
            Msg::RequestParamsList => {
                spawn_local(async move {
                    let res = make_query(GetParams::build_query(
                        get_params::Variables { ipt_param_arg: None }
                    )).await.unwrap();
                    link.send_message(Msg::GetParamsListResult(res));
                })
            },
            Msg::RequestAddParam(param_id, param_value) => {
                let ipt_param_data = put_component_params::IptParamData{
                    paramId: param_id as i64,
                    value: param_value,
                };
                let ipt_component_params_data = put_component_params::IptComponentParamsData{
                    componentUuid: self.props.component_uuid.clone(),
                    params: vec![ipt_param_data],
                };
                spawn_local(async move {
                    let res = make_query(PutComponentParams::build_query(
                        put_component_params::Variables { ipt_component_params_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetAddParamResult(res));
                })
            },
            Msg::RequestComponentParams => {
                let component_uuid = self.props.component_uuid.clone();
                spawn_local(async move {
                    let res = make_query(GetComponentParams::build_query(
                        get_component_params::Variables { component_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentParamsResult(res));
                })
            },
            Msg::GetParamsListResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<Param> =
                            serde_json::from_value(res_value.get("params").unwrap().clone()).unwrap();
                        debug!("params: {:?}", result);
                        self.param_list = result;
                        link.send_message(Msg::SetSelectParam);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetAddParamResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize =
                            serde_json::from_value(res_value.get("putComponentParams").unwrap().clone()).unwrap();
                        debug!("putComponentParams: {:?}", result);
                        self.hide_add_param_modal = result > 0;
                        self.request_set_param_value = String::new();
                        link.send_message(Msg::RequestComponentParams);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetComponentParamsResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<ComponentParam> =
                            serde_json::from_value(res_value.get("component").unwrap()
                                .get("componentParams").unwrap().clone()).unwrap();
                        debug!("componentParams: {:?}", result);
                        self.component_params = result;
                        self.param_ids = BTreeSet::new();
                        for param in self.component_params.clone() {
                            self.param_ids.insert(param.param.param_id);
                        };
                        link.send_message(Msg::SetSelectParam);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateParamValue(data) => self.request_set_param_value = data,
            Msg::ChangeHideAddParam => {
                if self.hide_add_param_modal && self.param_list.is_empty() {
                    link.send_message(Msg::RequestParamsList)
                }
                self.hide_add_param_modal = !self.hide_add_param_modal
            },
            Msg::SetSelectParam => {
                self.request_add_param_id = 0;
                for param in self.param_list.iter() {
                    if let None = self.param_ids.get(&param.param_id) {
                        self.request_add_param_id = param.param_id;
                        break;
                    }
                }
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
             self.props.component_params.len() == props.component_params.len() {
            false
        } else {
            self.param_ids = BTreeSet::new();
            for param in props.component_params.iter() {
                self.param_ids.insert(param.param.param_id);
            };
            self.hide_add_param_modal = true;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {self.modal_add_param()}
            {self.show_params()}
        </>}
    }
}

impl ComponentParamsTags {
    fn show_params(&self) -> Html {
        let onclick_delete_param = self.link
            .callback(|value: usize| Msg::DeleteComponentParam(value));

        let onclick_action_btn = self.link
            .callback(|_| Msg::ChangeHideAddParam);

        html!{<div class="card column">
          <table class="table is-fullwidth">
            <tbody>
               <th>{"Param"}</th>
               <th>{"Value"}</th>
               {match self.props.show_manage_btn {
                   true => html!{<>
                       <th>{"Change"}</th>
                       <th>{"Delete"}</th>
                   </>},
                   false => html!{},
               }}
               {for self.component_params.iter().map(|data| {
                   match self.param_ids.get(&data.param.param_id) {
                       Some(_) => html!{<ComponentParamTag
                           show_manage_btn = self.props.show_manage_btn
                           component_uuid = self.props.component_uuid.clone()
                           param_data = data.clone()
                           delete_param = Some(onclick_delete_param.clone())
                         />},
                       None => html!{},
                   }
               })}
            </tbody>
          </table>
          <button
                id="add-param-component"
                class="button is-fullwidth"
                onclick={onclick_action_btn} >
              <span class="icon" >
                  <i class="fas fa-plus" aria-hidden="true"></i>
              </span>
              <span>{"Add component param"}</span>
          </button>
        </div>}
    }

    fn modal_add_param(&self) -> Html {
        let onclick_add_param =
            self.link.callback(|(param_id, param_value)| Msg::RequestAddParam(param_id, param_value));

        let onclick_hide_modal = self.link.callback(|_| Msg::ChangeHideAddParam);

        let class_modal = match &self.hide_add_param_modal {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_hide_modal.clone() />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{"Add param for component"}</p>
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <RegisterParamnameBlock callback_add_param=onclick_add_param.clone() />
                    </section>
                  </div>
                </div>
              </div>
        }
    }
}
