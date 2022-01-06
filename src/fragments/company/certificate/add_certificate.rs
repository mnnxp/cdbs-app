use graphql_client::GraphQLQuery;
use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{
    html, Callback, ChangeData, Component, ComponentLink, DragEvent, Html,
    InputData, Properties, ShouldRender
};
// use serde_json::Value;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::gqls::make_query;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::services::{PutUploadFile, UploadData};
use crate::types::{UUID, UploadFile};

type FileName = String;
// type Chunks = bool;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct UploadCompanyCertificate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct ConfirmUploadCompleted;

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub company_uuid: String,
    pub callback: Callback<()>,
}

/// For viewing certificate data on page
#[derive(Debug)]
pub struct AddCompanyCertificateCard {
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
    description: String,
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
    UpdateDescription(String),
    HideNotification,
    ClearFileBoxed,
    ClearError,
    Ignore,
}

impl Component for AddCompanyCertificateCard {
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
            description: String::new(),
            active_loading_files_btn: false,
            dis_upload_btn: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestUploadData => {
                // see loading button
                self.active_loading_files_btn = true;

                if let Some(file) = &self.file {
                    // debug!("RequestUploadData: {:?}", &self.request_update);
                    let cert_data = upload_company_certificate::IptCompanyCertificateData {
                        companyUuid: self.props.company_uuid.clone(),
                        filename: file.name().to_string(),
                        description: self.description.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(UploadCompanyCertificate::build_query(
                            upload_company_certificate::Variables { cert_data },
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
                            res_value.get("uploadCompanyCertificate").unwrap().clone(),
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
            Msg::UpdateDescription(new_description) => self.description = new_description,
            Msg::HideNotification => {
                link.send_message(Msg::ClearFileBoxed);
                self.get_result_up_completed = !self.get_result_up_completed;
            },
            Msg::ClearFileBoxed => {
                self.file = None;
                self.description = String::new();
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

        html!{<div class="card">
          <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
          <div class="block">
            {match self.get_result_up_completed {
                true => html!{<div class="column">
                  { self.show_success_upload() }
                </div>},
                false => html!{<div class="column">
                  <label class="label">{"Upload new certificate"}</label>
                  { self.show_frame_upload_file() }
                  { self.show_input_description() }
                  <div class="buttons">
                      { self.show_btn_clear() }
                      { self.show_btn_upload() }
                  </div>
                </div>},
            }}
          </div>
        </div>}
    }
}

impl AddCompanyCertificateCard {
    fn show_frame_upload_file(&self) -> Html {
        let onchange_cert_file = self.link.callback(move |value| {
            if let ChangeData::Files(files) = value {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        let ondrop_cert_file = self.link.callback(move |value: DragEvent| {
            value.prevent_default();
            if let Some(files) = value.data_transfer().unwrap().files() {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        let ondragover_cert_file = self.link.callback(move |value: DragEvent| {
            value.prevent_default();
            Msg::Ignore
        });

        html!{<div class="block">
            <div class="columns">
                <div class="column">
                    <div class="file is-large is-boxed has-name">
                      <label
                        for="cert-file-input"
                        class="file-label"
                        style="width: 100%; text-align: center"
                      >
                        <input
                            id="cert-file-input"
                            class="file-input"
                            type="file"
                            accept="image/*,.pdf"
                            onchange={onchange_cert_file} />
                        <span class="file-cta" ondrop=ondrop_cert_file ondragover=ondragover_cert_file >
                          <span class="file-icon">
                            <i class="fas fa-upload"></i>
                          </span>
                          <span class="file-label">{"Drop certificate file here"}</span>
                        </span>
                      </label>
                    </div>
                </div>
                <div class="column">
                    <div class="has-text-grey-light" style="overflow-wrap: anywhere">
                        {"It is recommended to upload the certificate in image format."}
                    </div>
                    <br/>
                    <div id="select-file" style="overflow-wrap: anywhere">
                        <span class="overflow-title has-text-weight-bold">{"Select file: "}</span>
                        <span>{self.file.as_ref()
                            .map(|f| f.name().to_string())
                            .unwrap_or_default()}</span>
                    </div>
                </div>
            </div>
        </div>}
    }

    fn show_input_description(&self) -> Html {
        let oninput_cert_description = self.link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        html!{<div class="block">
            <label class="label">{"Description"}</label>

            <input
                id={"new-cert-description"}
                class="input"
                type="text"
                placeholder="certificate description"
                value={self.description.to_string()}
                oninput=oninput_cert_description />
        </div>}
    }

    fn show_btn_upload(&self) -> Html {
        let onclick_upload_cert = self.link.callback(|_| Msg::RequestUploadData);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <a id="btn-new-cert-upload"
                  class={class_upload_btn}
                  onclick=onclick_upload_cert
                  disabled={self.dis_upload_btn} >
                { "Upload" }
            </a>
        }
    }

    fn show_btn_clear(&self) -> Html {
        let onclick_clear_boxed = self.link.callback(|_| Msg::ClearFileBoxed);

        html!{
            <a id="btn-new-cert-clear"
                  // class="button is-danger"
                  class="button"
                  onclick=onclick_clear_boxed
                  disabled={self.dis_upload_btn} >
                { "Clear" }
            </a>
        }
    }

    fn show_success_upload(&self) -> Html {
        let onclick_hide_notification = self.link.callback(|_| Msg::HideNotification);

        html!{
            <article class="message is-success">
              <div class="message-header">
                <p>{ "Success" }</p>
                <button class="delete" aria-label="close" onclick=onclick_hide_notification.clone() />
              </div>
              <div class="message-body">
                { "This certificate upload!" }
              </div>
            </article>
        }
    }
}
