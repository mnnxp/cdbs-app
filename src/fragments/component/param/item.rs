use yew::{
    html, Callback, Component, ComponentLink,
    Html, Properties, ShouldRender, InputData
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::gqls::make_query;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ComponentParam};

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
struct DeleteComponentParams;

/// Param card for show data on component page
pub struct ComponentParamTag {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    current_param_value: String,
    request_set_param_value: String,
    hide_edit_param_value: bool,
    get_result_delete: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_manage_btn: bool,
    pub component_uuid: UUID,
    pub param_data: ComponentParam,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request_set_param_value = props.param_data.value.clone();
        ComponentParamTag {
            error: None,
            props,
            link,
            current_param_value: request_set_param_value.clone(),
            request_set_param_value,
            hide_edit_param_value: true,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ChangeParamValue => self.hide_edit_param_value = !self.hide_edit_param_value,
            Msg::RequestChangeValue => {
                let ipt_param_data = put_component_params::IptParamData{
                    paramId: self.props.param_data.param.param_id as i64,
                    value: self.request_set_param_value.clone(),
                };
                let ipt_component_param_data = put_component_params::IptComponentParamData{
                    componentUuid: self.props.component_uuid.clone(),
                    params: vec![ipt_param_data],
                };
                spawn_local(async move {
                    let res = make_query(PutComponentParams::build_query(
                        put_component_params::Variables { ipt_component_param_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetChangeValueResult(res));
                })
            },
            Msg::RequestDeleteParam => {
                let component_uuid = self.props.component_uuid.clone();
                let param_id = self.props.param_data.param.param_id as i64;
                spawn_local(async move {
                    let del_component_param_data = delete_component_params::DelComponentParamData{
                        componentUuid: component_uuid,
                        paramIds: vec![param_id],
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
                        match &self.props.delete_param {
                            Some(delete_param) => {
                                if result > 0 {
                                    self.get_result_delete = true;
                                    self.hide_edit_param_value = true;
                                    delete_param.emit(self.props.param_data.param.param_id);
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.param_data.param.param_id == props.param_data.param.param_id &&
              self.props.param_data.value == props.param_data.value {
            false
        } else {
            self.hide_edit_param_value = true;
            self.current_param_value = props.param_data.value.clone();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{<>
            <ListErrors error=self.error.clone()/>
            {self.modal_change_param_value()}
            {match self.get_result_delete {
                true => html!{},
                false => self.show_param(),
            }}
        </>}
    }
}

impl ComponentParamTag {
    fn show_param(&self) -> Html {
        let onclick_change_param = self.link
            .callback(|_| Msg::ChangeParamValue);

        let onclick_delete_param = self.link
            .callback(|_| Msg::RequestDeleteParam);

        html!{<tr>
            <td>{self.props.param_data.param.paramname.clone()}</td>
            <td>{self.current_param_value.clone()}</td>
            {match self.props.show_manage_btn {
                true => html!{<>
                    <td><a onclick={onclick_change_param.clone()}>{"change"}</a></td>
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

    fn modal_change_param_value(&self) -> Html {
        let onclick_change_param_value = self.link
            .callback(|_| Msg::RequestChangeValue);

        let onclick_hide_modal = self.link
            .callback(|_| Msg::ChangeParamValue);

        let oninput_set_param_value = self.link
            .callback(|ev: InputData| Msg::UpdateParamValue(ev.value));

        let class_modal = match &self.hide_edit_param_value {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_hide_modal.clone() />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{"Change param value"}</p>
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    // <label class="label">{"Set value"}</label>
                    <textarea
                        id="param-value"
                        class="textarea"
                        // rows="10"
                        type="text"
                        placeholder="param value"
                        value={self.request_set_param_value.clone()}
                        oninput=oninput_set_param_value
                        />
                    <button
                        id="change-param-value"
                        class="button"
                        disabled={self.request_set_param_value.is_empty() ||
                            self.current_param_value == self.request_set_param_value}
                        onclick={onclick_change_param_value} >
                        {"Change"}
                    </button>
                  </div>
                </div>
              </div>
        }
    }
}
