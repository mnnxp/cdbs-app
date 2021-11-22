use yew::{
    agent::Bridged, html, Bridge, Component, ComponentLink,
    FocusEvent, Html, InputData, ChangeData, Properties, ShouldRender,
};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::*
};
use chrono::NaiveDateTime;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::gqls::make_query;
use crate::error::{Error, get_error};
use crate::fragments::{
    list_errors::ListErrors,
    company_certificate::CompanyCertificateCard,
    company_add_certificate::AddCertificateCard,
    upload_favicon::UpdateFaviconCard,
    company_represent::CompanyRepresents,
    company_add_represent::AddCompanyRepresentCard,
};
use crate::routes::AppRoute;
use crate::services::is_authenticated;
use crate::types::{
    UUID, SlimUser, CompanyUpdateInfo, CompanyInfo, Region,
    CompanyType, TypeAccessTranslateListInfo, SlimCompany,
    Certificate, CompanyCertificate, CompanyRepresentInfo
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct GetCompanySettingDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct GetCompanyData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct CompanyUpdate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct ChangeCompanyAccess;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct DeleteCompany;

/// Get data current company
impl From<CompanyInfo> for CompanyUpdateInfo {
    fn from(data: CompanyInfo) -> Self {
        let CompanyInfo {
            orgname,
            shortname,
            inn,
            phone,
            email,
            description,
            address,
            site_url,
            time_zone,
            region,
            company_type,
            ..
        } = data;

        Self {
            orgname: Some(orgname),
            shortname: Some(shortname),
            inn: Some(inn),
            phone: Some(phone),
            email: Some(email),
            description: Some(description),
            address: Some(address),
            site_url: Some(site_url),
            time_zone: Some(time_zone),
            region_id: Some(region.region_id as i64),
            company_type_id: Some(company_type.company_type_id as i64),
        }
    }
}

pub enum Menu {
    Company,
    UpdataFavicon,
    Certificates,
    Represent,
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
    types_access: Vec<TypeAccessTranslateListInfo>,
    company_types: Vec<CompanyType>,
    get_result_update: usize,
    get_result_access: bool,
    get_result_remove_company: SlimCompany,
    select_menu: Menu,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub company_uuid: UUID,
}

pub enum Msg {
    SelectMenu(Menu),
    RequestUpdateCompany,
    RequestChangeAccess,
    RequestRemoveCompany,
    ResponseError(Error),
    GetUpdateAccessResult(String),
    GetUpdateCompanyData(String),
    GetUpdateCompanyResult(String),
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
    UpdateList(String),
    GetCurrentData,
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
            get_result_remove_company: SlimCompany::default(),
            select_menu: Menu::Company,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        self.company_uuid = match self.props.company_uuid.is_empty() {
            true => {
                // get company uuid for request
                let route_service: RouteService<()> = RouteService::new();
                // get target company from route
                route_service
                    .get_fragment()
                    .trim_start_matches("#/company/settings/")
                    .to_string()
            },
            false => self.props.company_uuid.clone(),
        };

        let company_uuid = self.company_uuid.clone();

        if first_render && is_authenticated() && !company_uuid.is_empty() {
            spawn_local(async move {
                let res = make_query(
                    GetCompanySettingDataOpt::build_query(get_company_setting_data_opt::Variables {
                        company_uuid
                    })
                ).await.unwrap();
                link.send_message(Msg::GetUpdateCompanyData(res.clone()));
                link.send_message(Msg::UpdateList(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::SelectMenu(value) => {
                self.select_menu = value;
                self.rendered(false);
            },
            Msg::RequestUpdateCompany => {
                let company_uuid = self.company_uuid.clone();
                let request_company = self.request_company.clone();
                spawn_local(async move {
                    let CompanyUpdateInfo {
                        orgname,
                        shortname,
                        inn,
                        phone,
                        email,
                        description,
                        address,
                        site_url,
                        time_zone,
                        region_id,
                        company_type_id,
                    } = request_company;
                    let data_company_update = company_update::IptUpdateCompanyData {
                        orgname,
                        shortname,
                        inn,
                        phone,
                        email,
                        description,
                        address,
                        siteUrl: site_url,
                        timeZone: time_zone,
                        regionId: region_id,
                        companyTypeId: company_type_id,
                    };
                    let res = make_query(CompanyUpdate::build_query(company_update::Variables {
                        company_uuid,
                        data_company_update
                    })).await;
                    link.send_message(Msg::GetUpdateCompanyResult(res.unwrap()));
                })
            },
            Msg::RequestChangeAccess => {
                let company_uuid = self.company_uuid.clone();
                let new_type_access = self.request_access.clone();
                spawn_local(async move {
                    let change_access_company_data = change_company_access::ChangeTypeAccessCompany{
                        companyUuid: company_uuid,
                        newTypeAccessId: new_type_access,
                    };

                    let res = make_query(ChangeCompanyAccess::build_query(
                        change_company_access::Variables {
                            change_access_company_data,
                        }
                    )).await;
                    link.send_message(Msg::GetUpdateAccessResult(res.unwrap()));
                })
            },
            Msg::RequestRemoveCompany => {
                let delete_company_uuid = self.company_uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteCompany::build_query(
                        delete_company::Variables { delete_company_uuid }
                    )).await;
                    link.send_message(Msg::GetRemoveCompanyResult(res.unwrap()));
                })
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
                // self.task = None;
            },
            Msg::GetUpdateAccessResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res.get("changeCompanyAccess").unwrap().clone()).unwrap();
                        debug!("Change company access: {:?}", result);
                        self.get_result_access = result;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::GetUpdateCompanyData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let company_data: CompanyInfo = serde_json::from_value(res.get("company").unwrap().clone()).unwrap();
                        debug!("Company data: {:?}", company_data);
                        self.current_data = Some(company_data.clone());
                        self.request_company = company_data.into();
                        self.rendered(false);
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::UpdateTypeAccessId(type_access_id) => {
                self.request_access = type_access_id.parse::<i64>().unwrap_or_default();
                debug!("Update: {:?}", type_access_id);
            },
            Msg::UpdateOrgname(orgname) => {
                self.request_company.orgname = Some(orgname);
            },
            Msg::UpdateShortname(shortname) => {
                self.request_company.shortname = Some(shortname);
            },
            Msg::UpdateInn(inn) => {
                self.request_company.inn = Some(inn);
            },
            Msg::UpdateEmail(email) => {
                self.request_company.email = Some(email);
            },
            Msg::UpdatePhone(phone) => {
                self.request_company.phone = Some(phone);
            },
            Msg::UpdateDescription(description) => {
                self.request_company.description = Some(description);
            },
            Msg::UpdateAddress(address) => {
                self.request_company.address = Some(address);
            },
            Msg::UpdateSiteUrl(site_url) => {
                self.request_company.site_url = Some(site_url);
            },
            Msg::UpdateTimeZone(time_zone) => {
                self.request_company.time_zone = Some(time_zone);
            },
            Msg::UpdateRegionId(region_id) => {
                self.request_company.region_id = Some(region_id.parse::<i64>().unwrap_or_default());
                debug!("Update: {:?}", region_id);
            },
            Msg::UpdateCompanyTypeId(type_id) => {
                self.request_company.company_type_id = Some(type_id.parse::<i64>().unwrap_or_default());
                debug!("Update: {:?}", type_id);
            },
            Msg::UpdateList(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();
                match res_value.is_null() {
                    false => {
                        // debug!("Result: {:#?}", res_value.clone);
                        self.regions =
                            serde_json::from_value(res_value.get("regions").unwrap().clone()).unwrap();
                        self.company_types =
                            serde_json::from_value(res_value.get("companyTypes").unwrap().clone()).unwrap();
                        self.types_access =
                            serde_json::from_value(res_value.get("typesAccess").unwrap().clone()).unwrap();
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
            },
            Msg::GetRemoveCompanyResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let delete_company: SlimCompany =
                            serde_json::from_value(res.get("deleteCompany").unwrap().clone()).unwrap();
                        debug!("Delete company: {:?}", delete_company);
                        self.get_result_remove_company = delete_company;
                        match &self.props.current_user {
                            Some(user) => self.router_agent.send(ChangeRoute(AppRoute::Profile(
                                user.username.clone()
                            ).into())),
                            None => self.router_agent.send(ChangeRoute(AppRoute::Home.into())),
                        }
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::GetUpdateCompanyResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let updated_rows: usize =
                            serde_json::from_value(res.get("putCompanyUpdate").unwrap().clone()).unwrap();
                        debug!("Updated rows: {:?}", updated_rows);
                        self.get_result_update = updated_rows;
                        link.send_message(Msg::GetCurrentData);
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::GetCurrentData => {
                let company_uuid = Some(self.company_uuid.clone());
                spawn_local(async move {
                    let res = make_query(
                        GetCompanyData::build_query(get_company_data::Variables {
                            company_uuid
                        })
                    ).await.unwrap();
                    link.send_message(Msg::GetUpdateCompanyData(res));
                })
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit_update_company = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestUpdateCompany
        });

        html! {
            <div class="settings-page">
                <ListErrors error=self.error.clone()/>
                <div class="container page">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-one-quarter">
                                { self.view_menu() }
                            </div>
                            // <h1 class="title">{ "Company Settings" }</h1>
                            <div class="column">
                                <div class="card">
                                  <div class="card-content">
                                    {match self.select_menu {
                                        // Show interface for change company data
                                        Menu::Company => html! {<>
                                            <span id="tag-info-updated-date" class="tag is-info is-light">{
                                                match &self.current_data {
                                                    Some(data) => format!("Last updated: {}", data.updated_at),
                                                    None => "Not data".to_string(),
                                                }
                                            }</span>
                                            <span id="tag-info-updated-rows" class="tag is-info is-light">
                                                { format!("Updated rows: {}", self.get_result_update.clone()) }
                                            </span>
                                            <form onsubmit=onsubmit_update_company>
                                                { self.fieldset_company() }
                                                <button
                                                    id="update-settings"
                                                    class="button"
                                                    type="submit"
                                                    disabled=false>
                                                    { "Update Company" }
                                                </button>
                                            </form>
                                        </>},
                                        // Show interface for change favicon company
                                        Menu::UpdataFavicon => html! {<>
                                            <span id="tag-info-updated-favicon-company" class="tag is-info is-light">
                                                // { format!("Updated certificates: {}", self.get_result_certificates.clone()) }
                                                { "Update favicon" }
                                            </span>
                                            { self.fieldset_update_favicon() }
                                        </>},
                                        // Show interface for add and update Certificates
                                        Menu::Certificates => html! {<>
                                            <span id="tag-info-updated-certificates" class="tag is-info is-light">
                                                // { format!("Updated certificates: {}", self.get_result_certificates.clone()) }
                                                { "Certificates" }
                                            </span>
                                            { self.fieldset_certificates() }
                                            <br/>
                                            { self.fieldset_add_certificate() }
                                        </>},
                                        // Show interface for add and update Represents
                                        Menu::Represent => html! {<>
                                            <span id="tag-info-updated-represents" class="tag is-info is-light">
                                                // { format!("Updated certificates: {}", self.get_result_certificates.clone()) }
                                                { "Represents" }
                                            </span>
                                            { self.fieldset_add_represent() }
                                            <br/>
                                            { self.fieldset_represents() }
                                        </>},
                                        // Show interface for manage Access
                                        Menu::Access => html! {<>
                                            <span id="tag-info-updated-represents" class="tag is-info is-light">
                                                // { format!("Updated certificates: {}", self.get_result_certificates.clone()) }
                                                { "Access" }
                                            </span>
                                            <br/>
                                            { self.fieldset_manage_access() }
                                        </>},
                                        // Show interface for remove company
                                        Menu::RemoveCompany => html! {<>
                                            <span id="tag-danger-remove-company" class="tag is-danger is-light">
                                              { "Warning: this removed all data related with company, it cannot be canceled!" }
                                            </span>
                                            <br/>
                                            <span id="tag-info-remove-company" class="tag is-info is-light">
                                              { format!("Company delete: {}", self.get_result_remove_company.shortname) }
                                            </span>
                                            <br/>
                                            { self.fieldset_remove_company() }
                                        </>},
                                    }}
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
    fn view_menu(
        &self
    ) -> Html {
        let onclick_company = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Company
            ));
        let onclick_favicon = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::UpdataFavicon
            ));
        let onclick_certificates = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Certificates
            ));
        let onclick_represents = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Represent
            ));
        let onclick_access = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::Access
            ));
        let onclick_remove_company = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::RemoveCompany
            ));

        let mut active_company = "";
        let mut active_favicon = "";
        let mut active_certificates = "";
        let mut active_represents = "";
        let mut active_access = "";
        let mut active_remove_company = "";

        match self.select_menu {
            Menu::Company => active_company = "is-active",
            Menu::UpdataFavicon => active_favicon = "is-active",
            Menu::Certificates => active_certificates = "is-active",
            Menu::Represent => active_represents = "is-active",
            Menu::Access => active_access = "is-active",
            Menu::RemoveCompany => active_remove_company = "is-active",
        }

        html! {
            <aside class="menu">
                <p class="menu-label">
                    {"Company Settings"}
                </p>
                <ul class="menu-list">
                    <li><a
                      id="company-data"
                      class=active_company
                      onclick=onclick_company>
                        { "Company" }
                    </a></li>
                    <li><a
                      id="company-favicon"
                      class=active_favicon
                      onclick=onclick_favicon>
                        { "Favicon" }
                    </a></li>
                    <li><a
                      id="certificates"
                      class=active_certificates
                      onclick=onclick_certificates>
                        { "Certificates" }
                    </a></li>
                    <li><a
                      id="Represents"
                      class=active_represents
                      onclick=onclick_represents>
                        { "Represents" }
                    </a></li>
                    <li><a
                      id="access"
                      class=active_access
                      onclick=onclick_access>
                        { "Access" }
                    </a></li>
                    <li><a
                      id="remove-company"
                      class=active_remove_company
                      onclick=onclick_remove_company>
                        { "Remove company" }
                    </a></li>
                </ul>
            </aside>
        }
    }

    fn fieldset_company(
        &self
    ) -> Html {
        let oninput_orgname = self
            .link
            .callback(|ev: InputData| Msg::UpdateOrgname(ev.value));
        let oninput_shortname = self
            .link
            .callback(|ev: InputData| Msg::UpdateShortname(ev.value));
        let oninput_inn = self
            .link
            .callback(|ev: InputData| Msg::UpdateInn(ev.value));
        let oninput_email = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_description = self
            .link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let oninput_phone = self
            .link
            .callback(|ev: InputData| Msg::UpdatePhone(ev.value));
        let oninput_address = self
            .link
            .callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_site_url = self
            .link
            .callback(|ev: InputData| Msg::UpdateSiteUrl(ev.value));
        // let oninput_time_zone = self
        //     .link
        //     .callback(|ev: InputData| Msg::UpdateTimeZone(ev.value));
        let onchange_region_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateRegionId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let onchange_company_type_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateCompanyTypeId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        html! {<>
            <fieldset class="columns">
                // first column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Orgname"}</label>
                        <input
                            id="orgname"
                            class="input"
                            type="text"
                            placeholder="orgname"
                            value={self.request_company.orgname
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_orgname />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Shortname"}</label>
                        <input
                            id="shortname"
                            class="input"
                            type="text"
                            placeholder="shortname"
                            value={self.request_company.shortname
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_shortname />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Inn"}</label>
                        <input
                            id="inn"
                            class="input"
                            type="text"
                            placeholder="inn"
                            value={self.request_company.inn
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_inn />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Region"}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="region"
                                  select={self.request_company.region_id.unwrap_or_default().to_string()}
                                  onchange=onchange_region_id
                                  >
                                { for self.regions.iter().map(|x|
                                    match self.current_data.as_ref().unwrap().region.region_id == x.region_id {
                                        true => {
                                            html!{
                                                <option value={x.region_id.to_string()} selected=true>{&x.region}</option>
                                            }
                                        },
                                        false => {
                                            html!{
                                                <option value={x.region_id.to_string()}>{&x.region}</option>
                                            }
                                        },
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </fieldset>

                // second column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Email"}</label>
                        <input
                            id="email"
                            class="input"
                            type="email"
                            placeholder="email"
                            value={self.request_company.email
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_email />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Phone"}</label>
                        <input
                            id="phone"
                            class="input"
                            type="text"
                            placeholder="phone"
                            value={self.request_company.phone
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_phone />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Address"}</label>
                        <input
                            id="address"
                            class="input"
                            type="text"
                            placeholder="address"
                            value={self.request_company.address
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_address />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Site"}</label>
                        <input
                            id="site_url"
                            class="input"
                            type="text"
                            placeholder="site_url"
                            value={self.request_company.site_url
                                .as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_default()}
                            oninput=oninput_site_url />
                    </fieldset>
                </fieldset>
            </fieldset>
            // separate fields
            <fieldset class="field">
                <label class="label">{"Company type"}</label>
                <div class="control">
                    <div class="select">
                      <select
                          id="company_type"
                          select={self.request_company.company_type_id.unwrap_or_default().to_string()}
                          onchange=onchange_company_type_id
                          >
                        { for self.company_types.iter().map(|x|
                            match self.current_data.as_ref().unwrap().company_type.company_type_id == x.company_type_id {
                                true => {
                                    html!{
                                        <option value={x.company_type_id.to_string()} selected=true>{&x.name}</option>
                                    }
                                },
                                false => {
                                    html!{
                                        <option value={x.company_type_id.to_string()}>{&x.name}</option>
                                    }
                                },
                            }
                        )}
                      </select>
                    </div>
                </div>
            </fieldset>
            <fieldset class="field">
                <label class="label">{"Description"}</label>
                <textarea
                    id="description"
                    class="input"
                    type="description"
                    placeholder="description"
                    value={self.request_company.description
                        .as_ref()
                        .map(|x| x.to_string())
                        .unwrap_or_default()}
                    oninput=oninput_description />
            </fieldset>
        </>}
    }

    fn fieldset_update_favicon(
        &self
    ) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::GetCurrentData);

        html! {
            <UpdateFaviconCard
                company_uuid = self.company_uuid.clone()
                callback=callback_update_favicon
                />
        }
    }

    fn fieldset_certificates(
        &self
    ) -> Html {
        let mut certificates: &[CompanyCertificate] = &Vec::new();
        if let Some(ref data) = self.current_data {
            certificates = data.company_certificates.as_ref();
        };

        match certificates.is_empty() {
            true => html!{
                <div>
                    <span id="tag-info-no-certificates" class="tag is-info is-light">
                        // { format!("Updated certificates: {}", self.get_result_certificates.clone()) }
                        { "Company don't have Certificates" }
                    </span>
                </div>
            },
            false => {
                html!{
                    // <p class="card-footer-item">
                    <>{
                        for certificates.iter().map(|cert| {
                            let view_cert: Certificate = cert.into();
                            html! {
                                <CompanyCertificateCard
                                    company_uuid = self.company_uuid.clone()
                                    certificate = view_cert
                                    show_cert_btn = true
                                    download_btn = false
                                    change_btn = true
                                    />
                            }
                        })
                    }</>
                    // </p>
                }
            },
        }
    }

    fn fieldset_add_certificate(
        &self
    ) -> Html {
        let company_uuid = self.current_data
            .as_ref()
            .map(|company| company.uuid.to_string())
            .unwrap_or_default();

        let callback_upload_cert = self.link.callback(|_| Msg::GetCurrentData);

        html! {
            <AddCertificateCard
                company_uuid = company_uuid
                callback=callback_upload_cert
                />
        }
    }

    fn fieldset_represents(
        &self
    ) -> Html {
        let mut represents: &[CompanyRepresentInfo] = &Vec::new();
        if let Some(ref data) = self.current_data {
            represents = data.company_represents.as_ref();
        };
        // debug!("first: {:?}", represents);

        match represents.is_empty() {
            true => html!{
                <div>
                    <span id="tag-info-no-represents" class="tag is-info is-light">
                        // { format!("Updated represents: {}", self.get_result_represents.clone()) }
                        { "Company don't have Represents" }
                    </span>
                </div>
            },
            false => {
                // debug!("false: {:?}", represents);
                html! {
                    <CompanyRepresents
                        show_manage_btn = true
                        list = represents.to_vec()
                        />
                }
            },
        }
    }

    fn fieldset_add_represent(
        &self
    ) -> Html {
        html! {
            <AddCompanyRepresentCard
                company_uuid = self.company_uuid.clone()
                />
        }
    }

    fn fieldset_manage_access(
        &self
    ) -> Html {
        let onsubmit_update_access = self.link.callback(|ev: FocusEvent| {
                ev.prevent_default();
                Msg::RequestChangeAccess
            });

        html! {
            <form onsubmit=onsubmit_update_access>
                { self.fieldset_access() }
                <button
                    id="update-access"
                    class="button"
                    type="submit"
                    disabled=false>
                    { "Update access" }
                </button>
            </form>
        }
    }

    fn fieldset_access(
        &self
    ) -> Html {
        let onchange_type_access_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        html! {
            <fieldset class="columns">
                // first column
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"Type Access"}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="types-access"
                                  select={self.request_access.to_string()}
                                  onchange=onchange_type_access_id
                                  >
                                { for self.types_access.iter().map(|x|
                                    match self.current_data.as_ref().unwrap().type_access.type_access_id == x.type_access_id {
                                        true => {
                                            html!{
                                                <option value={x.type_access_id.to_string()} selected=true>{&x.name}</option>
                                            }
                                        },
                                        false => {
                                            html!{
                                                <option value={x.type_access_id.to_string()}>{&x.name}</option>
                                            }
                                        },
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </fieldset>
            </fieldset>
        }
    }

    fn fieldset_remove_company(
        &self
    ) -> Html {
        let onclick_delete_company = self.link.callback(|_| Msg::RequestRemoveCompany);

        html! {
            <button
                id="button-delete-company"
                class="button"
                onclick=onclick_delete_company>
                { "Delete all company data" }
            </button>
        }
    }
}
