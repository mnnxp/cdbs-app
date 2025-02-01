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
use crate::fragments::switch_icon::res_file_btn;
use crate::types::{UUID, UploadFile};
use crate::gqls::make_query;
use crate::gqls::relate::{ConfirmUploadCompleted, confirm_upload_completed};

type FileName = String;

pub struct UploaderFiles {
    error: Option<Error>,
    response_upload_file: Callback<Result<Option<String>, Error>>,
    request_upload_confirm: Vec<UUID>,
    readers: HashMap<FileName, ReaderTask>,
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
    RequestUploadCompleted,
    ResponseError(Error),
    GetUploadFile,
    GetUploadCompleted(String),
    UpdateFiles(FileList),
    FinishUploadFiles,
    ClearFilesBoxed,
    ClearError,
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
                            upload_url: upload_data.upload_url.clone(),
                            file_data: data,
                        };
                        debug!("request: {:?}, {:?}", upload_data.file_uuid, upload_data.filename);
                        self.readers.remove(&filename);
                        self.task.push(
                            self.put_upload_file.put_file(
                                request,
                                self.response_upload_file.clone()
                            )
                        );
                        self.request_upload_confirm.push(upload_data.file_uuid.clone());
                    },
                    None => debug!("not found pre-signed url for upload the file: {:?}", filename),
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
            },
            Msg::ClearFilesBoxed => {
                self.files.clear();
                self.label_filenames.clear();
                self.files_index = 0;
            },
            Msg::ClearError => self.error = None,
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
        html!{<>
            <div class="file has-name is-boxed is-centered">
                <label class="file-label" style="width: 100%">
                  <input id="file-input"
                  class="file-input"
                  type="file"
                  // accept="image/*,application/vnd*,application/rtf,text/*,.pdf"
                  accept={self.props.accept.clone()}
                  onchange={onchange_upload_files}
                  multiple={self.props.multiple} />
                <span class="file-cta" ondrop={ondrop_upload_files} ondragover={ondragover_upload_files}>
                  <span class="file-icon">
                    <i class="fas fa-upload"></i>
                  </span>
                  // Choose files…
                  <span class="file-label">{get_value_field(&self.props.text_choose_files)}</span>
                  {self.accept_image()}
                </span>
                <div class="column">
                    {self.select_files()}
                </div>
              </label>
            </div>
        </>}
    }

    fn select_files(&self) -> Html {
        let onclick_file_info_btn = self.link.callback(|_| Msg::Ignore);
        match self.files.is_empty() {
            true => html!{<p class={"subtitle"}>{get_value_field(&194)}</p>}, // No file uploaded
            false => html!{<>
                {for self.label_filenames.iter().map(|f_name| html!{
                    {res_file_btn(onclick_file_info_btn.clone(), f_name.clone())}
                })}
                <p class="help">{get_value_field(&85)}</p>
            </>}
        }
    }

    fn accept_image(&self) -> Html {
        match &self.props.accept == "image/*" {
            true => html!{
                <span class="is-size-7" style="overflow-wrap: anywhere">
                    {get_value_field(&183)}
                    {": .apng, .avif, .gif, .jpg, .jpeg, .jpe, .jif, .jfif, .png, .svg, .webp"}
                </span>
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
                <span>{get_value_field(&87)}</span>
            </button>
        }
    }
}
