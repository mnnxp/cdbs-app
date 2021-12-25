use yew::{
    html, Component, ComponentLink, Html, Properties, ShouldRender, InputData,
};
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::gqls::make_query;

use crate::fragments::list_errors::ListErrors;
use crate::error::{Error, get_error};
use crate::types::{UUID, Certificate};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct UpdateUserCertificate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct DeleteUserCertificate;


#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub certificate: Certificate,
    pub show_cert_btn: bool,
    pub download_btn: bool,
    pub change_btn: bool,
}

/// For update user Certificate
#[derive(Default, Clone, Debug)]
pub struct ChangeUserCertData {
    file_uuid: String,
    description: String,
}

/// For viewing certificate data on page
#[derive(Debug)]
pub struct CertificateCard {
    error: Option<Error>,
    request_update: String,
    props: Props,
    link: ComponentLink<Self>,
    get_result_update: bool,
    get_result_delete: bool,
    show_cert: bool,
}

pub enum Msg {
    RequestUpdateDescription,
    RequestDeleteCert,
    ResponseError(Error),
    GetUpdateResult(String),
    GetDeleteCertResult(String),
    Ignore,
    UpdateDescription(String),
    ShowCert,
}

impl Component for CertificateCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_update: props.certificate.description.to_string(),
            // request_delete: DeleteUserCertData::default(),
            props,
            link,
            get_result_update: false,
            get_result_delete: false,
            show_cert: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestUpdateDescription => {
                debug!("Update user cert: {:?}", &self.request_update);
                let request_update = ChangeUserCertData{
                    file_uuid: self.props.certificate.file.uuid.to_string(),
                    description: self.request_update.to_string(),
                };
                spawn_local(async move {
                    let ChangeUserCertData {
                        file_uuid,
                        description,
                    } = request_update;
                    let ipt_update_user_certificate_data = update_user_certificate::IptUpdateUserCertificateData {
                        fileUuid: file_uuid,
                        description,
                    };
                    let res = make_query(UpdateUserCertificate::build_query(update_user_certificate::Variables {
                        ipt_update_user_certificate_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateResult(res));
                })
            },
            Msg::RequestDeleteCert => {
                let file_uuid = self.props.certificate.file.uuid.clone();
                spawn_local(async move {
                    let del_user_certificate_data = delete_user_certificate::DelUserCertificateData{
                        fileUuid: file_uuid,
                    };
                    let res = make_query(DeleteUserCertificate::build_query(delete_user_certificate::Variables {
                        del_user_certificate_data
                    })).await.unwrap();
                    link.send_message(Msg::GetDeleteCertResult(res));
                })
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
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
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::GetDeleteCertResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res_value.get("deleteUserCertificate").unwrap().clone()).unwrap();
                        debug!("Update user cert: {:?}", result);

                        self.get_result_delete = result;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::Ignore => {},
            Msg::UpdateDescription(description) => {
                // debug!("Description: {:?}", description);
                self.request_update = description;
            },
            Msg::ShowCert => {
                match self.show_cert {
                    true => self.show_cert = false,
                    false => self.show_cert = true,
                }
            },
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props = props;
        false
    }

    fn view(&self) -> Html {
        let Props {
            certificate,
            show_cert_btn,
            download_btn,
            change_btn,
        } = &self.props;

        if self.get_result_delete {
            html!(
                <div class="card">
                    <article class="message is-success">
                      <div class="message-header">
                        <p>{ "Success" }</p>
                        // <button class="delete" aria-label="delete"></button>
                      </div>
                      <div class="message-body">
                        { "This certificate removed!" }
                      </div>
                    </article>
                    // <span id="tag-danger-remove-profile" class="tag is-danger is-light">
                    //  { "This certificate removed!" }
                    // </span>
                </div>
            )
        } else {
            html!{
                <div class="card">
                    <ListErrors error=self.error.clone()/>
                    {match self.show_cert {
                        true => html!{
                            { self.show_certificate_on_page() }
                        },
                        false => html!{
                            { certificate.file.filename.to_string() }
                        },
                    }}
                    <div class="card-content">
                        {match (download_btn, change_btn) {
                            (true, false) => html!{<>
                                { self.show_cert_description() }
                                <br/>
                                { self.show_btn_download() }
                            </>},
                            (false, true) => html!{<>
                                { self.show_btn_change() }
                                // <br/>
                                { self.show_btn_delete() }
                            </>},
                            (true, true) => html!{<>
                                { self.show_btn_change() }
                                // <br/>
                                { self.show_btn_delete() }
                                <br/>
                                { self.show_btn_download() }
                            </>},
                            (false, false) => html!{<>
                                { self.show_cert_description() }
                                <br/>
                            </>},
                        }}
                        {match show_cert_btn {
                            true => html!{
                                { self.show_btn_certificate() }
                            },
                            false => html!{},
                        }}
                    </div>
                </div>
            }
        }
    }
}

impl CertificateCard {
    fn show_certificate_on_page(
        &self,
    ) -> Html {
        html!{<div class="card-image">
            <figure class="image is-4by5">
                <img src=
                    { self.props.certificate.file.download_url.to_string() }
                    loading="lazy" />
            </figure>
        </div>}
    }

    fn show_btn_certificate(
        &self,
    ) -> Html {
        let onclick_show_cert = self
            .link
            .callback(|_| Msg::ShowCert);

        let text_btn = match self.show_cert {
            true => "Hide",
            false => "Show",
        };

        html!{<a id={ format!(
            "btn-show-cert-{}", &self.props.certificate.file.uuid) }
            class="button"
            onclick=onclick_show_cert>
            { text_btn }
        </a>}
    }

    fn show_cert_description(
        &self,
    ) -> Html {
        html!{ self.props.certificate.description.to_string() }
    }

    fn show_btn_download(
        &self,
    ) -> Html {
        html!{
            <a id={ format!("btn-down-cert-{}", &self.props.certificate.file.uuid) }
                class="button"
                href={ self.props.certificate.file.download_url.to_string() }
                download={ self.props.certificate.file.filename.to_string() }>
                { "Download" }
            </a>
        }
    }

    fn show_btn_change(
        &self,
    ) -> Html {
        let oninput_cert_description = self
            .link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        let onclick_change_cert = self
            .link
            .callback(|_| Msg::RequestUpdateDescription);

        html!{<>
            <label class="label">{"Description"}</label>

            {if self.get_result_update {
                html!{<span id="tag-info-remove-profile" class="tag is-info is-light">
                 { "Description updated!" }
                </span>}
            } else { html!{} }}

            <input
                id={ format!("cert-description-{}", &self.props.certificate.file.uuid) }
                class="input"
                type="text"
                placeholder="certificate description"
                value={ self.request_update.to_string() }
                oninput=oninput_cert_description />
            <br/>
            <a id={ format!("btn-change-cert-{}", &self.props.certificate.file.uuid) }
                class="button"
                onclick=onclick_change_cert>
                { "Change" }
            </a>
        </>}
    }

    fn show_btn_delete(
        &self,
    ) -> Html {
        let onclick_delete_cert = self
            .link
            .callback(|_| Msg::RequestDeleteCert);

        html!{<a id={ format!(
            "btn-delete-cert-{}", &self.props.certificate.file.uuid) }
            class="button"
            onclick=onclick_delete_cert>
            { "Delete" }
        </a>}
    }
}
