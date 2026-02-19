mod commit_msg;
pub use commit_msg::commit_msg_field;

use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{classes, Component, Callback, ComponentLink, Html, Properties, ShouldRender, html, ChangeData, DragEvent};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use web_sys::FileList;

use crate::services::{put_file, UploadData, get_value_field, resp_parsing, image_detector};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
// use crate::fragments::switch_icon::res_file_btn;
use crate::types::{UUID, UploadFile};
use crate::gqls::make_query;
use crate::gqls::relate::{ConfirmUploadCompleted, confirm_upload_completed};

type FileName = String;

#[derive(Clone, Debug, PartialEq)]
pub enum UploadStatus {
    Pending,
    Uploading,
    Completed,
    Failed(String),
}

pub struct UploaderFiles {
    error: Option<Error>,
    response_upload_file: Callback<Result<Option<String>, Error>>,
    request_upload_confirm: Vec<UUID>,
    readers: HashMap<FileName, ReaderTask>,
    upload_progress: Callback<(Option<String>, f32)>,
    progress_indicator: HashMap<FileName, f32>,
    presigned_url: HashMap<FileName, UploadFile>,
    task: Vec<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
    files: Vec<File>,
    files_index: u32,
    get_result_up_file: bool,
    get_result_up_completed: usize,
    active_loading_files_btn: bool,
    label_filenames: Vec<String>,
    file_statuses: HashMap<FileName, UploadStatus>,
    is_dragging: bool,
    upload_success_count: usize,
    upload_failed_count: usize,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub text_choose_files: usize,
    pub callback_upload_filenames: Callback<Vec<FileName>>,
    pub request_upload_files: Option<Vec<UploadFile>>,
    pub callback_upload_confirm: Callback<usize>,
    #[prop_or(true)]
    pub multiple: bool,
    #[prop_or_default]
    pub accept: String,
}

#[derive(Clone)]
pub enum Msg {
    UploadFiles,
    ParsingUrls,
    PutFiles,
    RequestUploadFile(FileName, Vec<u8>),
    ResponseUploadFile(Result<Option<String>, Error>),
    UploadProgress((Option<String>, f32)),
    RequestUploadCompleted,
    ResponseError(Error),
    GetUploadFile,
    GetUploadCompleted(String),
    UpdateFiles(FileList),
    FinishUploadFiles,
    ClearFilesBoxed,
    ClearError,
    SetDragState(bool),
    RemoveFile(String),
    Ignore,
}

