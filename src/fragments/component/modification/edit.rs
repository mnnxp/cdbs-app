use std::collections::{HashMap, BTreeSet};
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html, InputData, ChangeData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::file::ManageModificationFilesCard;
use super::heads::ModificationTableHeads;
use super::item::ModificationTableItem;
use super::fileset::ManageModificationFilesets;
use crate::error::Error;
use crate::fragments::buttons::{ft_delete_btn, ft_save_btn, ft_add_btn};
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_value_field, resp_parsing};
use crate::types::{
    UUID, ComponentModificationInfo, Param, ActualStatus,
    ModificationUpdatePreData, FilesetProgramInfo,
};
use crate::gqls::make_query;
use crate::gqls::component::{
    RegisterComponentModification, register_component_modification,
    PutComponentModificationUpdate, put_component_modification_update,
    DeleteComponentModification, delete_component_modification,
    GetComponentModifications, get_component_modifications,
    ComponentModificationFilesets, component_modification_filesets,
    ComponentActualStatuses, component_actual_statuses,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub current_component_uuid: UUID,
    pub component_modifications: Vec<ComponentModificationInfo>,
}

pub struct ModificationsTableEdit {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    component_uuid: UUID,
    current_modifications: Vec<ComponentModificationInfo>,
    select_modification_uuid: UUID,
    modification_filesets: HashMap<UUID, Vec<(UUID, String)>>,
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
    finish_parsing_heads: bool,
    get_confirm: UUID,
}

pub enum Msg {
    ParseParams,
    ParseFilesets,
    RequestAddModificationData,
    RequestUpdateModificationData,
    RequestDeleteModificationData,
    RequestComponentModificationsData,
    RequestComponentModificationFilesetsData,
    RequestListOptData,
    GetAddModificationResult(String),
    GetUpdateModificationResult(String),
    GetDeleteModificationResult(String),
    GetComponentModificationsResult(String),
    GetComponentModificationFilesetResult(String),
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
    UpdateSelectModification,
    ChangeModificationData,
    ClearError,
}

