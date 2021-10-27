// use crate::{
//     content::Certificate,
//     generator::Generated,
//     switch::{AppAnchor, AppRoute},
// };
// use crate::routes::AppRoute;
use crate::types::Certificate;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
// use yew_router::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub certificate: Certificate,
}

pub struct CertificateCard {
    certificate: Certificate,
}

impl Component for CertificateCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            certificate: props.certificate,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.certificate.file.uuid == props.certificate.file.uuid {
            false
        } else {
            self.certificate = props.certificate;
            true
        }
    }

    fn view(&self) -> Html {
        let Self { certificate } = self;
        html! {
            <div class="card">
                // <div class="card-image">
                //     <figure class="image is-4by5">
                //         <img src={ certificate.file.download.download_url.to_string() } loading="lazy" />
                //     </figure>
                // </div>
                <div class="card-content">
                    { certificate.file.download.filename.to_string() }
                    <br/>
                    { certificate.description.to_string() }
                    <br/>
                    <a class="button"
                        href={ certificate.file.download.download_url.to_string() }
                        download={ certificate.file.download.filename.to_string() }>
                        { "Download" }
                    </a>
                </div>
            </div>
        }
    }
}
