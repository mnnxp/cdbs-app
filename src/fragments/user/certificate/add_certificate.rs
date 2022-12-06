use yew::{Component, Callback, Context, html, Html, Properties};
use yew::html::{Scope, TargetCast};
use web_sys::{InputEvent, DragEvent, Event, FileList, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use gloo::file::File;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::{get_error, Error};
use crate::fragments::files_frame::FilesFrame;
use crate::fragments::list_errors::ListErrors;
use crate::services::storage_upload::storage_upload;
use crate::services::get_value_field;
use crate::types::UploadFile;
use crate::gqls::make_query;
use crate::gqls::user::{
    UploadUserCertificate, upload_user_certificate,
};

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub user_uuid: String,
    pub callback: Callback<()>,
}

/// For upload user Certificate
#[derive(Default, Clone, Debug)]
pub struct NewUserCertData {
    filename: String,
    description: String,
}

/// For viewing certificate data on page
#[derive(Debug)]
pub struct AddUserCertificateCard {
    error: Option<Error>,
    get_result_up_completed: bool,
    file: Option<File>,
    description: String,
    active_loading_files_btn: bool,
    dis_upload_btn: bool,
}

pub enum Msg {
    RequestUploadData,
    UpdateFiles(Option<FileList>),
    GetUploadData(String),
    GetUploadCompleted(Result<usize, Error>),
    UpdateDescription(String),
    HideNotification,
    ClearFileBoxed,
    ClearError,
    Ignore,
}

impl Component for AddUserCertificateCard {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            get_result_up_completed: false,
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
                    let request_update = NewUserCertData {
                        filename: file.name().to_string(),
                        description: self.description.to_string(),
                    };
                    spawn_local(async move {
                        let NewUserCertData {
                            filename,
                            description,
                        } = request_update;
                        let cert_data = upload_user_certificate::IptUserCertificateData {
                            filename,
                            description,
                        };
                        let res = make_query(UploadUserCertificate::build_query(
                            upload_user_certificate::Variables { cert_data },
                        ))
                        .await;
                        link.send_message(Msg::GetUploadData(res.unwrap()));
                    })
                }
            },
            Msg::UpdateFiles(file_list) => {
                if let Some(files) = file_list {
                    self.file = files.get(0).map(|f| File::from(f));
                    self.dis_upload_btn = self.file.is_none();
                }
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UploadFile =
                            serde_json::from_value(res_value.get("uploadUserCertificate").unwrap().clone()).unwrap();

                        if let Some(file) = self.file.clone() {
                            let callback_confirm =
                                link.callback(|res: Result<usize, Error>| Msg::GetUploadCompleted(res));
                            storage_upload(vec![result], vec![file], callback_confirm);
                        }
                        debug!("file: {:?}", self.file);
                    }
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUploadCompleted(res) => {
                match res {
                    Ok(value) => self.get_result_up_completed = value == 1_usize,
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
                  <label class="label">{ get_value_field(&83) }</label> // "Upload new certificate"
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

impl AddUserCertificateCard {
    fn show_frame_upload_file(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onchange = link.callback(move |ev: Event| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateFiles(input.files())
        });
        let ondrop = link.callback(move |ev: DragEvent| {
            ev.prevent_default();
            Msg::UpdateFiles(ev.data_transfer().unwrap().files())
        });
        let ondragover = link.callback(move |ev: DragEvent| {
            ev.prevent_default();
            Msg::Ignore
        });
        let ondragenter = ondragover.clone();

        html!{<div class="block">
            <div class="columns">
                <div class="column">
                    <div class="file is-large is-boxed has-name">
                        <FilesFrame
                            {onchange}
                            {ondrop}
                            {ondragover}
                            {ondragenter}
                            input_id={"cert-file-input".to_string()}
                            accept={"image/*,.pdf".to_string()}
                            file_label={86}
                        />
                    </div>
                </div>
                <div class="column">
                    <div class="has-text-grey-light" style="overflow-wrap: anywhere">
                        { get_value_field(&84) }
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

    fn show_input_description(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_cert_description = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateDescription(input.value())
        });

        html!{<div class="block">
            <label class="label">{ get_value_field(&61) }</label> // "Description"
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
