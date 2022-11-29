use yew::{Component, Callback, Context, html, html::Scope, Html, Properties, Event};
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::fragments::list_errors::ListErrors;
use crate::error::{Error, get_error};
use crate::services::{image_detector, get_value_field};
use crate::types::{UUID, UserCertificate};
use crate::gqls::make_query;
use crate::gqls::user::{
    UpdateUserCertificate, update_user_certificate,
    DeleteUserCertificate, delete_user_certificate,
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub certificate: UserCertificate,
    pub show_cert_btn: bool,
    pub download_btn: bool,
    pub manage_btn: bool,
    pub callback_delete_cert: Callback<UUID>,
}

pub enum Msg {
    RequestUpdateDescription,
    RequestDeleteCert,
    GetUpdateResult(String),
    GetDeleteCertResult(String),
    UpdateDescription(String),
    ShowCert,
    ClearError,
    Ignore,
}

#[derive(Debug)]
pub struct UserCertificateItem {
    error: Option<Error>,
    file_uuid: UUID,
    request_update: String,
    get_result_update: bool,
    get_result_delete: bool,
    show_cert: bool,
}

impl Component for UserCertificateItem {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            file_uuid: String::new(),
            request_update: ctx.props().certificate.description.to_string(),
            // request_delete: DeleteUserCertData::default(),
            get_result_update: false,
            get_result_delete: false,
            show_cert: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestUpdateDescription => {
                debug!("Update user cert: {:?}", &self.request_update);
                let ipt_update_user_certificate_data = update_user_certificate::IptUpdateUserCertificateData {
                    file_uuid: ctx.props().certificate.file.uuid.clone(),
                    description: self.request_update.clone(),
                };

                spawn_local(async move {
                    let res = make_query(UpdateUserCertificate::build_query(update_user_certificate::Variables {
                        ipt_update_user_certificate_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateResult(res));
                })
            },
            Msg::RequestDeleteCert => {
                let file_uuid = ctx.props().certificate.file.uuid.clone();
                spawn_local(async move {
                    let del_user_certificate_data = delete_user_certificate::DelUserCertificateData{
                        file_uuid,
                    };
                    let res = make_query(DeleteUserCertificate::build_query(delete_user_certificate::Variables {
                        del_user_certificate_data
                    })).await.unwrap();
                    link.send_message(Msg::GetDeleteCertResult(res));
                })
            },
            Msg::GetUpdateResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res_value.get("updateUserCertificate").unwrap().clone()).unwrap();
                        debug!("Update user cert: {:?}", result);
                        self.get_result_update = result;
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetDeleteCertResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.get_result_delete = serde_json::from_value(res_value.get("deleteUserCertificate").unwrap().clone()).unwrap();
                        debug!("Update user cert: {:?}", self.get_result_delete);

                        if self.get_result_delete {
                            ctx.props().callback_delete_cert.emit(ctx.props().certificate.file.uuid.clone());
                        }
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateDescription(description) => self.request_update = description,
            Msg::ShowCert => self.show_cert = !self.show_cert,
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.file_uuid == ctx.props().certificate.file.uuid {
            false
        } else {
            self.file_uuid = ctx.props().certificate.file.uuid;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.get_result_delete {
            true => self.show_delete_certificate(),
            false => {
                match ctx.props().manage_btn {
                    true => self.show_certificate_update(ctx.link(), ctx.props()),
                    false => html!{<>
                        {self.modal_full_certificate(ctx.link(), ctx.props())}
                        {self.show_certificate_data(ctx.link(), ctx.props())}
                    </>},
                }
            },
        }
    }
}

impl UserCertificateItem {
    fn show_certificate_update(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_clear_error = link.callback(|_| Msg::ClearError);
        let onclick_delete_cert = link.callback(|_| Msg::RequestDeleteCert);
        let cert_url = match image_detector(&props.certificate.file.filename) {
            true => props.certificate.file.download_url.clone(),
            false => String::from("https://bulma.io/images/placeholders/128x128.png"),
        };

        html!{<>
            <br/>
            <div class="card">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <br/>
                <div class="media" >
                  <div class="media-left">
                    {match self.show_cert {
                        true => html!{<figure class="image is-128x128" style="margin-left: 1rem" >
                            <img
                                src={cert_url}
                                loading="lazy"
                            />
                        </figure>},
                        false => html!{},
                    }}
                  </div>
                  <div class="media-content" style="margin-right: 1rem;">
                    <div class="block" style="overflow-wrap: anywhere">
                        <span class="overflow-title has-text-weight-bold">{ get_value_field(&262) }</span>
                        <span class="overflow-title">{props.certificate.file.filename.clone()}</span>
                    </div>
                    {self.show_update_block(link, props)}
                    <div class="buttons">
                      {self.show_certificate_btn(link, props)}
                      <button id={"delete-cert"}
                          class="button is-danger is-fullwidth has-text-weight-bold"
                          onclick={onclick_delete_cert}>
                          { get_value_field(&135) }
                      </button>
                      {self.show_download_btn(props)}
                    </div>
                  </div>
                </div>
                <br/>
            </div>
        </>}
    }

