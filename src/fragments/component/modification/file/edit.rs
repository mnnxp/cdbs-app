use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html, ChangeData};
use log::debug;
use graphql_client::GraphQLQuery;
// use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use chrono::NaiveDateTime;
use web_sys::FileList;

use super::ModificationFileItem;
use crate::services::{PutUploadFile, UploadData};
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, ShowFileInfo, UploadFile};

type FileName = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComponentModificationFilesList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct UploadModificationFiles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct ConfirmUploadCompleted;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub modification_uuid: UUID,
}

pub struct ManageModificationFilesCard {
    error: Option<Error>,
    request_upload_data: Vec<UploadFile>,
    request_upload_file: Callback<Result<Option<String>, Error>>,
    request_upload_confirm: Vec<UUID>,
    task_read: Vec<(FileName, ReaderTask)>,
    task: Vec<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
    files_list: Vec<ShowFileInfo>,
    put_upload_file: PutUploadFile,
    files: Vec<File>,
    files_index: u32,
    get_result_up_file: bool,
    get_result_up_completed: usize,
    active_loading_files_btn: bool,
    show_full_files: bool,
}

#[derive(Clone)]
pub enum Msg {
    RequestModificationFilesList,
    RequestUploadModificationFiles,
    RequestUploadFile(Vec<u8>),
    ResponseUploadFile(Result<Option<String>, Error>),
    RequestUploadCompleted,
    ResponseError(Error),
    GetModificationFilesListResult(String),
    GetUploadData(String),
    GetUploadFile,
    GetUploadCompleted(String),
    UpdateFiles(FileList),
    FinishUploadFiles,
    ShowFullList,
    ClearFilesBoxed,
    ClearError,
    Ignore,
}

