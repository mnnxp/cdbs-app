use yew::{Component, Callback, Context, html, Html, Properties};
use yew::virtual_dom::VNode;
use yew::html::{Scope, TargetCast};
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use gloo::file::File;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::fragments::files_frame::FilesFrame;
use crate::fragments::list_errors::ListErrors;
use crate::services::storage_upload::storage_upload;
use crate::services::{image_detector, get_value_field, resp_parsing_item};
use crate::types::UploadFile;
use crate::gqls::make_query;
use crate::gqls::component::{
    UploadComponentFavicon, upload_component_favicon
};

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub component_uuid: String,
    pub callback: Callback<()>,
}

/// For viewing favicon data on page
#[derive(Debug)]
pub struct UpdateComponentFaviconCard {
    error: Option<Error>,
    get_result_up_completed: bool,
    file: Option<File>,
    active_loading_files_btn: bool,
    dis_upload_btn: bool,
    v_node: Option<VNode>,
}

pub enum Msg {
    RequestUploadData,
    UpdateFiles(Option<FileList>),
    GetUploadData(String),
    FinishUploadFiles(Result<usize, Error>),
    HideNotificationSuccess,
    ClearFileBoxed,
    ClearError,
    ResponseError(Error),
    Ignore,
}

impl Component for UpdateComponentFaviconCard {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            get_result_up_completed: false,
            file: None,
            active_loading_files_btn: false,
            dis_upload_btn: true,
            v_node: None,
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
                        let ipt_component_favicon_data = upload_component_favicon::IptComponentFaviconData {
                            component_uuid: ctx.props().component_uuid.clone(),
                            filename: file.name().to_string(),
                        };
                        spawn_local(async move {
                            let res = make_query(UploadComponentFavicon::build_query(
                                upload_component_favicon::Variables { ipt_component_favicon_data },
                            )).await.unwrap();
                            link.send_message(Msg::GetUploadData(res));
                        });
                    } else {
                        // select file is not image
                        link.send_message(Msg::ClearFileBoxed);
                    }
                }
            },
            Msg::UpdateFiles(file_list) => {
                if let Some(files) = file_list {
                    self.file = files.get(0).map(|f| File::from(f));
                    self.dis_upload_btn = self.file.is_none();
                }
            },
            Msg::GetUploadData(res) => {
                let result: UploadFile = resp_parsing_item(res, "uploadComponentFavicon")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if let Some(file) = self.file.clone() {
                    let callback_confirm =
                        link.callback(|res: Result<usize, Error>| Msg::FinishUploadFiles(res));
                    self.v_node = Some(storage_upload(vec![result], vec![file], callback_confirm));
                }
                debug!("file: {:?}", self.file);
            },
            Msg::FinishUploadFiles(res) => {
                match res {
                    Ok(value) => self.get_result_up_completed = value > 0,
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
            Msg::ResponseError(err) => self.error = Some(err),
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
          <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
          {match self.get_result_up_completed {
              true => html!{self.show_success_upload(ctx.link())},
              false => html!{self.show_frame_upload_file(ctx.link())},
          }}
        </>}
    }
}

impl UpdateComponentFaviconCard {
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

        html!{
            <>
                <div class="file is-boxed has-name">
                    <FilesFrame
                        {onchange}
                        {ondrop}
                        {ondragover}
                        {ondragenter}
                        input_id={"favicon-file-input".to_string()}
                        accept={"image/*".to_string()}
                        file_label={182} // Drop preview image here
                    />
                    // </label> todo!(Исправить стиль: сделать обёртку для рамки и выбранных файлов)
                    {match &self.v_node {
                        Some(v) => v.clone(),
                        None => html!{},
                    }}
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