    fn show_certificate_data(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_clear_error = link.callback(|_| Msg::ClearError);
        let onclick_show_cert = link.callback(|_| Msg::ShowCert);


        let cert_url = match image_detector(&props.certificate.file.filename) {
            true => props.certificate.file.download_url.clone(),
            false => String::from("https://bulma.io/images/placeholders/128x128.png"),
        };

        html!{<div class="boxItem" >
          <div class="innerBox" >
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <div class="imgBox" >
                <figure class="image is-256x256" onclick={onclick_show_cert} >
                    <img
                        // src="https://bulma.io/images/placeholders/128x128.png" alt="Image"
                        src={cert_url}
                        loading="lazy"
                    />
                </figure>
            </div>
            <div class="overflow-title has-text-weight-bold">{props.certificate.description.clone()}</div>
            <div class="btnBox">
              {self.show_certificate_btn(link, props)}
            </div>
          </div>
        </div>}
    }

    fn show_delete_certificate(&self) -> Html {
        html!{<div class="card">
            <div class="message is-success">
              <div class="message-header">{ get_value_field(&89) }</div>
              <div class="message-body">{ get_value_field(&263) }</div>
            </div>
        </div>}
    }

    fn show_update_description(&self) -> Html {
        match self.get_result_update {
            true => html!{<div class="column">
                <span id="remove-profile" class="tag is-info is-light">
                    { get_value_field(&140) }
                </span>
            </div>},
            false => html!{},
        }
    }

    fn show_certificate_btn(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_show_cert = link.callback(|_| Msg::ShowCert);

        let text_btn = match self.show_cert {
            true => "Hide",
            false => "Show",
        };

        match props.show_cert_btn {
            true => html!{
                <button id={"show-cert"}
                    class="button is-light is-fullwidth has-text-weight-bold"
                    onclick={onclick_show_cert}>
                    { text_btn }
                </button>
            },
            false => html!{},
        }
    }

    fn show_update_block(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let oninput_cert_description =
            link.callback(|ev: Event| Msg::UpdateDescription(ev.value));

        let onclick_change_cert = link.callback(|_| Msg::RequestUpdateDescription);

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
                        placeholder={get_value_field(&61)}
                        value={self.request_update.to_string()}
                        oninput={oninput_cert_description} />
                </div>
                <div class="column">
                    <button id={"change-cert"}
                        class="button is-light is-fullwidth has-text-weight-bold"
                        onclick={onclick_change_cert}>
                        { get_value_field(&46) }
                    </button>
                </div>
            </div>
        </div>}
    }

    fn show_download_btn(
        &self,
        props: &Props,
    ) -> Html {
        match props.download_btn {
            true => html!{<button id={"down-cert"}
                class="button is-light is-fullwidth has-text-weight-bold"
                href={ props.certificate.file.download_url.clone() }
                download={ props.certificate.file.filename.clone() }>
                { get_value_field(&126) }
            </button>},
            false => html!{},
        }
    }

    fn modal_full_certificate(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_show_cert = link.callback(|_| Msg::ShowCert);

        let class_modal = match &self.show_cert {
            true => "modal is-active",
            false => "modal",
        };

        let cert_url = match image_detector(&props.certificate.file.filename) {
            true => props.certificate.file.download_url.clone(),
            false => String::from("https://bulma.io/images/placeholders/128x128.png"),
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_show_cert.clone()} />
              <div class="modal-content">
                <p class="image is-4by3">
                  <img
                    src={cert_url}
                    loading="lazy"
                  />
                </p>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick={onclick_show_cert} />
            </div>
        }
    }
}
