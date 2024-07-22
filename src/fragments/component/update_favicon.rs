use graphql_client::GraphQLQuery;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::file::UploaderFiles;
use crate::services::{get_value_field, resp_parsing};
use crate::types::UploadFile;
use crate::gqls::make_query;
use crate::gqls::component::{UploadComponentFavicon, upload_component_favicon};

type FileName = String;

#[derive(Debug)]
pub struct UpdateComponentFaviconCard {
    error: Option<Error>,
    request_upload_data: Option<UploadFile>,
    props: Props,
    link: ComponentLink<Self>,
    get_result_up_completed: bool,
}

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub component_uuid: String,
    pub callback: Callback<()>,
}

pub enum Msg {
    RequestUploadData(Vec<FileName>),
    GetUploadData(String),
    ResponseError(Error),
    HideNotificationSuccess,
    UploadConfirm(usize),
    ClearError,
}

impl Component for UpdateComponentFaviconCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: None,
            props,
            link,
            get_result_up_completed: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestUploadData(filenames) => {
                let mut filename = &String::new();
                if let Some(f_name) = filenames.first() {
                    filename = f_name;
                };
                let ipt_component_favicon_data = upload_component_favicon::IptComponentFaviconData {
                    componentUuid: self.props.component_uuid.clone(),
                    filename: filename.clone(),
                };
                spawn_local(async move {
                    let res = make_query(UploadComponentFavicon::build_query(
                        upload_component_favicon::Variables { ipt_component_favicon_data },
                    )).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                });
            },
            Msg::GetUploadData(res) => {
                match resp_parsing(res, "uploadComponentFavicon") {
                    Ok(result) => self.request_upload_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::HideNotificationSuccess => self.get_result_up_completed = !self.get_result_up_completed,
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of favicon: {:?}", confirmations);
                self.get_result_up_completed = confirmations > 0;
                self.props.callback.emit(());
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let callback_upload_filenames =
            self.link.callback(move |filenames| Msg::RequestUploadData(filenames));
        let request_upload_files = self.request_upload_data.clone().map(|uf| vec![uf]);
        let callback_upload_confirm =
            self.link.callback(|confirmations| Msg::UploadConfirm(confirmations));
        html!{<>
          <ListErrors error=self.error.clone() clear_error=onclick_clear_error.clone()/>
          {match self.get_result_up_completed {
              true => html!{self.show_success_upload()},
              false => html!{
                <UploaderFiles
                    text_choose_files={182} // Drop preview image here
                    callback_upload_filenames={callback_upload_filenames}
                    request_upload_files={request_upload_files}
                    callback_upload_confirm={callback_upload_confirm}
                    multiple={false}
                    accept={"image/*".to_string()}
                    />
              },
          }}
        </>}
    }
}

impl UpdateComponentFaviconCard {
    fn show_success_upload(&self) -> Html {
        let onclick_hide_notification = self.link.callback(|_| Msg::HideNotificationSuccess);
        html!{
            <article class="message is-success">
              <div class="message-header">
                <p>{get_value_field(&89)}</p>
                <button class="delete" aria-label="close" onclick=onclick_hide_notification.clone() />
              </div>
              <div class="message-body">
                {get_value_field(&92)}
              </div>
            </article>
        }
    }
}
