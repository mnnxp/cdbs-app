mod add_certificate;
mod item;

pub use add_certificate::AddCompanyCertificateCard;
pub use item::CompanyCertificateItem;

use std::collections::BTreeSet;
use yew::{Component, Context, html, Html, Properties};
// use log::debug;

use crate::types::{UUID, CompanyCertificate};


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub certificates: Vec<CompanyCertificate>,
    pub show_cert_btn: bool,
    pub download_btn: bool,
    pub manage_btn: bool,
}

/// For viewing certificate data on page
#[derive(Debug)]
pub struct CompanyCertificatesCard {
    deleted_cert_list: BTreeSet<UUID>,
    certificates_first_uuid: UUID,
    certificates_len: UUID,
}

pub enum Msg {
    RemoveCertificate(UUID),
    Ignore,
}

impl Component for CompanyCertificatesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            deleted_cert_list: BTreeSet::new(),
            certificates_first_uuid: ctx.props().certificates.first().map(|x| &x.company_uuid),
            certificates_len: ctx.props().certificates.len(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // let link = ctx.link().clone();
        match msg {
            Msg::RemoveCertificate(cart_uuid) => self.deleted_cert_list.insert(cart_uuid),
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.certificates_first_uuid == ctx.props().certificates.first().map(|x| &x.company_uuid) &&
              self.certificates_len == ctx.props().certificates.len() {
            false
        } else {
            self.certificates_first_uuid = ctx.props().certificates.first().map(|x| &x.company_uuid);
            self.certificates_len = ctx.props().certificates.len();
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback_delete_cert = ctx.link().callback(|value: UUID| Msg::RemoveCertificate(value));

        html!{{ for ctx.props().certificates.iter().map(|certificate|
            match self.deleted_cert_list.get(&certificate.file.uuid) {
                Some(_) => html!{}, // deleted certificate
                None => html!{
                    <CompanyCertificateItem
                        certificate = {certificate.clone()}
                        show_cert_btn = {ctx.props().show_cert_btn}
                        download_btn = {ctx.props().download_btn}
                        manage_btn = {ctx.props().manage_btn}
                        callback_delete_cert = {callback_delete_cert.clone()}
                    />
                },
            }
        )}}
    }
}
