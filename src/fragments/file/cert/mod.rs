use yew::{html, Component, Callback, ComponentLink, Html, Properties, ShouldRender, InputData};
// use log::debug;

use crate::services::{image_detector, get_value_field};
use crate::types::{UUID, Certificate};

#[derive(Debug)]
pub struct CertificateItem {
    props: Props,
    link: ComponentLink<Self>,
    cert_url: String,
    cert_description: String,
    show_cert: bool,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub certificate: Certificate,
    pub show_cert_btn: bool,
    pub download_btn: bool,
    pub manage_btn: bool,
    pub get_result_update: bool,
    pub get_result_delete: bool,
    pub callback_update_descript: Option<Callback<(UUID, String)>>,
    pub callback_delete_cert: Option<Callback<UUID>>,
}

pub enum Msg {
    UpdateDescription(String),
    SetNewDescription,
    DeleteCert,
    ShowCert,
}

impl Component for CertificateItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cert_url = match image_detector(&props.certificate.file.filename) {
            true => props.certificate.file.download_url.clone(),
            false => String::from("https://bulma.io/images/placeholders/128x128.png"),
        };
        let cert_description = props.certificate.description.clone();
        Self {
            props,
            link,
            cert_url,
            cert_description,
            show_cert: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::UpdateDescription(description) => self.cert_description = description,
            Msg::SetNewDescription => {
                if let Some(callback_update_descript) = &self.props.callback_update_descript {
                    let (file_uuid, description) = (self.props.certificate.file.uuid.clone(), self.cert_description.clone());
                    callback_update_descript.emit((file_uuid, description));
                }
            },
            Msg::DeleteCert => {
                if let Some(callback_delete_cert) = &self.props.callback_delete_cert {
                    let file_uuid = self.props.certificate.file.uuid.clone();
                    callback_delete_cert.emit(file_uuid);
                }
            },
            Msg::ShowCert => self.show_cert = !self.show_cert,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.certificate.file.uuid == props.certificate.file.uuid &&
              self.props.get_result_update == props.get_result_update &&
              self.props.get_result_delete == props.get_result_delete {
            false
        } else {
            self.props = props;
            if image_detector(&self.props.certificate.file.filename) {
                self.cert_url = self.props.certificate.file.download_url.clone();
            }
            true
        }
    }

    fn view(&self) -> Html {
        match self.props.get_result_delete {
            true => self.show_delete_certificate(),
            false => {
                match self.props.manage_btn {
                    true => self.show_certificate_update(),
                    false => html!{<>
                        {self.modal_full_certificate()}
                        {self.show_certificate_data()}
                    </>},
                }
            },
        }
    }
}

impl CertificateItem {
    fn show_certificate_update(&self) -> Html {
        let onclick_delete_cert = self.link.callback(|_| Msg::DeleteCert);

        html!{<>
            <br/>
            <div class="card">
                <br/>
                <div class="media" >
                  <div class="media-left">
                    {match self.show_cert {
                        true => html!{<figure class="image is-128x128" style="margin-left: 1rem" >
                            <img
                                src={self.cert_url.clone()}
                                loading="lazy"
                            />
                        </figure>},
                        false => html!{},
                    }}
                  </div>
                  <div class="media-content" style="margin-right: 1rem;">
                    <div class="block" style="overflow-wrap: anywhere">
                        <span class="overflow-title has-text-weight-bold">{ get_value_field(&262) }</span>
                        <span class="overflow-title">{self.props.certificate.file.filename.clone()}</span>
                    </div>
                    {self.show_update_block()}
                    <div class="buttons">
                      {self.show_certificate_btn()}
                      <button id={"delete-cert"}
                          class="button is-danger is-fullwidth has-text-weight-bold"
                          onclick=onclick_delete_cert>
                          { get_value_field(&135) }
                      </button>
                      {self.show_download_btn()}
                    </div>
                  </div>
                </div>
                <br/>
            </div>
        </>}
    }

    fn show_certificate_data(&self) -> Html {
        let onclick_show_cert = self.link.callback(|_| Msg::ShowCert);

        html!{<div class="boxItem" >
          <div class="innerBox" >
            <div class="imgBox" >
                <figure class="image is-256x256" onclick=onclick_show_cert >
                    <img
                        // src="https://bulma.io/images/placeholders/128x128.png" alt="Image"
                        src={self.cert_url.clone()}
                        loading="lazy"
                    />
                </figure>
            </div>
            <div class="overflow-title has-text-weight-bold">{self.props.certificate.description.clone()}</div>
            <div class="btnBox">
              {self.show_certificate_btn()}
            </div>
          </div>
        </div>}
    }

    fn show_delete_certificate(&self) -> Html {
        html!{<div class="card">
            <div class="message is-success">
              <div class="message-header">{ get_value_field(&89) }</div>
              <div class="message-body">{ get_value_field(&139) }</div>
            </div>
        </div>}
    }

    fn show_update_description(&self) -> Html {
        match self.props.get_result_update {
            true => html!{<div class="column">
                <span id="remove-profile" class="tag is-info is-light">
                    { get_value_field(&140) }
                </span>
            </div>},
            false => html!{},
        }
    }

    fn show_certificate_btn(&self) -> Html {
        let onclick_show_cert = self.link.callback(|_| Msg::ShowCert);
        let text_btn = match self.show_cert {
            true => get_value_field(&314), // Hide
            false => get_value_field(&315), // Show
        };

        match self.props.show_cert_btn {
            true => html!{
                <button id={"show-cert"}
                    class="button is-light is-fullwidth has-text-weight-bold"
                    onclick=onclick_show_cert>
                    { text_btn }
                </button>
            },
            false => html!{},
        }
    }

    fn show_update_block(&self) -> Html {
        let oninput_cert_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let onclick_change_cert =
            self.link.callback(|_| Msg::SetNewDescription);

        html!{<div class="block">
            <div class="columns" style="margin-bottom: 0px">
                <div class="column">
                    <label class="label">{ get_value_field(&61) }</label>
                </div>
                {self.show_update_description()}
            </div>
            <div class="columns">
                <div class="column">
                    <input
                        id={"cert-description"}
                        class="input"
                        type="text"
                        placeholder=get_value_field(&61)
                        value={self.cert_description.clone()}
                        oninput=oninput_cert_description />
                </div>
                <div class="column">
                    <button id={"change-cert"}
                        class="button is-light is-fullwidth has-text-weight-bold"
                        onclick=onclick_change_cert>
                        { get_value_field(&46) }
                    </button>
                </div>
            </div>
        </div>}
    }

    fn show_download_btn(&self) -> Html {
        match self.props.download_btn {
            true => html!{<button id={"down-cert"}
                class="button is-light is-fullwidth has-text-weight-bold"
                href={self.props.certificate.file.download_url.clone()}
                download={self.props.certificate.file.filename.clone()}>
                { get_value_field(&126) }
            </button>},
            false => html!{},
        }
    }

    fn modal_full_certificate(&self) -> Html {
        let onclick_show_cert = self.link.callback(|_| Msg::ShowCert);
        let class_modal = match &self.show_cert {
            true => "modal is-active",
            false => "modal",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_show_cert.clone() />
              <div class="modal-content box">
                <p class="image is-4by3">
                  <img
                    src={self.cert_url.clone()}
                    loading="lazy"
                  />
                </p>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick=onclick_show_cert />
            </div>
        }
    }
}
