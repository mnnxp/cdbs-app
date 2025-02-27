use std::collections::{HashMap, BTreeMap};
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender, InputData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::ModificationTableItemModule;
use crate::error::Error;
use crate::fragments::{
    buttons::{ft_delete_btn, ft_save_btn},
    list_errors::ListErrors,
    component::param::RegisterParamnameBlock,
};
use crate::services::{get_value_field, resp_parsing};
use crate::types::{UUID, Param, ParamValue};
use crate::gqls::{
    make_query,
    relate::{GetParams, get_params},
    component::{
        PutModificationParams, put_modification_params,
        DeleteModificationParams, delete_modification_params,
    },
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub modification_uuid: UUID,
    pub collect_heads: Vec<Param>,
    pub collect_item: HashMap<usize, String>,
    pub select_item: bool,
    pub callback_new_modification_param: Option<Callback<UUID>>,
    pub callback_select_modification: Option<Callback<UUID>>,
    pub ordinal_indicator: usize,
}

pub struct ModificationTableItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    modification_uuid: UUID,
    collect_item: HashMap<usize, String>,
    select_item: bool,
    params_list: BTreeMap<usize, Param>,
    request_add_param: ParamValue,
    request_edit_param: ParamValue,
    update_add_param: bool,
    update_edit_param: bool,
    open_new_param_card: bool,
    open_add_param_card: bool,
    open_edit_param_card: bool,
    get_add_param_card: usize,
    get_change_param_card: usize,
    get_confirm: usize,
}

