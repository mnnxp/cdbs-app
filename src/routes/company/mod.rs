// pub mod register;
pub mod create;
pub mod settings;
pub mod show;
pub mod supplier;

pub use create::CreateCompany;
pub use settings::CompanySettings;
pub use show::ShowCompany;
pub use supplier::ShowSupplierCompany;

use yew::{html, Html};
use crate::fragments::company::SpecsTags;
use crate::services::content_adapter::{
    ContentDisplay, DateDisplay, ContactDisplay, SpecDisplay, two_dates_display
};
use crate::services::{get_lang, get_value_field};
use crate::types::CompanyInfo;

impl ContentDisplay for CompanyInfo {
    /// Returns a company name and type of the company, the sequence depends on the localization
    fn to_display(&self) -> Html {
        let company_name_short = html!{
            <span id="title-orgname" class="title is-6">{self.shortname.clone()}</span>
        };
        let company_name = html!{
            <span id="title-orgname" class="title is-4">{self.orgname.clone()}</span>
        };
        let company_type_short = html!{
            <span id="title-type" class="subtitle is-6">{self.company_type.shortname.clone()}</span>
        };
        let company_type = html!{
            <span id="title-type" class="subtitle is-4">{self.company_type.name.clone()}</span>
        };
        let lang = get_lang().unwrap_or(String::new());
        match lang.as_str() {
            "ru" => html!{<>
                <p>{company_type}<br/>{company_name}</p>
                <p>{company_type_short}<span>{" "}</span>{company_name_short}</p>
            </>},
            _ => html!{<>
                <p>{company_name}<span><br/></span>{company_type}</p>
                <p>{company_name_short}<span>{" "}</span>{company_type_short}</p>
            </>}
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
            <div id="company-email" hidden={self.email.is_empty()}>
                <span class="icon is-small"><i class="fas fa-envelope" /></span>
                <span>{ get_value_field(&278) }</span> // Email
                <span class="has-text-weight-bold">{self.email.clone()}</span>
            </div>
            <div id="company-phone" hidden={self.phone.is_empty()}>
                <span class="icon is-small"><i class="fas fa-phone" /></span>
                <span>{ get_value_field(&279) }</span> // Phone
                <span class="has-text-weight-bold">{self.phone.clone()}</span>
            </div>
            <div id="company-inn" hidden={self.inn.is_empty()}>
                <span class="icon is-small"><i class="fas fa-building" /></span>
                <span>{ get_value_field(&280) }</span> // Reg.№
                <span class="has-text-weight-bold">{self.inn.clone()}</span>
            </div>
            <div id="company-region">
                <span class="icon is-small"><i class="fas fa-map-marker-alt" /></span>
                <span>{ get_value_field(&281) }</span> // Location
                <span class="has-text-weight-bold">{self.region.region.clone()}</span>
                <span id="company-address" class="has-text-weight-bold" hidden={self.address.is_empty()}>
                    {format!(", {}", self.address.clone())}
                </span>
            </div>
            <div id="company-site_url" hidden={self.site_url.is_empty()}>
                <span class="icon is-small"><i class="fas fa-globe" /></span>
                <span>{ get_value_field(&282) }</span> // Site
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
                <div class="columns">
                    <div>
                        <span>{ get_value_field(&283) }</span> // Sphere of activity
                    </div>
                    <div class="px-1 mb-4">
                        <SpecsTags
                            show_manage_btn = false
                            company_uuid = self.uuid.clone()
                            specs = self.company_specs.clone()
                        />
                    </div>
                </div>
            }
        }
    }
}