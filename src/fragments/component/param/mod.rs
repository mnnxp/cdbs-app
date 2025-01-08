mod item;
mod add;

pub use item::ComponentParamTag;
pub use add::RegisterParamnameBlock;

use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::buttons::ft_add_btn;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::paginate::Paginate;
use crate::types::{ComponentParam, PaginateSet, Param, UUID};
use crate::services::{get_classes_table, get_value_field, resp_parsing, resp_parsing_two_level};
use crate::gqls::{
    make_query,
    relate::{GetParams, get_params},
    component::{
        PutComponentParams, put_component_params,
        GetComponentParams, get_component_params,
    },
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub component_uuid: UUID,
    pub params_count: i64,
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
    page_set: PaginateSet,
    current_items: i64,
    total_items: i64,
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
    ResponseError(Error),
    ChangePaginate(PaginateSet),
    ClearError,
}

impl Component for ComponentParamsTags {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let total_items = props.params_count;
        Self {
            error: None,
            props,
            link,
            param_ids: BTreeSet::new(),
            component_params: Vec::new(),
            param_list: Vec::new(),
            request_add_param_id: 0,
            request_set_param_value: String::new(),
            hide_add_param_modal: true,
            page_set: PaginateSet::new(),
            current_items: 0,
            total_items,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            debug!("First bulild params table");
            self.link.send_message(Msg::RequestComponentParams);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::DeleteComponentParam(param_id) => {
                self.param_ids.remove(&param_id);
                self.total_items -= 1;
                self.current_items -= 1;
                link.send_message(Msg::SetSelectParam);
            },
            Msg::RequestParamsList => {
                spawn_local(async move {
                    let res = make_query(GetParams::build_query(
                        get_params::Variables { param_ids: None, ipt_paginate: None }
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
                if self.props.component_uuid.len() != 36 {
                    return true
                }
                let component_uuid = self.props.component_uuid.clone();
                let ipt_paginate = Some(get_component_params::IptPaginate {
                    currentPage: self.page_set.current_page,
                    perPage: self.page_set.per_page,
                });
                spawn_local(async move {
                    let res = make_query(GetComponentParams::build_query(
                        get_component_params::Variables { component_uuid, ipt_sort: None, ipt_paginate }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentParamsResult(res));
                })
            },
            Msg::GetParamsListResult(res) => {
                match resp_parsing::<Vec<Param>>(res, "params") {
                    Ok(result) => {
                        debug!("params: {:?}", result);
                        self.param_list = result;
                        link.send_message(Msg::SetSelectParam);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetAddParamResult(res) => {
                match resp_parsing::<usize>(res, "putComponentParams") {
                    Ok(result) => {
                        debug!("putComponentParams: {:?}", result);
                        self.hide_add_param_modal = result > 0;
                        self.total_items += 1;
                        self.request_set_param_value = String::new();
                        link.send_message(Msg::RequestComponentParams);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetComponentParamsResult(res) => {
                match resp_parsing_two_level(res, "component", "componentParams") {
                    Ok(result) => {
                        debug!("componentParams: {:?}", result);
                        self.component_params = result;
                        self.param_ids = BTreeSet::new();
                        for param in &self.component_params {
                            self.param_ids.insert(param.param.param_id);
                        };
                        link.send_message(Msg::SetSelectParam);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
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
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ChangePaginate(page_set) => {
                self.page_set = page_set;
                self.link.send_message(Msg::RequestComponentParams);
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid {
            false
        } else {
            self.hide_add_param_modal = true;
            self.param_ids = BTreeSet::new();
            self.total_items = props.params_count;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));
        html!{<>
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">
                        {match self.props.show_manage_btn {
                            true => get_value_field(&185),  // Manage component characteristics
                            false => get_value_field(&101), // Сharacteristics of the component
                        }}
                    </p>
                </header>
                <div class="card-content">
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {self.modal_add_param()}
            {self.show_params()}
            <Paginate
                callback_change={onclick_paginate}
                current_items={self.current_items}
                current_page={Some(self.page_set.current_page)}
                per_page={Some(self.page_set.per_page)}
                total_items={self.total_items}
                />
            {match self.props.show_manage_btn {
                true => html!{
                    <footer class="card-footer">
                        {ft_add_btn(
                            "add-param-component",
                            get_value_field(&180),
                            self.link.callback(|_| Msg::ChangeHideAddParam),
                            true,
                            false
                        )}
                    </footer>
                },
                false => html!{},
            }}
        </div>
    </div>
        </>}
    }
}

impl ComponentParamsTags {
    fn show_params(&self) -> Html {
        let onclick_delete_param = match self.props.show_manage_btn {
            true => Some(self.link.callback(|value: usize| Msg::DeleteComponentParam(value))),
            false => None,
        };
        let classes_table = get_classes_table(self.component_params.len());
        let numero_offset = self.page_set.numero_offset();

        html!{
            <div class={"content"}>
            <div class={"table-container"}>
                <table class={classes_table}>
                    <thead>
                        <tr>
                            <th>{"\u{2116}"}</th> // Numero sign
                            <th>{get_value_field(&178)}</th> // Param
                            <th>{get_value_field(&179)}</th> // Value
                            {match self.props.show_manage_btn {
                                true => html!{<>
                                    <th>{get_value_field(&59)}</th> // Change
                                    <th>{get_value_field(&135)}</th> // Delete
                                </>},
                                false => html!{},
                            }}
                        </tr>
                    </thead>
                    <tbody>
                        {for self.component_params.iter().enumerate().map(|(numer, data)| {
                            match self.param_ids.get(&data.param.param_id) {
                                Some(_) => html!{
                                    <ComponentParamTag
                                        show_manage_btn={self.props.show_manage_btn}
                                        component_uuid={self.props.component_uuid.clone()}
                                        param_data={data.clone()}
                                        ordinal_indicator={numero_offset+numer}
                                        delete_param={onclick_delete_param.clone()}
                                        />
                                },
                                None => html!{},
                            }
                        })}
                    </tbody>
                </table>
            </div>
            </div>
        }
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
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_hide_modal.clone()} />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{get_value_field(&181)}</p> // Add a parameter to component
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
