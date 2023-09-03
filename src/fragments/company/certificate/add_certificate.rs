use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::file::UploaderFiles;
use crate::services::{get_value_field, resp_parsing};
use crate::types::UploadFile;
use crate::gqls::make_query;
use crate::gqls::company::{UploadCompanyCertificate, upload_company_certificate};

type FileName = String;

#[derive(Debug)]
pub struct AddCompanyCertificateCard {
    error: Option<Error>,
    request_upload_data: Option<UploadFile>,
    props: Props,
    link: ComponentLink<Self>,
    get_result_up_completed: bool,
    description: String,
}

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub company_uuid: String,
    pub callback: Callback<()>,
}

pub enum Msg {
    RequestUploadData(Vec<FileName>),
    GetUploadData(String),
    UpdateDescription(String),
    ResponseError(Error),
    HideNotification,
    UploadConfirm(usize),
    ClearError,
}

impl Component for AddCompanyCertificateCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: None,
            props,
            link,
            get_result_up_completed: false,
            description: String::new(),
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
                let cert_data = upload_company_certificate::IptCompanyCertificateData {
                    companyUuid: self.props.company_uuid.clone(),
                    filename: filename.clone(),
                    description: self.description.clone(),
                };
                spawn_local(async move {
                    let res = make_query(UploadCompanyCertificate::build_query(
                        upload_company_certificate::Variables { cert_data },
                    )).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                })
            },
            Msg::GetUploadData(res) => {
                match resp_parsing(res, "uploadCompanyCertificate") {
                    Ok(result) => self.request_upload_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateDescription(description) => self.description = description,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::HideNotification => self.get_result_up_completed = !self.get_result_up_completed,
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of favicon: {:?}", confirmations);
                self.get_result_up_completed = confirmations > 0;
                self.description.clear();
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
        html!{<div class="card">
          <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
          <div class="block">
            {match self.get_result_up_completed {
                true => html!{<div class="column">{self.show_success_upload()}</div>},
                false => html!{
                    <div class="column">
                        <label class="label">{ get_value_field(&83) }</label> // Upload new certificate
                        {self.show_input_description()}
                        <UploaderFiles
                            text_choose_files={86} // Drop certificate file here
                            callback_upload_filenames={callback_upload_filenames}
                            request_upload_files={request_upload_files}
                            callback_upload_confirm={callback_upload_confirm}
                            multiple={false}
                            accept={"image/*".to_string()}
                            />
                    </div>
                },
            }}
          </div>
        </div>}
    }
}

impl AddCompanyCertificateCard {
    fn show_input_description(&self) -> Html {
        let oninput_cert_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        html!{<div class="block">
            <label class="label">{ get_value_field(&61) }</label>

            <input
                id={"new-cert-description"}
                class="input"
                type="text"
                placeholder=get_value_field(&61)
                value={self.description.to_string()}
                oninput=oninput_cert_description />
        </div>}
    }

    fn show_success_upload(&self) -> Html {
        let onclick_hide_notification = self.link.callback(|_| Msg::HideNotification);
        html!{
            <article class="message is-success">
              <div class="message-header">
                <p>{ get_value_field(&89) }</p>
                <button class="delete" aria-label="close" onclick=onclick_hide_notification.clone() />
              </div>
              <div class="message-body">
                { get_value_field(&90) }
              </div>
            </article>
        }
    }
}
