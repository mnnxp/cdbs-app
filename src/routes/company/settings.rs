use yew::{
    agent::Bridged, html, classes, Bridge, ChangeData, Component, ComponentLink,
    FocusEvent, Html, Callback, InputData, Properties, ShouldRender, MouseEvent
};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    service::RouteService,
    prelude::RouteAgent,
};
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::fragments::company::CompanyAccessBlock;
use crate::fragments::form_input::InputConfig;
use crate::fragments::type_access::TypeAccessBlock;
use crate::gqls::make_query;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    buttons::{ft_save_btn, ft_submit_btn, ft_delete_class_btn},
    notification::show_notification,
    company::{
        CompanyCertificatesCard, AddCompanyCertificateCard,
        CompanyRepresents, SearchSpecsTags
    },
    list_errors::ListErrors,
    side_menu::{MenuBuilder, MenuItemTemplate},
    upload_favicon::UpdateFaviconBlock,
};
use crate::services::content_adapter::DateDisplay;
use crate::services::{get_logged_user, LocaleKey, resp_parsing, get_value_response, get_from_value};
use crate::types::{
    UUID, SlimUser, CompanyUpdateInfo, CompanyInfo, Region,
    CompanyType, TypeAccessInfo
};
use crate::gqls::company::{
    GetCompanySettingDataOpt, get_company_setting_data_opt,
    GetCompanyData, get_company_data,
    CompanyUpdate, company_update,
    ChangeCompanyAccess, change_company_access,
    DeleteCompany, delete_company,
};

impl MenuBuilder for CompanySettings {
    type TabType = Menu;

    fn menu_config() -> &'static [MenuItemTemplate<Menu>] {
        use Menu::*;
        &[
            MenuItemTemplate { lk_title: LocaleKey::OpenCompanyLabel, icon_classes: &[&["fas", "fa-angle-double-left"]], tab: OpenCompany, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::Company, icon_classes: &[&["fas", "fa-building"]], tab: Company, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::CompanyLogo, icon_classes: &[&["fas", "fa-image"]], tab: UpdateFavicon, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::Representations, icon_classes: &[&["fas", "fa-industry"]], tab: Represent, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::CertificatesLabel, icon_classes: &[&["fas", "fa-certificate"]], tab: Certificates, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::SphereOfActivity, icon_classes: &[&["fas", "fa-paperclip"]], tab: Spec, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::Members, icon_classes: &[&["fas", "fa-users"]], tab: Member, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::Access, icon_classes: &[&["fas", "fa-low-vision"]], tab: Access, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::RemoveCompany, icon_classes: &[&["fas", "fa-trash"]], tab: RemoveCompany, custom_class: Some("has-background-danger-light") },
        ]
    }

    fn is_active(&self, tab: &Menu) -> bool { self.select_menu == *tab }
    fn get_count(&self, _tab: &Menu) -> usize { 0 }
    fn is_extend(&self, _tab: &Menu) -> bool { false }
    fn get_action(&self, tab: &Menu) -> Callback<MouseEvent> {
        if *tab == Menu::OpenCompany {
            self.link.callback(|_| Msg::OpenCompany)
        } else {
            self.cb_generator(tab.clone())
        }
    }
}

