use std::collections::{HashMap, BTreeMap};
use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use web_sys::InputEvent;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use super::ModificationTableItemModule;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::fragments::component::param::RegisterParamnameBlock;
use crate::services::get_value_field;
use crate::types::{UUID, Param, ParamValue};
use crate::gqls::make_query;
use crate::gqls::relate::{GetParams, get_params};
use crate::gqls::component::{
    PutModificationParams, put_modification_params,
    DeleteModificationParams, delete_modification_params,
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_manage_btn: bool,
    pub modification_uuid: UUID,
    pub collect_heads: Vec<Param>,
    pub collect_item: HashMap<usize, String>,
    pub select_item: bool,
    pub open_modification_files: bool,
    #[prop_or_default]
    pub callback_new_modification_param: Option<Callback<UUID>>,
    #[prop_or_default]
    pub callback_select_modification: Option<Callback<UUID>>,
    #[prop_or_default]
    pub callback_open_modification_files: Option<Callback<()>>,
}

pub struct ModificationTableItem {
    error: Option<Error>,
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
    open_modification_files: bool,
    get_add_param_card: usize,
    get_change_param_card: usize,
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
    ShowModificationFilesList,
    UpdateValue(String),
    ShowNewParamCard,
    ShowAddParamCard(usize),
    ShowEditParamCard(usize),
    ClearError,
    Ignore,
}

