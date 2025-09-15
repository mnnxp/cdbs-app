use yew::{html, Component, Callback, ComponentLink, Html, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::fragments::file::CertificateItem;
use crate::fragments::list_errors::ListErrors;
use crate::error::Error;
use crate::services::resp_parsing;
use crate::types::{UUID, UserCertificate, Certificate};
use crate::gqls::make_query;
use crate::gqls::user::{
    UpdateUserCertificate, update_user_certificate,
    DeleteUserCertificate, delete_user_certificate,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub certificate: UserCertificate,
    pub show_cert_btn: bool,
    pub manage_btn: bool,
    pub callback_delete_cert: Callback<UUID>,
}

pub enum Msg {
    RequestUpdateDescription(UUID, String),
    RequestDeleteCert(UUID),
    GetUpdateResult(String),
    GetDeleteCertResult(String),
    UpdateDescription(String),
    ShowCert,
    ResponseError(Error),
    ClearError,
    Ignore,
}

#[derive(Debug)]
pub struct UserCertificateItem {
    error: Option<Error>,
    request_update: String,
    props: Props,
    link: ComponentLink<Self>,
    get_result_update: bool,
    get_result_delete: bool,
    show_cert: bool,
}

impl Component for UserCertificateItem {
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
            Msg::RequestUpdateDescription(file_uuid, description) => {
                debug!("Update user cert: {:?}", &description);
                let ipt_update_user_certificate_data = update_user_certificate::IptUpdateUserCertificateData {
                    fileUuid: file_uuid,
                    description,
                };

                spawn_local(async move {
                    let res = make_query(UpdateUserCertificate::build_query(update_user_certificate::Variables {
                        ipt_update_user_certificate_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateResult(res));
                })
            },
            Msg::RequestDeleteCert(file_uuid) => {
                // let file_uuid = self.props.certificate.file.uuid.clone();
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
            Msg::GetUpdateResult(res) => {
                match resp_parsing(res, "updateUserCertificate") {
                    Ok(result) => {
                        debug!("Update company cert: {:?}", result);
                        self.get_result_update = result;
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetDeleteCertResult(res) => {
                match resp_parsing(res, "deleteUserCertificate") {
                    Ok(result) => {
                        debug!("Update company cert: {:?}", result);
                        self.get_result_delete = result;
                        if self.get_result_delete {
                            self.props.callback_delete_cert.emit(self.props.certificate.file.uuid.clone());
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateDescription(description) => self.request_update = description,
            Msg::ShowCert => self.show_cert = !self.show_cert,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.certificate.file.uuid == props.certificate.file.uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_change_cert =
            self.link.callback(|(file_uuid, description)| Msg::RequestUpdateDescription(file_uuid, description));
        let onclick_delete_cert =
            self.link.callback(|file_uuid| Msg::RequestDeleteCert(file_uuid));
        let (onclick_change_cert, onclick_delete_cert) = match self.props.manage_btn {
            true => (Some(onclick_change_cert), Some(onclick_delete_cert)),
            false => (None, None),
        };

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <CertificateItem
                certificate={Certificate::from(self.props.certificate.clone())}
                show_cert_btn={self.props.show_cert_btn}
                manage_btn={self.props.manage_btn}
                get_result_update={self.get_result_update}
                get_result_delete={self.get_result_delete}
                callback_update_descript={onclick_change_cert}
                callback_delete_cert={onclick_delete_cert}
                />
        </>}
    }
}