/// Get data current company
impl From<CompanyInfo> for CompanyUpdateInfo {
    fn from(data: CompanyInfo) -> Self {
        Self {
            orgname: Some(data.orgname),
            shortname: Some(data.shortname),
            inn: Some(data.inn),
            phone: Some(data.phone),
            email: Some(data.email),
            description: Some(data.description),
            address: Some(data.address),
            site_url: Some(data.site_url),
            time_zone: Some(data.time_zone),
            region_id: Some(data.region.region_id as i64),
            company_type_id: Some(data.company_type.company_type_id as i64),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Menu {
    OpenCompany,
    Company,
    UpdateFavicon,
    Represent,
    Certificates,
    Spec,
    Access,
    RemoveCompany,
    Member,
}

/// Update settings of the author or logout
pub struct CompanySettings {
    error: Option<Error>,
    request_company: CompanyUpdateInfo,
    request_access: i64,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    company_uuid: String,
    current_data: Option<CompanyInfo>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessInfo>,
    company_types: Vec<CompanyType>,
    get_result_update: usize,
    get_result_access: bool,
    get_result_remove_company: bool,
    loading: bool,
    get_confirm: UUID,
    select_menu: Menu,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub company_uuid: UUID,
}

pub enum Msg {
    OpenCompany,
    SelectMenu(Menu),
    RequestUpdateCompany,
    RequestChangeAccess,
    RequestRemoveCompany,
    ReguestCompanyData,
    ResponseError(Error),
    GetUpdateAccessResult(String),
    GetCompanyDataResult(String),
    GetUpdateCompanyResult(String),
    GetUpdateListResult(String),
    GetRemoveCompanyResult(String),
    UpdateTypeAccessId(usize),
    UpdateOrgname(String),
    UpdateShortname(String),
    UpdateInn(String),
    UpdatePhone(String),
    UpdateEmail(String),
    UpdateDescription(String),
    UpdateAddress(String),
    UpdateSiteUrl(String),
    UpdateTimeZone(String),
    UpdateCompanyTypeId(String),
    UpdateRegionId(String),
    ClearError,
    Ignore,
}

impl Component for CompanySettings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CompanySettings {
            error: None,
            request_company: CompanyUpdateInfo::default(),
            request_access: 0,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            company_uuid: String::new(),
            current_data: None,
            regions: Vec::new(),
            types_access: Vec::new(),
            company_types: Vec::new(),
            get_result_update: 0,
            get_result_access: false,
            get_result_remove_company: false,
            loading: false,
            get_confirm: String::new(),
            select_menu: Menu::Company,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if let None = get_logged_user() {
            // route to login page if not found token
            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
        };

        // get company uuid for request
        let route_service: RouteService<()> = RouteService::new();
        // get target company from route
        let target_company_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/company/settings/")
            .to_string();

        // get flag changing current company in route
        let not_matches_company_uuid = target_company_uuid != self.company_uuid;

        if first_render || not_matches_company_uuid {
            let link = self.link.clone();
            self.company_uuid = target_company_uuid.clone();

            spawn_local(async move {
                let res = make_query(GetCompanySettingDataOpt::build_query(get_company_setting_data_opt::Variables{
                    company_uuid: target_company_uuid
                })).await.unwrap();
                link.send_message(Msg::GetCompanyDataResult(res.clone()));
                link.send_message(Msg::GetUpdateListResult(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::OpenCompany => {
                // Redirect to user page
                if let Some(company_data) = &self.current_data {
                    self.router_agent.send(ChangeRoute(
                        AppRoute::ShowCompany(company_data.uuid.clone()).into()
                    ));
                }
            },
            Msg::RequestUpdateCompany => {
                self.loading = true;
                let company_uuid = self.company_uuid.clone();
                let ipt_update_company_data = company_update::IptUpdateCompanyData {
                    orgname: self.request_company.orgname.clone(),
                    shortname: self.request_company.shortname.clone(),
                    inn: self.request_company.inn.clone(),
                    phone: self.request_company.phone.clone(),
                    email: self.request_company.email.clone(),
                    description: self.request_company.description.clone(),
                    address: self.request_company.address.clone(),
                    siteUrl: self.request_company.site_url.clone(),
                    timeZone: self.request_company.time_zone.clone(),
                    regionId: self.request_company.region_id.clone(),
                    companyTypeId: self.request_company.company_type_id.clone(),
                };
                spawn_local(async move {
                    let res = make_query(CompanyUpdate::build_query(company_update::Variables {
                        company_uuid,
                        ipt_update_company_data,
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateCompanyResult(res));
                })
            },
            Msg::RequestChangeAccess => {
                self.loading = true;
                let company_uuid = self.company_uuid.clone();
                let new_type_access = self.request_access.clone();
                spawn_local(async move {
                    let change_type_access_company = change_company_access::ChangeTypeAccessCompany {
                        companyUuid: company_uuid,
                        newTypeAccessId: new_type_access,
                    };

                    let res = make_query(ChangeCompanyAccess::build_query(
                        change_company_access::Variables{change_type_access_company}
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateAccessResult(res));
                })
            },
            Msg::RequestRemoveCompany => {
                self.loading = true;
                let delete_company_uuid = self.company_uuid.clone();
                if self.get_confirm == delete_company_uuid {
                    spawn_local(async move {
                        let res = make_query(DeleteCompany::build_query(
                            delete_company::Variables{delete_company_uuid}
                        )).await.unwrap();
                        link.send_message(Msg::GetRemoveCompanyResult(res));
                    })
                } else {
                    self.get_confirm = delete_company_uuid;
                }
            },
            Msg::ReguestCompanyData => {
                let company_uuid = self.company_uuid.clone();
                spawn_local(async move {
                    let res = make_query(GetCompanyData::build_query(
                        get_company_data::Variables{company_uuid}
                    )).await.unwrap();
                    link.send_message(Msg::GetCompanyDataResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetUpdateAccessResult(res) => {
                self.loading = false;
                match resp_parsing(res, "changeCompanyAccess") {
                    Ok(result) => self.get_result_access = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("Change company access: {:?}", self.get_result_access);
            },
            Msg::GetCompanyDataResult(res) => {
                self.loading = false;
                match resp_parsing::<CompanyInfo>(res, "company") {
                    Ok(company_data) => {
                        debug!("Company data: {:?}", company_data);
                        self.current_data = Some(company_data.clone());
                        self.request_company = company_data.into();
                        self.rendered(false);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateListResult(res) => {
                self.loading = false;
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.regions = get_from_value(value, "regions").unwrap_or_default();
                        self.company_types = get_from_value(value, "companyTypes").unwrap_or_default();
                        self.types_access = get_from_value(value, "typesAccess").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetRemoveCompanyResult(res) => {
                self.loading = false;
                match resp_parsing::<UUID>(res, "deleteCompany") {
                    Ok(delete_company_uuid) => {
                        debug!("Delete company: {:?}", delete_company_uuid);
                        self.get_result_remove_company = true;
                        match &self.props.current_user {
                            Some(user) =>
                                self.router_agent.send(ChangeRoute(AppRoute::Profile(user.username.clone()).into())),
                            None => self.router_agent.send(ChangeRoute(AppRoute::Home.into())),
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateCompanyResult(res) => {
                self.loading = false;
                match resp_parsing(res, "putCompanyUpdate") {
                    Ok(result) => {
                        self.get_result_update = result;
                        link.send_message(Msg::ReguestCompanyData);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateTypeAccessId(type_access_id) => self.request_access = type_access_id as i64,
            Msg::UpdateOrgname(orgname) => self.request_company.orgname = Some(orgname),
            Msg::UpdateShortname(shortname) => self.request_company.shortname = Some(shortname),
            Msg::UpdateInn(inn) => self.request_company.inn = Some(inn),
            Msg::UpdateEmail(email) => self.request_company.email = Some(email),
            Msg::UpdatePhone(phone) => self.request_company.phone = Some(phone),
            Msg::UpdateDescription(description) => self.request_company.description = Some(description),
            Msg::UpdateAddress(address) => self.request_company.address = Some(address),
            Msg::UpdateSiteUrl(site_url) => self.request_company.site_url = Some(site_url),
            Msg::UpdateTimeZone(time_zone) => self.request_company.time_zone = Some(time_zone),
            Msg::UpdateRegionId(region_id) =>
                self.request_company.region_id = Some(region_id.parse::<i64>().unwrap_or_default()),
            Msg::UpdateCompanyTypeId(type_id) =>
                self.request_company.company_type_id = Some(type_id.parse::<i64>().unwrap_or_default()),
            Msg::SelectMenu(value) => {
                self.select_menu = value;
                self.rendered(false);
                // clear flags
                self.get_result_update = 0;
                self.get_result_access = false;
                self.get_result_remove_company = false;
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.company_uuid == props.company_uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{
            <div class="settings-page">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
                <div class={"container is-fluid page pl-0"}>
                    <div class="row">
                        <div class="columns">
                            <div class="column is-flex side-menu-content-fix">
                                {self.render_menu()}
                                <div class="card is-flex-grow-1 side-menu-content-fix">
                                    <div class="card-content">
                                        {self.select_content()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl CompanySettings {
    fn cb_generator(&self, cb: Menu) -> Callback<MouseEvent> {
        self.link.callback(move |_| Msg::SelectMenu(cb.clone()))
    }

    fn select_content(&self) -> Html {
        match self.select_menu {
            Menu::OpenCompany => html!{},
            // Show interface for change company data
            Menu::Company => self.manage_master_data(),
            // Show interface for change favicon company
            Menu::UpdateFavicon => self.update_favicon_block(),
            // Show interface for add and update Represents
            Menu::Represent => self.represents_block(),
            // Show interface for add and update Certificates
            Menu::Certificates => html!{<>
                <h4 id="updated-certificates" class="title is-4">{LocaleKey::CertificatesLabel.get_value()}</h4>
                {self.add_certificate_block()}
                <br/>
                {self.certificates_block()}
            </>},
            // Show interface for add and update company catalogs
            Menu::Spec => self.manage_specs_block(),
            // Show interface for manage Access
            Menu::Access => self.manage_access_block(),
            // Show interface for remove company
            Menu::RemoveCompany => self.remove_company_block(),
            // Show interface for member company
            Menu::Member => html!{
                <CompanyAccessBlock company_uuid={self.company_uuid.clone()} />
            },
        }
    }

    fn manage_master_data(&self) -> Html {
        let onsubmit_update_company = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestUpdateCompany
        });

        html!{<>
            <h4 id="updated-company" class="title is-4">{LocaleKey::Company.get_value()}</h4>
            <div class="columns">
                {show_notification(
                    &format!("{} {}", LocaleKey::UpdatedRows.get_value(), self.get_result_update),
                    "is-success",
                    self.get_result_update > 0,
                )}
                <div class="column">
                    <span class={classes!("overflow-title", "has-text-weight-bold")}>{LocaleKey::LastUpdated.get_value()}</span>
                    {match &self.current_data {
                        Some(data) => html!{
                            <span class="overflow-title">
                                {data.updated_at.date_to_display()}
                            </span>
                        },
                        None => html!{<span>{LocaleKey::NoData.get_value()}</span>},
                    }}
                </div>
            </div>
            <form onsubmit={onsubmit_update_company} >
                {self.fieldset_company()}
                <div class={"mt-5"}>{ft_submit_btn("update-settings")}</div>
            </form>
        </>}
    }

    fn fieldset_company(&self) -> Html {
        let oninput_orgname = self.link.callback(|ev: InputData| Msg::UpdateOrgname(ev.value));
        let oninput_shortname = self.link.callback(|ev: InputData| Msg::UpdateShortname(ev.value));
        let oninput_inn = self.link.callback(|ev: InputData| Msg::UpdateInn(ev.value));
        let oninput_email = self.link.callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let oninput_phone = self.link.callback(|ev: InputData| Msg::UpdatePhone(ev.value));
        let oninput_address = self.link.callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_site_url = self.link.callback(|ev: InputData| Msg::UpdateSiteUrl(ev.value));
        // let oninput_time_zone = self.link.callback(|ev: InputData| Msg::UpdateTimeZone(ev.value));
        let onchange_region_id =
            self.link.callback(|ev: ChangeData| {
                Msg::UpdateRegionId(match ev {
                    ChangeData::Select(el) => el.value(),
                    _ => "1".to_string(),
                })
            });
        let onchange_company_type_id =
            self.link.callback(|ev: ChangeData| {
                Msg::UpdateCompanyTypeId(match ev {
                    ChangeData::Select(el) => el.value(),
                    _ => "1".to_string(),
                })
            });

            let current_type_id = self.request_company.company_type_id.unwrap_or_default();
            let current_region_id = self.request_company.region_id.unwrap_or_default();

            html!{
                <div class="settings-company-fields">
                    <div class="box mb-5">
                        <h5 class="title is-6 has-text-grey mb-4">
                            <span class="icon"><i class="fas fa-building"></i></span>
                        </h5>
                        <div class="field mb-4">
                            {InputConfig::company_input("orgname", LocaleKey::Orgname, self.request_company.orgname.as_ref(), oninput_orgname, self.loading)}
                        </div>
                        <div class="columns is-desktop mb-0">
                            <div class="column">
                                {InputConfig::company_input("shortname", LocaleKey::Shortname, self.request_company.shortname.as_ref(), oninput_shortname, self.loading)}
                            </div>
                            <div class="column">
                                {InputConfig::company_input("inn", LocaleKey::RegNumber, self.request_company.inn.as_ref(), oninput_inn, self.loading)}
                            </div>
                        </div>
                        <div class="columns is-desktop mb-0">
                            <div class="column">
                                <div class="field">
                                    <label class="label">{LocaleKey::CompanyType.get_value()}</label>
                                    <div class="control">
                                        <div class="select is-fullwidth">
                                            <select
                                                id="company_type"
                                                disabled={self.loading}
                                                onchange={onchange_company_type_id}
                                                value={current_type_id.to_string()}
                                            >
                                                {for self.company_types.iter().map(|x| html!{
                                                    <option
                                                        value={x.company_type_id.to_string()}
                                                        selected={x.company_type_id as i64 == current_type_id}
                                                    >
                                                        {&x.name}
                                                    </option>
                                                })}
                                            </select>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="column">
                                {InputConfig::company_input("site_url", LocaleKey::Site, self.request_company.site_url.as_ref(), oninput_site_url, self.loading)}
                            </div>
                        </div>
                    </div>
                    <div class="box mb-5">
                        <h5 class="title is-6 has-text-grey mb-4">
                            <span class="icon"><i class="fas fa-address-book"></i></span>
                        </h5>
                        <div class="columns is-desktop mb-0">
                            <div class="column">
                                {InputConfig::company_input("email", LocaleKey::Email, self.request_company.email.as_ref(), oninput_email, self.loading)}
                            </div>
                            <div class="column">
                                {InputConfig::company_input("tel", LocaleKey::Phone, self.request_company.phone.as_ref(), oninput_phone, self.loading)}
                            </div>
                        </div>
                        <div class="columns is-desktop mb-0">
                            <div class="column is-4-desktop">
                                <div class="field">
                                    <label class="label">{LocaleKey::Region.get_value()}</label>
                                    <div class="control">
                                        <div class="select is-fullwidth">
                                            <select
                                                id="region"
                                                disabled={self.loading}
                                                onchange={onchange_region_id}
                                                value={current_region_id.to_string()}
                                            >
                                                {for self.regions.iter().map(|x| html!{
                                                    <option
                                                        value={x.region_id.to_string()}
                                                        selected={x.region_id as i64 == current_region_id}
                                                    >
                                                        {&x.region}
                                                    </option>
                                                })}
                                            </select>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="column is-8-desktop">
                                {InputConfig::company_input("address", LocaleKey::Address, self.request_company.address.as_ref(), oninput_address, self.loading)}
                            </div>
                        </div>
                    </div>
                    <div class="box mb-0">
                        <h5 class="title is-6 has-text-grey mb-4">
                            <span class="icon"><i class="fas fa-align-left"></i></span>
                        </h5>
                        {InputConfig::company_input("description", LocaleKey::Description, self.request_company.description.as_ref(), oninput_description, self.loading)}
                    </div>
                </div>
            }
    }

    fn update_favicon_block(&self) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::ReguestCompanyData);

        html!{<>
            <h4 id="updated-favicon-company" class="title is-4">{LocaleKey::CompanyLogo.get_value()}</h4>
            <UpdateFaviconBlock
                company_uuid={self.company_uuid.clone()}
                callback={callback_update_favicon}
            />
        </>}
    }

    fn certificates_block(&self) -> Html {
        match &self.current_data {
            Some(current_data) => html!{
                <CompanyCertificatesCard
                    certificates={current_data.company_certificates.clone()}
                    show_cert_btn={true}
                    manage_btn={true}
                />
            },
            None => html!{
                <div class="notification is-info">
                    <span>{LocaleKey::NoCertificates.get_value()}</span>
                </div>
            },
        }
    }

    fn manage_specs_block(&self) -> Html {
        html!{<>
            <h4 id="updated-company-specs" class="title is-4">{LocaleKey::SphereOfActivity.get_value()}</h4>
            {match &self.current_data {
                Some(current_data) => html!{
                    <SearchSpecsTags
                        company_specs={current_data.company_specs.clone()}
                        company_uuid={current_data.uuid.clone()}
                     />
                },
                None => html!{},
            }}
        </>}
    }

    fn add_certificate_block(&self) -> Html {
        let company_uuid = self
            .current_data
            .as_ref()
            .map(|company| company.uuid.to_string())
            .unwrap_or_default();

        let callback_upload_cert = self.link.callback(|_| Msg::ReguestCompanyData);

        html!{
            <AddCompanyCertificateCard
                company_uuid={company_uuid}
                callback={callback_upload_cert}
            />
        }
    }

    fn represents_block(&self) -> Html {
        html!{
            <CompanyRepresents
                company_uuid={self.company_uuid.clone()}
                list={self.current_data.as_ref().map(|cd| cd.company_represents.clone()).unwrap_or_default()}
                show_manage_btn={true}
                />
        }
    }

    fn manage_access_block(&self) -> Html {
        let onchange_type_access = self.link.callback(|value| Msg::UpdateTypeAccessId(value));
        let onsubmit_update_access = self.link.callback(|_| Msg::RequestChangeAccess);

        html!{<>
            {show_notification(
                LocaleKey::UpdatedAccess.get_value(),
                "is-success",
                self.get_result_access,
            )}
            <h4 id="updated-access" class="title is-4">{LocaleKey::Access.get_value()}</h4>
            <div class="field">
                <label class="label">{LocaleKey::TypeAccess.get_value()}</label>
                <TypeAccessBlock
                    change_cb={onchange_type_access}
                    types={self.types_access.clone()}
                    selected={self.request_access as usize}
                    preset={self.current_data.as_ref().map(|data| data.type_access.type_access_id)}
                />
            </div>
            {ft_save_btn(
                "update-access-btn",
                onsubmit_update_access,
                true,
                false,
            )}
        </>}
    }

    fn remove_company_block(&self) -> Html {
        let onclick_delete_company = self.link.callback(|_| Msg::RequestRemoveCompany);

        html!{<>
            <h4 id="remove-company" class="title is-4">{LocaleKey::DeleteCompanyLabel.get_value()}</h4>
            {show_notification(
                &format!("{}: {}", LocaleKey::CompanyDelete.get_value(), self.get_result_remove_company),
                "is-success",
                self.get_result_remove_company,
            )}
            <div class="content is-medium">
                <p><strong>{LocaleKey::Warning.get_value()}</strong> {LocaleKey::CompanyDeleteWarning.get_value()}</p>
            </div>
            <div class="column is-half right-side">
            {ft_delete_class_btn(
                "button-delete-company",
                onclick_delete_company,
                self.get_confirm == self.company_uuid,
                false,
                classes!("is-fullwidth")
            )}
            </div>
        </>}
    }
}
