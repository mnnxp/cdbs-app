use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html, InputData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::ModificationFileItem;
use crate::fragments::paginate::Paginate;
use crate::services::{get_value_field, resp_parsing};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::file::{UploaderFiles, commit_msg_field};
use crate::types::{PaginateSet, ShowFileInfo, UploadFile, UUID};
use crate::gqls::make_query;
use crate::gqls::component::{
    ComponentModificationFilesList, component_modification_files_list,
    UploadModificationFiles, upload_modification_files,
};

type FileName = String;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub modification_uuid: UUID,
    pub files_count: i64,
}

pub struct ManageModificationFilesCard {
    error: Option<Error>,
    request_upload_data: Vec<UploadFile>,
    link: ComponentLink<Self>,
    props: Props,
    files_list: Vec<ShowFileInfo>,
    files_deleted_list: BTreeSet<UUID>,
    show_full_files: bool,
    page_set: PaginateSet,
    current_items: i64,
    total_items: i64,
    commit_msg: String,
}

#[derive(Clone)]
pub enum Msg {
    RequestModificationFilesList,
    RequestUploadModificationFiles(Vec<FileName>),
    ResponseError(Error),
    GetModificationFilesListResult(String),
    GetUploadData(String),
    UploadConfirm(usize),
    UpdateCommitMsg(String),
    ShowFullList,
    ChangePaginate(PaginateSet),
    RemoveFile(UUID),
    ClearError,
}

impl Component for ManageModificationFilesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let total_items = props.files_count;
        Self {
            error: None,
            request_upload_data: Vec::new(),
            link,
            props,
            files_list: Vec::new(),
            files_deleted_list: BTreeSet::new(),
            show_full_files: false,
            page_set: PaginateSet::new(),
            current_items: 0,
            total_items,
            commit_msg: String::new(),
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
                if self.props.modification_uuid.len() != 36 {
                    return true
                }
                let ipt_modification_files_arg = component_modification_files_list::IptModificationFilesArg{
                    filesUuids: None,
                    modificationUuid: self.props.modification_uuid.clone(),
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
            Msg::RequestUploadModificationFiles(filenames) => {
                debug!("filenames: {:?}", filenames);
                if self.props.modification_uuid.len() != 36 || filenames.is_empty() {
                    return false
                }
                let modification_uuid = self.props.modification_uuid.clone();
                let commit_msg = self.commit_msg.clone();
                spawn_local(async move {
                    let ipt_modification_files_data = upload_modification_files::IptModificationFilesData{
                        modificationUuid: modification_uuid,
                        filenames,
                        commitMsg: commit_msg,
                    };
                    let res = make_query(UploadModificationFiles::build_query(
                        upload_modification_files::Variables{ ipt_modification_files_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                })
            },
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of files: {:?}", confirmations);
                self.total_items += confirmations as i64;
                self.request_upload_data.clear();
                self.files_list.clear();
                self.commit_msg.clear();
                link.send_message(Msg::RequestModificationFilesList);
            },
            Msg::UpdateCommitMsg(data) => self.commit_msg = data,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetModificationFilesListResult(res) => {
                match resp_parsing(res, "componentModificationFilesList") {
                    Ok(result) => {
                        self.files_list = result;
                        debug!("componentModificationFilesList: {:?}", self.files_list);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUploadData(res) => {
                match resp_parsing(res, "uploadModificationFiles") {
                    Ok(result) => {
                        self.request_upload_data = result;
                        debug!("uploadModificationFiles: {:?}", self.request_upload_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            Msg::ChangePaginate(page_set) => {
                self.page_set = page_set;
                self.link.send_message(Msg::RequestModificationFilesList);
            },
            Msg::RemoveFile(file_uuid) => {
                self.total_items -= 1;
                self.current_items -= 1;
                self.files_deleted_list.insert(file_uuid);
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
            self.files_deleted_list.clear();
            self.files_list.clear();
            self.link.send_message(Msg::RequestModificationFilesList);
            true
        }
    }

    fn view(&self) -> Html {
        let oninput_commit_msg = self.link.callback(|ev: InputData| Msg::UpdateCommitMsg(ev.value));
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let callback_upload_filenames =
            self.link.callback(move |filenames| Msg::RequestUploadModificationFiles(filenames));
        let request_upload_files = match self.request_upload_data.is_empty() {
            true => None,
            false => Some(self.request_upload_data.clone()),
        };
        let callback_upload_confirm = self.link.callback(|confirmations| Msg::UploadConfirm(confirmations));
        let callback_delete_file = self.link.callback(|value: UUID| Msg::RemoveFile(value));
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <div class="column">
                <p class={"title is-4"}>{get_value_field(&202)}</p> // Upload modification files
            </div>
            {commit_msg_field(self.commit_msg.clone(), oninput_commit_msg.clone())}
            <div class="column">
              <UploaderFiles
                text_choose_files={201} // Choose modification files…
                callback_upload_filenames={callback_upload_filenames}
                request_upload_files={request_upload_files}
                callback_upload_confirm={callback_upload_confirm}
                />
            </div>
            <div class="column">
                <p class={"title is-4"}>{get_value_field(&203)}</p> // Files for modification
                <div class={"buttons"}>
                    {for self.files_list.iter().map(|file| {
                        match self.files_deleted_list.get(&file.uuid) {
                            Some(_) => html!{}, // removed file
                            None => html!{
                                <ModificationFileItem
                                    show_download_btn={self.props.show_download_btn}
                                    show_delete_btn={true}
                                    modification_uuid={self.props.modification_uuid.clone()}
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
            </div>
        </>}
    }
}