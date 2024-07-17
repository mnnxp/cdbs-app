use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender, InputData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_save_btn;
use crate::types::{UUID, ComponentParam};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::component::{
    PutComponentParams, put_component_params,
    DeleteComponentParams, delete_component_params,
};

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
                let ipt_component_params_data = put_component_params::IptComponentParamsData{
                    componentUuid: self.props.component_uuid.clone(),
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
                match resp_parsing::<usize>(res, "deleteComponentParams") {
                    Ok(result) => {
                        debug!("deleteComponentParams: {:?}", result);
                        self.get_result_delete = result > 0;
                        if self.get_result_delete {
                            if let Some(delete_param) = &self.props.delete_param {
                                delete_param.emit(self.props.param_data.param.param_id);
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetChangeValueResult(res) => {
                match resp_parsing::<usize>(res, "putComponentParams") {
                    Ok(result) => {
                        debug!("putComponentParams: {:?}", result);
                        if result > 0 {
                            self.hide_edit_param_value = true;
                            self.current_param_value = self.request_set_param_value.clone();
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateParamValue(data) => self.request_set_param_value = data,
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
            self.request_set_param_value = props.param_data.value.clone();
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
        let onclick_change_param = self.link.callback(|_| Msg::ChangeParamValue);
        let onclick_delete_param = self.link.callback(|_| Msg::RequestDeleteParam);

        html!{<tr>
            <td>{self.props.param_data.param.paramname.clone()}</td>
            <td>{self.current_param_value.clone()}</td>
            {match self.props.show_manage_btn {
                true => html!{<>
                    <td><a onclick={onclick_change_param.clone()} title={get_value_field(&59)}>
                        <span class="icon" >
                            <i class="fas fa-pencil-alt" aria-hidden="true"></i>
                        </span>
                    </a></td>
                    <td><a onclick={onclick_delete_param.clone()} title={get_value_field(&135)}>
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
        let onclick_change_param_value = self.link.callback(|_| Msg::RequestChangeValue);
        let onclick_hide_modal = self.link.callback(|_| Msg::ChangeParamValue);
        let oninput_set_param_value = self.link.callback(|ev: InputData| Msg::UpdateParamValue(ev.value));
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
                      <p class="modal-card-title">{ get_value_field(&211) }</p> // Changing the parameter value
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <div class="column">
                            <label class="label">{ get_value_field(&133) }</label> // Set a value
                            <input
                                id="param-value"
                                class="input is-fullwidth"
                                type="text"
                                placeholder=get_value_field(&133)
                                value={self.request_set_param_value.clone()}
                                oninput=oninput_set_param_value
                                />
                        </div>
                        <div class="column">
                            {ft_save_btn(
                                "save-param-value",
                                onclick_change_param_value,
                                true,
                                self.request_set_param_value.is_empty() ||
                                    self.current_param_value == self.request_set_param_value
                            )}
                        </div>
                      </section>
                  </div>
                </div>
              </div>
        }
    }
}