pub enum Msg {
    RequestParamsListData,
    RequestAddNewParam(usize, String),
    RequestAddParamData,
    RequestUpdateParamData,
    RequestDeleteParamData,
    GetParamsListResult(String),
    GetAddParamResult(String),
    GetUpdateParamResult(String),
    GetDeleteParamResult(String),
    SelectModification,
    UpdateValue(String),
    ShowNewParamCard,
    ShowAddParamCard(usize),
    ShowEditParamCard(usize),
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for ModificationTableItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let modification_uuid = props.modification_uuid.clone();
        let collect_item = props.collect_item.clone();
        let select_item = props.select_item;
        Self {
            error: None,
            props,
            link,
            modification_uuid,
            collect_item,
            select_item,
            params_list: BTreeMap::new(),
            request_add_param: ParamValue::default(),
            request_edit_param: ParamValue::default(),
            update_add_param: false,
            update_edit_param: false,
            open_new_param_card: false,
            open_add_param_card: false,
            open_edit_param_card: false,
            get_add_param_card: 0,
            get_change_param_card: 0,
            get_confirm: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestParamsListData => {
                debug!("RequestParamsListData: {:?}", self.params_list);
                spawn_local(async move {
                    let res = make_query(GetParams::build_query(
                        get_params::Variables { param_ids: None, ipt_paginate: None }
                    )).await.unwrap();
                    link.send_message(Msg::GetParamsListResult(res));
                })
            },
            Msg::RequestAddNewParam(param_id, param_value) => {
                self.request_add_param.param_id = param_id;
                self.request_add_param.value = param_value;
                link.send_message(Msg::RequestAddParamData);
            },
            Msg::RequestAddParamData => {
                debug!("RequestAddParamData: {:?}", self.request_add_param);
                let ipt_param_data = put_modification_params::IptParamData{
                    paramId: self.request_add_param.param_id as i64,
                    value: self.request_add_param.value.clone(),
                };
                let ipt_modification_param_data = put_modification_params::IptModificationParamData{
                    modificationUuid: self.props.modification_uuid.clone(),
                    params: vec![ipt_param_data],
                };
                spawn_local(async move {
                    let res = make_query(PutModificationParams::build_query(
                        put_modification_params::Variables { ipt_modification_param_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetAddParamResult(res));
                })
            },
            Msg::RequestUpdateParamData => {
                debug!("RequestUpdateParamData: {:?}", self.request_add_param);
                let ipt_param_data = put_modification_params::IptParamData{
                    paramId: self.request_edit_param.param_id as i64,
                    value: self.request_edit_param.value.clone(),
                };
                let ipt_modification_param_data = put_modification_params::IptModificationParamData{
                    modificationUuid: self.props.modification_uuid.clone(),
                    params: vec![ipt_param_data],
                };
                spawn_local(async move {
                    let res = make_query(PutModificationParams::build_query(
                        put_modification_params::Variables { ipt_modification_param_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateParamResult(res));
                })
            },
            Msg::RequestDeleteParamData => {
                if self.get_confirm == self.request_edit_param.param_id {
                    debug!("RequestDeleteParamData");
                    let del_modification_param_data = delete_modification_params::DelModificationParamData{
                        modificationUuid: self.props.modification_uuid.clone(),
                        paramIds: vec![self.request_edit_param.param_id as i64],
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteModificationParams::build_query(
                            delete_modification_params::Variables { del_modification_param_data }
                        )).await.unwrap();
                        link.send_message(Msg::GetDeleteParamResult(res));
                    })
                } else {
                    self.get_confirm = self.request_edit_param.param_id;
                }
            },
            Msg::GetParamsListResult(res) => {
                match resp_parsing::<Vec<Param>>(res, "params") {
                    Ok(result) => {
                        for x in result.iter() {
                            self.params_list.insert(x.param_id, x.clone());
                        }
                        for y in self.props.collect_heads.iter() {
                            self.params_list.remove(&y.param_id);
                        }
                        debug!("params: {:?}", self.params_list);
                        if let Some((_, param)) = self.params_list.iter().next() {
                            self.request_add_param.param_id = param.param_id;
                        };
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetAddParamResult(res) => {
                debug!("GetAddParamResult: {:?}", res);
                match resp_parsing(res, "putModificationParams") {
                    Ok(result) => {
                        self.get_add_param_card = result;
                        debug!("putModificationParams: {:?}", self.get_add_param_card);
                        if self.get_add_param_card > 0 {
                            self.collect_item.insert(
                                self.request_add_param.param_id,
                                self.request_add_param.value.clone()
                            );
                            match self.open_add_param_card {
                                true => self.open_add_param_card = false,
                                false => {
                                    self.params_list.remove(&self.request_add_param.param_id);
                                    self.open_new_param_card = false;
                                    if let Some(rollback) = &self.props.callback_new_modification_param {
                                        rollback.emit(self.props.modification_uuid.clone());
                                    }
                                },
                            }
                            self.request_add_param = ParamValue::default();
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateParamResult(res) => {
                debug!("GetUpdateParamResult: {:?}", res);
                match resp_parsing(res, "putModificationParams") {
                    Ok(result) => {
                        self.get_change_param_card = result;
                        debug!("putModificationParams: {:?}", self.get_change_param_card);
                        if self.get_change_param_card > 0 {
                            self.collect_item.insert(
                                self.request_edit_param.param_id,
                                self.request_edit_param.value.clone()
                            );
                            self.request_edit_param = ParamValue::default();
                            self.open_edit_param_card = false;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetDeleteParamResult(res) => {
                debug!("GetDeleteParamResult: {:?}", res);
                match resp_parsing(res, "deleteModificationParams") {
                    Ok(result) => {
                        self.get_change_param_card = result;
                        debug!("deleteModificationParams: {:?}", self.get_change_param_card);
                        if self.get_change_param_card > 0 {
                            self.collect_item.remove(&self.request_edit_param.param_id);
                            self.request_edit_param.param_id = 0;
                            self.open_edit_param_card = false;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::SelectModification => {
                debug!("Callback ITEM, modification uuid: {:?}, self.props.m...uuid: {:?} (Show modifications)",
                    self.modification_uuid,
                    self.props.modification_uuid
                );
                if let Some(select_modification) = &self.props.callback_select_modification {
                    select_modification.emit(self.modification_uuid.clone());
                }
            },
            Msg::UpdateValue(data) => {
                match self.open_edit_param_card {
                    true => {
                        self.request_edit_param.value = data;
                        self.update_edit_param = true;
                    },
                    // also for open_new_param_card
                    false => {
                        self.request_add_param.value = data;
                        self.update_add_param = true;
                    },
                }
            },
            Msg::ShowNewParamCard => {
                self.open_new_param_card = !self.open_new_param_card;
                if self.open_new_param_card {
                    match self.params_list.is_empty() {
                        true => link.send_message(Msg::RequestParamsListData),
                        false => {
                            if let Some((_, param)) = self.params_list.iter().next() {
                                // debug!("get first of params_list: {:?}", param);
                                self.request_add_param.param_id = param.param_id;
                            }
                        },
                    }
                    self.request_add_param = ParamValue::default();
                    debug!("ShowNewParamCard");
                    debug!("Select modification uuid {:?}", self.modification_uuid);
                }
            },
            Msg::ShowAddParamCard(param_id) => {
                self.open_add_param_card = !self.open_add_param_card;
                if self.open_add_param_card {
                    self.request_add_param.param_id = param_id;
                    self.request_add_param.value = String::new();
                    debug!("ShowAddParamCard {:?}", param_id);
                    debug!("Select modification uuid {:?}", self.modification_uuid);
                }
            },
            Msg::ShowEditParamCard(param_id) => {
                self.open_edit_param_card = !self.open_edit_param_card;
                if self.open_edit_param_card {
                    // clear the check flags
                    self.get_confirm = 0;
                    self.update_edit_param = false;
                    if let Some(value) = self.collect_item.get(&param_id) {
                        self.request_edit_param.value = value.clone();
                    }
                    self.request_edit_param.param_id = param_id;
                    debug!("ShowEditParamCard {:?}", param_id);
                    debug!("Select modification uuid {:?}", self.modification_uuid);
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.modification_uuid == props.modification_uuid &&
              self.select_item == props.select_item {
            false
        } else {
            self.modification_uuid = props.modification_uuid.clone();
            self.collect_item = props.collect_item.clone();
            self.select_item = props.select_item;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{<>
            {self.modal_new_value()}
            {self.modal_add_value()}
            {self.modal_change_value()}
            {self.show_modification_row()}
        </>}
    }
}

impl ModificationTableItem {
    fn show_modification_row(&self) -> Html {
        let onclick_select_modification = self.link.callback(|_| Msg::SelectModification);
        let class_style = match &self.props.select_item {
            true => "is-selected",
            false => "",
        };
        let (title_text, click_icon) =
        match (&self.props.show_manage_btn, &self.props.select_item) {
            (true, true) => (get_value_field(&127), "fas fa-pencil-alt"),   // edit
            (false, true) => (get_value_field(&128), "fas fa-info"),        // info
            (_, false) => (get_value_field(&129), "far fa-hand-pointer"),   // select
        };

        html!{<tr class={class_style}>
            <th onclick={onclick_select_modification.clone()}>{self.props.ordinal_indicator}</th>
            <td onclick={onclick_select_modification}>
                <a title={title_text}>
                    <span class="icon">
                        <i class={click_icon} aria-hidden="true"></i>
                    </span>
                </a>
            </td>
            {match self.collect_item.get(&0) {
                Some(value) => html!{<td>{value.clone()}</td>},
                None => html!{<td></td>},
            }}
            {self.show_items()}
        </tr>}
    }

    fn show_items(&self) -> Html {
        let onclick_new_param_card = self.link.callback(|_| Msg::ShowNewParamCard);
        let onclick_add_param_card =
            self.link.callback(|value: usize| Msg::ShowAddParamCard(value));
        let onclick_edit_param_card =
            self.link.callback(|value: usize| Msg::ShowEditParamCard(value));

        match self.props.show_manage_btn {
            true => html!{<>
                {for self.props.collect_heads.iter().map(|param| {
                    match self.collect_item.get(&param.param_id) {
                        Some(value) => html!{<ModificationTableItemModule
                            param_id={param.param_id}
                            value={Some(value.clone())}
                            callback_change_param={Some(onclick_edit_param_card.clone())}
                        />},
                        None => html!{<ModificationTableItemModule
                            param_id={param.param_id}
                            value={None}
                            callback_change_param={Some(onclick_add_param_card.clone())}
                        />},
                    }
                })}
                // for add new param
                <ModificationTableItemModule
                    param_id={0}
                    value={None}
                    callback_change_param={Some(onclick_new_param_card)}
                />
            </>},
            false => html!{<>
                {for self.props.collect_heads.iter().map(|param| {
                    match self.collect_item.get(&param.param_id) {
                        Some(value) => html!{<ModificationTableItemModule
                            param_id={param.param_id}
                            value={Some(value.clone())}
                            callback_change_param={None}
                        />},
                        None => html!{<ModificationTableItemModule
                            param_id={param.param_id}
                            value={None}
                            callback_change_param={None}
                        />},
                    }
                })}
            </>},
        }
    }

    fn modal_new_value(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_add_new_param =
            self.link.callback(|(param_id, param_value)| Msg::RequestAddNewParam(param_id, param_value));
        let onclick_close_param_card = self.link.callback(|_| Msg::ShowNewParamCard);
        let class_modal = match &self.open_new_param_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_close_param_card.clone()} />
          <div class="card">
            <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{get_value_field(&130)}</p> // Add new parameter name
                    <button class="delete" aria-label="close" onclick={onclick_close_param_card} />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                      <div class="media-content">
                          <RegisterParamnameBlock callback_add_param={onclick_add_new_param.clone()} />
                      </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }

    fn modal_add_value(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let oninput_param_value = self.link.callback(|ev: InputData| Msg::UpdateValue(ev.value));
        let onclick_close_add_param = self.link.callback(|_| Msg::ShowAddParamCard(0));
        let onclick_param_add = self.link.callback(|_| Msg::RequestAddParamData);
        let class_modal = match &self.open_add_param_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_close_add_param.clone()} />
          <div class="card">
            <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{get_value_field(&131)}</p> // Add a parameter to modification
                    <button class="delete" aria-label="close" onclick={onclick_close_add_param} />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                      <div class="media-content">
                          <label class="label">{get_value_field(&133)}</label> // Set a value
                          <input
                              id="change-modification-param-value"
                              class="input is-fullwidth"
                              type="text"
                              placeholder={get_value_field(&133)}
                              value={self.request_add_param.value.clone()}
                              oninput={oninput_param_value} />
                      <br/>
                      {ft_save_btn(
                        "update-modification-param",
                        onclick_param_add,
                        true,
                        !self.update_add_param
                      )}
                    </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }

    fn modal_change_value(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let oninput_param_value = self.link.callback(|ev: InputData| Msg::UpdateValue(ev.value));
        let onclick_edit_param_card = self.link.callback(|_| Msg::ShowEditParamCard(0));
        let onclick_param_update = self.link.callback(|_| Msg::RequestUpdateParamData);
        let onclick_delete_param = self.link.callback(|_| Msg::RequestDeleteParamData);
        let class_modal = match &self.open_edit_param_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_edit_param_card.clone()} />
          <div class="card">
            <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{get_value_field(&132)}</p> // Change the value
                    <button class="delete" aria-label="close" onclick={onclick_edit_param_card} />
                </header>
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <div class="box itemBox">
                  <article class="media center-media">
                      <div class="media-content">
                          <label class="label">{get_value_field(&134)}</label> // Change value
                          <input
                              id="change-modification-param-value"
                              class="input is-fullwidth"
                              type="text"
                              placeholder={get_value_field(&134)}
                              value={self.request_edit_param.value.clone()}
                              oninput={oninput_param_value} />
                      <br/>
                      <div class="columns">
                        <div class="column">
                          {ft_delete_btn(
                              "delete-modification-param",
                              onclick_delete_param,
                              self.get_confirm == self.request_edit_param.param_id,
                              false
                          )}
                        </div>
                        <div class="column">
                          {ft_save_btn(
                            "update-modification-param",
                            onclick_param_update,
                            true,
                            !self.update_edit_param
                          )}
                        </div>
                      </div>
                    </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }
}
