use std::collections::BTreeSet;
use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{Component, Callback, Context, html, html::Scope, Html, Properties, Event};
use log::debug;
use graphql_client::GraphQLQuery;
// use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use web_sys::FileList;

use super::ModificationFileItem;
use crate::services::{PutUploadFile, UploadData, get_value_field};
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo, UploadFile};
use crate::gqls::{
    make_query,
    relate::{ConfirmUploadCompleted, confirm_upload_completed},
    component::{
        ComponentModificationFilesList, component_modification_files_list,
        UploadModificationFiles, upload_modification_files,
    },
};

type FileName = String;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub modification_uuid: UUID,
}

pub struct ManageModificationFilesCard {
    error: Option<Error>,
    modification_uuid: UUID,
    request_upload_data: Vec<UploadFile>,
    request_upload_file: Callback<Result<Option<String>, Error>>,
    request_upload_confirm: Vec<UUID>,
    task_read: Vec<(FileName, ReaderTask)>,
    task: Vec<FetchTask>,
    files_list: Vec<ShowFileInfo>,
    files_deleted_list: BTreeSet<UUID>,
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
    RemoveFile(UUID),
    ClearFilesBoxed,
    ClearError,
    Ignore,
}

impl Component for ManageModificationFilesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            modification_uuid: ctx.props().modification_uuid,
            request_upload_data: Vec::new(),
            request_upload_file: ctx.link().callback(Msg::ResponseUploadFile),
            request_upload_confirm: Vec::new(),
            task_read: Vec::new(),
            task: Vec::new(),
            files_list: Vec::new(),
            files_deleted_list: BTreeSet::new(),
            put_upload_file: PutUploadFile::new(),
            files: Vec::new(),
            files_index: 0,
            get_result_up_file: false,
            get_result_up_completed: 0,
            active_loading_files_btn: false,
            show_full_files: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render && ctx.props().modification_uuid.len() == 36 {
            debug!("First render modification files list");
            // self.clear_current_data();
            ctx.link().send_message(Msg::RequestModificationFilesList);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestModificationFilesList => {
                let modification_uuid = ctx.props().modification_uuid.clone();
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
                if !self.files.is_empty() && ctx.props().modification_uuid.len() == 36 {
                    // see loading button
                    self.active_loading_files_btn = true;

                    let mut filenames: Vec<String> = Vec::new();
                    for file in &self.files {
                        filenames.push(file.name().clone());
                    }
                    debug!("filenames: {:?}", filenames);
                    let modification_uuid = ctx.props().modification_uuid.clone();

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
                let file_uuids = self.request_upload_confirm.clone();
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
                                    let callback = ctx.link()
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
            Msg::RemoveFile(file_uuid) => {
                self.files_deleted_list.insert(file_uuid);
            },
            Msg::ClearFilesBoxed => {
                self.files.clear();
                self.files_index = 0;
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.modification_uuid == ctx.props().modification_uuid {
            debug!("not update modification files {:?}", self.modification_uuid);
            false
        } else {
            debug!("update modification files {:?}", ctx.props().modification_uuid);
            self.files_deleted_list.clear();
            self.files_list.clear();
            if ctx.props().modification_uuid.len() == 36 {
                ctx.link().send_message(Msg::RequestModificationFilesList);
            }
            self.modification_uuid = ctx.props().modification_uuid;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            <div class="columns">
                <div class="column">
                  <h2>{ get_value_field(&203) }</h2> // Files for modification
                  {self.show_files_list(ctx.link(), ctx.props())}
                </div>
                <div class="column">
                  <h2>{ get_value_field(&202) }</h2> // Upload modification files
                  {self.show_frame_upload_files(ctx.link(), ctx.props())}
                </div>
            </div>
        </>}
    }
}

impl ManageModificationFilesCard {
    fn show_files_list(
        &self,
        link: &Scope<Self>,
        props: &Properties,
    ) -> Html {
        html!{<>
            {for self.files_list.iter().enumerate().map(|(index, file)| {
                match (index >= 3, self.show_full_files) {
                    // show full list
                    (_, true) => self.show_file_info(link, props, &file),
                    // show full list or first 3 items
                    (false, false) => self.show_file_info(link, props, &file),
                    _ => html!{},
                }
            })}
            {match self.files_list.len() {
                0 => html!{<span>{ get_value_field(&204) }</span>},
                0..=3 => html!{},
                _ => self.show_see_btn(link, props),
            }}
        </>}
    }

    fn show_file_info(
        &self,
        link: &Scope<Self>,
        props: &Properties,
        file_info: &ShowFileInfo,
    ) -> Html {
        let callback_delete_file = link.callback(|value: UUID| Msg::RemoveFile(value));

        match self.files_deleted_list.get(&file_info.uuid) {
            Some(_) => html!{}, // removed file
            None => html!{
                <ModificationFileItem
                  show_download_btn = {props.show_download_btn}
                  show_delete_btn = {true}
                  modification_uuid = {props.modification_uuid.clone()}
                  file = {file_info.clone()}
                  callback_delete_file = {callback_delete_file.clone()}
                />
            },
        }
    }

    fn show_see_btn(
        &self,
        link: &Scope<Self>,
        props: &Properties,
    ) -> Html {
        let show_full_files_btn = link.callback(|_| Msg::ShowFullList);

        match self.show_full_files {
            true => html!{<>
              <button class="button is-white"
                  onclick={show_full_files_btn}
                >{ get_value_field(&99) }</button>
            </>},
            false => html!{<>
              <button class="button is-white"
                  onclick={show_full_files_btn}
                >{ get_value_field(&98) }</button>
            </>},
        }
    }

    fn show_frame_upload_files(
        &self,
        link: &Scope<Self>,
        props: &Properties,
    ) -> Html {
        let onchange_upload_files = link.callback(move |value| {
            if let Event::Files(files) = value {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });

        html!{<>
            <div class="file has-name is-boxed is-centered">
                <label class="file-label" style="width: 100%">
                  <input id="component-file-input"
                  class="file-input"
                  type="file"
                  // accept="image/*,application/vnd*,application/rtf,text/*,.pdf"
                  onchange={onchange_upload_files}
                  multiple={true} />
                <span class="file-cta">
                  <span class="file-icon">
                    <i class="fas fa-upload"></i>
                  </span>
                  <span class="file-label">
                    { get_value_field(&201) } // Choose modification files…
                  </span>
                </span>
                {match self.files.is_empty() {
                    true => html!{<span class="file-name">{ get_value_field(&194) }</span>}, // No file uploaded
                    false => html!{for self.files.iter().map(|f| html!{
                        <span class="file-name">{f.name().clone()}</span>
                    })}
                }}
              </label>
            </div>
            <div class="buttons">
                {self.show_clear_btn(link, props)}
                {self.show_upload_files_btn(link, props)}
            </div>
        </>}
    }

    fn show_clear_btn(
        &self,
        link: &Scope<Self>,
        props: &Properties,
    ) -> Html {
        let onclick_clear_boxed = link.callback(|_| Msg::ClearFilesBoxed);

        html!{
            <button id="clear-upload-modification-files"
              class="button"
              onclick={onclick_clear_boxed}
              disabled={self.files.is_empty()} >
                // <span class="icon" >
                //     <i class="fas fa-boom" aria-hidden="true"></i>
                // </span>
                <span>{ get_value_field(&88) }</span>
            </button>
        }
    }

    fn show_upload_files_btn(
        &self,
        link: &Scope<Self>,
        props: &Properties,
    ) -> Html {
        let onclick_upload_files = link.callback(|_| Msg::RequestUploadModificationFiles);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <button
              id="upload-modification-files"
              class={class_upload_btn}
              disabled={self.files.is_empty() || props.modification_uuid.len() != 36}
              onclick={onclick_upload_files} >
                // <span class="icon" >
                //     <i class="fas fa-angle-double-up" aria-hidden="true"></i>
                // </span>
                <span>{ get_value_field(&87) }</span>
            </button>
        }
    }
}
