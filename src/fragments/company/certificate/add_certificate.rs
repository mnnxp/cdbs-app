use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use graphql_client::GraphQLQuery;
use web_sys::{DragEvent, Event, File};
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::services::storage_upload::storage_upload;
use crate::services::get_value_field;
use crate::types::UploadFile;
use crate::gqls::{
    make_query,
    relate::{ConfirmUploadCompleted, confirm_upload_completed},
    company::{UploadCompanyCertificate, upload_company_certificate},
};

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
    // request_upload_file: Callback<Result<Option<String>, Error>>,
    // task_read: Option<(FileName, ReaderTask)>,
    // task: Option<FetchTask>,
    get_result_up_file: bool,
    get_result_up_completed: bool,
    // put_upload_file: PutUploadFile,
    file: Option<File>,
    description: String,
    active_loading_files_btn: bool,
    dis_upload_btn: bool,
}

pub enum Msg {
    RequestUploadData,
    // RequestUploadFile(Vec<u8>),
    ResponseUploadFile(Result<(), Error>),
    // RequestUploadCompleted,
    UpdateFile(Option<File>),
    GetUploadData(String),
    // GetUploadFile(Option<String>),
    GetUploadCompleted(Result<usize, Error>),
    UpdateDescription(String),
    HideNotification,
    ClearFileBoxed,
    ClearError,
    Ignore,
}

impl Component for AddCompanyCertificateCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: UploadFile::default(),
            // request_upload_file: ctx.link().callback(Msg::ResponseUploadFile),
            // task_read: None,
            // task: None,
            get_result_up_file: false,
            get_result_up_completed: false,
            // put_upload_file: PutUploadFile::new(),
            file: None,
            description: String::new(),
            active_loading_files_btn: false,
            dis_upload_btn: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestUploadData => {
                // see loading button
                self.active_loading_files_btn = true;

                if let Some(file) = &self.file {
                    // debug!("RequestUploadData: {:?}", &self.request_update);
                    let cert_data = upload_company_certificate::IptCompanyCertificateData {
                        company_uuid: ctx.props().company_uuid.clone(),
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
            // Msg::RequestUploadFile(data) => {
            //     let request = UploadData {
            //         upload_url: self.request_upload_data.upload_url.to_string(),
            //         file_data: data,
            //     };
            //     self.task = Some(self.put_upload_file.put_file(request, self.request_upload_file.clone()));
            // },
            Msg::ResponseUploadFile(Ok(res)) => {
                debug!("res: {:?}", res);
                self.get_result_up_file = true;
                // link.send_message(Msg::RequestUploadCompleted)
            },
            Msg::ResponseUploadFile(Err(err)) => {
                self.error = Some(err);
                // self.task = None;
                // self.task_read = None;
            },
            // Msg::RequestUploadCompleted => {
            //     let file_uuids = vec![self.request_upload_data.file_uuid.clone()];
            //     spawn_local(async move {
            //         let res = make_query(ConfirmUploadCompleted::build_query(
            //             confirm_upload_completed::Variables { file_uuids })
            //         ).await.unwrap();
            //         debug!("ConfirmUploadCompleted: {:?}", res);
            //         link.send_message(Msg::GetUploadCompleted(res));
            //     });
            // },
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
                        let result = serde_json::from_value(
                            res_value.get("uploadCompanyCertificate").unwrap().clone(),
                        ).unwrap();

                        if let Some(file) = self.file.clone() {
                            let callback_confirm =
                                link.callback(|res: Result<usize, Error>| Msg::GetUploadCompleted(res));
                            storage_upload(result, vec![file], callback_confirm);
                            // let file_name = file.name().clone();
                            // let task = {
                            //     let callback = ctx.link().callback(move |data: FileData| {
                            //         Msg::RequestUploadFile(data.content)
                            //     });
                            //     ReaderService::read_file(file, callback).unwrap()
                            // };
                            // self.task_read = Some((file_name, task));
                        }
                        debug!("file: {:?}", self.file);
                    }
                    true => self.error = Some(get_error(&data)),
                }
            },
            // Msg::GetUploadFile(res) => {
            //     debug!("res: {:?}", res);
            //     self.get_result_up_file = true;
            //     link.send_message(Msg::RequestUploadCompleted)
            // },
            Msg::GetUploadCompleted(res) => {
                match res {
                    Ok(value) => self.get_result_up_completed = value > 0,
                    Err(err) => self.error = Some(err),
                }
                self.active_loading_files_btn = false;
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

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<div class="card">
          <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
          <div class="block">
            {match self.get_result_up_completed {
                true => html!{<div class="column">
                  { self.show_success_upload(ctx.link()) }
                </div>},
                false => html!{<div class="column">
                  <label class="label">{ get_value_field(&83) }</label> // Upload new certificate
                  { self.show_frame_upload_file(ctx.link()) }
                  { self.show_input_description(ctx.link()) }
                  <div class="buttons">
                      { self.show_btn_clear(ctx.link()) }
                      { self.show_btn_upload(ctx.link()) }
                  </div>
                </div>},
            }}
          </div>
        </div>}
    }
}

