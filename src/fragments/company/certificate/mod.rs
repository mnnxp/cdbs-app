mod add_certificate;
mod item;

pub use add_certificate::AddCompanyCertificateCard;
pub use item::CompanyCertificateItem;

use std::collections::BTreeSet;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
// use log::debug;

use crate::types::{UUID, CompanyCertificate};


#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub certificates: Vec<CompanyCertificate>,
    pub show_cert_btn: bool,
    pub manage_btn: bool,
}

/// For viewing certificate data on page
#[derive(Debug)]
pub struct CompanyCertificatesCard {
    props: Props,
    link: ComponentLink<Self>,
    deleted_cert_list: BTreeSet<UUID>,
}

pub enum Msg {
    RemoveCertificate(UUID),
    Ignore,
}

impl Component for CompanyCertificatesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            deleted_cert_list: BTreeSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::RemoveCertificate(cart_uuid) => {
                self.deleted_cert_list.insert(cart_uuid);
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.certificates.first().map(|x| &x.company_uuid) == props.certificates.first().map(|x| &x.company_uuid) &&
              self.props.certificates.len() == props.certificates.len() {
            false
        } else {
            self.props = props;
            false
        }
    }

    fn view(&self) -> Html {
        let callback_delete_cert = self.link.callback(|value: UUID| Msg::RemoveCertificate(value));

        html!{{ for self.props.certificates.iter().map(|certificate|
            match self.deleted_cert_list.get(&certificate.file.uuid) {
                Some(_) => html!{}, // deleted certificate
                None => html!{
                    <CompanyCertificateItem
                        certificate={certificate.clone()}
                        show_cert_btn={self.props.show_cert_btn}
                        manage_btn={self.props.manage_btn}
                        callback_delete_cert={callback_delete_cert.clone()}
                    />
                },
            }
        )}}
    }
}
