use yew::{Component, Callback, Context, html, Html, Properties};
use yew::virtual_dom::VNode;
use yew::html::{Scope, TargetCast};
use gloo::file::File;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::fragments::files_frame::FilesFrame;
use crate::fragments::list_errors::ListErrors;
use crate::error::Error;
use crate::services::storage_upload::storage_upload;
use crate::services::{get_value_field, resp_parsing_item};
use crate::types::UploadFile;
use crate::gqls::make_query;
use crate::gqls::user::{
    UploadUserFavicon, upload_user_favicon
};
use crate::gqls::company::{
    UploadCompanyFavicon, upload_company_favicon
};

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub callback: Callback<String>,
    #[prop_or_default]
    pub company_uuid: Option<String>,
}

/// For upload favicon file to user and company
#[derive(Debug)]
pub struct UpdateFaviconBlock {
    error: Option<Error>,
    get_result_up_completed: bool,
    file: Option<File>,
    dis_upload_btn: bool,
    active_loading_files_btn: bool,
    v_node: Option<VNode>,
}

pub enum Msg {
    RequestUploadData,
    RequestUploadUserData,
    RequestUploadCompanyData,
    UpdateFiles(Option<FileList>),
    GetUploadData(String),
    FinishUploadFiles(Result<usize, Error>),
    ClearFileBoxed,
    ClearError,
    ResponseError(Error),
    Ignore,
}

impl Component for UpdateFaviconBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            get_result_up_completed: false,
            file: None,
            dis_upload_btn: true,
            active_loading_files_btn: false,
            v_node: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestUploadData => {
                // see loading button
                self.active_loading_files_btn = true;
                match &ctx.props().company_uuid {
                    Some(_) => ctx.link().send_message(Msg::RequestUploadCompanyData),
                    None => ctx.link().send_message(Msg::RequestUploadUserData),
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
                        )).await.unwrap();
                        link.send_message(Msg::GetUploadData(res));
                    })
                }
            },
            Msg::RequestUploadCompanyData => {
                if let Some(file) = &self.file {
                    let company_uuid = ctx.props().company_uuid.as_ref().map(|u| u.clone()).unwrap();
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
            Msg::UpdateFiles(file_list) => {
                if let Some(files) = file_list {
                    self.file = files.get(0).map(|f| File::from(f));
                    self.dis_upload_btn = self.file.is_none();
                }
            },
            Msg::GetUploadData(res) => {
                let key_word = match &ctx.props().company_uuid {
                    Some(_) => "uploadCompanyFavicon",
                    None => "uploadFavicon",
                };
                let result: UploadFile = resp_parsing_item(res, key_word)
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if let Some(file) = self.file.clone() {
                    let callback_confirm =
                        link.callback(|res: Result<usize, Error>| Msg::FinishUploadFiles(res));
                    self.v_node = Some(storage_upload(vec![result], vec![file], callback_confirm));
                    debug!("res_stor: {:?}", self.v_node);
                }
                debug!("file: {:?}", self.file);
            },
            Msg::FinishUploadFiles(res) => {
                match res {
                    Ok(value) => self.get_result_up_completed = value == 1_usize,
                    Err(err) => self.error = Some(err),
                }
                self.active_loading_files_btn = false;
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
          <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error)}/>
          <div class="column">
              {match self.get_result_up_completed {
                  true => self.show_success_upload(),
                  false => html!{<>
                      { self.show_frame_upload_file(ctx.link()) }
                      <br/>
                      <div class="buttons">
                          { self.show_btn_clear(ctx.link()) }
                          { self.show_btn_upload(ctx.link()) }
                      </div>
                  </>},
              }}
          </div>
        </>}
    }
}

impl UpdateFaviconBlock {
    fn show_frame_upload_file(
        &self,
        link: &Scope<Self>,
    )  -> Html {
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
            Msg::UpdateFiles(ev.data_transfer().unwrap().files())
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
                            input_id={"favicon-file-input".to_string()}
                            accept={"image/*".to_string()}
                            file_label={86}
                        />
                    </div>
                    {match &self.v_node {
                        Some(v) => v.clone(),
                        None => html!{},
                    }}
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

    fn show_btn_upload(
        &self,
        link: &Scope<Self>,
    )  -> Html {
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
    )  -> Html {
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

    fn show_success_upload(&self)  -> Html {
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
