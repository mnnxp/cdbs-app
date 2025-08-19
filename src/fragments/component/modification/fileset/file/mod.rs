mod list_item;

pub use list_item::FilesetFileItem;

use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use crate::services::resp_parsing;
use crate::error::Error;
use crate::fragments::paginate::Paginate;
use crate::types::{UUID, ShowFileInfo, FilesetProgramInfo, PaginateSet};
use crate::gqls::make_query;
use crate::gqls::component::{ComModFilesOfFileset, com_mod_files_of_fileset};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub upload_files: usize,
    pub show_delete_btn: bool,
    pub select_fileset: FilesetProgramInfo,
}

pub struct FilesetFilesBlock {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    files: Vec<ShowFileInfo>,
    files_deleted_list: BTreeSet<UUID>,
    page_set: PaginateSet,
    current_items: i64,
}

#[derive(Clone)]
pub enum Msg {
    RequestFilesOfFileset,
    ResponseError(Error),
    ChangePaginate(PaginateSet),
    GetFilesOfFilesetResult(String),
    RemoveFile(UUID),
}

impl Component for FilesetFilesBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            files: Vec::new(),
            files_deleted_list: BTreeSet::new(),
            page_set: PaginateSet::new(),
            current_items: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestFilesOfFileset);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestFilesOfFileset => {
                self.files.clear();
                if self.props.select_fileset.uuid.len() == 36 {
                    let ipt_file_of_fileset_arg = com_mod_files_of_fileset::IptFileOfFilesetArg{
                        filesetUuid: self.props.select_fileset.uuid.clone(),
                        fileUuids: None,
                    };
                    let ipt_sort = Some(com_mod_files_of_fileset::IptSort {
                        byField: "name".to_string(),
                        asDesc: false,
                    });
                    let ipt_paginate = Some(com_mod_files_of_fileset::IptPaginate {
                        currentPage: self.page_set.current_page,
                        perPage: self.page_set.per_page,
                    });
                    spawn_local(async move {
                        let res = make_query(ComModFilesOfFileset::build_query(com_mod_files_of_fileset::Variables {
                            ipt_file_of_fileset_arg,
                            ipt_sort,
                            ipt_paginate,
                        })).await.unwrap();

                        link.send_message(Msg::GetFilesOfFilesetResult(res));
                    })
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ChangePaginate(page_set) => {
                debug!("Change page_set, old: {:?}, new: {:?} (Show fileset)", self.page_set, page_set);
                if self.page_set.compare(&page_set) {
                    return true
                }
                self.page_set = page_set;
                self.link.send_message(Msg::RequestFilesOfFileset);
            },
            Msg::GetFilesOfFilesetResult(res) => {
                match resp_parsing(res, "componentModificationFilesOfFileset") {
                    Ok(res) => {
                        self.files = res;
                        debug!("componentModificationFilesOfFileset: {:?}", self.files.len());
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::RemoveFile(file_uuid) => {self.files_deleted_list.insert(file_uuid);},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_fileset.uuid == props.select_fileset.uuid &&
        self.props.upload_files == props.upload_files &&
        self.props.show_delete_btn == props.show_delete_btn {
            debug!("no change fileset uuid: {:?}", props.select_fileset.uuid);
            false
        } else {
            debug!("change fileset uuid: {:?}", props.select_fileset.uuid);
            self.files_deleted_list.clear();
            self.props = props;
            self.link.send_message(Msg::RequestFilesOfFileset);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));
        html!{<>
            <div class={"buttons"}>
                {for self.files.iter().map(|file| self.show_file_info(&file))}
            </div>
            <footer class="card-footer">
                <Paginate
                    callback_change={onclick_paginate}
                    current_items={self.current_items}
                    current_page={Some(self.page_set.current_page)}
                    per_page={Some(self.page_set.per_page)}
                    total_items={self.props.select_fileset.files_count + self.props.upload_files as i64}
                />
            </footer>
        </>}
    }
}

impl FilesetFilesBlock {
    fn show_file_info(
        &self,
        file_info: &ShowFileInfo,
    ) -> Html {
        let callback_delete_file =
            self.link.callback(|value: UUID| Msg::RemoveFile(value));

        match self.files_deleted_list.get(&file_info.uuid) {
            Some(_) => html!{}, // removed file
            None => html!{
                <FilesetFileItem
                  show_delete_btn={self.props.show_delete_btn}
                  select_fileset_uuid={self.props.select_fileset.uuid.clone()}
                  file={file_info.clone()}
                  callback_delete_file={callback_delete_file.clone()}
                />
            },
        }
    }
}