impl Component for ModificationTableItem {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            modification_uuid: ctx.props().modification_uuid.clone(),
            collect_item: ctx.props().collect_item.clone(),
            select_item: ctx.props().select_item,
            params_list: BTreeMap::new(),
            request_add_param: ParamValue::default(),
            request_edit_param: ParamValue::default(),
            update_add_param: false,
            update_edit_param: false,
            open_new_param_card: false,
            open_add_param_card: false,
            open_edit_param_card: false,
            open_modification_files: ctx.props().open_modification_files,
            get_add_param_card: 0,
            get_change_param_card: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestParamsListData => {
                debug!("RequestParamsListData: {:?}", self.params_list);
                spawn_local(async move {
                    let res = make_query(GetParams::build_query(
                        get_params::Variables { ipt_param_arg: None }
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
                    param_id: self.request_add_param.param_id as i64,
                    value: self.request_add_param.value.clone(),
                };
                let ipt_modification_param_data = put_modification_params::IptModificationParamData{
                    modification_uuid: ctx.props().modification_uuid.clone(),
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
                    param_id: self.request_edit_param.param_id as i64,
                    value: self.request_edit_param.value.clone(),
                };
                let ipt_modification_param_data = put_modification_params::IptModificationParamData{
                    modification_uuid: ctx.props().modification_uuid.clone(),
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
                debug!("RequestDeleteParamData");
                let del_modification_param_data = delete_modification_params::DelModificationParamData{
                    modification_uuid: ctx.props().modification_uuid.clone(),
                    param_ids: vec![self.request_edit_param.param_id as i64],
                };
                spawn_local(async move {
                    let res = make_query(DeleteModificationParams::build_query(
                        delete_modification_params::Variables { del_modification_param_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteParamResult(res));
                })
            },
            Msg::GetParamsListResult(res) => {
                // debug!("GetParamsListResult: {:?}", res);
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<Param> = serde_json::from_value(
                            res_value.get("params").unwrap().clone()
                        ).unwrap();
                        for x in result.iter() {
                            self.params_list.insert(x.param_id, x.clone());
                        }
                        for y in ctx.props().collect_heads.iter() {
                            self.params_list.remove(&y.param_id);
                        }
                        debug!("params: {:?}", self.params_list);
                        if let Some((_, param)) = self.params_list.iter().next() {
                            // debug!("get first of params_list: {:?}", param);
                            self.request_add_param.param_id = param.param_id;
                        };
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetAddParamResult(res) => {
                debug!("GetAddParamResult: {:?}", res);
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.get_add_param_card = serde_json::from_value(
                            res_value.get("putModificationParams").unwrap().clone()
                        ).unwrap();
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
                                    if let Some(rollback) = &ctx.props().callback_new_modification_param {
                                        rollback.emit(ctx.props().modification_uuid.clone());
                                    }
                                },
                            }

                            self.request_add_param = ParamValue::default();
                        }
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUpdateParamResult(res) => {
                debug!("GetUpdateParamResult: {:?}", res);
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.get_change_param_card = serde_json::from_value(
                            res_value.get("putModificationParams").unwrap().clone()
                        ).unwrap();
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
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetDeleteParamResult(res) => {
                debug!("GetDeleteParamResult: {:?}", res);
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.get_change_param_card = serde_json::from_value(
                            res_value.get("deleteModificationParams").unwrap().clone()
                        ).unwrap();
                        debug!("deleteModificationParams: {:?}", self.get_change_param_card);
                        if self.get_change_param_card > 0 {
                            self.collect_item.remove(&self.request_edit_param.param_id);
                            self.request_edit_param.param_id = 0;
                            self.open_edit_param_card = false;
                        }
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::SelectModification => {
                if let Some(select_modification) = &ctx.props().callback_select_modification {
                    select_modification.emit(ctx.props().modification_uuid.clone());
                }
            },
            Msg::ShowModificationFilesList => {
                if let Some(open_files) = &ctx.props().callback_open_modification_files {
                    open_files.emit(());
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
                    if let Some(value) = self.collect_item.get(&param_id) {
                        self.request_edit_param.value = value.clone();
                    }
                    self.request_edit_param.param_id = param_id;
                    debug!("ShowEditParamCard {:?}", param_id);
                    debug!("Select modification uuid {:?}", self.modification_uuid);
                }
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.modification_uuid == ctx.props().modification_uuid &&
              self.select_item == ctx.props().select_item &&
                self.open_modification_files == ctx.props().open_modification_files {
            debug!("no change open_modification_files {:?}", self.open_modification_files);
            false
        } else {
            debug!("change open_modification_files {:?}", ctx.props().open_modification_files);
            self.open_modification_files = ctx.props().open_modification_files;
            self.modification_uuid = ctx.props().modification_uuid.clone();
            self.collect_item = ctx.props().collect_item.clone();
            self.select_item = ctx.props().select_item;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<>
            {self.modal_new_value(ctx.link())}
            {self.modal_add_value(ctx.link())}
            {self.modal_change_value(ctx.link())}
            {self.show_modification_row(ctx.link(), ctx.props())}
        </>}
    }
}

impl ModificationTableItem {
    fn show_modification_row(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_select_modification = link.callback(|_| Msg::SelectModification);
        let onclick_show_modification_files = link.callback(|_| Msg::ShowModificationFilesList);
        let class_style = match &props.select_item {
            true => "is-selected",
            false => "",
        };
        let files_click_icon = match &props.open_modification_files {
            true => "far fa-folder-open",
            false => "far fa-folder",
        };
        let (double_click_text, double_click_icon) = match &props.show_manage_btn {
            true => (get_value_field(&127), "fas fa-pencil-ruler"), // edit
            false => (get_value_field(&128), "fas fa-info"), // info
        };

        html!{<tr class={class_style}>
            <td>
                <a onclick={onclick_select_modification}>
                    {match &props.select_item {
                        true => html!{<>
                            <span>{double_click_text}</span>
                            <span class="icon">
                                <i class={double_click_icon} aria-hidden="true"></i>
                            </span>
                        </>},
                        false => html!{<>
                            <span>{ get_value_field(&129) }</span> // select
                            <span class="icon is-small">
                                <i class="far fa-hand-pointer" aria-hidden="true"></i>
                            </span>
                        </>},
                    }}
                </a>
                {match props.select_item && !props.show_manage_btn {
                    true => html!{<>
                        <span>{" | "}</span>
                        <a onclick={onclick_show_modification_files}>
                            // <span>{files_click_text}</span>
                            <span class="icon">
                                <i class={files_click_icon} aria-hidden="true"></i>
                            </span>
                        </a>
                    </>},
                    false => html!{},
                }}
            </td>
            {match self.collect_item.get(&0) {
                Some(value) => html!{<td>{value.clone()}</td>},
                None => html!{<td></td>},
            }}
            {self.show_items(link, props)}
        </tr>}
    }

    fn show_items(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_new_param_card = link.callback(|_| Msg::ShowNewParamCard);
        let onclick_add_param_card =
            link.callback(|value: usize| Msg::ShowAddParamCard(value));
        let onclick_edit_param_card =
            link.callback(|value: usize| Msg::ShowEditParamCard(value));

        match props.show_manage_btn {
            true => html!{<>
                {for props.collect_heads.iter().map(|param| {
                    match self.collect_item.get(&param.param_id) {
                        Some(value) => html!{<ModificationTableItemModule
                            param_id = {param.param_id}
                            value = {Some(value.clone())}
                            callback_change_param = {Some(onclick_edit_param_card.clone())}
                        />},
                        None => html!{<ModificationTableItemModule
                            param_id = {param.param_id}
                            // value = {None}
                            callback_change_param = {Some(onclick_add_param_card.clone())}
                        />},
                    }
                })}
                // for add new param
                <ModificationTableItemModule
                    param_id = {0}
                    // value = {None}
                    callback_change_param = {Some(onclick_new_param_card)}
                />
            </>},
            false => html!{<>
                {for props.collect_heads.iter().map(|param| {
                    match self.collect_item.get(&param.param_id) {
                        Some(value) => html!{<ModificationTableItemModule
                            param_id = {param.param_id}
                            value = {Some(value.clone())}
                            // callback_change_param = {None}
                        />},
                        None => html!{<ModificationTableItemModule
                            param_id = {param.param_id}
                            // value = {None}
                            // callback_change_param = {None}
                        />},
                    }
                })}
            </>},
        }
    }

    fn modal_new_value(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_clear_error = link.callback(|_| Msg::ClearError);
        let onclick_add_new_param =
            link.callback(|(param_id, param_value)| Msg::RequestAddNewParam(param_id, param_value));
        let onclick_close_param_card = link.callback(|_| Msg::ShowNewParamCard);
        let class_modal = match &self.open_new_param_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_close_param_card.clone()} />
          <div class="card">
            <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{ get_value_field(&130) }</p> // Add new parameter name
                    <button class="delete" aria-label="close" onclick={onclick_close_param_card} />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
                      <div class="media-content">
                          <RegisterParamnameBlock callback_add_param={onclick_add_new_param.clone()} />
                      </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }

    fn modal_add_value(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_clear_error = link.callback(|_| Msg::ClearError);
        let oninput_param_value = link.callback(|ev: InputEvent| Msg::UpdateValue(ev.input_type()));
        let onclick_close_add_param = link.callback(|_| Msg::ShowAddParamCard(0));
        let onclick_param_add = link.callback(|_| Msg::RequestAddParamData);
        let class_modal = match &self.open_add_param_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_close_add_param.clone()} />
          <div class="card">
            <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{ get_value_field(&131) }</p> // Add a parameter to modification
                    <button class="delete" aria-label="close" onclick={onclick_close_add_param} />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
                      <div class="media-content">
                          <label class="label">{ get_value_field(&133) }</label> // Set a value
                          <input
                              id="change-modification-param-value"
                              class="input is-fullwidth"
                              type="text"
                              placeholder={ get_value_field(&133) }
                              value={self.request_add_param.value.clone()}
                              oninput={oninput_param_value}  />
                      <br/>
                      <button
                          id="update-modification-param"
                          class="button is-fullwidth"
                          disabled={!self.update_add_param}
                          onclick={onclick_param_add} >
                          { get_value_field(&117) }
                      </button>
                    </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }

    fn modal_change_value(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_clear_error = link.callback(|_| Msg::ClearError);
        let oninput_param_value =
            link.callback(|ev: InputEvent| Msg::UpdateValue(ev.input_type()));
        let onclick_edit_param_card = link.callback(|_| Msg::ShowEditParamCard(0));
        let onclick_param_update = link.callback(|_| Msg::RequestUpdateParamData);
        let onclick_delete_param = link.callback(|_| Msg::RequestDeleteParamData);
        let class_modal = match &self.open_edit_param_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_edit_param_card.clone()} />
          <div class="card">
            <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{ get_value_field(&132) }</p> // Change the value
                    <button class="delete" aria-label="close" onclick={onclick_edit_param_card} />
                </header>
                <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
                <div class="box itemBox">
                  <article class="media center-media">
                      <div class="media-content">
                          <label class="label">{ get_value_field(&134) }</label> // Change value
                          <input
                              id="change-modification-param-value"
                              class="input is-fullwidth"
                              type="text"
                              placeholder={ get_value_field(&134) }
                              value={self.request_edit_param.value.clone()}
                              oninput={oninput_param_value}  />
                      <br/>
                      <div class="columns">
                          <div class="column">
                              <button
                                  id="delete-modification-param"
                                  class="button is-danger is-fullwidth"
                                  onclick={onclick_delete_param} >
                                  { get_value_field(&135) }
                              </button>
                          </div>
                          <div class="column">
                              <button
                                  id="update-modification-param"
                                  class="button is-fullwidth"
                                  disabled={!self.update_edit_param}
                                  onclick={onclick_param_update} >
                                  { get_value_field(&46) }
                              </button>
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
