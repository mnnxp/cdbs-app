use std::collections::{HashMap, BTreeSet};
use yew::{
    Callback, Component, ComponentLink, Html, Properties,
    ShouldRender, html, InputData, ChangeData
};
use chrono::NaiveDateTime;

use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use super::heads::ModificationTableHeads;
use super::item::ModificationTableItem;
// use crate::routes::AppRoute;
use crate::error::{get_error, Error};
use crate::gqls::make_query;
use crate::fragments::list_errors::ListErrors;
use crate::types::{
    UUID, ComponentModificationInfo, Param, ActualStatus,
    ModificationUpdatePreData,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct RegisterComponentModification;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct PutComponentModificationUpdate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct DeleteComponentModification;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetComponentModifications;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComponentActualStatuses;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub current_component_uuid: UUID,
    pub modifications: Vec<ComponentModificationInfo>,
    pub select_modification: UUID,
    pub callback_select_modification: Option<Callback<UUID>>,
    // pub callback_add_modification: Option<Callback<UUID>>,
}

pub struct ModificationsTableEdit {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    component_uuid: UUID,
    current_modifications: Vec<ComponentModificationInfo>,
    actual_statuses: Vec<ActualStatus>,
    collect_heads: Vec<Param>,
    collect_items: Vec<(UUID, HashMap<usize, String>)>,
    collect_columns: HashMap<usize, String>,
    valid_modification_uuids: BTreeSet<UUID>,
    request_add_modification: ModificationUpdatePreData,
    request_edit_modification: ModificationUpdatePreData,
    update_add_modification: bool,
    update_edit_modification: bool,
    open_add_modification_card: bool,
    open_edit_modification_card: bool,
    set_modification_uuid: UUID,
}

pub enum Msg {
    ParseParams,
    RequestAddModificationData,
    RequestUpdateModificationData,
    RequestDeleteModificationData,
    RequestComponentModificationsData,
    RequestListOptData,
    GetAddModificationResult(String),
    GetUpdateModificationResult(String),
    GetDeleteModificationResult(String),
    GetComponentModificationsResult(String),
    GetListOptResult(String),
    ResponseError(Error),
    UpdateAddName(String),
    UpdateAddDescription(String),
    UpdateAddActualStatusId(String),
    UpdateEditName(String),
    UpdateEditDescription(String),
    UpdateEditActualStatusId(String),
    ShowAddModificationCard,
    ShowEditModificationCard,
    ChangeNewModificationParam,
    ChangeSelectModification(UUID),
    SelectModification,
    ChangeModificationData,
    ClearError,
    Ignore,
}

