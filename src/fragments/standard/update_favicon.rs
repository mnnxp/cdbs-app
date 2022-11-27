use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use graphql_client::GraphQLQuery;
use web_sys::{DragEvent, Event, File};
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::services::storage_upload::{StorageUpload, storage_upload};
use crate::services::{image_detector, get_value_field};
use crate::types::UploadFile;
use crate::gqls::{
    make_query,
    relate::{ConfirmUploadCompleted, confirm_upload_completed},
    standard::{UploadStandardFavicon, upload_standard_favicon},
};

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
    // request_upload_file: Callback<Result <(), Error>>,
    // task_read: Option<(FileName, ReaderTask)>,
    // task: Option<FetchTask>,
    get_result_up_file: bool,
    get_result_up_completed: bool,
    // put_upload_file: PutUploadFile,
    file: Option<File>,
    active_loading_files_btn: bool,
    dis_upload_btn: bool,
}

pub enum Msg {
    RequestUploadData,
    // RequestUploadFile(Vec<u8>),
    ResponseUploadFile(Result<(), Error>),
    RequestUploadCompleted,
    UpdateFile(Option<File>),
    GetUploadData(String),
    // GetUploadFile(Option<String>),
    GetUploadCompleted(Result<usize, Error>),
    HideNotificationSuccess,
    ClearFileBoxed,
    ClearError,
    Ignore,
}

impl Component for UpdateStandardFaviconCard {
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
            active_loading_files_btn: false,
            dis_upload_btn: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestUploadData => {
                if let Some(file) = &self.file {
                    if image_detector(file.name().as_str()) {
                        // see loading button
                        self.active_loading_files_btn = true;

                        // debug!("RequestUploadData: {:?}", &self.request_update);
                        let ipt_standard_favicon_data = upload_standard_favicon::IptStandardFaviconData {
                            standardUuid: ctx.props().standard_uuid.clone(),
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
            // Msg::RequestUploadFile(data) => {
            //     let request = UploadData {
            //         upload_url: self.request_upload_data.upload_url.to_string(),
            //         file_data: data,
            //     };
            //     self.task = Some(self.put_upload_file.put_file(request, self.request_upload_file.clone()));
            // },
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
                        let result =
                            serde_json::from_value(res_value.get("uploadStandardFavicon").unwrap().clone()).unwrap();

                        if let Some(file) = self.file.clone() {
                            let callback_confirm =
                                link.callback(|res: Result<usize, Error>| Msg::GetUploadCompleted(res));
                            storage_upload(&result, &vec![file], callback_confirm);
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
                    Ok(value) => self.get_result_up_completed = value,
                    Err(err) => self.error = Some(err),
                }
                self.active_loading_files_btn = false;
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

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
          <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error)}/>
          {match self.get_result_up_completed {
              true => html!{self.show_success_upload(ctx.link())},
              false => html!{self.show_frame_upload_file(ctx.link())},
          }}
        </>}
    }
}

impl UpdateStandardFaviconCard {
    fn show_frame_upload_file(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onchange_favicon_file = link.callback(move |value| {
            if let Event::Files(files) = value {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        let ondrop_favicon_file = link.callback(move |value: DragEvent| {
            value.prevent_default();
            if let Some(files) = value.data_transfer().unwrap().files() {
                Msg::UpdateFile(files.get(0))
            } else {
                Msg::Ignore
            }
        });

        let ondragover_favicon_file = link.callback(move |value: DragEvent| {
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
                    <span class="file-cta" ondrop={ondrop_favicon_file} ondragover={ondragover_favicon_file} >
                      <span class="file-icon">
                        <i class="fas fa-upload"></i>
                      </span>
                      <span class="file-label">{ get_value_field(&182) }</span> // Drop preview image here
                    </span>
                    <div class="columns">
                        <div class="column">
                            <span class="file-name" style="overflow-wrap: anywhere">
                                {self.file.as_ref().map(|f| f.name().to_string()).unwrap_or_default()}
                            </span>
                        </div>
                        <div class="column">
                            <span class="has-text-grey-light is-size-6" style="overflow-wrap: anywhere">
                                { get_value_field(&183) }
                                {": .apng, .avif, .gif, .jpg, .jpeg, .jpe, .jif, .jfif, .png, .svg, .webp."}
                            </span>
                        </div>
                    </div>
                  </label>
                </div>
                <div class="buttons">
                    { self.show_btn_clear(link) }
                    { self.show_btn_upload(link) }
                </div>
            </>
        }
    }

    fn show_btn_upload(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_upload_favicon = link.callback(|_| Msg::RequestUploadData);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <a id="btn-new-favicon-upload"
                  class={class_upload_btn}
                  onclick={onclick_upload_favicon}
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
            <a id="btn-new-favicon-clear"
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
        let onclick_hide_notification = link.callback(|_| Msg::HideNotificationSuccess);

        html!{
            <article class="message is-success">
              <div class="message-header">
                <p>{ get_value_field(&89) }</p>
                <button class="delete" aria-label="close" onclick={onclick_hide_notification.clone()} />
              </div>
              <div class="message-body">
                { get_value_field(&92) }
              </div>
            </article>
        }
    }
}
