mod catalog;
mod certificate;
mod represent;
mod spec;
mod menu_block;

pub(crate) use menu_block::{view_content, view_certificates, view_represents, view_components, view_standards, view_services};
pub use catalog::{CatalogCompanies, ListItemCompany};
pub use certificate::{CompanyCertificatesCard, CompanyCertificateItem, AddCompanyCertificateCard};
pub use represent::{CompanyRepresents, AddCompanyRepresentCard};
pub use spec::{SpecsTags, SpecTagItem, SearchSpecsTags};

use yew::{html, Html};

pub(crate) fn diamond_svg(is_supplier: bool, size: &'static str) -> Html {
    html!{
        <span hidden={!is_supplier}>
            <i class="fa fa-diamond" aria-hidden="true"></i>
            <svg height={size} viewBox="0 0 197.249 197.25" width={size} xmlns="http://www.w3.org/2000/svg"><g transform="translate(-11.136 -18.506)"><path d="m44.396 115.725 25.955-33.866h77.2l26.287 33.346-63.596 68.922z" style="fill:#1872f012;stroke:#1872f08a;stroke-width:.434;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"></path><path d="m43.338 116.783 129.441-.52M122.778 81.857l17.736 33.672-30.272 68.598-31.858-68.419 17.978-33.843z" style="fill:none;stroke:#1872f08a;stroke-width:.434204px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"></path><path d="M208.167 130.384v-26.505c-13.539-4.814-22.092-6.167-26.398-16.557v-.008c-4.321-10.423.82-17.5 6.946-30.4l-18.738-18.739c-12.801 6.085-19.952 11.276-30.4 6.946h-.008c-10.406-4.313-11.768-12.924-16.557-26.398H96.508C91.735 32.131 90.365 40.8 79.95 45.121h-.007c-10.423 4.33-17.483-.804-30.4-6.946L30.805 56.914c6.11 12.858 11.276 19.96 6.946 30.4-4.322 10.423-12.99 11.792-26.398 16.565v26.505c13.383 4.756 22.076 6.142 26.398 16.557 4.346 10.513-.935 17.762-6.946 30.4l18.738 18.746c12.81-6.093 19.96-11.276 30.4-6.946h.008c10.415 4.314 11.776 12.95 16.557 26.398h26.504c4.773-13.416 6.151-22.06 16.623-26.422h.008c10.35-4.297 17.386.828 30.326 6.979l18.739-18.747c-6.101-12.818-11.276-19.952-6.954-30.392 4.321-10.423 13.022-11.809 26.414-16.573z" style="fill:none;stroke:#1872f08a;stroke-width:.434;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"></path><ellipse cx="109.449" cy="115.983" rx="69.586" ry="69.587" style="fill:none;stroke:#1872f08a;stroke-width:.433999;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"></ellipse></g></svg>
        </span>
    }
}