impl Component for UploaderFiles {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            response_upload_file: link.callback(Msg::ResponseUploadFile),
            request_upload_confirm: Vec::new(),
            readers: HashMap::new(),
            upload_progress: link.callback(Msg::UploadProgress),
            progress_indicator: HashMap::new(),
            presigned_url: HashMap::new(),
            task: Vec::new(),
            link,
            props,
            files: Vec::new(),
            files_index: 0,
            get_result_up_file: false,
            get_result_up_completed: 0,
            active_loading_files_btn: false,
            label_filenames: Vec::new(),
            file_statuses: HashMap::new(),
            is_dragging: false,
            upload_success_count: 0,
            upload_failed_count: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::UploadFiles => {
                let mut filenames = Vec::new();
                for file in &self.files {
                    filenames.push(file.name().clone());
                }
                if filenames.is_empty() {
                    return false
                }
                // Initialize file statuses
                for filename in &filenames {
                    self.file_statuses.insert(filename.clone(), UploadStatus::Pending);
                }
                self.upload_success_count = 0;
                self.upload_failed_count = 0;
                // see loading button
                self.active_loading_files_btn = true;
                self.props.callback_upload_filenames.emit(filenames);
            },
            Msg::ParsingUrls => {
                self.presigned_url.clear();
                if let Some(upload_files) = &self.props.request_upload_files {
                    for purl in upload_files {
                        self.presigned_url.insert(purl.filename.clone(), purl.clone());
                    }
                }
                if self.presigned_url.is_empty() {
                    return false
                }
                link.send_message(Msg::PutFiles);
            },
            Msg::PutFiles => {
                if self.files.is_empty() {
                    return false
                }
                debug!("files: {:#?}", self.files);
                for file in &self.files {
                    let file_name = file.name().clone();
                    debug!("file name: {:?}", file_name);
                    // Update file status to uploading
                    self.file_statuses.insert(file_name.clone(), UploadStatus::Uploading);
                    let task = {
                        let callback = self.link.callback(move |data: FileData|
                            Msg::RequestUploadFile(
                                data.name,
                                data.content,
                            ));
                        ReaderService::read_file(file.clone(), callback).unwrap()
                    };
                    self.readers.insert(file_name, task);
                }
            },
            Msg::RequestUploadFile(filename, data) => {
                match self.presigned_url.get(&filename) {
                    Some(file_data) => {
                        let upload_data = UploadData {
                            filename: filename.clone(),
                            upload_url: file_data.upload_url.clone(),
                            file_data: data,
                        };
                        debug!("Upload file: {:?}, {:?}", file_data.file_uuid, file_data.filename);
                        self.readers.remove(&filename);
                        put_file(
                            upload_data,
                            self.response_upload_file.clone(),
                            self.upload_progress.clone(),
                        );
                        self.request_upload_confirm.push(file_data.file_uuid.clone());
                    },
                    None => {
                        debug!("not found pre-signed url for upload the file: {:?}", filename);
                        // Update file status to failed
                        self.file_statuses.insert(filename, UploadStatus::Failed("Presigned URL not found".to_string()));
                        self.upload_failed_count += 1;
                    },
                }
            },
            Msg::ResponseUploadFile(Ok(res)) => {
                debug!("ResponseUploadFile: {:?}", res);
                link.send_message(Msg::GetUploadFile);
            },
            Msg::ResponseUploadFile(Err(err)) => {
                debug!("ResponseUploadFile Err: {:?}", err);
                self.error = Some(err);
                link.send_message(Msg::FinishUploadFiles);
            },
            Msg::UploadProgress((file_name_op, upload_progress)) => {
                if let Some(file_name) = file_name_op {
                    self.progress_indicator.insert(file_name.clone(), upload_progress);
                    // Update file status
                    if upload_progress >= 1.0 && self.file_statuses.get(&file_name) != Some(&UploadStatus::Completed) {
                        self.file_statuses.insert(file_name, UploadStatus::Completed);
                        self.upload_success_count += 1;
                    }
                }
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
            Msg::ResponseError(err) => self.error = Some(err),
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
                match resp_parsing(res, "uploadCompleted") {
                    Ok(result) => {
                        self.get_result_up_completed = result;
                        debug!("uploadCompleted: {:?}", self.get_result_up_completed);
                        link.send_message(Msg::FinishUploadFiles);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateFiles(files) => {
                let mut temp_index = 0;
                while let Some(file) = files.get(temp_index) {
                    debug!("temp_index: {:?}", temp_index);
                    if !self.props.multiple && temp_index != 0 {
                        debug!("Multiple disabled");
                        return true
                    }
                    if &self.props.accept == "image/*" && !image_detector(&file.name()){
                        debug!("File is not recognized as a picture");
                        return true
                    }
                    temp_index += 1;
                    self.files.push(file.clone());
                    self.label_filenames.push(file.name().clone());
                }
                self.files_index += temp_index;
                debug!("files_index: {:?}", self.files_index);
            },
            Msg::FinishUploadFiles => {
                link.send_message(Msg::ClearFilesBoxed);
                self.props.callback_upload_confirm.emit(self.get_result_up_completed);
                self.get_result_up_completed = 0;
                self.active_loading_files_btn = false;
                self.task.clear();
                self.readers.clear();
                self.presigned_url.clear();
                self.request_upload_confirm.clear();
            },
            Msg::ClearFilesBoxed => {
                self.files.clear();
                self.label_filenames.clear();
                self.files_index = 0;
                self.file_statuses.clear();
                self.upload_success_count = 0;
                self.upload_failed_count = 0;
            },
            Msg::RemoveFile(filename) => {
                // Remove file from files list
                if let Some(index) = self.label_filenames.iter().position(|f| f == &filename) {
                    self.files.remove(index);
                    self.label_filenames.remove(index);
                    self.files_index = self.files_index.saturating_sub(1);
                }
                // Remove from file statuses
                self.file_statuses.remove(&filename);
            },
            Msg::ClearError => self.error = None,
            Msg::SetDragState(is_dragging) => self.is_dragging = is_dragging,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.request_upload_files.is_some() == props.request_upload_files.is_some() {
            debug!("UploaderFiles not change");
            false
        } else {
            debug!("UploaderFiles change: {:?}", self.props.request_upload_files);
            self.props = props;
            if self.props.request_upload_files.is_some() {
                self.link.send_message(Msg::ParsingUrls);
            }
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {self.show_frame_upload_files()}
            <div class="buttons">
                {self.show_clear_btn()}
                {self.show_upload_files_btn()}
            </div>
        </>}
    }
}

impl UploaderFiles {
    fn show_frame_upload_files(&self) -> Html {
        let mut onchange_upload_files = self.link.callback(move |value| {
            if let ChangeData::Files(files) = value {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });
        let mut ondrop_upload_files = self.link.callback(move |value: DragEvent| {
            value.prevent_default();
            if let Some(files) = value.data_transfer().unwrap().files() {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });
        let mut ondragover_upload_files = self.link.callback(move |value: DragEvent| {
            value.prevent_default();
            Msg::Ignore
        });
        let ondragenter = self.link.callback(|_: DragEvent| Msg::SetDragState(true));
        let ondragleave = self.link.callback(|_: DragEvent| Msg::SetDragState(false));

        let is_clickable = true;

        let drag_classes = match (self.is_dragging, is_clickable) {
            (true, true) => "has-background-primary-light is-clickable",
            (false, true) => "has-background-white-ter is-clickable",
            _ => "has-background-grey-lighter is-disabled",
        };

        if self.active_loading_files_btn {
            onchange_upload_files = self.link.callback(|_: ChangeData| Msg::Ignore);
            ondrop_upload_files = self.link.callback(|_: DragEvent| Msg::Ignore);
            ondragover_upload_files = self.link.callback(|_: DragEvent| Msg::Ignore);
        }

        html!{<>
            <div class={classes!("box", "has-text-centered", "upload-dropbox", drag_classes)}
                 ondrop={ondrop_upload_files}
                 ondragover={ondragover_upload_files}
                 ondragenter={ondragenter}
                 ondragleave={ondragleave}>
                <div class="is-relative">
                    <div class="mb-4">
                        <i class="fas fa-cloud-upload-alt fa-3x has-text-info"></i>
                    </div>
                    <div style="pointer-events: none;">
                        <h3 class="title is-4 mb-5">{get_value_field(&449)}</h3> // Drag here or click to select
                        <p class="subtitle is-6 has-text-grey">{get_value_field(&self.props.text_choose_files)}</p>
                        {self.accept_image()}
                    </div>
                    <input id="file-input"
                           class="absolute-overlay"
                           type="file"
                           accept={self.props.accept.clone()}
                           onchange={onchange_upload_files}
                           disabled={self.active_loading_files_btn}
                           multiple={self.props.multiple} />
                </div>
            </div>
            <div class="mt-5">
                {self.select_files()}
            </div>
        </>}
    }

    fn select_files(&self) -> Html {
        match self.files.is_empty() {
            true => html!{
                <div class="has-text-centered py-6">
                    <i class="fas fa-file-upload fa-3x has-text-grey-light mb-4"></i>
                    <p class="has-text-grey">{get_value_field(&194)}</p>
                </div>
            },
            false => html!{<>
                <div class="is-flex is-justify-content-space-between is-align-items-center mb-4 pb-3" style="border-bottom: 1px solid #eee;">
                    <h4 class="title is-5 mb-0">{get_value_field(&450)}</h4> // Selected Files
                    <span class="tag is-white">{format!("{} files", self.files.len())}</span>
                </div>
                // <hr />
                <div class="columns is-multiline is-variable is-3 mb-5">
                    {for self.label_filenames.chunks(2).map(|chunk| {
                        html!{
                            <div class="column is-6">
                                <div class="columns is-multiline is-mobile">
                                    {for chunk.iter().map(|f_name| {
                                        html!{
                                            <div class="column is-12">
                                                {self.render_file_item(f_name)}
                                            </div>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    })}
                </div>
                {self.show_upload_summary()}
            </>}
        }
    }

    fn render_file_item(&self, filename: &str) -> Html {
        let filename_owned = filename.to_string();
        let file_status = self.file_statuses.get(filename).unwrap_or(&UploadStatus::Pending);
        let progress = self.progress_indicator.get(filename).unwrap_or(&0.0);

        let (status_classes, icon_color, status_icon, status_text) = match file_status {
            UploadStatus::Pending => (
                "has-border-warning has-background-warning-light",
                "has-text-warning",
                "fas fa-clock",
                get_value_field(&451) // Pending
            ),
            UploadStatus::Uploading => (
                "has-border-primary has-background-primary-light",
                "has-text-primary",
                "fas fa-spinner fa-spin",
                get_value_field(&452) // Uploading
            ),
            UploadStatus::Completed => (
                "has-border-success has-background-success-light",
                "has-text-success",
                "fas fa-check-circle",
                get_value_field(&453) // Completed
            ),
            UploadStatus::Failed(_) => (
                "has-border-danger has-background-danger-light",
                "has-text-danger",
                "fas fa-exclamation-circle",
                get_value_field(&454) // Failed
            ),
        };
        let onclick_remove = self.link.callback(move |_| Msg::RemoveFile(filename_owned.clone()));

        html!{
            <div class={classes!("box", "p-3", status_classes, "text-wrap-anywhere")}>
                <div class="is-flex is-align-items-center mb-2">
                    <div class={classes!("mr-3", icon_color)}>
                        <i class={format!("{} is-size-5", status_icon)} />
                    </div>
                    <div class="is-flex-grow-1 mr-2 text-break-all">
                        <div class="is-size-7 has-text-weight-semibold mb-1">{filename}</div>
                        <div class="is-size-7 has-text-grey">
                            {status_text}
                            {match file_status {
                                UploadStatus::Uploading => format!(" {:.2}%", progress * 100.0),
                                _ => String::new(),
                            }}
                        </div>
                    </div>
                    {match file_status {
                        UploadStatus::Pending => html!{<button class="delete is-small" onclick={onclick_remove} />},
                        _ => html!{},
                    }}
                </div>

                {match file_status {
                    UploadStatus::Uploading | UploadStatus::Completed =>
                        html!{<progress class="progress is-small is-primary" value={progress.to_string()} max="1"></progress>},
                    UploadStatus::Failed(error) => html!{
                        <div class="is-flex is-align-items-center mt-2">
                            <i class="fas fa-exclamation-triangle has-text-danger mr-2"></i>
                            <span class="is-size-7 has-text-danger">{error}</span>
                        </div>
                    },
                    _ => html!{},
                }}
            </div>
        }
    }

    fn show_upload_summary(&self) -> Html {
        if self.upload_success_count > 0 || self.upload_failed_count > 0 {
            html!{
                <div class="notification is-white mb-2">
                    {if self.upload_success_count > 0 {
                        html!{
                            <div class="is-flex is-align-items-center mb-2">
                                <i class="fas fa-check-circle has-text-success mr-2"></i>
                                <div class="has-text-success">
                                    <span>{self.upload_success_count}</span>
                                    <span>{get_value_field(&455)}</span> // files uploaded successfully
                                </div>
                            </div>
                        }
                    } else {
                        html!{}
                    }}
                    {if self.upload_failed_count > 0 {
                        html!{
                            <div class="is-flex is-align-items-center">
                                <i class="fas fa-exclamation-circle has-text-danger mr-2"></i>
                                <div class="has-text-danger">
                                    <span>{self.upload_failed_count}</span>
                                    <span>{get_value_field(&456)}</span> // files failed to upload
                                </div>
                            </div>
                        }
                    } else {
                        html!{}
                    }}
                </div>
            }
        } else {
            html!{}
        }
    }

    fn accept_image(&self) -> Html {
        match &self.props.accept == "image/*" {
            true => html!{
                <div class="is-size-7" style="overflow-wrap: anywhere">
                    {get_value_field(&183)}
                    {": .apng, .avif, .gif, .jpg, .jpeg, .jpe, .jif, .jfif, .png, .svg, .webp"}
                </div>
            },
            false => html!{},
        }
    }

    fn show_clear_btn(&self) -> Html {
        let onclick_clear_boxed = self.link.callback(|_| Msg::ClearFilesBoxed);
        html!{
            <button
              class="button is-fullwidth is-warning"
              onclick={onclick_clear_boxed}
              disabled={self.files.is_empty() || self.active_loading_files_btn} >
                <span class="icon"><i class="fas fa-trash"></i></span>
                <span>{get_value_field(&88)}</span>
            </button>
        }
    }

    fn show_upload_files_btn(&self) -> Html {
        let onclick_upload_files = self.link.callback(|_| Msg::UploadFiles);
        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-fullwidth is-success is-loading",
            false => "button is-fullwidth is-success",
        };
        html!{
            <button
              class={class_upload_btn}
              disabled={self.files.is_empty()}
              onclick={onclick_upload_files} >
                <span class="icon"><i class="fas fa-upload"></i></span>
                <span>{get_value_field(&87)}</span>
            </button>
        }
    }
}