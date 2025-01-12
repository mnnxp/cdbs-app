mod edit;
mod item;

pub use edit::ManageComponentFilesCard;
pub use item::ComponentFileItem;

use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::paginate::Paginate;
use crate::types::{PaginateSet, ShowFileInfo, UUID};
use crate::services::resp_parsing;
use crate::gqls::make_query;
use crate::gqls::component::{ComponentFilesList, component_files_list};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub files_count: i64,
}

pub struct ComponentFilesBlock {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    files_deleted_list: BTreeSet<UUID>,
    files_list: Vec<ShowFileInfo>,
    page_set: PaginateSet,
    current_items: i64,
    total_items: i64,
}

#[derive(Clone)]
pub enum Msg {
    RequestComponentFilesList,
    ResponseError(Error),
    GetComponentFilesListResult(String),
    ChangePaginate(PaginateSet),
    RemoveFile(UUID),
    ClearError,
    Ignore,
}

impl Component for ComponentFilesBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            files_deleted_list: BTreeSet::new(),
            files_list: Vec::new(),
            page_set: PaginateSet::new(),
            current_items: 0,
            total_items: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.total_items = self.props.files_count;
            self.link.send_message(Msg::RequestComponentFilesList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestComponentFilesList => {
                if self.props.component_uuid.len() != 36 {
                    return false
                }
                let ipt_component_files_arg = component_files_list::IptComponentFilesArg{
                    filesUuids: None,
                    componentUuid: self.props.component_uuid.clone(),
                };
                let ipt_paginate = Some(component_files_list::IptPaginate {
                    currentPage: self.page_set.current_page,
                    perPage: self.page_set.per_page,
                });
                spawn_local(async move {
                    let res = make_query(ComponentFilesList::build_query(
                        component_files_list::Variables { ipt_component_files_arg, ipt_sort: None, ipt_paginate }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentFilesListResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetComponentFilesListResult(res) => {
                match resp_parsing(res, "componentFilesList") {
                    Ok(result) => {
                        self.files_list = result;
                        self.files_deleted_list.clear();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.current_items = self.files_list.len() as i64;
                debug!("componentFilesList {:?}", self.files_list.len());
            },
            Msg::ChangePaginate(page_set) => {
                if self.page_set.compare(&page_set) {
                    return true
                }
                self.page_set = page_set;
                self.link.send_message(Msg::RequestComponentFilesList);
            },
            Msg::RemoveFile(file_uuid) => {
                self.total_items -= 1;
                self.current_items -= 1;
                self.files_deleted_list.insert(file_uuid);
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
            self.props.files_count == props.files_count {
            false
        } else {
            self.total_items = props.files_count;
            self.props = props;
            self.files_deleted_list.clear();
            self.files_list.clear();
            self.link.send_message(Msg::RequestComponentFilesList);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));
        let callback_delete_file = self.link.callback(|value: UUID| Msg::RemoveFile(value));
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <div class={"buttons"}>
                {for self.files_list.iter().map(|file| {
                    match self.files_deleted_list.get(&file.uuid) {
                        Some(_) => html!{}, // removed file
                        None => html!{
                            <ComponentFileItem
                                show_download_btn={self.props.show_download_btn}
                                show_delete_btn={self.props.show_delete_btn}
                                component_uuid={self.props.component_uuid.clone()}
                                file={file.clone()}
                                callback_delete_file={callback_delete_file.clone()}
                                />
                        },
                    }
                })}
            </div>
            <Paginate
                callback_change={onclick_paginate}
                current_items={self.current_items}
                total_items={self.total_items}
                />
        </>}
    }
}