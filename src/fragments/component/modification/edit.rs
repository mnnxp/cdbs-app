use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html, InputData, ChangeData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::file::ManageModificationFilesCard;
use super::table::ModificationsTable;
use super::fileset::ManageModificationFilesets;
use crate::error::Error;
use crate::fragments::paginate::Paginate;
use crate::fragments::buttons::{ft_delete_btn, ft_save_btn, ft_add_btn, ft_back_btn};
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_value_field, resp_parsing};
use crate::services::content_adapter::DateDisplay;
use crate::types::{UUID, ComponentModificationInfo, ActualStatus, ModificationUpdatePreData, PaginateSet};
use crate::gqls::make_query;
use crate::gqls::component::{
    RegisterComponentModification, register_component_modification,
    PutComponentModificationUpdate, put_component_modification_update,
    DeleteComponentModification, delete_component_modification,
    GetComponentModifications, get_component_modifications,
    ComponentActualStatuses, component_actual_statuses,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub current_component_uuid: UUID,
    pub modifications_count: i64,
}

pub enum ActiveTab {
    Data,
    ModificationFiles,
    Fileset
}

pub struct ModificationsTableEdit {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    current_modifications: Vec<ComponentModificationInfo>,
    select_modification_uuid: UUID,
    actual_statuses: Vec<ActualStatus>,
    invalid_modification_uuids: BTreeSet<UUID>,
    request_add_modification: ModificationUpdatePreData,
    request_edit_modification: ModificationUpdatePreData,
    update_add_modification: bool,
    modification_changed: bool,
    open_add_modification_card: bool,
    open_edit_modification_card: bool,
    get_confirm: UUID,
    change_page: bool,
    skip_change_page: bool,
    page_set: PaginateSet,
    current_items: i64,
    total_items: i64,
    active_tab: ActiveTab,
}

pub enum Msg {
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
    ChangeNewModificationParam(UUID),
    ChangeSelectModification(UUID),
    ChangeActiveTab(ActiveTab),
    UpdateSelectModification,
    ChangeModificationData,
    ChangePaginate(PaginateSet),
    ClearError,
}

