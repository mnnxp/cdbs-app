use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{
    html, Callback, Component, ComponentLink, Html,
    Properties, ShouldRender, ChangeData, DragEvent
};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::fragments::list_errors::ListErrors;
use crate::error::{Error, get_error};
use crate::services::{PutUploadFile, UploadData, get_value_field};
use crate::types::UploadFile;
use crate::gqls::{
    make_query,
    user::{UploadUserFavicon, upload_user_favicon},
    company::{UploadCompanyFavicon, upload_company_favicon},
    relate::{ConfirmUploadCompleted, confirm_upload_completed},
};

type FileName = String;

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub callback: Callback<String>,
    pub company_uuid: Option<String>,
}

/// For upload favicon file to user and company
#[derive(Debug)]
pub struct UpdateFaviconBlock {
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
    dis_upload_btn: bool,
    active_loading_files_btn: bool,
}

pub enum Msg {
    RequestUploadData,
    RequestUploadUserData,
    RequestUploadCompanyData,
    RequestUploadFile(Vec<u8>),
    ResponseUploadFile(Result<Option<String>, Error>),
    RequestUploadCompleted,
    ResponseError(Error),
    UpdateFile(Option<File>),
    GetUploadData(String),
    GetUploadFile(Option<String>),
    GetUploadCompleted(String),
    ClearFileBoxed,
    ClearError,
    Ignore,
}

impl Component for UpdateFaviconBlock {
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
            dis_upload_btn: true,
            active_loading_files_btn: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestUploadData => {
                // see loading button
                self.active_loading_files_btn = true;

                match &self.props.company_uuid {
                    Some(_) => self.link.send_message(Msg::RequestUploadCompanyData),
                    None => self.link.send_message(Msg::RequestUploadUserData),
                }
            },
            Msg::RequestUploadUserData => {
                if let Some(file) = &self.file {
                    // debug!("RequestUploadData: {:?}", &self.request_update);
                    let filename_upload_favicon = file.name().to_string();
                    spawn_local(async move {
                        let res = make_query(UploadUserFavicon::build_query(
                            upload_user_favicon::Variables {
                                filename_upload_favicon,
                            }
                        )).await;
                        link.send_message(Msg::GetUploadData(res.unwrap()));
                    })
                }
            },
            Msg::RequestUploadCompanyData => {
                if let Some(file) = &self.file {
                    let company_uuid = self.props.company_uuid.as_ref().map(|u| u.clone()).unwrap();
                    let filename_upload_favicon = file.name().clone();
                    spawn_local(async move {
                        let res = make_query(UploadCompanyFavicon::build_query(
                            upload_company_favicon::Variables {
                                company_uuid,
                                filename_upload_favicon,
                            }
                        )).await.unwrap();
                        link.send_message(Msg::GetUploadData(res));
                    })
                }
            },
            Msg::RequestUploadFile(data) => {
                let request = UploadData {
                    upload_url: self.request_upload_data.upload_url.to_string(),
                    file_data: data,
                };
                self.task = Some(self.put_upload_file.put_file(request, self.request_upload_file.clone()));
            },
            Msg::ResponseUploadFile(Ok(res)) => {
                link.send_message(Msg::GetUploadFile(res))
            },
            Msg::ResponseUploadFile(Err(err)) => {
                self.error = Some(err);
                self.task = None;
                self.task_read = None;
            },
            Msg::RequestUploadCompleted => {
                let file_uuids = vec![self.request_upload_data.file_uuid.clone()];
                spawn_local(async move {
                    let res = make_query(ConfirmUploadCompleted::build_query(confirm_upload_completed::Variables {
                        file_uuids,
                    })).await.unwrap();
                    link.send_message(Msg::GetUploadCompleted(res));
                });
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
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
                        self.request_upload_data = match &self.props.company_uuid {
                            Some(_) => serde_json::from_value(res_value.get("uploadCompanyFavicon").unwrap().clone()).unwrap(),
                            None => serde_json::from_value(res_value.get("uploadFavicon").unwrap().clone()).unwrap(),
                        };

                        if let Some(file) = self.file.clone() {
                            let file_name = file.name().clone();
                            let task = {
                                let callback = self
                                    .link
                                    .callback(move |data: FileData| Msg::RequestUploadFile(data.content));
                                ReaderService::read_file(file, callback).unwrap()
                            };
                            self.task_read = Some((file_name, task));
                        }
                        debug!("file: {:?}", self.file);
                    },
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
                        self.active_loading_files_btn = false;
                    },
                    true => self.error = Some(get_error(&data)),
                }
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
          <div class="column">
              {match self.get_result_up_completed {
                  true => self.show_success_upload(),
                  false => html!{<>
                      { self.show_frame_upload_file() }
                      <br/>
                      <div class="buttons">
                          { self.show_btn_clear() }
                          { self.show_btn_upload() }
                      </div>
                  </>},
              }}
          </div>
        </>}
    }
}

impl UpdateFaviconBlock {
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

        html!{<div class="block">
            <div class="columns">
                <div class="column">
                    <div class="file is-large is-boxed has-name">
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
                          <span class="file-label">{ get_value_field(&93) }</span>
                        </span>
                      </label>
                    </div>
                </div>
                <div class="column">
                    <div class="has-text-grey-light" style="overflow-wrap: anywhere">
                        { get_value_field(&91) }
                    </div>
                    <br/>
                    <div id="select-file" style="overflow-wrap: anywhere">
                        <span>{ get_value_field(&85) }</span>
                        <span class="overflow-title has-text-weight-bold">
                            {self.file.as_ref()
                                .map(|f| f.name().to_string())
                                .unwrap_or_default()}
                        </span>
                    </div>
                </div>
            </div>
        </div>}
    }

    fn show_btn_upload(&self) -> Html {
        let onclick_upload_favicon = self.link
            .callback(|_| Msg::RequestUploadData);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <a id="btn-new-favicon-upload"
                  class={class_upload_btn}
                  onclick=onclick_upload_favicon
                  disabled={self.dis_upload_btn} >
                { get_value_field(&87) }
            </a>
        }
    }

    fn show_btn_clear(&self) -> Html {
        let onclick_clear_boxed = self
            .link
            .callback(|_| Msg::ClearFileBoxed);

        html!{
            <a id="btn-new-favicon-clear"
                  // class="button is-danger"
                  class="button"
                  onclick=onclick_clear_boxed
                  disabled={self.dis_upload_btn} >
                { get_value_field(&88) }
            </a>
        }
    }

    fn show_success_upload(&self) -> Html {
        html!{
            <article class="message is-success">
              <div class="message-header">
                <p>{ get_value_field(&88) }</p>
              </div>
              <div class="message-body">
                { get_value_field(&92) }
              </div>
            </article>
        }
    }
}
