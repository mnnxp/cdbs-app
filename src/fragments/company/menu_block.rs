use yew::{html, Html};
use crate::fragments::company::{CompanyCertificatesCard, CompanyRepresents};
use crate::fragments::component::CatalogComponents;
use crate::fragments::list_empty::ListEmpty;
use crate::fragments::standard::CatalogStandards;
use crate::fragments::supplier_service::CatalogServices;
use crate::types::{CompanyInfo, ComponentsQueryArg, ServicesQueryArg, StandardsQueryArg, UUID};
use crate::services::content_adapter::{Markdownable, ContactDisplay, SpecDisplay};

pub(crate) fn view_content(company_data: &CompanyInfo) -> Html {
    html! {
        <div class="profileBox" >
            <div class="column">
                <div class="columns">
                    <div class="column is-two-thirds">
                        <div id="description" class="content">
                            {company_data.description.to_markdown()}
                        </div>
                    </div>
                    <div class="column is-narrow">
                        {company_data.contact_block()}
                    </div>
                </div>
                {company_data.spec_block()}
            </div>
        </div>
    }
}

pub(crate) fn view_certificates(company_data: &CompanyInfo) -> Html {
    if company_data.company_certificates.is_empty() {
        html!{<ListEmpty />}
    } else {
        html!{<div class="profileBox" >
            <CompanyCertificatesCard
                certificates={company_data.company_certificates.clone()}
                show_cert_btn={false}
                download_btn={false}
                manage_btn={false}
             />
        </div>}
    }
}

pub(crate) fn view_represents(company_data: &CompanyInfo) -> Html {
    html!{
        <CompanyRepresents
            show_manage_btn={false}
            list={company_data.company_represents.clone()}
        />
    }
}

pub(crate) fn view_components(company_uuid: &UUID) -> Html {
    html!{
        <CatalogComponents
            show_create_btn={false}
            arguments={ComponentsQueryArg::set_company_uuid(company_uuid)}
        />
    }
}

pub(crate) fn view_standards(company_uuid: &UUID) -> Html {
    html!{
        <CatalogStandards
            show_create_btn={true}
            arguments={StandardsQueryArg::set_company_uuid(company_uuid)}
        />
    }
}

pub(crate) fn view_services(company_uuid: &UUID) -> Html {
    html!{
        <CatalogServices arguments={ServicesQueryArg::set_company_uuid(company_uuid)} />
    }
}

// pub(crate) fn view_members(company_uuid: &UUID) -> Html {
//     html!{<CatalogUsers arguments = UsersQueryArg::set_favorite() />}
// }