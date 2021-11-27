use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{
    html, Callback, Component, ComponentLink, Html, InputData,
    Properties, ShouldRender, ChangeData,
};
use graphql_client::GraphQLQuery;
// use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::gqls::make_query;

use crate::fragments::list_errors::ListErrors;
use crate::error::{Error, get_error};
use crate::services::{PutUploadFile, UploadData};
use crate::types::{
    UUID, UploadFile,
    // Certificate,
};

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
    pub callback: Callback<String>,
    pub company_uuid: String,
}

/// For upload company Certificate
#[derive(Default, Clone, Debug)]
pub struct NewCompanyCertData {
    company_uuid: String,
    filename: String,
    description: String,
}

/// For viewing certificate data on page
#[derive(Debug)]
pub struct AddCertificateCard {
    error: Option<Error>,
    request_upload_data: UploadFile,
    request_upload_file: Callback<Result<Option<String>, Error>>,
    request_upload_completed: String,
    response_upload_file: Option<Result<String, Error>>,
    task_read: Option<(FileName, ReaderTask)>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
    get_result_up_data: bool,
    get_result_up_file: bool,
    get_result_up_completed: usize,
    put_upload_file: PutUploadFile,
    file: Option<File>,
    // filename: String,
    // file_data: Vec<u8>,
    description: String,
    dis_upload_btn: bool,
}

pub enum Msg {
    RequestUploadData,
    RequestUploadFile(Vec<u8>),
    ResponseUploadFile(Result<Option<String>, Error>),
    RequestUploadCompleted,
    ResponseError(Error),
    UpdateFile(Option<File>),
    UpdateDescription(String),
    GetUploadData(String),
    GetUploadFile(Option<String>),
    GetUploadCompleted(String),
    ClearFileBoxed,
    Ignore,
}

impl Component for AddCertificateCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: UploadFile::default(),
            request_upload_file: link.callback(Msg::ResponseUploadFile),
            request_upload_completed: String::new(),
            response_upload_file: None,
            task_read: None,
            task: None,
            props,
            link,
            get_result_up_data: false,
            get_result_up_file: false,
            get_result_up_completed: 0,
            put_upload_file: PutUploadFile::new(),
            // filename: String::new(),
            // file_data: Vec::new(),
            file: None,
            description: String::new(),
            dis_upload_btn: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestUploadData => {
                // see loading button
                self.get_result_up_data = true;

                if let Some(file) = &self.file {
                    // debug!("RequestUploadData: {:?}", &self.request_update);
                    let request_update = NewCompanyCertData {
                        company_uuid: self.props.company_uuid.clone(),
                        filename: file.name().to_string(),
                        description: self.description.clone(),
                    };
                    spawn_local(async move {
                        let NewCompanyCertData {
                            company_uuid,
                            filename,
                            description,
                        } = request_update;
                        let cert_data = upload_company_certificate::IptCompanyCertificateData {
                            companyUuid: company_uuid,
                            filename,
                            description,
                        };
                        let res = make_query(UploadCompanyCertificate::build_query(
                            upload_company_certificate::Variables {
                                cert_data,
                            }
                        )).await;
                        link.send_message(Msg::GetUploadData(res.unwrap()));
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
                    crate::yewLog!(res);
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
            Msg::UpdateDescription(new_description) => {
                debug!("new_description: {}", new_description);
                self.description = new_description;
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UploadFile = serde_json::from_value(res_value.get("uploadCompanyCertificate").unwrap().clone()).unwrap();
                        // crate::yewLog!(result);
                        self.request_upload_data = result;

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
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
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
                        let result: usize = serde_json::from_value(res_value.get("uploadCompleted").unwrap().clone()).unwrap();
                        crate::yewLog!(result);
                        self.get_result_up_completed = result;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::ClearFileBoxed => {
                self.file = None;
                self.description = String::new();
                self.dis_upload_btn = true;
            },
            Msg::Ignore => {},
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props = props;
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="card">
              <ListErrors error=self.error.clone()/>
              <div class="columns">
                {match self.get_result_up_completed > 0 {
                    true => html! {
                        <div class="column">
                          { self.show_success_upload() }
                        </div>
                    },
                    false => html! {
                        <div class="column">
                          <label class="label">{"Upload new certificate:"}</label>
                          { self.show_input_description() }
                          <br/>
                          { self.show_frame_upload_file() }
                          <br/>
                          { self.show_btn_clear() }
                          { self.show_btn_upload() }
                        </div>
                    },
                }}
              </div>
            </div>
        }
    }
}

impl AddCertificateCard {
    fn show_frame_upload_file(
        &self,
    ) -> Html {
        let onchange_cert_file = self.link.callback(move |value| {
            if let ChangeData::Files(files) = value {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        html! {<>
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
                <span class="file-cta">
                  <span class="file-icon">
                    <i class="fas fa-upload"></i>
                  </span>
                  <span class="file-label"> {"Drop file here"} </span>
                </span>
              </label>
            </div>
            <p>
                {"Upload new certificate, recommended upload file format image"}
            </p>
            <output id="select-file">{
                format!(" {}", self.file.as_ref()
                    .map(|f| f.name().to_string())
                    .unwrap_or_default())
            }</output>
        </>}
    }

    fn show_input_description(
        &self,
    ) -> Html {
        let oninput_cert_description = self
            .link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        html! {<>
            <label class="label">{"Description"}</label>

            <input
                id={ "new-cert-description" }
                class="input"
                type="text"
                placeholder="certificate description"
                value={ self.description.to_string() }
                oninput=oninput_cert_description />
        </>}
    }

    fn show_btn_upload(
        &self,
    ) -> Html {
        let onclick_upload_cert = self
            .link
            .callback(|_| Msg::RequestUploadData);

        let mut class_btn = "button";

        if self.get_result_up_data {
            // enable "loading" btn if send data
            class_btn = "button is-loading";
        }

        if self.get_result_up_completed > 0 {
            // enable "success" btn if send data
            // class_btn = "button is-success";
            class_btn = "button";
        }

        html! {
            <a id="btn-new-cert-upload"
                  class={class_btn}
                  onclick=onclick_upload_cert
                  disabled={self.dis_upload_btn} >
                { "Upload" }
            </a>
        }
    }

    fn show_btn_clear(
        &self,
    ) -> Html {
        let onclick_clear_boxed = self
            .link
            .callback(|_| Msg::ClearFileBoxed);

        html! {
            <a id="btn-new-cert-clear"
                  // class="button is-danger"
                  class="button"
                  onclick=onclick_clear_boxed
                  disabled={self.dis_upload_btn} >
                { "Clear" }
            </a>
        }
    }

    fn show_success_upload(
        &self,
    ) -> Html {
        html! {
            <article class="message is-success">
              <div class="message-header">
                <p>{ "Success" }</p>
              </div>
              <div class="message-body">
                { "This certificate upload!" }
              </div>
            </article>
        }
    }
}