impl Component for ManageModificationFilesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: Vec::new(),
            request_upload_file: link.callback(Msg::ResponseUploadFile),
            request_upload_confirm: Vec::new(),
            task_read: Vec::new(),
            task: Vec::new(),
            link,
            props,
            files_list: Vec::new(),
            put_upload_file: PutUploadFile::new(),
            files: Vec::new(),
            files_index: 0,
            get_result_up_file: false,
            get_result_up_completed: 0,
            active_loading_files_btn: false,
            show_full_files: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.props.modification_uuid.len() == 36 {
            debug!("First render modification files list");
            // self.clear_current_data();
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
            Msg::RequestUploadModificationFiles => {
                if !self.files.is_empty() && self.props.modification_uuid.len() == 36 {
                    // see loading button
                    self.active_loading_files_btn = true;

                    let mut filenames: Vec<String> = Vec::new();
                    for file in &self.files {
                        filenames.push(file.name().clone());
                    }
                    debug!("filenames: {:?}", filenames);
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
                }
            },
            Msg::RequestUploadFile(data) => {
                if let Some(upload_data) = self.request_upload_data.pop() {
                    let request = UploadData {
                        upload_url: upload_data.upload_url.clone(),
                        file_data: data,
                    };
                    debug!("request: {:?}", request);

                    self.task.push(self.put_upload_file.put_file(request, self.request_upload_file.clone()));
                    self.request_upload_confirm.push(upload_data.file_uuid.clone());
                };
            },
            Msg::RequestUploadCompleted => {
                let file_uuids = self.request_upload_confirm.to_vec();
                spawn_local(async move {
                    let res = make_query(ConfirmUploadCompleted::build_query(
                        confirm_upload_completed::Variables { file_uuids }
                    )).await.unwrap();
                    // debug!("ConfirmUploadCompleted: {:?}", res);
                    link.send_message(Msg::GetUploadCompleted(res));
                });
            },
            Msg::ResponseUploadFile(Ok(res)) => {
                debug!("ResponseUploadFile: {:?}", res);
                link.send_message(Msg::GetUploadFile);
            },
            Msg::ResponseUploadFile(Err(err)) => {
                self.error = Some(err);
                self.task.clear();
                self.task_read.clear();
                self.files_index = 0;
                self.request_upload_confirm.clear();
                self.get_result_up_completed = 0;
                self.active_loading_files_btn = false;
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetModificationFilesListResult(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.files_list = serde_json::from_value(
                            res_value.get("componentModificationFilesList").unwrap().clone()
                        ).unwrap();
                        debug!("componentModificationFilesList {:?}", self.files_list.len());
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.request_upload_data = serde_json::from_value(
                            res_value.get("uploadModificationFiles").unwrap().clone()
                        ).unwrap();
                        debug!("uploadModificationFiles {:?}", self.request_upload_data);

                        if !self.files.is_empty() {
                            for file in self.files.iter().rev() {
                                let file_name = file.name().clone();
                                debug!("file name: {:?}", file_name);
                                let task = {
                                    let callback = self.link
                                        .callback(move |data: FileData| Msg::RequestUploadFile(data.content));
                                    ReaderService::read_file(file.clone(), callback).unwrap()
                                };
                                self.task_read.push((file_name, task));
                            }
                        }
                        debug!("file: {:#?}", self.files);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetUploadFile => {
                debug!("next: {:?}", self.files_index);
                self.files_index -= 1;
                if self.files_index == 0 {
                    self.get_result_up_file = true;
                    debug!("finish: {:?}", self.request_upload_confirm.len());
                    link.send_message(Msg::RequestUploadCompleted);
                }
            },
            Msg::GetUploadCompleted(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.get_result_up_completed = serde_json::from_value(
                            res_value.get("uploadCompleted").unwrap().clone()
                        ).unwrap();
                        debug!("uploadCompleted: {:?}", self.get_result_up_completed);

                        link.send_message(Msg::FinishUploadFiles);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::UpdateFiles(files) => {
                while let Some(file) = files.get(self.files_index) {
                    debug!("self.files_index: {:?}", self.files_index);
                    self.files_index += 1;
                    self.files.push(file.clone());
                }
                // self.files_index = 0;
            },
            Msg::FinishUploadFiles => {
                self.files_list.clear();
                link.send_message(Msg::RequestModificationFilesList);
                self.active_loading_files_btn = false;
                self.task.clear();
                self.task_read.clear();
                self.request_upload_confirm.clear();
                self.files.clear();
                self.files_index = 0;
            },
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            Msg::ClearFilesBoxed => {
                self.files.clear();
                self.files_index = 0;
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
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

        html!{<>
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            <div class="columns">
                <div class="column">
                  <h2>{"Modification files"}</h2>
                  {self.show_files_card()}
                </div>
                <div class="column">
                  <h2>{"Upload modification files"}</h2>
                  {self.show_frame_upload_files()}
                </div>
            </div>
        </>}
    }
}

impl ManageModificationFilesCard {
    fn show_files_card(&self) -> Html {
        html!{
            <div id="files" class="card">
                {for self.files_list.iter().enumerate().map(|(index, file)| {
                    match (index >= 3, self.show_full_files) {
                        // show full list
                        (_, true) => html!{<ModificationFileItem
                          show_download_btn = self.props.show_download_btn
                          show_delete_btn = true
                          modification_uuid = self.props.modification_uuid.clone()
                          file = file.clone()
                        />},
                        // show full list or first 3 items
                        (false, false) => html!{<ModificationFileItem
                          show_download_btn = self.props.show_download_btn
                          show_delete_btn = true
                          modification_uuid = self.props.modification_uuid.clone()
                          file = file.clone()
                        />},
                        _ => html!{},
                    }
                })}
                {match self.files_list.len() {
                    0 => html!{<span>{"Files not found"}</span>},
                    0..=3 => html!{},
                    _ => self.show_see_btn(),
                }}
            </div>
        }
    }

    fn show_see_btn(&self) -> Html {
        let show_full_files_btn = self.link
            .callback(|_| Msg::ShowFullList);

        match self.show_full_files {
            true => html!{<>
              <button class="button is-white"
                  onclick=show_full_files_btn
                >{"See less"}</button>
            </>},
            false => html!{<>
              <button class="button is-white"
                  onclick=show_full_files_btn
                >{"See more"}</button>
            </>},
        }
    }

    fn show_frame_upload_files(&self) -> Html {
        let onchange_upload_files = self.link.callback(move |value| {
            if let ChangeData::Files(files) = value {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });

        html!{<div class="card">
            <div class="file has-name is-boxed is-centered">
                <label class="file-label" style="width: 100%">
                  <input id="component-file-input"
                  class="file-input"
                  type="file"
                  // accept="image/*,application/vnd*,application/rtf,text/*,.pdf"
                  onchange={onchange_upload_files}
                  multiple=true />
                <span class="file-cta">
                  <span class="file-icon">
                    <i class="fas fa-upload"></i>
                  </span>
                  <span class="file-label">
                    {"Choose modification files…"}
                  </span>
                </span>
                {match self.files.is_empty() {
                    true => html!{<span class="file-name">{"No file uploaded"}</span>},
                    false => html!{for self.files.iter().map(|f| html!{
                        <span class="file-name">{f.name().clone()}</span>
                    })}
                }}
              </label>
            </div>
            <div class="buttons">
                {self.show_clear_btn()}
                {self.show_upload_files_btn()}
            </div>
        </div>}
    }

    fn show_clear_btn(&self) -> Html {
        let onclick_clear_boxed = self.link.callback(|_| Msg::ClearFilesBoxed);

        html!{
            <button id="clear-upload-fileset-files"
              class="button"
              onclick=onclick_clear_boxed
              disabled={self.files.is_empty()} >
                // <span class="icon" >
                //     <i class="fas fa-boom" aria-hidden="true"></i>
                // </span>
                <span>{"Clear"}</span>
            </button>
        }
    }

    fn show_upload_files_btn(&self) -> Html {
        let onclick_upload_files = self.link.callback(|_| Msg::RequestUploadModificationFiles);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <button
              id="upload-fileset-files"
              class={class_upload_btn}
              disabled={self.files.is_empty() || self.props.modification_uuid.len() != 36}
              onclick={onclick_upload_files} >
                // <span class="icon" >
                //     <i class="fas fa-angle-double-up" aria-hidden="true"></i>
                // </span>
                <span>{"Upload"}</span>
            </button>
        }
    }
}
