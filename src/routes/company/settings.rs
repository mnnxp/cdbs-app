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

use crate::gqls::make_query;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    buttons::{ft_save_btn, ft_submit_btn, ft_delete_btn},
    notification::show_notification,
    company::{
        CompanyCertificatesCard, AddCompanyCertificateCard,
        AddCompanyRepresentCard, CompanyRepresents, SearchSpecsTags
    },
    list_errors::ListErrors,
    side_menu::{MenuItem, SideMenu},
    upload_favicon::UpdateFaviconBlock,
};
use crate::services::content_adapter::DateDisplay;
use crate::services::{get_logged_user, get_value_field, resp_parsing, get_value_response, get_from_value};
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
    Company,
    UpdateFavicon,
    Certificates,
    Represent,
    Spec,
    Access,
    RemoveCompany,
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
    UpdateTypeAccessId(String),
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
                match resp_parsing(res, "changeCompanyAccess") {
                    Ok(result) => self.get_result_access = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("Change company access: {:?}", self.get_result_access);
            },
            Msg::GetCompanyDataResult(res) => {
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
                match resp_parsing(res, "putCompanyUpdate") {
                    Ok(result) => {
                        self.get_result_update = result;
                        link.send_message(Msg::ReguestCompanyData);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateTypeAccessId(type_access_id) =>
                self.request_access = type_access_id.parse::<i64>().unwrap_or_default(),
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
                <div class="container page">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-flex">
                                {self.view_menu()}
                                <div class="card is-flex-grow-1" >
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
    fn fileset_generator(
        &self,
        id: &str,
        label: &str,
        // placeholder: &str,
        value: String,
        oninput: Callback<InputData>,
    ) -> Html {
        let placeholder = label;
        let mut class = "input";
        let (input_tag, input_type) = match id {
            "email" => ("input", "email"),
            "description" => {
                class = "textarea";
                ("textarea", "text")
            },
            "password" => ("input", "password"),
            _ => ("input", "text"),
        };

        html!{
            <fieldset class="field">
                <label class="label">{label.to_string()}</label>
                <@{input_tag}
                    id={id.to_string()}
                    class={class}
                    type={input_type}
                    placeholder={placeholder.to_string()}
                    value={value}
                    oninput={oninput} ></@>
            </fieldset>
        }
    }

    fn cb_generator(&self, cb: Menu) -> Callback<MouseEvent> {
        self.link.callback(move |_| Msg::SelectMenu(cb.clone()))
    }

    fn view_menu(&self) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            // return company page MenuItem
            MenuItem {
                title: get_value_field(&265).to_string(), // Open company
                action: self.link.callback(|_| Msg::OpenCompany),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-angle-double-left")],
                is_active: false,
                ..Default::default()
            },
            // Company MenuItem
            MenuItem {
                title: get_value_field(&109).to_string(), // Company
                action: self.cb_generator(Menu::Company),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-building")],
                is_active: self.select_menu == Menu::Company,
                ..Default::default()
            },
            // Favicon MenuItem
            MenuItem {
                title: get_value_field(&78).to_string(), // Favicon
                action: self.cb_generator(Menu::UpdateFavicon),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-image")],
                is_active: self.select_menu == Menu::UpdateFavicon,
                ..Default::default()
            },
            // Certificates MenuItem
            MenuItem {
                title: get_value_field(&64).to_string(), // Certificates
                action: self.cb_generator(Menu::Certificates),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-certificate")],
                is_active: self.select_menu == Menu::Certificates,
                ..Default::default()
            },
            // Represent MenuItem
            MenuItem {
                title: get_value_field(&266).to_string(), // Representations
                action: self.cb_generator(Menu::Represent),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-industry")],
                is_active: self.select_menu == Menu::Represent,
                ..Default::default()
            },
            // Spec MenuItem
            MenuItem {
                title: get_value_field(&283).to_string(), // Sphere of activity
                action: self.cb_generator(Menu::Spec),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-paperclip")],
                is_active: self.select_menu == Menu::Spec,
                ..Default::default()
            },
            // Access MenuItem
            MenuItem {
                title: get_value_field(&65).to_string(), // Access
                action: self.cb_generator(Menu::Access),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-low-vision")],
                is_active: self.select_menu == Menu::Access,
                ..Default::default()
            },
            // RemoveCompany MenuItem
            MenuItem {
                title: get_value_field(&267).to_string(), // Remove Company
                action: self.cb_generator(Menu::RemoveCompany),
                item_class: classes!("has-background-danger-light"),
                icon_classes: vec![classes!("fas", "fa-trash")],
                is_active: self.select_menu == Menu::RemoveCompany,
                ..Default::default()
            },
        ];

        html! {
          <div style="margin-right: 18px;z-index: 1;" >
              <SideMenu menu_arr={menu_arr} />
          </div>
        }
    }

    fn select_content(&self) -> Html {
        match self.select_menu {
            // Show interface for change company data
            Menu::Company => self.manage_master_data(),
            // Show interface for change favicon company
            Menu::UpdateFavicon => self.update_favicon_block(),
            // Show interface for add and update Certificates
            Menu::Certificates => html!{<>
                <h4 id="updated-certificates" class="title is-4">{get_value_field(&64)}</h4> // Certificates
                {self.add_certificate_block()}
                <br/>
                {self.certificates_block()}
            </>},
            // Show interface for add and update Represents
            Menu::Represent => html!{<>
                <h4 id="updated-represents" class="title is-4">{get_value_field(&266)}</h4> // Represents
                <AddCompanyRepresentCard company_uuid={self.company_uuid.clone()} />
                <br/>
                {self.represents_block()}
            </>},
            // Show interface for add and update company catalogs
            Menu::Spec => self.manage_specs_block(),
            // Show interface for manage Access
            Menu::Access => self.manage_access_block(),
            // Show interface for remove company
            Menu::RemoveCompany => self.remove_company_block(),
        }
    }

    fn manage_master_data(&self) -> Html {
        let onsubmit_update_company = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestUpdateCompany
        });

        html!{<>
            <h4 id="updated-company" class="title is-4">{get_value_field(&109)}</h4> // Company
            <div class="columns">
                {show_notification(
                    &format!("{} {}", get_value_field(&72), self.get_result_update),
                    "is-success",
                    self.get_result_update > 0,
                )}
                <div class="column">
                    <span class={classes!("overflow-title", "has-text-weight-bold")}>{get_value_field(&73)}</span>
                    {match &self.current_data {
                        Some(data) => html!{
                            <span class="overflow-title">
                                {data.updated_at.date_to_display()}
                            </span>
                        },
                        None => html!{<span>{get_value_field(&75)}</span>},
                    }}
                </div>
            </div>
            <form onsubmit={onsubmit_update_company} >
                {self.fieldset_company()}
                {ft_submit_btn("update-settings")}
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

        html!{<>
            // first column
            {self.fileset_generator(
                "orgname", get_value_field(&170), // Orgname
                self.request_company.orgname.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                oninput_orgname.clone()
            )}

            // second column
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "shortname", get_value_field(&171), // Shortname
                        self.request_company.shortname.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_shortname.clone()
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "inn", get_value_field(&81), // Inn
                        self.request_company.inn.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_inn.clone()
                    )}
                </div>
            </div>

            // third column
            <div class="columns">
                <div class="column">
                    <fieldset class="field">
                        <label class="label">{get_value_field(&51)}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="company_type"
                                  select={self.request_company.company_type_id.unwrap_or_default().to_string()}
                                  onchange={onchange_company_type_id}
                                  >
                                {for self.company_types.iter().map(|x|
                                    html!{
                                        <option value={x.company_type_id.to_string()}
                                              selected={x.company_type_id as i64 == self.request_company.company_type_id.unwrap_or_default()} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "site_url", get_value_field(&66), // Site
                        self.request_company.site_url.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_site_url.clone()
                    )}
                </div>
            </div>

            // fourth column
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "email", get_value_field(&22), // Email
                        self.request_company.email.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_email.clone()
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "phone", get_value_field(&56), // Phone
                        self.request_company.phone.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_phone.clone()
                    )}
                </div>
            </div>

            // fifth column
            <div class="columns">
                <div class="column">
                    <fieldset class="field">
                        <label class="label">{get_value_field(&27)}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="region"
                                  select={self.request_company.region_id.unwrap_or_default().to_string()}
                                  onchange={onchange_region_id}
                                  >
                                {for self.regions.iter().map(|x|
                                    html!{
                                        <option value={x.region_id.to_string()}
                                              selected={x.region_id as i64 == self.request_company.region_id.unwrap_or_default()} >
                                            {&x.region}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "address", get_value_field(&57), // Address
                        self.request_company.address.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_address.clone()
                    )}
                </div>
            </div>

            // sixth column
            {self.fileset_generator(
                "description", get_value_field(&61), // Description
                self.request_company.description.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                oninput_description.clone()
            )}
        </>}
    }

    fn update_favicon_block(&self) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::ReguestCompanyData);

        html!{<>
            <h4 id="updated-favicon-company" class="title is-4">{get_value_field(&78)}</h4> // Favicon
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
                    download_btn={false}
                    manage_btn={true}
                />
            },
            None => html!{
                <div class="notification is-info">
                    <span>{get_value_field(&74)}</span>
                </div>
            },
        }
    }

    fn manage_specs_block(&self) -> Html {
        html!{<>
            <h4 id="updated-company-specs" class="title is-4">{get_value_field(&283)}</h4> // Sphere of activity
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
        match self.current_data {
            Some(ref data) => html!{
                <CompanyRepresents
                    show_manage_btn={true}
                    list={data.company_represents.clone()}
                />
            },
            None => html!{
                <div class="notification is-info">
                    <span>{get_value_field(&270)}</span>
                </div>
            },
        }
    }

    fn manage_access_block(&self) -> Html {
        let onchange_type_access_id = self.link.callback(|ev: ChangeData| {
            Msg::UpdateTypeAccessId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            })
        });
        let onsubmit_update_access = self.link.callback(|_| Msg::RequestChangeAccess);

        html!{<>
            {show_notification(
                get_value_field(&68), // Updated access
                "is-success",
                self.get_result_access,
            )}
            <h4 id="updated-access" class="title is-4">{get_value_field(&65)}</h4> // Access
            <div class="field">
                <label class="label">{get_value_field(&58)}</label>
                <div class="control">
                    <div class="select is-fullwidth">
                    <select
                        id="types-access"
                        select={self.request_access.to_string()}
                        onchange={onchange_type_access_id}
                        >
                        {for self.types_access.iter().map(|x|
                            html!{
                                <option value={x.type_access_id.to_string()}
                                    selected={x.type_access_id as i64 == self.request_access} >
                                    {&x.name}
                                </option>
                            }
                        )}
                    </select>
                    </div>
                </div>
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
            <h4 id="remove-company" class="title is-4">{get_value_field(&268)}</h4>
            {show_notification(
                &format!("{}: {}", get_value_field(&274), self.get_result_remove_company),
                "is-success",
                self.get_result_remove_company,
            )}
            <div class="content is-medium">
                <p><strong>{get_value_field(&272)}</strong> {get_value_field(&273)}</p>
            </div>
            {ft_delete_btn(
                "button-delete-company",
                onclick_delete_company,
                self.get_confirm == self.company_uuid,
                false
            )}
        </>}
    }
}
