mod add_certificate;
mod item;

pub use add_certificate::AddUserCertificateCard;
pub use item::UserCertificateItem;

use std::collections::BTreeSet;
use yew::{Component, Context, html, Html, Properties};
// use log::debug;

use crate::types::{UUID, UserCertificate};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub user_uuid: UUID,
    pub certificates: Vec<UserCertificate>,
    pub show_cert_btn: bool,
    pub download_btn: bool,
    pub manage_btn: bool,
}

pub enum Msg {
    RemoveCertificate(UUID),
    Ignore,
}

#[derive(Debug)]
pub struct UserCertificatesCard {
    user_uuid: UUID,
    certificates_len: usize,
    deleted_cert_list: BTreeSet<UUID>,
}

impl Component for UserCertificatesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user_uuid: ctx.props().user_uuid,
            certificates_len: ctx.props().certificates.len(),
            deleted_cert_list: BTreeSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RemoveCertificate(cart_uuid) => {
                self.deleted_cert_list.insert(cart_uuid);
            },
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.user_uuid == ctx.props().user_uuid &&
              self.certificates_len == ctx.props().certificates.len() {
            false
        } else {
            self.user_uuid = ctx.props().user_uuid;
            self.certificates_len = ctx.props().certificates.len();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback_delete_cert = ctx.link().callback(|value: UUID| Msg::RemoveCertificate(value));

        html!{{ for ctx.props().certificates.iter().map(|certificate|
            match self.deleted_cert_list.get(&certificate.file.uuid) {
                Some(_) => html!{}, // deleted certificate
                None => html!{
                    <UserCertificateItem
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
