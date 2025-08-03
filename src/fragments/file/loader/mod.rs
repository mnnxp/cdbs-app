mod commit_msg;
pub use commit_msg::commit_msg_field;

use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use base64::Engine;
use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html, ChangeData, DragEvent, MouseEvent};

use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use web_sys::FileList;

use crate::services::{PutUploadFile, UploadData, get_value_field, resp_parsing, image_detector};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
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
    show_image_modal: bool, // Whether to show image modal
    modal_image_url: String, // Image URL displayed in modal
    simulated_progress: HashMap<FileName, f32>, // Simulated progress
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
    #[prop_or_default]
    pub existing_images: Vec<String>, // URLs of existing images to display
    #[prop_or(false)]
    pub single_image_mode: bool, // Whether in single image mode (e.g., update image for preview)
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
    FileRead(FileName, Result<Vec<u8>, Error>), // File read result
    FinishUploadFiles,
    ClearFilesBoxed,
    ClearError,
    SetDragState(bool),
    RemoveFile(String),
    ShowImageModal(String), // Show image modal
    HideImageModal, // Hide image modal
    SimulateProgress(String), // Simulate progress bar
    UploadSuccess(String), // Upload success handler
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
            show_image_modal: false,
            modal_image_url: String::new(),
            simulated_progress: HashMap::new(),
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
                for file in &self.files {
                    let file_name = file.name().clone();
                    
                    // Update file status to uploading
                    if let Some(status) = self.file_statuses.get_mut(&file_name) {
                        status.status = UploadStatus::Uploading;
                    }
                    
                    // Start simulated progress
                    let filename_clone = file_name.clone();
                    let link = self.link.clone();
                    spawn_local(async move {
                        // Delay 50ms before starting simulation
                        gloo_timers::callback::Timeout::new(50, move || {
                            link.send_message(Msg::SimulateProgress(filename_clone.clone()));
                        }).forget();
                    });
                    
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
                        // Update file status to failed
                        if let Some(status) = self.file_statuses.get_mut(&filename) {
                            status.status = UploadStatus::Failed;
                            status.error = Some("Presigned URL not found".to_string());
                        }
                        self.upload_failed_count += 1;
                    },
                }
            },
            Msg::ResponseUploadFile(Ok(_)) => {
                // When interface returns success, set progress to 100% and status to Completed
                // Then delay 300ms before continuing with subsequent logic
                for (filename, status) in self.file_statuses.iter_mut() {
                    if status.status == UploadStatus::Uploading {
                        status.status = UploadStatus::Completed;
                        status.progress = 100.0;
                        
                        // Trigger delayed success handling
                        let filename_clone = filename.clone();
                        let link = self.link.clone();
                        spawn_local(async move {
                            // Delay 300ms before continuing
                            gloo_timers::callback::Timeout::new(300, move || {
                                link.send_message(Msg::UploadSuccess(filename_clone.clone()));
                            }).forget();
                        });
                    }
                }
            },
            Msg::ResponseUploadFile(Err(err)) => {
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
                            // Set status to Completed and progress to 100%
                            status.status = UploadStatus::Completed;
                            status.progress = 100.0;
                            self.upload_success_count += 1;
                            
                            // Trigger delayed success handling
                            let filename_clone = file_name.clone();
                            let link = self.link.clone();
                            spawn_local(async move {
                                // Delay 300ms before continuing
                                gloo_timers::callback::Timeout::new(300, move || {
                                    link.send_message(Msg::UploadSuccess(filename_clone.clone()));
                                }).forget();
                            });
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
                    link.send_message(Msg::GetUploadCompleted(res));
                });
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetUploadFile => {
                self.files_index -= 1;
                if self.files_index == 0 {
                    self.get_result_up_file = true;
                    link.send_message(Msg::RequestUploadCompleted);
                }
            },
            Msg::GetUploadCompleted(res) => {
                match resp_parsing(res, "uploadCompleted") {
                    Ok(result) => {
                        self.get_result_up_completed = result;
                        link.send_message(Msg::FinishUploadFiles);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateFiles(files) => {
                // Clear previous files if in single image mode
                if self.props.single_image_mode {
                    self.files.clear();
                    self.label_filenames.clear();
                    self.file_statuses.clear();
                    self.file_previews.clear();
                    self.progress_indicator.clear();
                    self.simulated_progress.clear();
                    self.readers.clear();
                    self.files_index = 0;
                }
                
                let mut temp_index = 0;
                while let Some(file) = files.get(temp_index) {
                    if !self.props.multiple && temp_index != 0 {
                        return true
                    }
                    if &self.props.accept == "image/*" && !image_detector(&file.name()){
                        return true
                    }
                    temp_index += 1;
                    self.files.push(file.clone());
                    self.label_filenames.push(file.name().clone());
                    
                    // Initialize file status
                    let filename = file.name().clone();
                    self.file_statuses.insert(filename.clone(), FileUploadStatus {
                        filename: filename.clone(),
                        progress: 0.0,
                        status: UploadStatus::Pending,
                        error: None,
                    });
                    
                    // Read file for preview if it's an image
                    if image_detector(&file.name()) {
                        let filename_clone = filename.clone();
                        let callback = self.link.callback(move |file_data: FileData| {
                            Msg::FileRead(filename_clone.clone(), Ok(file_data.content))
                        });
                        let reader = ReaderService::read_file(file, callback).unwrap();
                        self.readers.insert(filename, reader);
                    }
                }
                self.files_index += temp_index;
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
                self.file_previews.clear();
                self.progress_indicator.clear();
                self.simulated_progress.clear();
            },
            Msg::ClearFilesBoxed => {
                self.files.clear();
                self.label_filenames.clear();
                self.files_index = 0;
                self.file_statuses.clear();
                self.file_previews.clear();
                self.progress_indicator.clear();
                self.readers.clear();
                self.task.clear();
                self.presigned_url.clear();
                self.request_upload_confirm.clear();
                self.upload_success_count = 0;
                self.upload_failed_count = 0;
                self.active_loading_files_btn = false;
                self.get_result_up_file = false;
                self.get_result_up_completed = 0;
                self.simulated_progress.clear();
            },
            Msg::RemoveFile(filename) => {
                // Remove file from files list
                if let Some(index) = self.label_filenames.iter().position(|f| f == &filename) {
                    self.files.remove(index);
                    self.label_filenames.remove(index);
                    self.files_index = self.files_index.saturating_sub(1);
                }
                // Remove from all related collections
                self.file_statuses.remove(&filename);
                self.file_previews.remove(&filename);
                self.progress_indicator.remove(&filename);
                self.presigned_url.remove(&filename);
            },
            Msg::ShowImageModal(image_url) => {
                self.show_image_modal = true;
                self.modal_image_url = image_url;
            }
            Msg::FileRead(filename, result) => {
                match result {
                    Ok(data) => {
                        // Convert to base64
                        let base64_data = base64::engine::general_purpose::STANDARD.encode(&data);
                        let mime_type = self.get_file_extension(&filename);
                        let data_url = format!("data:image/{};base64,{}", mime_type, base64_data);
                        self.file_previews.insert(filename, data_url);
                    },
                    Err(_) => {
                        // Failed to read file for preview
                    }
                }
            },
            Msg::HideImageModal => {
                self.show_image_modal = false;
                self.modal_image_url.clear();
            }
            Msg::SimulateProgress(filename) => {
                // Simulate progress bar, slowly increase to 96%
                let current_progress = self.simulated_progress.get(&filename).unwrap_or(&0.0);
                let new_progress = if *current_progress < 96.0 {
                    current_progress + 2.0 // Increase by 2% each time
                } else {
                    96.0
                };
                self.simulated_progress.insert(filename.clone(), new_progress);
                
                // Update file status
                if let Some(status) = self.file_statuses.get_mut(&filename) {
                    status.progress = new_progress;
                }
                
                // Continue simulation if not yet at 96%
                if new_progress < 96.0 {
                    let filename_clone = filename.clone();
                    let link = self.link.clone();
                    spawn_local(async move {
                        // Delay 100ms before continuing
                        gloo_timers::callback::Timeout::new(100, move || {
                            link.send_message(Msg::SimulateProgress(filename_clone.clone()));
                        }).forget();
                    });
                }
            }
            Msg::UploadSuccess(_filename) => {
                // Upload success handling logic
                
                // Continue with subsequent success logic
                link.send_message(Msg::GetUploadFile);
            }
            Msg::ClearError => self.error = None,
            Msg::SetDragState(is_dragging) => self.is_dragging = is_dragging,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Check if existing_images has changed
        let existing_images_changed = self.props.existing_images != props.existing_images;
        
        // Check if request_upload_files has changed
        let request_upload_files_changed = self.props.request_upload_files.is_some() != props.request_upload_files.is_some();
        
        if !existing_images_changed && !request_upload_files_changed {
            false
        } else {
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
            <div class="upload-container">
                {self.show_main_upload_area()}
                // Only show selected files when no current images to avoid duplication
                {if self.props.existing_images.is_empty() {
                    html!{}
                } else {
                    self.show_selected_files()
                }}
            </div>
            {self.show_upload_summary()}
            <div class="buttons">
                {self.show_clear_btn()}
                {self.show_upload_files_btn()}
            </div>
            {self.show_image_modal()}
        </>}
    }
}

impl UploaderFiles {
    fn show_existing_images(&self) -> Html {
        if self.props.existing_images.is_empty() {
            return html!{};
        }
        
        html!{
            <div class="existing-images-section">
                <div class="existing-images-header">
                    <h4 class="existing-images-header__title">{"Current Image"}</h4>
                </div>
                <div class="existing-image-main">
                    {for self.props.existing_images.iter().enumerate().map(|(index, image_url)| {
                        let image_url_clone = image_url.clone();
                        let onclick_show_modal = self.link.callback(move |_| Msg::ShowImageModal(image_url_clone.clone()));
                        html!{
                            <div class="existing-image-main-item">
                                <img class="existing-image-main__img clickable" 
                                     src={image_url.clone()} 
                                     alt={format!("Current image {}", index + 1)}
                                     onclick={onclick_show_modal} />
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    }

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



    fn show_main_upload_area(&self) -> Html {
        if self.props.existing_images.is_empty() {
            // If no current images, show complete upload area
            self.show_frame_upload_files()
        } else {
            // If current images exist, show left-right layout
            html!{
                <div class="upload-main-layout">
                    <div class="upload-main-left">
                        {self.show_existing_images()}
                    </div>
                    <div class="upload-main-right">
                        {self.show_upload_zone()}
                    </div>
                </div>
            }
        }
    }

    fn show_upload_zone(&self) -> Html {
        self.render_upload_zone(false)
    }

    fn show_frame_upload_files(&self) -> Html {
        html!{<>
            {self.render_upload_zone(true)}
            <div class="upload-files-list">
                {self.select_files()}
            </div>
        </>}
    }

    fn render_upload_zone(&self, _include_files_list: bool) -> Html {
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
        
        html!{
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
        }
    }

    fn show_selected_files(&self) -> Html {
        self.render_files_grid()
    }

    fn select_files(&self) -> Html {
        self.render_files_grid()
    }

    fn render_files_grid(&self) -> Html {
        match self.files.is_empty() {
            true => html!{},
            false => {
                if self.props.single_image_mode {
                    // Single image mode: only show the latest image
                    if let Some(latest_filename) = self.label_filenames.last() {
                        html!{<>
                            <div class="upload-files-grid">
                                {self.render_file_item(latest_filename)}
                            </div>
                        </>}
                    } else {
                        html!{}
                    }
                } else {
                    // Multi-file mode: show all selected files
                    html!{<>
                        <div class="upload-files-grid">
                            {for self.label_filenames.iter().map(|filename| self.render_file_item(filename))}
                        </div>
                    </>}
                }
            }
        }
    }

    fn render_file_item(&self, filename: &str) -> Html {
        let filename_owned = filename.to_string();
        let status = self.file_statuses.get(filename).cloned();
        let progress = self.get_file_progress(filename, &status);
        let file_obj = self.files.iter().find(|f| f.name() == filename);
        
        let status_info = self.get_status_info(&status);
        let is_image = self.is_image_file(file_obj);



        let onclick_remove = self.link.callback(move |_| Msg::RemoveFile(filename_owned.clone()));

        html!{
            <div class={format!("file-item {}", status_info.class)}>
                <div class="file-item__content">
                    {if let Some(ref status) = status {
                        if (status.status == UploadStatus::Pending || status.status == UploadStatus::Uploading || status.status == UploadStatus::Completed) && is_image {
                            let filename_owned = filename.to_string();
                            let preview_url = if let Some(preview) = self.file_previews.get(filename) {
                                preview.clone()
                            } else {
                                "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAYEBQYFBAYGBQYHBwYIChAKCgkJChQODwwQFxQYGBcUFhYaHSUfGhsjHBYWICwgIyYnKSopGR8tMC0oMCUoKSj/2wBDAQcHBwoIChMKChMoGhYaKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCj/wAARCAABAAEDASIAAhEBAxEB/8QAFQABAQAAAAAAAAAAAAAAAAAAAAv/xAAUEAEAAAAAAAAAAAAAAAAAAAAA/8QAFQEBAQAAAAAAAAAAAAAAAAAAAAX/xAAUEQEAAAAAAAAAAAAAAAAAAAAA/9oADAMBAAIRAxEAPwCdABmX/9k=".to_string()
                            };
                            let preview_url_clone = preview_url.clone();
                            let onclick_show_modal = self.link.callback(move |_| Msg::ShowImageModal(preview_url_clone.clone()));
                            html!{
                                <div class="file-item__thumbnail">
                                    <img class="file-item__thumbnail-img clickable" 
                                         src={preview_url} 
                                         alt={filename_owned}
                                         onclick={onclick_show_modal} />
                                </div>
                            }
                        } else {
                            html!{
                                <div class="file-item__thumbnail">
                                    <div class="file-item__thumbnail-placeholder">
                                        <i class="fas fa-file"></i>
                                    </div>
                                </div>
                            }
                        }
                    } else {
                        html!{
                            <div class="file-item__thumbnail">
                                <div class="file-item__thumbnail-placeholder">
                                    <i class="fas fa-file"></i>
                                </div>
                            </div>
                            }
                        }
                    }
                    
                    <div class="file-item__main">
                        <div class="file-item__header">
                            <div class="file-item__icon">
                                {status_info.icon}
                            </div>
                            <div class="file-item__info">
                                <div class="file-item__name">{filename}</div>
                                <div class="file-item__status">{status_info.text}</div>
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
                            if status.status == UploadStatus::Uploading || status.status == UploadStatus::Completed {
                                let display_progress = if status.status == UploadStatus::Completed { 100.0 } else { *progress };
                                html!{
                                    <div class="file-item__progress">
                                        <div class="progress-bar">
                                            <div class="progress-bar__fill" style={format!("width: {}%", display_progress)}></div>
                                        </div>
                                        <span class="progress-text">{format!("{:.0}%", display_progress)}</span>
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
                </div>
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

    fn show_image_modal(&self) -> Html {
        if !self.show_image_modal {
            return html!{};
        }
        
        let onclick_hide_modal = self.link.callback(|_| Msg::HideImageModal);
        let onclick_modal_content = self.link.callback(|e: MouseEvent| {
            e.stop_propagation();
            Msg::Ignore
        });
        
        html!{
            <div class="modal is-active" onclick={onclick_hide_modal.clone()}>
                <div class="modal-background"></div>
                <div class="modal-content" onclick={onclick_modal_content}>
                    <p class="image">
                        <img loading="lazy" src={self.modal_image_url.clone()} alt="Full size image" />
                    </p>
                </div>
                <button aria-label="close" class="modal-close is-large" onclick={onclick_hide_modal}></button>
            </div>
        }
    }

    // Helper methods for file rendering
    fn get_file_progress(&self, filename: &str, status: &Option<FileUploadStatus>) -> &f32 {
        if let Some(status) = status.as_ref() {
            if status.status == UploadStatus::Uploading {
                // Use simulated progress during upload
                self.simulated_progress.get(filename).unwrap_or(&0.0)
            } else {
                // Use actual progress for other statuses
                self.progress_indicator.get(filename).unwrap_or(&0.0)
            }
        } else {
            &0.0
        }
    }

    fn get_status_info(&self, status: &Option<FileUploadStatus>) -> StatusInfo {
        let status_enum = status.as_ref().map(|s| &s.status);
        
        let class = match status_enum {
            Some(UploadStatus::Pending) => "file-item--pending",
            Some(UploadStatus::Uploading) => "file-item--uploading",
            Some(UploadStatus::Completed) => "file-item--completed",
            Some(UploadStatus::Failed) => "file-item--failed",
            None => "file-item--pending",
        };

        let icon = match status_enum {
            Some(UploadStatus::Pending) => html!{<i class="fas fa-clock"></i>},
            Some(UploadStatus::Uploading) => html!{<i class="fas fa-spinner fa-spin"></i>},
            Some(UploadStatus::Completed) => html!{<i class="fas fa-check-circle"></i>},
            Some(UploadStatus::Failed) => html!{<i class="fas fa-exclamation-circle"></i>},
            None => html!{<i class="fas fa-clock"></i>},
        };

        let text = match status_enum {
            Some(UploadStatus::Pending) => "Pending",
            Some(UploadStatus::Uploading) => "Uploading",
            Some(UploadStatus::Completed) => "Completed",
            Some(UploadStatus::Failed) => "Failed",
            None => "Pending",
        };

        StatusInfo { class, icon, text }
    }

    fn is_image_file(&self, file_obj: Option<&File>) -> bool {
        file_obj.map_or(false, |f| {
            let name = f.name().to_lowercase();
            name.ends_with(".jpg") || name.ends_with(".jpeg") || name.ends_with(".png") || 
            name.ends_with(".gif") || name.ends_with(".webp") || name.ends_with(".svg") ||
            name.ends_with(".bmp") || name.ends_with(".ico")
        })
    }
}

// Helper struct for status information
struct StatusInfo {
    class: &'static str,
    icon: Html,
    text: &'static str,
}