impl Component for ModificationsTableEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let component_uuid = props.current_component_uuid.clone();
        let current_modifications = props.component_modifications.clone();
        let select_modification_uuid = props.component_modifications
            .first()
            .map(|m| m.uuid.clone())
            .unwrap_or_default();
        let mut modification_filesets: HashMap<UUID, Vec<(UUID, String)>> = HashMap::new();
        for component_modification in &props.component_modifications {
            let mut fileset_data: Vec<(UUID, String)> = Vec::new();
            for fileset in &component_modification.filesets_for_program {
                fileset_data.push((fileset.uuid.clone(), fileset.program.name.clone()));
            }
            modification_filesets.insert(
                component_modification.uuid.clone(),
                fileset_data.clone()
            );
        }

        Self {
            error: None,
            props,
            link,
            component_uuid,
            current_modifications,
            select_modification_uuid,
            modification_filesets,
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
            finish_parsing_heads: false,
            get_confirm: String::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render || self.component_uuid != self.props.current_component_uuid {
            self.component_uuid = self.props.current_component_uuid.clone();
            debug!("Clear modification data");
            self.clear_current_data();
            self.link.send_message(Msg::ParseParams);
            self.link.send_message(Msg::ParseFilesets);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ParseParams => {
                let mut set_heads: Vec<usize> = vec![0];
                let mut collect_heads: Vec<Param> = Vec::new();

                for modification in &self.current_modifications {
                    self.valid_modification_uuids.insert(modification.uuid.clone());
                    self.collect_columns.clear();
                    self.collect_columns.insert(0, modification.modification_name.clone());
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
                debug!("collect_heads: {:?}", collect_heads);
                self.collect_heads = collect_heads;
                self.finish_parsing_heads = true;
            },
            Msg::ParseFilesets => {
                let mut modification_filesets: HashMap<UUID, Vec<(UUID, String)>> = HashMap::new();
                for component_modification in &self.current_modifications {
                    let mut fileset_data: Vec<(UUID, String)> = Vec::new();
                    for fileset in &component_modification.filesets_for_program {
                        fileset_data.push((fileset.uuid.clone(), fileset.program.name.clone()));
                    }
                    modification_filesets.insert(
                        component_modification.uuid.clone(),
                        fileset_data.clone()
                    );
                }
                self.modification_filesets = modification_filesets;
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
            Msg::RequestComponentModificationFilesetsData => {
                let ipt_fileset_program_arg = component_modification_filesets::IptFilesetProgramArg{
                    modificationUuid: self.select_modification_uuid.clone(),
                    programIds: None,
                    limit: None,
                    offset: None,
                };

                spawn_local(async move {
                    let res = make_query(ComponentModificationFilesets::build_query(
                        component_modification_filesets::Variables { ipt_fileset_program_arg }
                    )).await.unwrap();

                    link.send_message(Msg::GetComponentModificationFilesetResult(res));
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
                        link.send_message(Msg::RequestComponentModificationsData);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateModificationResult(res) => {
                match resp_parsing::<usize>(res, "putComponentModificationUpdate") {
                    Ok(result) => {
                        debug!("putComponentModificationUpdate: {:?}", result);
                        if result > 0 {
                            link.send_message(Msg::RequestComponentModificationsData);
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.open_edit_modification_card = false;
            },
            Msg::GetDeleteModificationResult(res) => {
                match resp_parsing::<UUID>(res, "deleteComponentModification") {
                    Ok(result) => {
                        debug!("deleteComponentModification: {:?}", result);
                        self.valid_modification_uuids.remove(&result);
                        self.select_modification_uuid = String::new();
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
                        debug!("Update modifications list");
                        link.send_message(Msg::ParseParams);
                        link.send_message(Msg::ParseFilesets);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetComponentModificationFilesetResult(res) => {
                match resp_parsing::<Vec<FilesetProgramInfo>>(res, "componentModificationFilesets") {
                    Ok(filesets) => {
                        debug!("Update modification filesets list");
                        let component_modification_uuid = filesets.first().map(|x| x.modification_uuid.clone()).unwrap_or_default();
                        let mut fileset_data: Vec<(UUID, String)> = Vec::new();
                        for fileset in &filesets {
                            fileset_data.push((fileset.uuid.clone(), fileset.program.name.clone()));
                        }
                        self.modification_filesets.remove(&component_modification_uuid);
                        self.modification_filesets.insert(
                            component_modification_uuid,
                            fileset_data.clone()
                        );
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
                if self.open_edit_modification_card {
                    // clear the check flags
                    self.get_confirm.clear();
                    self.update_edit_modification = false;
                    link.send_message(Msg::UpdateSelectModification);
                }
            },
            Msg::ChangeNewModificationParam(modification_uuid) => {
                debug!("Add new modification parameter name");
                link.send_message(Msg::RequestComponentModificationsData);
                self.select_modification_uuid = modification_uuid;
            },
            Msg::ChangeSelectModification(modification_uuid) => {
                match self.select_modification_uuid == modification_uuid {
                    true => link.send_message(Msg::ShowEditModificationCard),
                    false => {
                        self.select_modification_uuid = modification_uuid;
                        link.send_message(Msg::RequestComponentModificationFilesetsData);
                    },
                }
            },
            Msg::UpdateSelectModification => {
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
            }
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.current_component_uuid == props.current_component_uuid {
            debug!("not update modifications {:?}", props.component_modifications.len());
            false
        } else {
            debug!("update modifications {:?}", props.component_modifications.len());
            self.current_modifications = props.component_modifications.clone();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        match self.finish_parsing_heads {
            true => html!{<>
                {self.show_modifications_table()}
                <br/>
                {self.show_modification_files()}
                <br/>
                {self.show_fileset_files_card()}
            </>},
            false => html!{},
        }
    }
}

impl ModificationsTableEdit {
    fn show_modifications_table(&self) -> Html {
        let onclick_new_modification_param =
            self.link.callback(|value: UUID| Msg::ChangeNewModificationParam(value));
        let onclick_select_modification =
            self.link.callback(|value: UUID| Msg::ChangeSelectModification(value));
        let onclick_add_modification_card = self.link.callback(|_| Msg::ShowAddModificationCard);
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<div class="card">
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <header class="card-header">
                <p class="card-header-title">{get_value_field(&60)}</p> // Manage component modifications
            </header>
            <div class="card-content">
                <div class="content">
                    {self.modal_add_modification_card()}
                    {self.modal_edit_modification_card()}
                    <div class="table-container">
                        <table class="table is-fullwidth">
                            <ModificationTableHeads
                            show_new_column={true}
                            component_uuid={self.component_uuid.clone()}
                            params={self.collect_heads.clone()}
                            />
                            {for self.collect_items.iter().map(|(modification_uuid, item)|
                                match self.valid_modification_uuids.get(modification_uuid) {
                                    Some(_) => html!{<ModificationTableItem
                                        show_manage_btn={true}
                                        modification_uuid={modification_uuid.clone()}
                                        collect_heads={self.collect_heads.clone()}
                                        collect_item={item.clone()}
                                        select_item={&self.select_modification_uuid == modification_uuid}
                                        open_modification_files={false}
                                        callback_new_modification_param={Some(onclick_new_modification_param.clone())}
                                        callback_select_modification={Some(onclick_select_modification.clone())}
                                    />},
                                    None => html!{},
                                }
                            )}
                        </table>
                    </div>
                </div>
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
        </div>}
    }

    fn show_modification_files(&self) -> Html {
        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&172)}</p> // Manage modification files
                </header>
                <div class="card-content">
                    <div class="content">
                        <ManageModificationFilesCard
                            show_download_btn={false}
                            modification_uuid={self.select_modification_uuid.clone()}
                        />
                    </div>
                </div>
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

    fn modal_edit_modification_card(&self) -> Html {
        let oninput_modification_name = self.link.callback(|ev: InputData| Msg::UpdateEditName(ev.value));
        let oninput_modification_description = self.link.callback(|ev: InputData| Msg::UpdateEditDescription(ev.value));
        let onchange_modification_actual_status_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateEditActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let onclick_modification_card = self.link.callback(|_| Msg::ShowEditModificationCard);
        let onclick_delete_component_modification = self.link.callback(|_| Msg::RequestDeleteModificationData);
        let onclick_component_modification_update = self.link.callback(|_| Msg::RequestUpdateModificationData);
        let class_modal = match &self.open_edit_modification_card {
            true => "modal is-active",
            false => "modal",
        };
        let modification_data: Option<&ComponentModificationInfo> =
            self.current_modifications.iter().find(|x| x.uuid == self.select_modification_uuid);

        match modification_data {
            Some(modification_data) => html!{<div class={class_modal}>
              <div class="modal-background" onclick={onclick_modification_card.clone()} />
                <div class="card">
                  <div class="modal-content">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{get_value_field(&177)}</p> // Change modification data
                        <button class="delete" aria-label="close" onclick={onclick_modification_card} />
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
                                  placeholder={modification_data.modification_name.clone()}
                                  value={self.request_edit_modification.modification_name.clone()}
                                  oninput={oninput_modification_name} />
                          </div>
                          <div class="column">
                            <label class="label">{get_value_field(&61)}</label>
                            <textarea
                                  id="update-modification-description"
                                  class="textarea is-fullwidth"
                                  // rows="10"
                                  type="text"
                                  placeholder={modification_data.description.clone()}
                                  value={self.request_edit_modification.description.clone()}
                                  oninput={oninput_modification_description} />
                          </div>
                          <div class="column">
                            <label class="label">{get_value_field(&96)}</label>
                            <div class="select">
                              <select
                                  id="update-modification-actual-status"
                                  select={modification_data.actual_status.actual_status_id.to_string()}
                                  onchange={onchange_modification_actual_status_id}
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
                          <br/>
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
                                    !self.update_edit_modification
                                  )}
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
        self.modification_filesets = HashMap::new();
        self.collect_heads.clear();
        self.collect_items.clear();
        self.collect_columns.clear();
        self.valid_modification_uuids.clear();
        self.request_add_modification = ModificationUpdatePreData::new();
        self.request_edit_modification = ModificationUpdatePreData::default();
        self.update_add_modification = false;
        self.update_edit_modification = false;
        self.finish_parsing_heads = false;
    }

    fn show_fileset_files_card(&self) -> Html {
        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&173)}</p> // Manage modification filesets
                </header>
                <div class="card-content">
                    <div class="content">
                        <ManageModificationFilesets
                            select_modification_uuid={self.select_modification_uuid.clone()}
                            filesets_program = {self.modification_filesets
                                .get(&self.select_modification_uuid)
                                .map(|f| f.clone())
                                .unwrap_or_default()}
                        />
                    </div>
                </div>
            </div>
        }
    }
}