impl AddCompanyCertificateCard {
    fn show_frame_upload_file(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onchange_cert_file = link.callback(move |value| {
            if let Event::Files(files) = value {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        let ondrop_cert_file = link.callback(move |value: DragEvent| {
            value.prevent_default();
            if let Some(files) = value.data_transfer().unwrap().files() {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        let ondragover_cert_file = link.callback(move |value: DragEvent| {
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
                        <span class="file-cta" ondrop={ondrop_cert_file} ondragover={ondragover_cert_file} >
                          <span class="file-icon">
                            <i class="fas fa-upload"></i>
                          </span>
                          <span class="file-label">{ get_value_field(&86) }</span> // Drop certificate file here
                        </span>
                      </label>
                    </div>
                </div>
                <div class="column">
                    <div class="has-text-grey-light" style="overflow-wrap: anywhere">
                        { get_value_field(&84) } // It is recommended to upload the certificate in image format.
                    </div>
                    <br/>
                    <div id="select-file" style="overflow-wrap: anywhere">
                        <span>{ get_value_field(&85) }</span> // Select file
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

    fn show_input_description(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_cert_description =
            link.callback(|ev: Event| Msg::UpdateDescription(ev.value));

        html!{<div class="block">
            <label class="label">{ get_value_field(&61) }</label>

            <input
                id={"new-cert-description"}
                class="input"
                type="text"
                placeholder={get_value_field(&61)}
                value={self.description.to_string()}
                oninput={oninput_cert_description} />
        </div>}
    }

    fn show_btn_upload(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_upload_cert = link.callback(|_| Msg::RequestUploadData);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <a id="btn-new-cert-upload"
                  class={class_upload_btn}
                  onclick={onclick_upload_cert}
                  disabled={self.dis_upload_btn} >
                { get_value_field(&87) }
            </a>
        }
    }

    fn show_btn_clear(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_clear_boxed = link.callback(|_| Msg::ClearFileBoxed);

        html!{
            <a id="btn-new-cert-clear"
                  // class="button is-danger"
                  class="button"
                  onclick={onclick_clear_boxed}
                  disabled={self.dis_upload_btn} >
                { get_value_field(&88) }
            </a>
        }
    }

    fn show_success_upload(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_hide_notification = link.callback(|_| Msg::HideNotification);

        html!{
            <article class="message is-success">
              <div class="message-header">
                <p>{ get_value_field(&89) }</p>
                <button class="delete" aria-label="close" onclick={onclick_hide_notification.clone()} />
              </div>
              <div class="message-body">
                { get_value_field(&90) }
              </div>
            </article>
        }
    }
}