impl Component for ModificationsTableEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let total_items = props.modifications_count;
        Self {
            error: None,
            props,
            link,
            current_modifications: Vec::new(),
            select_modification_uuid: String::new(),
            actual_statuses: Vec::new(),
            invalid_modification_uuids: BTreeSet::new(),
            request_add_modification: ModificationUpdatePreData::new(),
            request_edit_modification: ModificationUpdatePreData::default(),
            update_add_modification: false,
            modification_changed: false,
            open_add_modification_card: false,
            open_edit_modification_card: false,
            get_confirm: String::new(),
            change_page: false,
            skip_change_page: false,
            page_set: PaginateSet::new(),
            current_items: 0,
            total_items,
            active_tab: ActiveTab::Data,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestComponentModificationsData);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
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
                self.modification_changed = false;
                let modification_uuid = self.select_modification_uuid.clone();
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
                if self.get_confirm == self.select_modification_uuid {
                    let del_component_modification_data = delete_component_modification::DelComponentModificationData{
                        componentUuid: self.props.current_component_uuid.clone(),
                        modificationUuid: self.select_modification_uuid.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteComponentModification::build_query(
                            delete_component_modification::Variables {
                                del_component_modification_data
                            }
                        )).await.unwrap();
                        link.send_message(Msg::GetDeleteModificationResult(res));
                    })
                } else {
                    self.get_confirm = self.select_modification_uuid.clone();
                }
            },
            Msg::RequestComponentModificationsData => {
                let component_uuid = self.props.current_component_uuid.clone();
                let ipt_sort = Some(get_component_modifications::IptSort {
                    byField: "name".to_string(),
                    asDesc: false,
                });
                let ipt_paginate = Some(get_component_modifications::IptPaginate {
                    currentPage: self.page_set.current_page,
                    perPage: self.page_set.per_page,
                });
                spawn_local(async move {
                    let res = make_query(GetComponentModifications::build_query(
                        get_component_modifications::Variables {
                            component_uuid,
                            ipt_sort,
                            ipt_paginate
                        }
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
                match resp_parsing(res, "registerComponentModification") {
                    Ok(result) => {
                        self.select_modification_uuid = result;
                        self.open_add_modification_card = false;
                        self.total_items += 1;
                        link.send_message(Msg::RequestComponentModificationsData);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateModificationResult(res) => {
                match resp_parsing::<usize>(res, "putComponentModificationUpdate") {
                    Ok(result) => debug!("putComponentModificationUpdate: {:?}", result),
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                // clear the check flags
                self.get_confirm.clear();
                self.modification_changed = false;
            },
            Msg::GetDeleteModificationResult(res) => {
                match resp_parsing::<UUID>(res, "deleteComponentModification") {
                    Ok(result) => {
                        debug!("deleteComponentModification: {:?}", result);
                        self.invalid_modification_uuids.insert(result);
                        self.total_items -= 1;
                        self.current_items -= 1;
                        self.select_modification_uuid.clear();
                        for m in &self.current_modifications {
                            if let None = self.invalid_modification_uuids.get(&m.uuid) {
                                self.select_modification_uuid = m.uuid.clone();
                                break;
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.open_edit_modification_card = false;
            },
            Msg::GetComponentModificationsResult(res) => {
                match resp_parsing(res, "componentModifications") {
                    Ok(result) => {
                        self.clear_current_data();
                        self.current_modifications = result;
                        self.current_items = self.current_modifications.len() as i64;
                        self.change_page = false;
                        // do not change the selected modification if the old one is in the list
                        let mut change_select = true;
                        for cm in &self.current_modifications {
                            if self.select_modification_uuid == cm.uuid {
                                change_select = false;
                                break;
                            }
                        }
                        if change_select {
                            self.select_modification_uuid = self.current_modifications.first().map(|m| m.uuid.clone()).unwrap_or_default();
                        }
                        debug!("Update modifications list");
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetListOptResult(res) => {
                match resp_parsing(res, "componentActualStatuses") {
                    Ok(result) => self.actual_statuses = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
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
                self.modification_changed = true;
            },
            Msg::UpdateEditDescription(data) => {
                self.request_edit_modification.description = data;
                self.modification_changed = true;
            },
            Msg::UpdateEditActualStatusId(data) => {
                self.request_edit_modification.actual_status_id = data.parse::<usize>().unwrap_or_default();
                self.modification_changed = true;
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
                if self.open_edit_modification_card {
                    link.send_message(Msg::UpdateSelectModification);
                } else {
                    link.send_message(Msg::RequestComponentModificationsData);
                }
            },
            Msg::ChangeNewModificationParam(modification_uuid) => {
                debug!("Add new modification parameter name");
                link.send_message(Msg::RequestComponentModificationsData);
                self.select_modification_uuid = modification_uuid;
            },
            Msg::ChangeSelectModification(modification_uuid) => {
                debug!("Callback EDIT CARD, modification uuid set: {:?}, old: {:?} (Show modifications)",
                    modification_uuid,
                    self.select_modification_uuid,
                );
                match self.select_modification_uuid == modification_uuid {
                    true => {
                        self.skip_change_page = true;
                        link.send_message(Msg::ShowEditModificationCard);
                    },
                    false => {
                        self.select_modification_uuid = modification_uuid;
                        link.send_message(Msg::ChangeActiveTab(ActiveTab::Data));
                    },
                }
            },
            Msg::ChangeActiveTab(set_tab) => self.active_tab = set_tab,
            Msg::UpdateSelectModification => {
                // clear the check flags
                self.get_confirm.clear();
                self.modification_changed = false;
                for current_modification in self.current_modifications.iter() {
                    if current_modification.uuid == self.select_modification_uuid {
                        self.request_edit_modification.modification_name = current_modification.modification_name.clone();
                        self.request_edit_modification.description = current_modification.description.clone();
                        self.request_edit_modification.actual_status_id = current_modification.actual_status.actual_status_id;
                        break;
                    }
                }
            },
            Msg::ChangeModificationData => {
                for modification in self.current_modifications.iter_mut() {
                    if modification.uuid == self.select_modification_uuid {
                        if self.request_edit_modification.modification_name.is_empty() {
                            modification.modification_name = self.request_edit_modification.modification_name.clone();
                        }
                        if self.request_edit_modification.description.is_empty() {
                            modification.description = self.request_edit_modification.description.clone();
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
            },
            Msg::ChangePaginate(page_set) => {
                debug!("Change page_set, old: {:?}, new: {:?} (Show modifications)", self.page_set, page_set);
                if self.skip_change_page {
                    debug!("Skip change page after return from modification card");
                    self.skip_change_page = false;
                    return true
                }
                self.page_set = page_set;
                if self.props.current_component_uuid.len() == 36 {
                    self.change_page = true;
                    self.link.send_message(Msg::RequestComponentModificationsData);
                }
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.current_component_uuid == props.current_component_uuid {
            false
        } else {
            self.total_items = props.modifications_count;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_modification_card = self.link.callback(|_| Msg::ShowEditModificationCard);
        html!{
            <div class="card">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <header class="card-header">
                    <p class="card-header-title">
                        {match &self.open_edit_modification_card {
                            true => ft_back_btn("open-modifications", onclick_modification_card, get_value_field(&115)),
                            false => html!{get_value_field(&100)} // Modifications,
                        }}
                    </p>
                </header>
                {match self.open_edit_modification_card {
                    true => self.show_modification_tabs(),
                    false => self.show_modifications_table(),
                }}
            </div>
        }
    }
}

impl ModificationsTableEdit {
    fn show_modifications_table(&self) -> Html {
        let onclick_new_modification_param = self.link.callback(|value: UUID| Msg::ChangeNewModificationParam(value));
        let onclick_add_modification_card = self.link.callback(|_| Msg::ShowAddModificationCard);
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));
        let onclick_select_modification = self.link.callback(|value: UUID| Msg::ChangeSelectModification(value));
        let mut modifications = Vec::new();
        for m in &self.current_modifications {
            if let None = self.invalid_modification_uuids.get(&m.uuid) {
                modifications.push(m.clone());
            }
        }
        html!{
            <div class="card-content">
                {self.modal_add_modification_card()}
                <ModificationsTable
                    modifications={modifications}
                    select_modification_uuid={self.select_modification_uuid.clone()}
                    callback_select_modification={onclick_select_modification}
                    callback_new_modification_param={Some(onclick_new_modification_param)}
                />
                <Paginate
                    callback_change={onclick_paginate}
                    current_items={self.current_items}
                    current_page={Some(self.page_set.current_page)}
                    per_page={Some(self.page_set.per_page)}
                    total_items={self.total_items}
                />
                <footer class="card-footer">
                    {ft_add_btn(
                        "add-component-modification",
                        get_value_field(&174),
                        onclick_add_modification_card,
                        true,
                        false
                    )}
                </footer>
            </div>
        }
    }

    fn modal_add_modification_card(&self) -> Html {
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateAddName(ev.value));
        let oninput_description = self.link.callback(|ev: InputData| Msg::UpdateAddDescription(ev.value));
        let onchange_actual_status_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateAddActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let onclick_add_modification_card = self.link.callback(|_| Msg::ShowAddModificationCard);
        let onclick_add_component_modification = self.link.callback(|_| Msg::RequestAddModificationData);
        let class_modal = match &self.open_add_modification_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_add_modification_card.clone()} />
            <div class="card">
              <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{get_value_field(&175)}</p> // Create new modification
                    <button class="delete" aria-label="close" onclick={onclick_add_modification_card.clone()} />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <div class="media-content">
                          <div class="column">
                              <label class="label">{get_value_field(&176)}</label> // Modification name
                              <input
                                  id="add-modification-name"
                                  class="input is-fullwidth"
                                  type="text"
                                  placeholder={get_value_field(&176)}
                                  value={self.request_add_modification.modification_name.clone()}
                                  oninput={oninput_name} />
                          </div>
                          <div class="column">
                              <label class="label">{get_value_field(&61)}</label>
                              <textarea
                                  id="add-modification-description"
                                  class="textarea is-fullwidth"
                                  // rows="10"
                                  type="text"
                                  placeholder={get_value_field(&61)}
                                  value={self.request_add_modification.description.clone()}
                                  oninput={oninput_description} />
                          </div>
                          <div class="column">
                              <label class="label">{get_value_field(&96)}</label>
                              <div class="select">
                                  <select
                                      id="add-modification-actual-status"
                                      select={self.request_add_modification.actual_status_id.to_string()}
                                      onchange={onchange_actual_status_id}
                                      >
                                    { for self.actual_statuses.iter().map(|x|
                                        html!{
                                            <option value={x.actual_status_id.to_string()}
                                                  selected={x.actual_status_id == self.request_add_modification.actual_status_id} >
                                                {&x.name}
                                            </option>
                                        }
                                    )}
                                  </select>
                              </div>
                          </div>
                          <div class="column">
                              {ft_save_btn(
                                "add-component-modification",
                                onclick_add_component_modification,
                                true,
                                self.request_add_modification.modification_name.is_empty()
                              )}
                          </div>
                    </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }

    fn show_modification_tabs(&self) -> Html {
        let onclick_tab_data = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::Data));
        let onclick_tab_files = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::ModificationFiles));
        let onclick_tab_fileset = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::Fileset));
        let at = match self.active_tab {
            ActiveTab::Data => ("is-active","",""),
            ActiveTab::ModificationFiles => ("","is-active",""),
            ActiveTab::Fileset => ("","","is-active"),
        };
        html!{<>
            <div class="tabs is-centered">
                <ul>
                    <li class={at.0} onclick={onclick_tab_data}><a>{get_value_field(&177)}</a></li>
                    <li class={at.1} onclick={onclick_tab_files}><a>{get_value_field(&172)}</a></li>
                    <li class={at.2} onclick={onclick_tab_fileset}><a>{get_value_field(&173)}</a></li>
                </ul>
            </div>
            <div class="card-content">
                {match self.active_tab {
                    ActiveTab::Data => self.show_modification_card(),
                    ActiveTab::ModificationFiles => html!{
                        <div class="content">
                            <ManageModificationFilesCard
                                show_download_btn={false}
                                modification_uuid={self.select_modification_uuid.clone()}
                                />
                        </div>
                    },
                    ActiveTab::Fileset => html!{
                        <div class="content">
                            <ManageModificationFilesets select_modification_uuid={self.select_modification_uuid.clone()} />
                        </div>
                    },
                }}
            </div>
        </>}
    }

    fn show_modification_card(&self) -> Html {
        let oninput_modification_name = self.link.callback(|ev: InputData| Msg::UpdateEditName(ev.value));
        let oninput_modification_description = self.link.callback(|ev: InputData| Msg::UpdateEditDescription(ev.value));
        let onchange_modification_actual_status_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateEditActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let onclick_delete_component_modification = self.link.callback(|_| Msg::RequestDeleteModificationData);
        let onclick_component_modification_update = self.link.callback(|_| Msg::RequestUpdateModificationData);
        let modification_data = self.current_modifications.iter().find(|x| x.uuid == self.select_modification_uuid);
        match modification_data {
            Some(mod_data) => html!{<>
                <div class="content">
                    <div class="columns">
                        <div class="column" title={get_value_field(&96)}>
                            <label class="label">{get_value_field(&96)}</label>
                            <div class="select">
                            <select
                                id={"update-modification-actual-status"}
                                select={mod_data.actual_status.actual_status_id.to_string()}
                                onchange={onchange_modification_actual_status_id}
                                >
                                {for self.actual_statuses.iter().map(|x|
                                    html!{
                                        <option value={x.actual_status_id.to_string()}
                                            selected={x.actual_status_id == self.request_add_modification.actual_status_id} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                            </select>
                            </div>
                        </div>
                        <div class="column is-4">
                            {get_value_field(&30)}
                            {mod_data.updated_at.date_to_display()}
                        </div>
                    </div>
                    <div class="column" title={get_value_field(&176)}>
                        <label class="label">{get_value_field(&176)}</label> // Modification name
                        <input
                                id="add-modification-name"
                                class="input is-fullwidth"
                                type="text"
                                placeholder={mod_data.modification_name.clone()}
                                value={self.request_edit_modification.modification_name.clone()}
                                oninput={oninput_modification_name} />
                    </div>
                    <div class="column" title={{get_value_field(&61)}}> // Description
                        <label class="label">{get_value_field(&61)}</label>
                        <textarea
                                id="update-modification-description"
                                class="textarea is-fullwidth"
                                // rows="10"
                                type="text"
                                placeholder={mod_data.description.clone()}
                                value={self.request_edit_modification.description.clone()}
                                oninput={oninput_modification_description} />
                    </div>
                </div>
                <div class="columns">
                    <div class="column">
                        {ft_delete_btn(
                            "delete-component-modification",
                            onclick_delete_component_modification,
                            self.get_confirm == self.select_modification_uuid,
                            false
                        )}
                    </div>
                    <div class="column">
                        {ft_save_btn(
                            "update-component-modification",
                            onclick_component_modification_update,
                            true,
                            !self.modification_changed
                        )}
                    </div>
                </div>
            </>},
            None => html!{},
        }
    }

    fn clear_current_data(&mut self) {
        self.invalid_modification_uuids.clear();
        self.request_add_modification = ModificationUpdatePreData::new();
        self.request_edit_modification = ModificationUpdatePreData::default();
        self.update_add_modification = false;
        self.modification_changed = false;
    }
}
