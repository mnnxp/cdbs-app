use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::paginate::Paginate;
use crate::fragments::buttons::ft_back_btn;
use crate::services::content_adapter::{DateDisplay, Markdownable};
use crate::services::{get_value_field, resp_parsing};
use crate::types::{UUID, ComponentModificationInfo, PaginateSet};
use crate::routes::other_component::modification::ImportModificationsData;
use crate::gqls::make_query;
use crate::gqls::component::{GetComponentModifications, get_component_modifications};
use super::table::ModificationsTable;
use super::file::ModificationFilesTableCard;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub component_uuid: UUID,
    pub modifications_count: i64,
    pub callback_select_modification: Option<Callback<UUID>>,
    pub user_owner: bool,
}

pub struct ModificationsTableCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    component_uuid: UUID,
    select_modification_uuid: UUID,
    open_modification_card: bool,
    modifications: Vec<ComponentModificationInfo>,
    skip_change_page: bool,
    page_set: PaginateSet,
    current_items: i64,
}

pub enum Msg {
    RequestComponentModificationsData,
    GetComponentModificationsResult(String),
    ResponseError(Error),
    SelectModification(UUID),
    CallOfChange,
    ShowModificationCard,
    ChangePaginate(PaginateSet),
    ClearError,
}

impl Component for ModificationsTableCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let component_uuid = props.component_uuid.clone();
        Self {
            error: None,
            props,
            link,
            component_uuid,
            select_modification_uuid: String::new(),
            open_modification_card: false,
            modifications: Vec::new(),
            skip_change_page: false,
            page_set: PaginateSet::new(),
            current_items: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            debug!("First show modifications");
            self.component_uuid = self.props.component_uuid.clone();
            self.link.send_message(Msg::RequestComponentModificationsData);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestComponentModificationsData => {
                if self.component_uuid.len() != 36 {
                    return true
                }
                let component_uuid = self.component_uuid.clone();
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
                            filter: None,
                            ipt_sort,
                            ipt_paginate,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentModificationsResult(res));
                })
            },
            Msg::GetComponentModificationsResult(res) => {
                match resp_parsing(res, "componentModifications") {
                    Ok(result) => {
                        self.modifications = result;
                        self.current_items = self.modifications.len() as i64;
                        self.select_modification_uuid = self.modifications.first().map(|m| m.uuid.clone()).unwrap_or_default();
                        self.link.send_message(Msg::CallOfChange);
                        debug!("Update modifications list");
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::SelectModification(modification_uuid) => {
                debug!("Callback CARD, modification uuid set: {:?}, old: {:?} (Show modifications)",
                    modification_uuid,
                    self.select_modification_uuid,
                );
                match self.select_modification_uuid == modification_uuid {
                    true => {
                        self.skip_change_page = true;
                        link.send_message(Msg::ShowModificationCard);
                    },
                    false => {
                        self.select_modification_uuid = modification_uuid;
                        self.link.send_message(Msg::CallOfChange);
                    },
                }
            },
            Msg::CallOfChange => {
                if let Some(select_modification) = &self.props.callback_select_modification {
                    select_modification.emit(self.select_modification_uuid.clone());
                }
            },
            Msg::ShowModificationCard => self.open_modification_card = !self.open_modification_card,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ChangePaginate(page_set) => {
                debug!("Change page_set, old: {:?}, new: {:?} (Show modifications)", self.page_set, page_set);
                if self.skip_change_page {
                    debug!("Skip change page after return from modification card");
                    self.skip_change_page = false;
                    return true
                }
                if self.page_set.compare(&page_set) {
                    return true
                }
                self.page_set = page_set;
                self.link.send_message(Msg::RequestComponentModificationsData);
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        debug!("Show modifications CARD, modification uuid: {:?}", self.select_modification_uuid);
        if self.props.component_uuid == props.component_uuid {
            false
        } else {
            self.component_uuid = props.component_uuid.clone();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_modification_card = self.link.callback(|_| Msg::ShowModificationCard);
        let callback_finish_import = self.link.callback(|_| Msg::RequestComponentModificationsData);
        html!{
            <div class="card">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <header class="card-header">
                    <p class="card-header-title">
                        {match &self.open_modification_card {
                            true => ft_back_btn("open-modifications", onclick_modification_card, get_value_field(&42)),
                            false => html!{get_value_field(&100)} // Modifications,
                        }}
                    </p>
                    {match self.props.user_owner && !self.open_modification_card {
                        true => html!{
                            <div class="card-header-title">
                                <ImportModificationsData
                                    component_uuid={self.props.component_uuid.clone()}
                                    callback_finish_import={callback_finish_import}
                                    />
                            </div>
                        },
                        false => html!{}
                    }}
                </header>
                {self.show_modifications_table()}
                {match self.open_modification_card {
                    true => self.show_modification_card(),
                    false => html!{},
                }}
            </div>
        }
    }
}

impl ModificationsTableCard {
    fn show_modifications_table(&self) -> Html {
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));
        let onclick_select_modification = self.link.callback(|value: UUID| Msg::SelectModification(value));
        html!{
            <div class="card-content">
                <ModificationsTable
                    modifications={self.modifications.clone()}
                    select_modification_uuid={self.select_modification_uuid.clone()}
                    callback_select_modification={onclick_select_modification}
                    numero_offset={self.page_set.numero_offset()}
                />
                <Paginate
                    callback_change={onclick_paginate}
                    current_items={self.current_items}
                    current_page={Some(self.page_set.current_page)}
                    per_page={Some(self.page_set.per_page)}
                    total_items={self.props.modifications_count}
                />
            </div>
        }
    }

    fn show_modification_card(&self) -> Html {
        let modification_data = self.modifications.iter().find(|x| x.uuid == self.select_modification_uuid);
        match modification_data {
            Some(mod_data) => html!{
                <div class="card-content" style="padding-top: 0px;">
                    <div class="content">
                        <div class="column" title={get_value_field(&176)}>
                            <p class="overflow-title has-text-weight-bold">
                                {mod_data.modification_name.clone()}
                            </p>
                        </div>
                        <div class="column">
                        <div class="columns">
                            <div class="column" title={get_value_field(&96)}>
                                {&mod_data.actual_status.name}
                            </div>
                            <div class="column is-4">
                                {get_value_field(&30)}
                                {mod_data.updated_at.date_to_display()}
                            </div>
                        </div>
                        </div>
                        <div class="column" title={{get_value_field(&61)}}> // Description
                            <p>{mod_data.description.to_markdown()}</p>
                        </div>
                    </div>
                    <ModificationFilesTableCard
                        show_download_btn={true}
                        modification_uuid={self.select_modification_uuid.clone()}
                        files_count={mod_data.files_count}
                    />
                </div>
            },
            None => html!{},
        }
    }
}