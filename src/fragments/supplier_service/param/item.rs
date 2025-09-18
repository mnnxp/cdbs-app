use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender, InputData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::{ft_save_btn, ft_delete_small_btn};
use crate::services::content_adapter::Markdownable;
use crate::types::{UUID, ServiceParam};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::supplier_service::{
    PutServiceParams, put_service_params,
    DeleteServiceParams, delete_service_params,
};

/// Param card for show data on service page
pub struct ServiceParamTag {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    current_param_value: String,
    request_set_param_value: String,
    hide_edit_param_value: bool,
    get_result_delete: bool,
    get_confirm: usize,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_manage_btn: bool,
    pub service_uuid: UUID,
    pub param_data: ServiceParam,
    pub ordinal_indicator: usize,
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
    ClearError,
}

impl Component for ServiceParamTag {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request_set_param_value = props.param_data.value.clone();
        ServiceParamTag {
            error: None,
            props,
            link,
            current_param_value: request_set_param_value.clone(),
            request_set_param_value,
            hide_edit_param_value: true,
            get_result_delete: false,
            get_confirm: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ChangeParamValue => self.hide_edit_param_value = !self.hide_edit_param_value,
            Msg::RequestChangeValue => {
                let ipt_param_data = put_service_params::IptParamData{
                    paramId: self.props.param_data.param.param_id as i64,
                    value: self.request_set_param_value.clone(),
                };
                let ipt_service_params_data = put_service_params::IptServiceParamsData{
                    serviceUuid: self.props.service_uuid.clone(),
                    params: vec![ipt_param_data],
                };
                spawn_local(async move {
                    let res = make_query(PutServiceParams::build_query(
                        put_service_params::Variables { ipt_service_params_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetChangeValueResult(res));
                })
            },
            Msg::RequestDeleteParam => {
                if self.get_confirm == self.props.param_data.param.param_id {
                    let service_uuid = self.props.service_uuid.clone();
                    let param_id = self.props.param_data.param.param_id as i64;
                    spawn_local(async move {
                        let del_service_param_data = delete_service_params::DelServiceParamData{
                            serviceUuid: service_uuid,
                            paramIds: vec![param_id],
                        };
                        let res = make_query(DeleteServiceParams::build_query(
                            delete_service_params::Variables { del_service_param_data }
                        )).await.unwrap();
                        link.send_message(Msg::GetDeleteParamResult(res));
                    })
                } else {
                    self.get_confirm = self.props.param_data.param.param_id;
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDeleteParamResult(res) => {
                match resp_parsing::<usize>(res, "deleteServiceParams") {
                    Ok(result) => {
                        debug!("deleteServiceParams: {:?}", result);
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
                match resp_parsing::<usize>(res, "putServiceParams") {
                    Ok(result) => {
                        debug!("putServiceParams: {:?}", result);
                        if result > 0 {
                            self.hide_edit_param_value = true;
                            self.current_param_value = self.request_set_param_value.clone();
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateParamValue(data) => self.request_set_param_value = data,
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.param_data.param.param_id == props.param_data.param.param_id &&
              self.props.param_data.value == props.param_data.value {
            false
        } else {
            self.hide_edit_param_value = true;
            self.get_confirm = 0;
            self.current_param_value = props.param_data.value.clone();
            self.request_set_param_value = props.param_data.value.clone();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
            {self.modal_change_param_value()}
            {match self.get_result_delete {
                true => html!{},
                false => self.show_param(),
            }}
        </>}
    }
}

impl ServiceParamTag {
    fn show_param(&self) -> Html {
        let onclick_change_param = self.link.callback(|_| Msg::ChangeParamValue);
        let onclick_delete_btn = self.link.callback(|_| Msg::RequestDeleteParam);

        html!{<tr>
            <th>{self.props.ordinal_indicator}</th>
            <td>{self.props.param_data.param.paramname.to_markdown()}</td>
            <td>{self.current_param_value.clone()}</td>
            {match self.props.show_manage_btn {
                true => html!{<>
                    <td>
                        <a onclick={onclick_change_param} title={get_value_field(&59)}>
                            <span class="icon" >
                                <i class="fas fa-pencil-alt" aria-hidden="true"></i>
                            </span>
                        </a>
                    </td>
                    <td>
                        {ft_delete_small_btn(
                            "service-param-delete",
                            onclick_delete_btn,
                            self.get_confirm == self.props.param_data.param.param_id,
                        )}
                    </td>
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
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_hide_modal.clone()} />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{get_value_field(&211)}</p> // Changing the parameter value
                      <button class="delete" aria-label="close" onclick={onclick_hide_modal.clone()} />
                    </header>
                    <section class="modal-card-body">
                        <div class="column">
                            <label class="label">{get_value_field(&133)}</label> // Set a value
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
