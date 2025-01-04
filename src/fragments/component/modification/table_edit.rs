use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::ModificationEdit;
use super::table::ModificationsTable;
use crate::error::Error;
use crate::fragments::component::modification::ImportModificationsData;
use crate::fragments::paginate::Paginate;
use crate::fragments::buttons::{ft_add_btn, ft_back_btn};
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_value_field, resp_parsing};
use crate::types::{UUID, ComponentModificationInfo, ActualStatus, ModificationUpdatePreData, PaginateSet};
use crate::gqls::make_query;
use crate::gqls::component::{
    RegisterComponentModification, register_component_modification,
    GetComponentModifications, get_component_modifications,
    ComponentActualStatuses, component_actual_statuses,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub current_component_uuid: UUID,
    pub modifications_count: i64,
}

pub struct ModificationsTableEdit {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    current_modifications: Vec<ComponentModificationInfo>,
    select_modification_uuid: UUID,
    actual_statuses: Vec<ActualStatus>,
    invalid_modification_uuids: BTreeSet<UUID>,
    open_add_modification_card: bool,
    open_edit_modification_card: bool,
    change_page: bool,
    skip_change_page: bool,
    page_set: PaginateSet,
    current_items: i64,
    total_items: i64,
}

pub enum Msg {
    RequestAddModificationData,
    RequestComponentModificationsData,
    RequestListOptData,
    GetAddModificationResult(String),
    SetModificationAsRemote(String),
    GetComponentModificationsResult(String),
    GetListOptResult(String),
    ResponseError(Error),
    AddNewModification,
    ShowEditModificationCard,
    ChangeNewModificationParam(UUID),
    ChangeSelectModification(UUID),
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
            open_add_modification_card: false,
            open_edit_modification_card: false,
            change_page: false,
            skip_change_page: false,
            page_set: PaginateSet::new(),
            current_items: 0,
            total_items,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestComponentModificationsData);
            self.link.send_message(Msg::RequestListOptData);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestAddModificationData => {
                let modification_data = ModificationUpdatePreData::new();
                let ipt_component_modification_data = register_component_modification::IptComponentModificationData{
                    componentUuid: self.props.current_component_uuid.clone(),
                    parentModificationUuid: None,
                    modificationName: modification_data.modification_name,
                    description: modification_data.description,
                    actualStatusId: modification_data.actual_status_id as i64,
                };
                spawn_local(async move {
                    let res = make_query(RegisterComponentModification::build_query(
                        register_component_modification::Variables { ipt_component_modification_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetAddModificationResult(res));
                })
            },
            Msg::RequestComponentModificationsData => {
                let component_uuid = self.props.current_component_uuid.clone();
                // if a new modification is added, only it is requested, and sorting with pagination is not required
                let (filter, ipt_sort, ipt_paginate) = match self.open_add_modification_card {
                    true => (Some(vec![self.select_modification_uuid.clone()]), None, None),
                    false => (
                        None,
                        Some(get_component_modifications::IptSort {byField: "name".to_string(), asDesc: false}),
                        Some(get_component_modifications::IptPaginate {currentPage: self.page_set.current_page, perPage: self.page_set.per_page})
                    ),
                };
                spawn_local(async move {
                    let res = make_query(GetComponentModifications::build_query(
                        get_component_modifications::Variables {
                            component_uuid,
                            filter,
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
                        self.total_items += 1;
                        link.send_message(Msg::RequestComponentModificationsData);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::SetModificationAsRemote(res) => {
                debug!("SetModificationAsRemote: {:?}", res);
                self.invalid_modification_uuids.insert(res);
                self.total_items -= 1;
                self.current_items -= 1;
                self.select_modification_uuid.clear();
                for m in &self.current_modifications {
                    if let None = self.invalid_modification_uuids.get(&m.uuid) {
                        self.select_modification_uuid = m.uuid.clone();
                        break;
                    }
                }
                self.open_edit_modification_card = false;
            },
            Msg::GetComponentModificationsResult(res) => {
                match resp_parsing(res, "componentModifications") {
                    Ok(result) => {
                        self.invalid_modification_uuids.clear();
                        if self.open_add_modification_card {
                            // switch to edit new modification
                            self.open_add_modification_card = false;
                            self.open_edit_modification_card = true;
                        }
                        self.current_modifications = result;
                        self.current_items = self.current_modifications.len() as i64;
                        self.change_page = false;
                        // don't change the selected modification if the old one is in the list
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
            Msg::AddNewModification => {
                self.open_add_modification_card = !self.open_add_modification_card;
                if self.actual_statuses.is_empty() {
                    link.send_message(Msg::RequestListOptData);
                }
                if self.open_add_modification_card {
                    link.send_message(Msg::RequestAddModificationData);
                }
            },
            Msg::ShowEditModificationCard => {
                debug!("open_edit_modification_card: {:?}", self.open_edit_modification_card);
                self.open_edit_modification_card = !self.open_edit_modification_card;
                if self.actual_statuses.is_empty() {
                    link.send_message(Msg::RequestListOptData);
                }
                if !self.open_edit_modification_card {
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
                    false => self.select_modification_uuid = modification_uuid,
                }
            },
            Msg::ChangePaginate(page_set) => {
                debug!("Change page_set, old: {:?}, new: {:?} (Show modifications)", self.page_set, page_set);
                if self.skip_change_page {
                    debug!("Skip change page after return from modification card");
                    self.skip_change_page = false;
                    return true
                }
                let check = !self.page_set.compare(&page_set);
                self.page_set = page_set;
                if self.props.current_component_uuid.len() == 36 && check {
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
        let callback_finish_import = self.link.callback(|_| Msg::RequestComponentModificationsData);
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
                    <div class="card-header-title">
                        <ImportModificationsData
                            component_uuid={self.props.current_component_uuid.clone()}
                            callback_finish_import={callback_finish_import}
                            />
                    </div>
                </header>
                {match self.open_edit_modification_card {
                    true => self.show_modification_edit(),
                    false => self.show_modifications_table(),
                }}
            </div>
        }
    }
}

impl ModificationsTableEdit {
    fn show_modifications_table(&self) -> Html {
        let onclick_new_modification_param = self.link.callback(|value: UUID| Msg::ChangeNewModificationParam(value));
        let onclick_add_new_modification = self.link.callback(|_| Msg::AddNewModification);
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
                <ModificationsTable
                    modifications={modifications}
                    select_modification_uuid={self.select_modification_uuid.clone()}
                    callback_select_modification={onclick_select_modification}
                    callback_new_modification_param={Some(onclick_new_modification_param)}
                    numero_offset={self.page_set.numero_offset()}
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
                        onclick_add_new_modification,
                        true,
                        false
                    )}
                </footer>
            </div>
        }
    }

    fn show_modification_edit(&self) -> Html {
        let callback_delete_modification = self.link.callback(|value| Msg::SetModificationAsRemote(value));
        let modification_data = self.current_modifications.iter().find(|x| x.uuid == self.select_modification_uuid);
        match modification_data {
            Some(md) => html!{
                <ModificationEdit
                    modification={md.clone()}
                    actual_statuses={self.actual_statuses.clone()}
                    callback_delete_modification={callback_delete_modification}
                    />
            },
            None => html!{},
        }
    }
}
