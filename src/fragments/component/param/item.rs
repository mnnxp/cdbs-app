use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use web_sys::InputEvent;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ComponentParam};
use crate::services::get_value_field;
use crate::gqls::make_query;
use crate::gqls::component::{
    PutComponentParams, put_component_params,
    DeleteComponentParams, delete_component_params,
};

/// Param card for show data on component page
pub struct ComponentParamTag {
    error: Option<Error>,
    param_id: usize,
    current_param_value: String,
    request_set_param_value: String,
    hide_edit_param_value: bool,
    get_result_delete: bool,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_manage_btn: bool,
    pub component_uuid: UUID,
    pub param_data: ComponentParam,
    #[prop_or_default]
    pub delete_param: Option<Callback<usize>>,
}

#[derive(Clone)]
pub enum Msg {
    ChangeParamValue,
    RequestChangeValue,
    RequestDeleteParam,
    ResponseError(Error),
    GetDeleteParamResult(String),
    GetChangeValueResult(String),
    UpdateParamValue(String),
    Ignore,
}

impl Component for ComponentParamTag {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            param_id: ctx.props().param_data.param.param_id.clone(),
            current_param_value: ctx.props().param_data.value.clone(),
            request_set_param_value: ctx.props().param_data.value.clone(),
            hide_edit_param_value: true,
            get_result_delete: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::ChangeParamValue => self.hide_edit_param_value = !self.hide_edit_param_value,
            Msg::RequestChangeValue => {
                let ipt_param_data = put_component_params::IptParamData{
                    param_id: ctx.props().param_data.param.param_id as i64,
                    value: self.request_set_param_value.clone(),
                };
                let ipt_component_params_data = put_component_params::IptComponentParamsData{
                    component_uuid: ctx.props().component_uuid.clone(),
                    params: vec![ipt_param_data],
                };
                spawn_local(async move {
                    let res = make_query(PutComponentParams::build_query(
                        put_component_params::Variables { ipt_component_params_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetChangeValueResult(res));
                })
            },
            Msg::RequestDeleteParam => {
                let component_uuid = ctx.props().component_uuid.clone();
                let param_id = ctx.props().param_data.param.param_id as i64;
                spawn_local(async move {
                    let del_component_param_data = delete_component_params::DelComponentParamData{
                        component_uuid,
                        param_ids: vec![param_id],
                    };
                    let res = make_query(DeleteComponentParams::build_query(
                        delete_component_params::Variables { del_component_param_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteParamResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDeleteParamResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res.get("deleteComponentParams").unwrap().clone()).unwrap();
                        debug!("deleteComponentParams: {:?}", result);
                        match &ctx.props().delete_param {
                            Some(delete_param) => {
                                if result > 0 {
                                    self.get_result_delete = true;
                                    self.hide_edit_param_value = true;
                                    delete_param.emit(ctx.props().param_data.param.param_id);
                                };
                            },
                            None => self.get_result_delete = result > 0,
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetChangeValueResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize =
                            serde_json::from_value(res_value.get("putComponentParams").unwrap().clone()).unwrap();
                        debug!("putComponentParams: {:?}", result);
                        if result > 0 {
                            self.hide_edit_param_value = true;
                            self.current_param_value = self.request_set_param_value.clone();
                        }
                        // self.request_set_param_value = String::new();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateParamValue(data) => self.request_set_param_value = data,
            Msg::Ignore => {}
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.param_id == ctx.props().param_data.param.param_id &&
              self.current_param_value == ctx.props().param_data.value {
            false
        } else {
            self.hide_edit_param_value = true;
            self.current_param_value = ctx.props().param_data.value.clone();
            self.request_set_param_value = ctx.props().param_data.value.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<>
            <ListErrors error={self.error.clone()}/>
            {self.modal_change_param_value(ctx.link())}
            {match self.get_result_delete {
                true => html!{},
                false => self.show_param(ctx.link(), ctx.props()),
            }}
        </>}
    }
}

impl ComponentParamTag {
    fn show_param(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_change_param = link.callback(|_| Msg::ChangeParamValue);
        let onclick_delete_param = link.callback(|_| Msg::RequestDeleteParam);

        html!{<tr>
            <td>{props.param_data.param.paramname.clone()}</td>
            <td>{self.current_param_value.clone()}</td>
            {match props.show_manage_btn {
                true => html!{<>
                    <td><a onclick={onclick_change_param.clone()}>
                        <span class="icon" >
                            <i class="fas fa-pen" aria-hidden="true"></i>
                        </span>
                    </a></td>
                    <td><a onclick={onclick_delete_param.clone()}>
                        <span class="icon" >
                          <i class="fa fa-trash" aria-hidden="true"></i>
                        </span>
                    </a></td>
                </>},
                false => html!{},
            }}
        </tr>}
    }

    fn modal_change_param_value(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_change_param_value = link.callback(|_| Msg::RequestChangeValue);
        let onclick_hide_modal = link.callback(|_| Msg::ChangeParamValue);
        let oninput_set_param_value =
            link.callback(|ev: InputEvent| Msg::UpdateParamValue(ev.input_type()));
        let class_modal = match &self.hide_edit_param_value {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_hide_modal.clone()} />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{ get_value_field(&211) }</p> // Changing the parameter value
                      <button class="delete" aria-label="close" onclick={onclick_hide_modal.clone()} />
                    </header>
                    <section class="modal-card-body">
                        <div class="column">
                            <label class="label">{ get_value_field(&133) }</label> // Set a value
                            <input
                                id="param-value"
                                class="input is-fullwidth"
                                type="text"
                                placeholder={get_value_field(&133)}
                                value={self.request_set_param_value.clone()}
                                oninput={oninput_set_param_value}
                                />
                        </div>
                        <div class="column">
                            <button
                                id="change-param-value"
                                class="button is-fullwidth"
                                disabled={self.request_set_param_value.is_empty() ||
                                    self.current_param_value == self.request_set_param_value}
                                onclick={onclick_change_param_value} >
                                { get_value_field(&59) } // Change
                            </button>
                        </div>
                      </section>
                  </div>
                </div>
              </div>
        }
    }
}
