mod edit;
mod list_item;

pub use edit::ManageModificationFilesCard;
pub use list_item::ModificationFileItem;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::paginate::Paginate;
use crate::fragments::file::{FileHeadersShow, FileInfoItemShow};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo, PaginateSet};
use crate::services::resp_parsing;
use crate::gqls::make_query;
use crate::gqls::component::{ComponentModificationFilesList, component_modification_files_list};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub modification_uuid: UUID,
    pub files_count: i64,
}

pub struct ModificationFilesTableCard {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    files_list: Vec<ShowFileInfo>,
    page_set: PaginateSet,
    current_items: i64,
}

#[derive(Clone)]
pub enum Msg {
    RequestModificationFilesList,
    ResponseError(Error),
    GetModificationFilesListResult(String),
    ChangePaginate(PaginateSet),
    ClearError,
}

impl Component for ModificationFilesTableCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            files_list: Vec::new(),
            page_set: PaginateSet::new(),
            current_items: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.props.modification_uuid.len() == 36 {
            debug!("First render modification files list");
            self.link.send_message(Msg::RequestModificationFilesList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestModificationFilesList => {
                let modification_uuid = self.props.modification_uuid.clone();
                let ipt_modification_files_arg = component_modification_files_list::IptModificationFilesArg{
                    filesUuids: None,
                    modificationUuid: modification_uuid,
                };
                let ipt_paginate = Some(component_modification_files_list::IptPaginate {
                    currentPage: self.page_set.current_page,
                    perPage: self.page_set.per_page,
                });
                spawn_local(async move {
                    let res = make_query(ComponentModificationFilesList::build_query(
                        component_modification_files_list::Variables { ipt_modification_files_arg, ipt_paginate }
                    )).await.unwrap();
                    link.send_message(Msg::GetModificationFilesListResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetModificationFilesListResult(res) => {
                match resp_parsing(res, "componentModificationFilesList") {
                    Ok(result) => self.files_list = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.current_items = self.files_list.len() as i64;
                debug!("componentModificationFilesList {:?}", self.files_list.len());
            },
            Msg::ChangePaginate(page_set) => {
                self.page_set = page_set;
                if self.props.modification_uuid.len() == 36 {
                    self.link.send_message(Msg::RequestModificationFilesList);
                }
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.modification_uuid == props.modification_uuid {
            debug!("not update modification files {:?}", props.modification_uuid);
            false
        } else {
            debug!("update modification files {:?}", props.modification_uuid);
            self.props = props;
            self.files_list.clear();
            if self.props.modification_uuid.len() == 36 {
                self.link.send_message(Msg::RequestModificationFilesList);
            }
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));

        html!{
            <div class="content">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <div class="table-container">
                    <table class="table is-fullwidth is-striped">
                        <FileHeadersShow show_download_btn={self.props.show_download_btn} />
                        <tbody>
                            {for self.files_list.iter().enumerate().map(|(numer, file)| html!{
                                <FileInfoItemShow
                                    file_info={file.clone()}
                                    show_download_btn={self.props.show_download_btn}
                                    ordinal_indicator={self.page_set.numero_offset()+numer}
                                    />
                            })}
                        </tbody>
                    </table>
                </div>
                <Paginate
                    callback_change={onclick_paginate}
                    current_items={self.current_items}
                    total_items={Some(self.props.files_count)}
                    />
            </div>
        }
    }
}