impl Component for ModificationsTableEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let component_uuid = props.current_component_uuid.clone();
        let set_modification_uuid = props.select_modification.clone();
        let current_modifications = props.modifications.clone();
        Self {
            error: None,
            props,
            link,
            component_uuid,
            current_modifications,
            actual_statuses: Vec::new(),
            collect_heads: Vec::new(),
            collect_items: Vec::new(),
            collect_columns: HashMap::new(),
            valid_modification_uuids: BTreeSet::new(),
            request_add_modification: ModificationUpdatePreData::new(),
            request_edit_modification: ModificationUpdatePreData::default(),
            update_add_modification: false,
            update_edit_modification: false,
            open_add_modification_card: false,
            open_edit_modification_card: false,
            set_modification_uuid,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render || self.component_uuid != self.props.current_component_uuid {
            self.component_uuid = self.props.current_component_uuid.clone();
            debug!("Clear modification data");
            self.clear_current_data();
            self.link.send_message(Msg::ParseParams);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ParseParams => {
                // let mut first = true;
                let mut set_heads: Vec<usize> = vec![0];
                let mut collect_heads: Vec<Param> = Vec::new();

                for modification in &self.current_modifications {
                    self.valid_modification_uuids.insert(modification.uuid.clone());

                    self.collect_columns.clear();
                    self.collect_columns.insert(
                        0, modification.modification_name.clone(),
                    );
                    for modification_param in &modification.modification_params {
                        let mut flag = true;
                        // debug!("modification_param: {:?}", modification_param.param);
                        for head_id in &set_heads {
                            if head_id == &modification_param.param.param_id {
                                // debug!("head: {:?}", modification_param.param.param_id);
                                flag = false;
                                break;
                            }
                        }
                        if flag {
                            set_heads.push(modification_param.param.param_id);
                            collect_heads.push(modification_param.param.clone());
                        }
                        self.collect_columns.insert(
                            modification_param.param.param_id,
                            modification_param.value.clone(),
                        );
                        // debug!("collect_heads: {:?}", collect_heads);
                    }
                    // debug!("collect_columns: {:?}", self.collect_columns);
                    self.collect_items.push((
                        modification.uuid.clone(),
                        self.collect_columns.clone()
                    ));
                }
                // debug!("collect_items: {:?}", self.collect_items);
                self.collect_heads = collect_heads;
            },
            Msg::RequestAddModificationData => {
                let ipt_component_modification_data = register_component_modification::IptComponentModificationData{
                    componentUuid: self.props.current_component_uuid.clone(),
                    parentModificationUuid: None,
                    modificationName: self.request_add_modification.modification_name.clone(),
                    description: self.request_add_modification.description.clone(),
                    actualStatusId: self.request_add_modification.actual_status_id as i64,
                };
                spawn_local(async move {
                    let res = make_query(RegisterComponentModification::build_query(
                        register_component_modification::Variables { ipt_component_modification_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetAddModificationResult(res));
                })
            },
            Msg::RequestUpdateModificationData => {
                self.update_edit_modification = false;
                let modification_uuid = self.set_modification_uuid.clone();
                let ipt_update_component_modification_data = put_component_modification_update::IptUpdateComponentModificationData{
                    modificationName: match self.request_edit_modification.modification_name.is_empty() {
                        true => None,
                        false => Some(self.request_edit_modification.modification_name.clone())
                    },
                    description: match self.request_edit_modification.description.is_empty() {
                        true => None,
                        false => Some(self.request_edit_modification.description.clone())
                    },
                    actualStatusId: match self.request_edit_modification.actual_status_id == 0 {
                        true => None,
                        false => Some(self.request_edit_modification.actual_status_id as i64)
                    },
                };
                spawn_local(async move {
                    let res = make_query(PutComponentModificationUpdate::build_query(
                        put_component_modification_update::Variables {
                            modification_uuid,
                            ipt_update_component_modification_data
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateModificationResult(res));
                })
            },
            Msg::RequestDeleteModificationData => {
                let del_component_modification_data = delete_component_modification::DelComponentModificationData{
                    componentUuid: self.props.current_component_uuid.clone(),
                    modificationUuid: self.set_modification_uuid.clone(),
                };
                spawn_local(async move {
                    let res = make_query(DeleteComponentModification::build_query(
                        delete_component_modification::Variables {
                            del_component_modification_data
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteModificationResult(res));
                })
            },
            Msg::RequestComponentModificationsData => {
                let ipt_component_modification_arg = get_component_modifications::IptComponentModificationArg{
                    componentUuid: self.props.current_component_uuid.clone(),
                    limit: None,
                    offset: None,
                };

                spawn_local(async move {
                    let res = make_query(GetComponentModifications::build_query(
                        get_component_modifications::Variables { ipt_component_modification_arg }
                    )).await.unwrap();

                    link.send_message(Msg::GetComponentModificationsResult(res));
                })
            },
            Msg::RequestListOptData => {
                spawn_local(async move {
                    let res = make_query(ComponentActualStatuses::build_query(
                        component_actual_statuses::Variables { filter_int: None }
                    )).await.unwrap();
                    debug!("ComponentActualStatuses: {:?}", res);
                    link.send_message(Msg::GetListOptResult(res));
                });
            },
            Msg::GetAddModificationResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.set_modification_uuid = serde_json::from_value(
                            res_value.get("registerComponentModification").unwrap().clone()
                        ).unwrap();
                        self.open_add_modification_card = false;
                        // if let Some(select_modification) = &self.props.callback_add_modification {
                        //     select_modification.emit(self.set_modification_uuid.clone());
                        // }
                        link.send_message(Msg::RequestComponentModificationsData);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUpdateModificationResult(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res_value.get("putComponentModificationUpdate").unwrap().clone()).unwrap();
                        debug!("putComponentModificationUpdate: {:?}", result);
                        if result > 0 {
                            link.send_message(Msg::RequestComponentModificationsData);
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
                self.open_edit_modification_card = false;
            },
            Msg::GetDeleteModificationResult(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UUID = serde_json::from_value(res_value.get("deleteComponentModification").unwrap().clone()).unwrap();
                        debug!("deleteComponentModification: {:?}", result);
                        self.valid_modification_uuids.remove(&result);
                        self.set_modification_uuid = String::new();
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
                self.open_edit_modification_card = false;
            },
            Msg::GetComponentModificationsResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.current_modifications = serde_json::from_value(
                            res_value.get("componentModifications").unwrap().clone()
                        ).unwrap();
                        debug!("Update modifications list");
                        self.clear_current_data();
                        link.send_message(Msg::ParseParams);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetListOptResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.actual_statuses = serde_json::from_value(
                            res_value.get("componentActualStatuses").unwrap().clone()
                        ).unwrap();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::UpdateAddName(data) => {
                self.update_add_modification = !data.is_empty();
                self.request_add_modification.modification_name = data;
            },
            Msg::UpdateAddDescription(data) => {
                self.update_add_modification = !data.is_empty();
                self.request_add_modification.description = data;
            },
            Msg::UpdateAddActualStatusId(data) =>
                self.request_add_modification.actual_status_id = data.parse::<usize>().unwrap_or_default(),
            Msg::UpdateEditName(data) => {
                self.request_edit_modification.modification_name = data;
                self.update_edit_modification = true;
            },
            Msg::UpdateEditDescription(data) => {
                self.request_edit_modification.description = data;
                self.update_edit_modification = true;
            },
            Msg::UpdateEditActualStatusId(data) => {
                self.request_edit_modification.actual_status_id = data.parse::<usize>().unwrap_or_default();
                self.update_edit_modification = true;
            },
            Msg::ShowAddModificationCard => {
                self.open_add_modification_card = !self.open_add_modification_card;
                if self.actual_statuses.is_empty() {
                    link.send_message(Msg::RequestListOptData);
                }
            },
            Msg::ShowEditModificationCard => {
                debug!("open_edit_modification_card: {:?}", self.open_edit_modification_card);
                self.open_edit_modification_card = !self.open_edit_modification_card;
                if self.actual_statuses.is_empty() {
                    link.send_message(Msg::RequestListOptData);
                }
            },
            Msg::ChangeNewModificationParam => {
                debug!("Add new modification param");
                link.send_message(Msg::RequestComponentModificationsData);
            },
            Msg::ChangeSelectModification(modification_uuid) => {
                match self.set_modification_uuid == modification_uuid {
                    true => link.send_message(Msg::ShowEditModificationCard),
                    false => {
                        self.set_modification_uuid = modification_uuid;
                        link.send_message(Msg::SelectModification);
                    },
                }
            },
            Msg::SelectModification => {
                if let Some(select_modification) = &self.props.callback_select_modification {
                    select_modification.emit(self.set_modification_uuid.clone());
                }

                for current_modification in self.current_modifications.iter() {
                    if current_modification.uuid == self.set_modification_uuid {
                        self.request_edit_modification.modification_name = current_modification.modification_name.clone();
                        self.request_edit_modification.description = current_modification.description.clone();
                        self.request_edit_modification.actual_status_id = current_modification.actual_status.actual_status_id;
                        break;
                    }
                }
            },
            Msg::ChangeModificationData => {
                for modification in self.current_modifications.iter_mut() {
                    if modification.uuid == self.set_modification_uuid {
                        if self.request_edit_modification.modification_name.is_empty() {
                            modification.modification_name = self.request_edit_modification.modification_name.clone()
                        }

                        if self.request_edit_modification.description.is_empty() {
                            modification.description = self.request_edit_modification.description.clone()
                        }

                        if self.request_edit_modification.actual_status_id == 0 {
                            for actual_status in self.actual_statuses.iter() {
                                if actual_status.actual_status_id == self.request_edit_modification.actual_status_id {
                                    modification.actual_status = actual_status.clone();
                                    break;
                                }
                            }
                        }
                        break;
                    }
                }
            }
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.current_component_uuid == props.current_component_uuid {
            debug!("not update modifications {:?}", props.modifications.len());
            false
        } else {
            debug!("update modifications {:?}", props.modifications.len());
            self.current_modifications = props.modifications.clone();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_new_modification_param = self.link
            .callback(|_| Msg::ChangeNewModificationParam);

        let onclick_select_modification = self.link
            .callback(|value: UUID| Msg::ChangeSelectModification(value));

        let onclick_add_modification_card = self.link
            .callback(|_| Msg::ShowAddModificationCard);

        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<div class="card">
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {self.modal_add_modification_card()}
            {self.modal_edit_modification_card()}
            <div class="table-container">
              <table class="table is-fullwidth is-striped">
                <ModificationTableHeads
                  show_new_column = true
                  component_uuid = self.component_uuid.clone()
                  params = self.collect_heads.clone()
                />

                {for self.collect_items.iter().map(|(modification_uuid, item)|
                    match self.valid_modification_uuids.get(modification_uuid) {
                        Some(_) => html!{<ModificationTableItem
                            show_manage_btn = true
                            modification_uuid = modification_uuid.clone()
                            collect_heads = self.collect_heads.clone()
                            collect_item = item.clone()
                            select_item = &self.set_modification_uuid == modification_uuid
                            callback_new_modification_param = Some(onclick_new_modification_param.clone())
                            callback_select_modification = Some(onclick_select_modification.clone())
                        />},
                        None => html!{},
                    }
                 )}
              </table>
            </div>
            <button
                  id="add-component-modification"
                  class="button is-fullwidth"
                  onclick={onclick_add_modification_card} >
                <span class="icon" >
                    <i class="fas fa-plus" aria-hidden="true"></i>
                </span>
                <span>{"Add new modification"}</span>
            </button>
        </div>}
    }
}

impl ModificationsTableEdit {
    fn modal_add_modification_card(&self) -> Html {
        let oninput_name = self.link
            .callback(|ev: InputData| Msg::UpdateAddName(ev.value));

        let oninput_description = self.link
            .callback(|ev: InputData| Msg::UpdateAddDescription(ev.value));

        let onchange_actual_status_id = self.link
            .callback(|ev: ChangeData| Msg::UpdateAddActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let onclick_add_modification_card = self.link
            .callback(|_| Msg::ShowAddModificationCard);

        let onclick_add_component_modification = self.link
            .callback(|_| Msg::RequestAddModificationData);

        let class_modal = match &self.open_add_modification_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class=class_modal>
          <div class="modal-background" onclick=onclick_add_modification_card.clone() />
            <div class="card">
              <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{"Create new modification"}</p>
                    <button class="delete" aria-label="close" onclick=onclick_add_modification_card.clone() />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <div class="media-content">
                          <label class="label">{"Modification name"}</label>
                          <input
                              id="add-modification-name"
                              class="input is-fullwidth"
                              type="text"
                              placeholder="component name"
                              value={self.request_add_modification.modification_name.clone()}
                              oninput=oninput_name />
                          <label class="label">{"Description"}</label>
                          <textarea
                              id="add-modification-description"
                              class="textarea is-fullwidth"
                              // rows="10"
                              type="text"
                              placeholder="component description"
                              value={self.request_add_modification.description.clone()}
                              oninput=oninput_description />
                      <label class="label">{"Actual status"}</label>
                      <div class="select">
                          <select
                              id="add-modification-actual-status"
                              select={self.request_add_modification.actual_status_id.to_string()}
                              onchange=onchange_actual_status_id
                              >
                            { for self.actual_statuses.iter().map(|x|
                                match self.request_add_modification.actual_status_id == x.actual_status_id {
                                    true => html!{<option value={x.actual_status_id.to_string()} selected=true>{&x.name}</option>},
                                    false => html!{<option value={x.actual_status_id.to_string()}>{&x.name}</option>},
                                }
                            )}
                          </select>
                      </div>
                      <br/>
                      <button
                          id="add-component-modification"
                          class="button"
                          disabled={self.request_add_modification.modification_name.is_empty()}
                          onclick={onclick_add_component_modification} >
                          {"Add"}
                      </button>
                    </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }

    fn modal_edit_modification_card(&self) -> Html {
        let oninput_modification_name = self.link
            .callback(|ev: InputData| Msg::UpdateEditName(ev.value));

        let oninput_modification_description = self.link
            .callback(|ev: InputData| Msg::UpdateEditDescription(ev.value));

        let onchange_modification_actual_status_id = self.link
            .callback(|ev: ChangeData| Msg::UpdateEditActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let onclick_modification_card = self.link
            .callback(|_| Msg::ShowEditModificationCard);

        let onclick_delete_component_modification = self.link
            .callback(|_| Msg::RequestDeleteModificationData);

        let onclick_component_modification_update = self.link
            .callback(|_| Msg::RequestUpdateModificationData);

        let class_modal = match &self.open_edit_modification_card {
            true => "modal is-active",
            false => "modal",
        };

        let modification_data: Option<&ComponentModificationInfo> = self.current_modifications.iter()
            .find(|x| x.uuid == self.set_modification_uuid);

        match modification_data {
            Some(modification_data) => html!{<div class=class_modal>
              <div class="modal-background" onclick=onclick_modification_card.clone() />
                <div class="card">
                  <div class="modal-content">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Change modification data"}</p>
                        <button class="delete" aria-label="close" onclick=onclick_modification_card />
                    </header>
                    <div class="box itemBox">
                      <article class="media center-media">
                          <div class="media-content">
                              <label class="label">{"Modification name"}</label>
                              <input
                                  id="add-modification-name"
                                  class="input is-fullwidth"
                                  type="text"
                                  placeholder={modification_data.modification_name.clone()}
                                  value={self.request_edit_modification.modification_name.clone()}
                                  oninput=oninput_modification_name />
                              <label class="label">{"Description"}</label>
                              <textarea
                                  id="update-modification-description"
                                  class="textarea is-fullwidth"
                                  // rows="10"
                                  type="text"
                                  placeholder={modification_data.description.clone()}
                                  value={self.request_edit_modification.description.clone()}
                                  oninput=oninput_modification_description />
                          <label class="label">{"Actual status"}</label>
                          <div class="select">
                              <select
                                  id="update-modification-actual-status"
                                  select={modification_data.actual_status.actual_status_id.to_string()}
                                  onchange=onchange_modification_actual_status_id
                                  >
                                { for self.actual_statuses.iter().map(|x|
                                    match self.request_edit_modification.actual_status_id == x.actual_status_id {
                                        true => html!{<option value={x.actual_status_id.to_string()} selected=true>{&x.name}</option>},
                                        false => html!{<option value={x.actual_status_id.to_string()}>{&x.name}</option>},
                                    }
                                )}
                              </select>
                          </div>
                          <br/>
                          <div class="columns">
                              <div class="column">
                                  <button
                                      id="delete-component-modification"
                                      class="button is-danger"
                                      onclick={onclick_delete_component_modification} >
                                      {"Delete"}
                                  </button>
                              </div>
                              <div class="column">
                                  <button
                                      id="update-component-modification"
                                      class="button"
                                      disabled={!self.update_edit_modification}
                                      onclick={onclick_component_modification_update} >
                                      {"Update"}
                                  </button>
                              </div>
                          </div>
                        </div>
                      </article>
                    </div>
                  </div>
              </div>
            </div>},
            None => html!{},
        }
    }

    fn clear_current_data(&mut self) {
        self.collect_heads.clear();
        self.collect_items.clear();
        self.collect_columns.clear();
        self.valid_modification_uuids.clear();
        self.request_add_modification = ModificationUpdatePreData::new();
        self.request_edit_modification = ModificationUpdatePreData::default();
        self.update_add_modification = false;
        self.update_edit_modification = false;
    }
}