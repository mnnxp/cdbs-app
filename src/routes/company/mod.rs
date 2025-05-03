// pub mod register;
pub mod create;
pub mod settings;
pub mod show;
pub mod supplier;

pub use create::CreateCompany;
pub use settings::CompanySettings;
pub use show::ShowCompany;
pub use supplier::ShowSupplierCompany;

use yew::virtual_dom::VNode;
use yew::{classes, html, Classes, Html};
use crate::fragments::company::SpecsTags;
use crate::services::content_adapter::{
    ContentDisplay, DateDisplay, ContactDisplay, SpecDisplay, two_dates_display
};
use crate::services::{get_lang, get_value_field};
use crate::types::{CompanyInfo, CompanyType};

impl ContentDisplay for CompanyInfo {
    /// Returns a company name and type of the company, the sequence depends on the localization
    fn to_display(&self) -> Html {
        let company_name = html!{
            <span id="title-orgname" class="title is-4">{self.orgname.clone()}</span>
        };
        if self.company_type.company_type_id == 10 {
            // Do not show if set "Other legal entity"
            return html!{<p>{company_name}</p>}
        }
        let company_name_short = html!{
            <span id="title-orgname" class="title is-6">{self.shortname.clone()}</span>
        };
        html!{<>
            <p>{company_name}</p>
            {self.company_type.to_dispaly_order(classes!("is-6"), company_name_short)}
        </>}
    }
}

impl CompanyType {
    /// Determines the order in which the name and type of the company are displayed depending on the set language.
    /// If the company type is specified as "Other legal entity" (id 10), then only the name in the <p> tag is returned.
    /// For example, for Russian, first the type, then the name, and for English, vice versa.
    pub(crate) fn to_dispaly_order(&self, classes_type: Classes, name: VNode) -> Html {
        if self.company_type_id == 10 {
            // Do not show if set "Other legal entity"
            return html!{<p>{name}</p>}
        }
        let company_type_short = html!{
            <span id="title-type" class={classes_type}>{self.shortname.clone()}</span>
        };
        let lang = get_lang().unwrap_or(String::new());
        match lang.as_str() {
            "ru" => html!{<p>{company_type_short}<span>{" "}</span>{name}</p>},
            _ => html!{<p>{name}<span>{" "}</span>{company_type_short}</p>}
        }
    }
}

impl DateDisplay for CompanyInfo {
    /// Returns VNode (Html) with convert dates to display.
    /// Update date and creation/update with time in abbr tag.
    fn date_to_display(&self) -> Html {
        two_dates_display(&self.created_at, &self.updated_at)
    }
}

impl ContactDisplay for CompanyInfo {
    /// Returns company contact information with description and icons, blank fields are skipped
    fn contact_block(&self) -> Html {
        html!{<>
            <div id="company-inn" hidden={self.inn.is_empty()}>
                <span class="icon is-small"><i class="fas fa-building" /></span>
                <span>{get_value_field(&280)}</span> // Reg.№
                <span class="has-text-weight-bold">{self.inn.clone()}</span>
            </div>
            <div id="company-email" hidden={self.email.is_empty()}>
                <span class="icon is-small"><i class="fas fa-envelope" /></span>
                <span>{get_value_field(&278)}</span> // Email
                <span class="has-text-weight-bold">{self.email.clone()}</span>
            </div>
            <div id="company-phone" hidden={self.phone.is_empty()}>
                <span class="icon is-small"><i class="fas fa-phone" /></span>
                <span>{get_value_field(&279)}</span> // Phone
                <span class="has-text-weight-bold">{self.phone.clone()}</span>
            </div>
            <div id="company-region" hidden={self.region.region_id == 8 && self.address.is_empty()}>
                <span class="icon is-small"><i class="fas fa-map-marker-alt" /></span>
                <span>{get_value_field(&281)}</span> // Location
                <span class="has-text-weight-bold">
                    {match self.address.is_empty() {
                        true => {self.region.region.clone()},
                        false => {self.address.clone()},
                    }}
                </span>
            </div>
            <div id="company-site_url" hidden={self.site_url.is_empty()}>
                <span class="icon is-small"><i class="fas fa-globe" /></span>
                <span>{get_value_field(&282)}</span> // Site
                <span class="has-text-weight-bold">{self.site_url.clone()}</span>
            </div>
        </>}
    }
}

impl SpecDisplay for CompanyInfo {
    /// Returns company-related catalogs and specifics
    fn spec_block(&self) -> Html {
        match self.company_specs.is_empty() {
            true => html!{},
            false => html!{
                <div id={"company-related-catalogs"} class={"column p-0"}>
                    <p class={"title is-6"}>
                        <span class={"icon is-small"}><i class={"fas fa-cubes"}></i></span>
                        <span>{" "}{get_value_field(&283)}</span>
                    </p> // Sphere of activity
                    <SpecsTags
                        show_manage_btn={false}
                        company_uuid={self.uuid.clone()}
                        specs={self.company_specs.clone()}
                    />
                </div>
            }
        }
    }
}