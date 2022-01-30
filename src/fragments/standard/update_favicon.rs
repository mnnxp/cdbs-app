use graphql_client::GraphQLQuery;
use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{
    html, Callback, ChangeData, Component, ComponentLink, DragEvent, Html,
    Properties, ShouldRender
};
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::gqls::make_query;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::services::{PutUploadFile, UploadData, image_detector};
use crate::types::{UUID, UploadFile};

type FileName = String;
// type Chunks = bool;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct UploadStandardFavicon;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct ConfirmUploadCompleted;

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub standard_uuid: String,
    pub callback: Callback<()>,
}

/// For viewing favicon data on page
#[derive(Debug)]
pub struct UpdateStandardFaviconCard {
    error: Option<Error>,
    request_upload_data: UploadFile,
    request_upload_file: Callback<Result<Option<String>, Error>>,
    task_read: Option<(FileName, ReaderTask)>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
    get_result_up_file: bool,
    get_result_up_completed: bool,
    put_upload_file: PutUploadFile,
    file: Option<File>,
    active_loading_files_btn: bool,
    dis_upload_btn: bool,
}

pub enum Msg {
    RequestUploadData,
    RequestUploadFile(Vec<u8>),
    ResponseUploadFile(Result<Option<String>, Error>),
    RequestUploadCompleted,
    UpdateFile(Option<File>),
    GetUploadData(String),
    GetUploadFile(Option<String>),
    GetUploadCompleted(String),
    HideNotificationSuccess,
    ClearFileBoxed,
    ClearError,
    Ignore,
}

impl Component for UpdateStandardFaviconCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: UploadFile::default(),
            request_upload_file: link.callback(Msg::ResponseUploadFile),
            task_read: None,
            task: None,
            props,
            link,
            get_result_up_file: false,
            get_result_up_completed: false,
            put_upload_file: PutUploadFile::new(),
            file: None,
            active_loading_files_btn: false,
            dis_upload_btn: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestUploadData => {
                if let Some(file) = &self.file {
                    if image_detector(file.name().as_str()) {
                        // see loading button
                        self.active_loading_files_btn = true;

                        // debug!("RequestUploadData: {:?}", &self.request_update);
                        let ipt_standard_favicon_data = upload_standard_favicon::IptStandardFaviconData {
                            standardUuid: self.props.standard_uuid.clone(),
                            filename: file.name().to_string(),
                        };
                        spawn_local(async move {
                            let res = make_query(UploadStandardFavicon::build_query(
                                upload_standard_favicon::Variables { ipt_standard_favicon_data },
                            )).await.unwrap();
                            link.send_message(Msg::GetUploadData(res));
                        });
                    } else {
                        // select file is not image
                        link.send_message(Msg::ClearFileBoxed);
                    }
                }
            },
            Msg::RequestUploadFile(data) => {
                let request = UploadData {
                    upload_url: self.request_upload_data.upload_url.to_string(),
                    file_data: data,
                };
                self.task = Some(self.put_upload_file.put_file(request, self.request_upload_file.clone()));
            },
            Msg::ResponseUploadFile(Ok(res)) => link.send_message(Msg::GetUploadFile(res)),
            Msg::ResponseUploadFile(Err(err)) => {
                self.error = Some(err);
                self.task = None;
                self.task_read = None;
            },
            Msg::RequestUploadCompleted => {
                let file_uuids = vec![self.request_upload_data.file_uuid.clone()];
                spawn_local(async move {
                    let res = make_query(ConfirmUploadCompleted::build_query(
                        confirm_upload_completed::Variables { file_uuids })
                    ).await.unwrap();
                    debug!("ConfirmUploadCompleted: {:?}", res);
                    link.send_message(Msg::GetUploadCompleted(res));
                });
            },
            Msg::UpdateFile(op_file) => {
                if op_file.is_some() {
                    // enable bnt if file selected
                    self.dis_upload_btn = false;
                }
                self.file = op_file.clone();
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.request_upload_data = serde_json::from_value(
                            res_value.get("uploadStandardFavicon").unwrap().clone(),
                        ).unwrap();

                        if let Some(file) = self.file.clone() {
                            let file_name = file.name().clone();
                            let task = {
                                let callback = self.link.callback(move |data: FileData| {
                                    Msg::RequestUploadFile(data.content)
                                });
                                ReaderService::read_file(file, callback).unwrap()
                            };
                            self.task_read = Some((file_name, task));
                        }
                        debug!("file: {:?}", self.file);
                    }
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUploadFile(res) => {
                debug!("res: {:?}", res);
                self.get_result_up_file = true;
                link.send_message(Msg::RequestUploadCompleted)
            },
            Msg::GetUploadCompleted(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(
                            res_value.get("uploadCompleted").unwrap().clone()
                        ).unwrap();
                        self.get_result_up_completed = result > 0;
                        self.props.callback.emit(());
                        self.active_loading_files_btn = false;
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::HideNotificationSuccess => {
                link.send_message(Msg::ClearFileBoxed);
                self.get_result_up_completed = !self.get_result_up_completed;
            },
            Msg::ClearFileBoxed => {
                self.file = None;
                self.dis_upload_btn = true;
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
          <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
          {match self.get_result_up_completed {
              true => html!{self.show_success_upload()},
              false => html!{self.show_frame_upload_file()},
          }}
        </>}
    }
}

