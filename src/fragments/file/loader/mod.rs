mod commit_msg;
pub use commit_msg::commit_msg_field;

use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html, ChangeData, DragEvent};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use web_sys::FileList;

use crate::services::{PutUploadFile, UploadData, get_value_field, resp_parsing, image_detector};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
// use crate::fragments::switch_icon::res_file_btn;
use crate::types::{UUID, UploadFile};
use crate::gqls::make_query;
use crate::gqls::relate::{ConfirmUploadCompleted, confirm_upload_completed};

type FileName = String;

#[derive(Clone, Debug)]
pub struct FileUploadStatus {
    pub filename: String,
    pub progress: f32,
    pub status: UploadStatus,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UploadStatus {
    Pending,
    Uploading,
    Completed,
    Failed,
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
    put_upload_file: PutUploadFile,
    files: Vec<File>,
    files_index: u32,
    get_result_up_file: bool,
    get_result_up_completed: usize,
    active_loading_files_btn: bool,
    label_filenames: Vec<String>,
    file_statuses: HashMap<FileName, FileUploadStatus>,
    is_dragging: bool,
    upload_success_count: usize,
    upload_failed_count: usize,
    file_previews: HashMap<FileName, String>, // Store base64 previews
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
            put_upload_file: PutUploadFile::new(),
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
            file_previews: HashMap::new(),
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
                    self.file_statuses.insert(filename.clone(), FileUploadStatus {
                        filename: filename.clone(),
                        progress: 0.0,
                        status: UploadStatus::Pending,
                        error: None,
                    });
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
                    if let Some(status) = self.file_statuses.get_mut(&file_name) {
                        status.status = UploadStatus::Uploading;
                    }
                    
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
                    Some(upload_data) => {
                        let request = UploadData {
                            filename: filename.clone(),
                            upload_url: upload_data.upload_url.clone(),
                            file_data: data,
                        };
                        debug!("request: {:?}, {:?}", upload_data.file_uuid, upload_data.filename);
                        self.readers.remove(&filename);
                        self.task.push(
                            self.put_upload_file.put_file(
                                request,
                                self.response_upload_file.clone(),
                                self.upload_progress.clone(),
                            )
                        );
                        self.request_upload_confirm.push(upload_data.file_uuid.clone());
                    },
                    None => {
                        debug!("not found pre-signed url for upload the file: {:?}", filename);
                        // Update file status to failed
                        if let Some(status) = self.file_statuses.get_mut(&filename) {
                            status.status = UploadStatus::Failed;
                            status.error = Some("Presigned URL not found".to_string());
                        }
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
                self.error = Some(err.clone());
                // Update all uploading files status to failed
                for status in self.file_statuses.values_mut() {
                    if status.status == UploadStatus::Uploading {
                        status.status = UploadStatus::Failed;
                        status.error = Some("Upload failed".to_string());
                    }
                }
                self.upload_failed_count += self.file_statuses.values().filter(|s| s.status == UploadStatus::Failed).count();
                link.send_message(Msg::FinishUploadFiles);
            },
            Msg::UploadProgress((file_name_op, upload_progress)) => {
                if let Some(file_name) = file_name_op {
                    self.progress_indicator.insert(file_name.clone(), upload_progress);
                    // Update file status
                    if let Some(status) = self.file_statuses.get_mut(&file_name) {
                        status.progress = upload_progress;
                        if upload_progress >= 100.0 {
                            status.status = UploadStatus::Completed;
                            self.upload_success_count += 1;
                        }
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
                self.props.callback_upload_confirm.emit(self.get_result_up_completed);
                self.get_result_up_completed = 0;
                self.active_loading_files_btn = false;
                self.task.clear();
                self.readers.clear();
                self.presigned_url.clear();
                self.request_upload_confirm.clear();
                self.files.clear();
                self.label_filenames.clear();
                self.files_index = 0;
                self.file_statuses.clear();
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
            {self.show_upload_summary()}
            <div class="buttons">
                {self.show_clear_btn()}
                {self.show_upload_files_btn()}
            </div>
        </>}
    }
}

impl UploaderFiles {
    fn get_file_extension(&self, filename: &str) -> String {
        if let Some(dot_index) = filename.rfind('.') {
            let ext = &filename[dot_index + 1..];
            match ext.to_lowercase().as_str() {
                "jpg" | "jpeg" => "jpeg".to_string(),
                "png" => "png".to_string(),
                "gif" => "gif".to_string(),
                "webp" => "webp".to_string(),
                "svg" => "svg+xml".to_string(),
                "bmp" => "bmp".to_string(),
                "ico" => "x-icon".to_string(),
                _ => "jpeg".to_string(), // default
            }
        } else {
            "jpeg".to_string()
        }
    }

    fn get_file_base64(&self, _file: &File) -> String {
        // For now, we'll use a placeholder. In a real implementation,
        // you would need to read the file and convert it to base64
        // This is a simplified version - you might want to implement
        // proper file reading and base64 conversion
        "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAYEBQYFBAYGBQYHBwYIChAKCgkJChQODwwQFxQYGBcUFhYaHSUfGhsjHBYWICwgIyYnKSopGR8tMC0oMCUoKSj/2wBDAQcHBwoIChMKChMoGhYaKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCj/wAARCAABAAEDASIAAhEBAxEB/8QAFQABAQAAAAAAAAAAAAAAAAAAAAv/xAAUEAEAAAAAAAAAAAAAAAAAAAAA/8QAFQEBAQAAAAAAAAAAAAAAAAAAAAX/xAAUEQEAAAAAAAAAAAAAAAAAAAAA/9oADAMBAAIRAxEAPwCdABmX/9k=".to_string()
    }

    fn show_frame_upload_files(&self) -> Html {
        let onchange_upload_files = self.link.callback(move |value| {
            if let ChangeData::Files(files) = value {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });
        let ondrop_upload_files = self.link.callback(move |value: DragEvent| {
            value.prevent_default();
            if let Some(files) = value.data_transfer().unwrap().files() {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });
        let ondragover_upload_files = self.link.callback(move |value: DragEvent| {
            value.prevent_default();
            Msg::Ignore
        });
        let ondragenter = self.link.callback(|_: DragEvent| Msg::SetDragState(true));
        let ondragleave = self.link.callback(|_: DragEvent| Msg::SetDragState(false));
        
        let drag_class = if self.is_dragging {
            "upload-zone upload-zone--dragging"
        } else {
            "upload-zone"
        };
        
        html!{<>
            <div class={drag_class} 
                 ondrop={ondrop_upload_files} 
                 ondragover={ondragover_upload_files}
                 ondragenter={ondragenter}
                 ondragleave={ondragleave}>
                <div class="upload-zone__content">
                    <div class="upload-zone__icon">
                        <i class="fas fa-cloud-upload-alt"></i>
                    </div>
                    <div class="upload-zone__text">
                        <h3 class="upload-zone__title">{get_value_field(&self.props.text_choose_files)}</h3>
                        <p class="upload-zone__subtitle">{"Drag files here or click to select"}</p>
                        {self.accept_image()}
                    </div>
                    <input id="file-input"
                           class="upload-zone__input"
                           type="file"
                           accept={self.props.accept.clone()}
                           onchange={onchange_upload_files}
                           multiple={self.props.multiple} />
                </div>
            </div>
            <div class="upload-files-list">
                {self.select_files()}
            </div>
        </>}
    }

    fn select_files(&self) -> Html {
        match self.files.is_empty() {
            true => html!{
                <div class="upload-empty-state">
                    <i class="fas fa-file-upload upload-empty-state__icon"></i>
                    <p class="upload-empty-state__text">{get_value_field(&194)}</p>
                </div>
            },
            false => html!{<>
                <div class="upload-files-header">
                    <h4 class="upload-files-header__title">{"Selected Files"}</h4>
                    <span class="upload-files-header__count">{format!("{} files", self.files.len())}</span>
                </div>
                <div class="upload-files-grid">
                    {for self.label_filenames.iter().map(|f_name| self.render_file_item(f_name))}
                </div>
                <p class="help">{get_value_field(&85)}</p>
            </>}
        }
    }

    fn render_file_item(&self, filename: &str) -> Html {
        let filename_owned = filename.to_string();
        let status = self.file_statuses.get(filename).cloned();
        let progress = self.progress_indicator.get(filename).unwrap_or(&0.0);
        
        // Find the file object for preview
        let file_obj = self.files.iter().find(|f| f.name() == filename);
        
        let status_class = match status.as_ref().map(|s| &s.status) {
            Some(UploadStatus::Pending) => "file-item--pending",
            Some(UploadStatus::Uploading) => "file-item--uploading",
            Some(UploadStatus::Completed) => "file-item--completed",
            Some(UploadStatus::Failed) => "file-item--failed",
            None => "file-item--pending",
        };

        let status_icon = match status.as_ref().map(|s| &s.status) {
            Some(UploadStatus::Pending) => html!{<i class="fas fa-clock"></i>},
            Some(UploadStatus::Uploading) => html!{<i class="fas fa-spinner fa-spin"></i>},
            Some(UploadStatus::Completed) => html!{<i class="fas fa-check-circle"></i>},
            Some(UploadStatus::Failed) => html!{<i class="fas fa-exclamation-circle"></i>},
            None => html!{<i class="fas fa-clock"></i>},
        };

        let status_text = match status.as_ref().map(|s| &s.status) {
            Some(UploadStatus::Pending) => "Pending",
            Some(UploadStatus::Uploading) => "Uploading",
            Some(UploadStatus::Completed) => "Completed",
            Some(UploadStatus::Failed) => "Failed",
            None => "Pending",
        };

        // Check if file is an image for preview
        let is_image = file_obj.map_or(false, |f| {
            let name = f.name().to_lowercase();
            name.ends_with(".jpg") || name.ends_with(".jpeg") || name.ends_with(".png") || 
            name.ends_with(".gif") || name.ends_with(".webp") || name.ends_with(".svg") ||
            name.ends_with(".bmp") || name.ends_with(".ico")
        });

        let onclick_remove = self.link.callback(move |_| Msg::RemoveFile(filename_owned.clone()));

        html!{
            <div class={format!("file-item {}", status_class)}>
                <div class="file-item__header">
                    <div class="file-item__icon">
                        {status_icon}
                    </div>
                    <div class="file-item__info">
                        <div class="file-item__name">{filename}</div>
                        <div class="file-item__status">{status_text}</div>
                    </div>
                    {if let Some(ref status) = status {
                        if status.status == UploadStatus::Pending {
                            html!{
                                <button class="file-item__remove-btn" onclick={onclick_remove}>
                                    <i class="fas fa-times"></i>
                                </button>
                            }
                        } else {
                            html!{}
                        }
                    } else {
                        html!{}
                    }}
                </div>
                
                {if let Some(ref status) = status {
                    if status.status == UploadStatus::Pending && is_image {
                        let filename_owned = filename.to_string();
                        html!{
                            <div class="file-item__preview">
                                <img class="file-item__preview-img" src={format!("data:image/{};base64,{}", 
                                    self.get_file_extension(filename), 
                                    self.get_file_base64(file_obj.unwrap())
                                )} alt={filename_owned} />
                            </div>
                        }
                    } else {
                        html!{}
                    }
                } else {
                    html!{}
                }}
                
                {if let Some(ref status) = status {
                    if status.status == UploadStatus::Uploading || status.status == UploadStatus::Completed {
                        html!{
                            <div class="file-item__progress">
                                <div class="progress-bar">
                                    <div class="progress-bar__fill" style={format!("width: {}%", progress)}></div>
                                </div>
                                <span class="progress-text">{format!("{:.1}%", progress)}</span>
                            </div>
                        }
                    } else {
                        html!{}
                    }
                } else {
                    html!{}
                }}
                {if let Some(ref status) = status {
                    if let Some(error) = &status.error {
                        html!{
                            <div class="file-item__error">
                                <i class="fas fa-exclamation-triangle"></i>
                                <span>{error}</span>
                            </div>
                        }
                    } else {
                        html!{}
                    }
                } else {
                    html!{}
                }}
            </div>
        }
    }

    fn show_upload_summary(&self) -> Html {
        if self.upload_success_count > 0 || self.upload_failed_count > 0 {
            html!{
                <div class="upload-summary">
                    {if self.upload_success_count > 0 {
                        html!{
                            <div class="upload-summary__item upload-summary__item--success">
                                <i class="fas fa-check-circle"></i>
                                <span>{format!("{} files uploaded successfully", self.upload_success_count)}</span>
                            </div>
                        }
                    } else {
                        html!{}
                    }}
                    {if self.upload_failed_count > 0 {
                        html!{
                            <div class="upload-summary__item upload-summary__item--error">
                                <i class="fas fa-exclamation-circle"></i>
                                <span>{format!("{} files failed to upload", self.upload_failed_count)}</span>
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
                <div class="upload-zone__formats">
                    <span class="upload-zone__formats-label">{"Supported formats"}</span>
                    <span class="upload-zone__formats-list">{".apng, .avif, .gif, .jpg, .jpeg, .jpe, .jif, .jfif, .png, .svg, .webp"}</span>
                </div>
            },
            false => html!{},
        }
    }

    fn show_clear_btn(&self) -> Html {
        let onclick_clear_boxed = self.link.callback(|_| Msg::ClearFilesBoxed);
        html!{
            <button
              class="button is-fullwidth is-warning upload-btn upload-btn--clear"
              onclick={onclick_clear_boxed}
              disabled={self.files.is_empty() || self.active_loading_files_btn} >
                <span class="button__icon"><i class="fas fa-trash"></i></span>
                <span>{get_value_field(&88)}</span>
            </button>
        }
    }

    fn show_upload_files_btn(&self) -> Html {
        let onclick_upload_files = self.link.callback(|_| Msg::UploadFiles);
        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-fullwidth is-success is-loading upload-btn upload-btn--upload",
            false => "button is-fullwidth is-success upload-btn upload-btn--upload",
        };
        html!{
            <button
              class={class_upload_btn}
              disabled={self.files.is_empty()}
              onclick={onclick_upload_files} >
                <span class="button__icon"><i class="fas fa-upload"></i></span>
                <span>{get_value_field(&87)}</span>
            </button>
        }
    }
}
