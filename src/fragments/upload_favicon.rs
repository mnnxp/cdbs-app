use yew::{html, Component, Callback, ComponentLink, Html, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::file::UploaderFiles;
use crate::error::Error;
use crate::services::{get_value_field, resp_parsing};
use crate::types::UploadFile;
use crate::gqls::{
    make_query,
    user::{UploadUserFavicon, upload_user_favicon},
    company::{UploadCompanyFavicon, upload_company_favicon},
};

type FileName = String;

/// For upload favicon file to user and company
#[derive(Debug)]
pub struct UpdateFaviconBlock {
    error: Option<Error>,
    request_upload_data: Option<UploadFile>,
    props: Props,
    link: ComponentLink<Self>,
    get_result_up_completed: bool,
}

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub callback: Callback<()>,
    pub company_uuid: Option<String>,
}

pub enum Msg {
    RequestUploadData(Vec<FileName>),
    RequestUploadUserData(FileName),
    RequestUploadCompanyData(FileName),
    ResponseError(Error),
    GetUploadData(String),
    UploadConfirm(usize),
    ClearError,
}

impl Component for UpdateFaviconBlock {
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
                if let Some(filename) = filenames.first() {
                    match &self.props.company_uuid {
                        Some(_) => self.link.send_message(Msg::RequestUploadCompanyData(filename.clone())),
                        None => self.link.send_message(Msg::RequestUploadUserData(filename.clone())),
                    }
                }
            },
            Msg::RequestUploadUserData(filename_upload_favicon) => {
                spawn_local(async move {
                    let res = make_query(UploadUserFavicon::build_query(
                        upload_user_favicon::Variables {
                            filename_upload_favicon,
                        }
                    )).await;
                    link.send_message(Msg::GetUploadData(res.unwrap()));
                })
            },
            Msg::RequestUploadCompanyData(filename_upload_favicon) => {
                let company_uuid = self.props.company_uuid.as_ref().map(|u| u.clone()).unwrap();
                spawn_local(async move {
                    let res = make_query(UploadCompanyFavicon::build_query(
                        upload_company_favicon::Variables {
                            company_uuid,
                            filename_upload_favicon,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetUploadData(res) => {
                let target_key = match &self.props.company_uuid {
                    Some(_) => "uploadCompanyFavicon",
                    None => "uploadFavicon",
                };
                match resp_parsing(res, target_key) {
                    Ok(result) => self.request_upload_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
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
          <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
          <div class="column">
              {match self.get_result_up_completed {
                  true => self.show_success_upload(),
                  false => html!{
                    <UploaderFiles
                        text_choose_files={93} // Drop favicon file here
                        callback_upload_filenames={callback_upload_filenames}
                        request_upload_files={request_upload_files}
                        callback_upload_confirm={callback_upload_confirm}
                        multiple={false}
                        accept={"image/*".to_string()}
                        />
                  },
              }}
          </div>
        </>}
    }
}

impl UpdateFaviconBlock {
    fn show_success_upload(&self) -> Html {
        html!{
            <article class="message is-success">
              <div class="message-header">
                <p>{ get_value_field(&89) }</p>
              </div>
              <div class="message-body">
                { get_value_field(&92) }
              </div>
            </article>
        }
    }
}