impl UpdateStandardFaviconCard {
    fn show_frame_upload_file(&self) -> Html {
        let onchange_favicon_file = self.link.callback(move |value| {
            if let ChangeData::Files(files) = value {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        let ondrop_favicon_file = self.link.callback(move |value: DragEvent| {
            value.prevent_default();
            if let Some(files) = value.data_transfer().unwrap().files() {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        let ondragover_favicon_file = self.link.callback(move |value: DragEvent| {
            value.prevent_default();
            Msg::Ignore
        });

        html!{
            <>
                <div class="file is-boxed has-name">
                  <label
                    for="favicon-file-input"
                    class="file-label"
                    style="width: 100%; text-align: center"
                  >
                    <input
                        id="favicon-file-input"
                        class="file-input"
                        type="file"
                        accept="image/*"
                        onchange={onchange_favicon_file} />
                    <span class="file-cta" ondrop=ondrop_favicon_file ondragover=ondragover_favicon_file >
                      <span class="file-icon">
                        <i class="fas fa-upload"></i>
                      </span>
                      <span class="file-label">{"Drop preview image here"}</span>
                    </span>
                    <div class="columns">
                        <div class="column">
                            <span class="file-name" style="overflow-wrap: anywhere">
                                {self.file.as_ref().map(|f| f.name().to_string()).unwrap_or_default()}
                            </span>
                        </div>
                        <div class="column">
                            <span class="has-text-grey-light is-size-6" style="overflow-wrap: anywhere">
                                {"Possible format: .apng, .avif, .gif, .jpg, .jpeg, .jpe, .jif, .jfif, .png, .svg, .webp."}
                            </span>
                        </div>
                    </div>
                  </label>
                </div>
                <div class="buttons">
                    { self.show_btn_clear() }
                    { self.show_btn_upload() }
                </div>
            </>
        }
    }

    fn show_btn_upload(&self) -> Html {
        let onclick_upload_favicon = self.link.callback(|_| Msg::RequestUploadData);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <a id="btn-new-favicon-upload"
                  class={class_upload_btn}
                  onclick=onclick_upload_favicon
                  disabled={self.dis_upload_btn} >
                { "Upload" }
            </a>
        }
    }

    fn show_btn_clear(&self) -> Html {
        let onclick_clear_boxed = self.link.callback(|_| Msg::ClearFileBoxed);

        html!{
            <a id="btn-new-favicon-clear"
                  // class="button is-danger"
                  class="button"
                  onclick=onclick_clear_boxed
                  disabled={self.dis_upload_btn} >
                { "Clear" }
            </a>
        }
    }

    fn show_success_upload(&self) -> Html {
        let onclick_hide_notification = self.link.callback(|_| Msg::HideNotificationSuccess);

        html!{
            <article class="message is-success">
              <div class="message-header">
                <p>{ "Success" }</p>
                <button class="delete" aria-label="close" onclick=onclick_hide_notification.clone() />
              </div>
              <div class="message-body">
                { "This favicon upload!" }
              </div>
            </article>
        }
    }
}
