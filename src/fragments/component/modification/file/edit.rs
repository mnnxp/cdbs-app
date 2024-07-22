use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::ModificationFileItem;
use crate::services::{get_value_field, resp_parsing};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::file::UploaderFiles;
use crate::fragments::buttons::ft_see_btn;
use crate::types::{UUID, ShowFileInfo, UploadFile};
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
}

pub struct ManageModificationFilesCard {
    error: Option<Error>,
    request_upload_data: Vec<UploadFile>,
    link: ComponentLink<Self>,
    props: Props,
    files_list: Vec<ShowFileInfo>,
    files_deleted_list: BTreeSet<UUID>,
    show_full_files: bool,
}

#[derive(Clone)]
pub enum Msg {
    RequestModificationFilesList,
    RequestUploadModificationFiles(Vec<FileName>),
    ResponseError(Error),
    GetModificationFilesListResult(String),
    GetUploadData(String),
    UploadConfirm(usize),
    FinishUploadFiles,
    ShowFullList,
    RemoveFile(UUID),
    ClearError,
}

impl Component for ManageModificationFilesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: Vec::new(),
            link,
            props,
            files_list: Vec::new(),
            files_deleted_list: BTreeSet::new(),
            show_full_files: false,
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
                spawn_local(async move {
                    let ipt_modification_files_arg = component_modification_files_list::IptModificationFilesArg{
                        filesUuids: None,
                        modificationUuid: modification_uuid,
                        limit: None,
                        offset: None,
                    };
                    let res = make_query(ComponentModificationFilesList::build_query(
                        component_modification_files_list::Variables { ipt_modification_files_arg }
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
                spawn_local(async move {
                    let ipt_modification_files_data = upload_modification_files::IptModificationFilesData{
                        modificationUuid: modification_uuid,
                        filenames,
                    };
                    let res = make_query(UploadModificationFiles::build_query(
                        upload_modification_files::Variables{ ipt_modification_files_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                })
            },
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of files: {:?}", confirmations);
                link.send_message(Msg::FinishUploadFiles);
            },
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
            Msg::FinishUploadFiles => {
                self.request_upload_data.clear();
                self.files_list.clear();
                link.send_message(Msg::RequestModificationFilesList);
            },
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            Msg::RemoveFile(file_uuid) => {
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
            if self.props.modification_uuid.len() == 36 {
                self.link.send_message(Msg::RequestModificationFilesList);
            }
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let callback_upload_filenames =
            self.link.callback(move |filenames| Msg::RequestUploadModificationFiles(filenames));
        let request_upload_files = match self.request_upload_data.is_empty() {
            true => None,
            false => Some(self.request_upload_data.clone()),
        };
        let callback_upload_confirm =
            self.link.callback(|confirmations| Msg::UploadConfirm(confirmations));

        html!{<>
            <ListErrors error=self.error.clone() clear_error=onclick_clear_error.clone()/>
            <div class="columns">
                <div class="column">
                  <h2>{get_value_field(&203)}</h2> // Files for modification
                  {self.show_files_list()}
                </div>
                <div class="column">
                  <h2>{get_value_field(&202)}</h2> // Upload modification files
                  <UploaderFiles
                    text_choose_files={201} // Choose modification files…
                    callback_upload_filenames={callback_upload_filenames}
                    request_upload_files={request_upload_files}
                    callback_upload_confirm={callback_upload_confirm}
                    />
                </div>
            </div>
        </>}
    }
}

impl ManageModificationFilesCard {
    fn show_files_list(&self) -> Html {
        html!{<>
            {for self.files_list.iter().enumerate().map(|(index, file)| {
                match (index >= 3, self.show_full_files) {
                    // show full list
                    (_, true) => self.show_file_info(&file),
                    // show full list or first 3 items
                    (false, false) => self.show_file_info(&file),
                    _ => html!{},
                }
            })}
            {match self.files_list.len() {
                0 => html!{<span>{get_value_field(&204)}</span>},
                0..=3 => html!{},
                _ => self.show_see_btn(),
            }}
        </>}
    }

    fn show_file_info(&self, file_info: &ShowFileInfo) -> Html {
        let callback_delete_file =
            self.link.callback(|value: UUID| Msg::RemoveFile(value));

        match self.files_deleted_list.get(&file_info.uuid) {
            Some(_) => html!{}, // removed file
            None => html!{
                <ModificationFileItem
                  show_download_btn = self.props.show_download_btn
                  show_delete_btn = true
                  modification_uuid = self.props.modification_uuid.clone()
                  file = file_info.clone()
                  callback_delete_file = callback_delete_file.clone()
                />
            },
        }
    }

    fn show_see_btn(&self) -> Html {
        let show_full_files_btn = self.link.callback(|_| Msg::ShowFullList);
        ft_see_btn(show_full_files_btn, self.show_full_files)
    }
}
