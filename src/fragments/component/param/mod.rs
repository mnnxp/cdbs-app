mod item;
mod add;

pub use item::ComponentParamTag;
pub use add::RegisterParamnameBlock;

use std::collections::BTreeSet;
use yew::{Component, Context, html, html::Scope, Html, Properties};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ComponentParam, Param};
use crate::services::{get_value_field, resp_parsing, resp_parsing_item, resp_parsing_two_level};
use crate::gqls::make_query;
use crate::gqls::relate::{GetParams, get_params};
use crate::gqls::component::{
    PutComponentParams, put_component_params,
    GetComponentParams, get_component_params,
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_manage_btn: bool,
    pub component_uuid: UUID,
    pub component_params: Vec<ComponentParam>,
}

pub struct ComponentParamsTags {
    error: Option<Error>,
    component_uuid: UUID,
    component_params_len: usize,
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
    ResponseError(Error),
    Ignore,
}

impl Component for ComponentParamsTags {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut param_ids: BTreeSet<usize> = BTreeSet::new();
        for param in ctx.props().component_params.clone() {
            param_ids.insert(param.param.param_id);
        };

        Self {
            error: None,
            component_uuid: ctx.props().component_uuid.clone(),
            component_params_len: ctx.props().component_params.len(),
            param_ids,
            component_params: ctx.props().component_params.clone(),
            param_list: Vec::new(),
            request_add_param_id: 0,
            request_set_param_value: String::new(),
            hide_add_param_modal: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

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
                    param_id: param_id as i64,
                    value: param_value,
                };
                let ipt_component_params_data = put_component_params::IptComponentParamsData{
                    component_uuid: ctx.props().component_uuid.clone(),
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
                let component_uuid = ctx.props().component_uuid.clone();
                spawn_local(async move {
                    let res = make_query(GetComponentParams::build_query(
                        get_component_params::Variables { component_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentParamsResult(res));
                })
            },
            Msg::GetParamsListResult(res) => {
                self.param_list = resp_parsing(res, "params")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                link.send_message(Msg::SetSelectParam);
            },
            Msg::GetAddParamResult(res) => {
                let result: usize = resp_parsing_item(res, "putComponentParams")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                self.hide_add_param_modal = result > 0;
                self.request_set_param_value = String::new();
                link.send_message(Msg::RequestComponentParams);
            },
            Msg::GetComponentParamsResult(res) => {
                self.component_params = resp_parsing_two_level(res, "component", "componentParams")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                self.param_ids = BTreeSet::new();
                for param in self.component_params.clone() {
                    self.param_ids.insert(param.param.param_id);
                }
                link.send_message(Msg::SetSelectParam);
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
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.component_uuid == ctx.props().component_uuid &&
             self.component_params_len == ctx.props().component_params.len() {
            false
        } else {
            self.param_ids = BTreeSet::new();
            for param in ctx.props().component_params.iter() {
                self.param_ids.insert(param.param.param_id);
            };
            self.hide_add_param_modal = true;
            self.component_uuid = ctx.props().component_uuid.clone();
            self.component_params_len = ctx.props().component_params.len();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            {self.modal_add_param(ctx.link())}
            {self.show_params(ctx.link(), ctx.props())}
        </>}
    }
}

impl ComponentParamsTags {
    fn show_params(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_delete_param =
            link.callback(|value: usize| Msg::DeleteComponentParam(value));
        let onclick_action_btn = link.callback(|_| Msg::ChangeHideAddParam);

        html!{<div class="card column">
          <table class="table is-fullwidth">
            <tbody>
               <th>{ get_value_field(&178) }</th> // Param
               <th>{ get_value_field(&179) }</th> // Value
               {match props.show_manage_btn {
                   true => html!{<>
                       <th>{ get_value_field(&59) }</th> // Change
                       <th>{ get_value_field(&135) }</th> // Delete
                   </>},
                   false => html!{},
               }}
               {for self.component_params.iter().map(|data| {
                   match self.param_ids.get(&data.param.param_id) {
                       Some(_) => html!{<ComponentParamTag
                           show_manage_btn = {props.show_manage_btn}
                           component_uuid = {props.component_uuid.clone()}
                           param_data = {data.clone()}
                           delete_param = {Some(onclick_delete_param.clone())}
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
              <span>{ get_value_field(&180) }</span> // Add component parameter name
          </button>
        </div>}
    }

    fn modal_add_param(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_add_param =
            link.callback(|(param_id, param_value)| Msg::RequestAddParam(param_id, param_value));
        let onclick_hide_modal = link.callback(|_| Msg::ChangeHideAddParam);
        let class_modal = match &self.hide_add_param_modal {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_hide_modal.clone()} />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{ get_value_field(&181) }</p> // Add a parameter to component
                      <button class="delete" aria-label="close" onclick={onclick_hide_modal.clone()} />
                    </header>
                    <section class="modal-card-body">
                        <RegisterParamnameBlock callback_add_param={onclick_add_param.clone()} />
                    </section>
                  </div>
                </div>
              </div>
        }
    }
}
