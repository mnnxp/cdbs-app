use chrono::NaiveDateTime;
use graphql_client::GraphQLQuery;
use log::debug;
use serde_json::Value;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::services::timeout::{TimeoutService, TimeoutTask};
use yew::{
    agent::Bridged, html, classes, Bridge, ChangeData, Component, ComponentLink, FocusEvent, Html,
    NodeRef, Callback, InputData, Properties, ShouldRender
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*, service::RouteService};

use crate::error::{get_error, Error};
use crate::fragments::{
    company::{CompanyCertificatesCard, AddCompanyCertificateCard, AddCompanyRepresentCard, CompanyRepresents},
    list_errors::ListErrors,
    spec::{SpecsTags, SearchSpecsTags},
    upload_favicon::UpdateFaviconBlock,
};
use crate::gqls::make_query;
use crate::routes::AppRoute;
use crate::services::is_authenticated;
use crate::types::{
    UUID, SlimUser, CompanyUpdateInfo, CompanyInfo, Region,
    CompanyType, TypeAccessInfo, SpecPathInfo, Spec
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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/specs.graphql",
    response_derives = "Debug"
)]
struct SearchSpecs;

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

pub enum Menu {
    Company,
    UpdataFavicon,
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
    select_menu: Menu,
    edit_specs: bool,
    ipt_timer: Option<TimeoutTask>,
    ipt_ref: NodeRef,
    specs_search_loading: bool,
    search_specs: Vec<SpecPathInfo>,
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
    EditSpecs(bool),
    SetIptTimer(String),
    GetCurrentData,
    GetSearchRes(String),
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
            select_menu: Menu::Company,
            edit_specs: false,
            ipt_timer: None,
            ipt_ref: NodeRef::default(),
            specs_search_loading: false,
            search_specs: Vec::new(),
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
            }
            false => self.props.company_uuid.clone(),
        };

        let company_uuid = self.company_uuid.clone();

        if first_render && is_authenticated() && !company_uuid.is_empty() {
            spawn_local(async move {
                let res = make_query(GetCompanySettingDataOpt::build_query(get_company_setting_data_opt::Variables{
                    company_uuid
                })).await.unwrap();
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
                        change_company_access::Variables{ change_type_access_company }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateAccessResult(res));
                })
            },
            Msg::RequestRemoveCompany => {
                let delete_company_uuid = self.company_uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteCompany::build_query(
                        delete_company::Variables { delete_company_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetRemoveCompanyResult(res));
                })
            },
            Msg::GetUpdateAccessResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(
                            res.get("changeCompanyAccess").unwrap().clone()
                        ).unwrap();
                        debug!("Change company access: {:?}", result);
                        self.get_result_access = result;
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUpdateCompanyData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let company_data: CompanyInfo =
                            serde_json::from_value(res.get("company").unwrap().clone()).unwrap();
                        debug!("Company data: {:?}", company_data);
                        self.current_data = Some(company_data.clone());
                        self.request_company = company_data.into();
                        self.rendered(false);
                    },
                    true => self.error = Some(get_error(&data)),
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
            Msg::UpdateList(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();
                match res_value.is_null() {
                    false => {
                        // debug!("Result: {:#?}", res_value.clone);
                        self.regions = serde_json::from_value(
                            res_value.get("regions").unwrap().clone()
                        ).unwrap();
                        self.company_types = serde_json::from_value(
                            res_value.get("companyTypes").unwrap().clone()
                        ).unwrap();
                        self.types_access = serde_json::from_value(
                            res_value.get("typesAccess").unwrap().clone()
                        ).unwrap();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetRemoveCompanyResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let delete_company: bool = serde_json::from_value(
                            res.get("deleteCompany").unwrap().clone()
                        ).unwrap();
                        debug!("Delete company: {:?}", delete_company);
                        self.get_result_remove_company = delete_company;
                        match &self.props.current_user {
                            Some(user) => self
                                .router_agent
                                .send(ChangeRoute(AppRoute::Profile(user.username.clone()).into())),
                            None => self.router_agent.send(ChangeRoute(AppRoute::Home.into())),
                        }
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUpdateCompanyResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        self.get_result_update = serde_json::from_value(
                            res.get("putCompanyUpdate").unwrap().clone()
                        ).unwrap();
                        // debug!("Updated rows: {:?}", self.get_result_update);
                        link.send_message(Msg::GetCurrentData);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetCurrentData => {
                let company_uuid = self.company_uuid.clone();
                spawn_local(async move {
                    let res = make_query(GetCompanyData::build_query(
                        get_company_data::Variables { company_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateCompanyData(res));
                })
            },
            Msg::EditSpecs(mode) => self.edit_specs = mode,
            Msg::SetIptTimer(val) => {
                debug!("ipt_val: {:?}", val.clone());
                if val.is_empty() {
                    self.ipt_timer = None;
                    self.search_specs = Vec::new();
                    return true;
                }
                self.specs_search_loading = true;
                let cb_link = link.clone();
                self.ipt_timer = Some(TimeoutService::spawn(
                    Duration::from_millis(800),
                    cb_link.callback(move |_| {
                        let ipt_val = val.clone();
                        let res_link = link.clone();
                        spawn_local(async move {
                            let ipt_search_spec_arg = search_specs::IptSearchSpecArg {
                                text: ipt_val.clone(),
                                splitChar: None,
                                depthLevel: None,
                                limit: None,
                                offset: None,
                            };
                            let res = make_query(SearchSpecs::build_query(search_specs::Variables{
                                ipt_search_spec_arg
                            })).await.unwrap();
                            res_link.send_message(Msg::GetSearchRes(res));
                        });
                        debug!("time up: {:?}", val.clone());
                        Msg::Ignore
                    }),
                ));

                // let server = self.ipt_timer.unwrap();
            },
            Msg::GetSearchRes(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();
                let search_specs: Vec<SpecPathInfo> =
                    serde_json::from_value(res.get("searchSpecs").unwrap().clone()).unwrap();
                // debug!(
                //     "specs res:{:?} {:?}",
                //     search_specs.iter().map(|x| Spec::from(x.clone())).collect::<Vec<Spec>>(),
                //     Spec::from(search_specs[0].clone())
                // );
                self.specs_search_loading = false;
                self.search_specs = search_specs;
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
                <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error) />
                <div class="container page">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-flex">
                                { self.view_menu() }
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
        placeholder: &str,
        value: String,
        oninput: Callback<InputData>,
    ) -> Html {
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
                    oninput=oninput ></@>
            </fieldset>
        }
    }

    fn view_menu(&self) -> Html {
        let onclick_company = self.link.callback(|_| Msg::SelectMenu(Menu::Company));
        let onclick_favicon = self.link.callback(|_| Msg::SelectMenu(Menu::UpdataFavicon));
        let onclick_certificates = self.link.callback(|_| Msg::SelectMenu(Menu::Certificates));
        let onclick_represents = self.link.callback(|_| Msg::SelectMenu(Menu::Represent));
        let onclick_specs = self.link.callback(|_| Msg::SelectMenu(Menu::Spec));
        let onclick_access = self.link.callback(|_| Msg::SelectMenu(Menu::Access));
        let onclick_remove_company = self.link.callback(|_| Msg::SelectMenu(Menu::RemoveCompany));

        let mut active_company = "";
        let mut active_favicon = "";
        let mut active_certificates = "";
        let mut active_represents = "";
        let mut active_specs = "";
        let mut active_access = "";
        let mut active_remove_company = "";

        match self.select_menu {
            Menu::Company => active_company = "is-active",
            Menu::UpdataFavicon => active_favicon = "is-active",
            Menu::Certificates => active_certificates = "is-active",
            Menu::Represent => active_represents = "is-active",
            Menu::Spec => active_specs = "is-active",
            Menu::Access => active_access = "is-active",
            Menu::RemoveCompany => active_remove_company = "is-active",
        }

        html!{
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
                      id="represents"
                      class=active_represents
                      onclick=onclick_represents>
                        { "Represents" }
                    </a></li>
                    <li><a
                      id="specs"
                      class=active_specs
                      onclick=onclick_specs>
                        { "Specs" }
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

    fn select_content(&self) -> Html {
        let onsubmit_update_company = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestUpdateCompany
        });

        match self.select_menu {
            // Show interface for change company data
            Menu::Company => html!{<>
                <h4 id="updated-company" class="title is-4">{"Company"}</h4>
                <div class="media">
                    <div class="media-left">
                        <span id="updated-date" class="tag is-info is-light">{
                            match &self.current_data {
                                Some(data) => format!("Last updated: {}", data.updated_at.to_string()),
                                None => "Not data".to_string(),
                            }
                        }</span>
                    </div>
                    <div class="media-content">
                    </div>
                    <div class="media-right">
                        <span id="updated-rows" class="tag is-info is-light">
                            { format!("Updated rows: {}", self.get_result_update.clone()) }
                        </span>
                    </div>
                </div>
                <form onsubmit=onsubmit_update_company >
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
            Menu::UpdataFavicon => html!{<>
                <h4 id="updated-favicon-company" class="title is-4">{"Favicon"}</h4>
                { self.fieldset_update_favicon() }
            </>},
            // Show interface for add and update Certificates
            Menu::Certificates => html!{<>
                <h4 id="updated-certificates" class="title is-4">{"Certificates"}</h4>
                { self.fieldset_add_certificate() }
                <br/>
                { self.fieldset_certificates() }
            </>},
            // Show interface for add and update Represents
            Menu::Represent => html!{<>
                <h4 id="updated-represents" class="title is-4">{"Represents"}</h4>
                <AddCompanyRepresentCard company_uuid = self.company_uuid.clone() />
                <br/>
                { self.fieldset_represents() }
            </>},
            // Show interface for add and update company Specs
            Menu::Spec => html!{<>
                <h4 id="updated-specs" class="title is-4">{"Specs"}</h4>
                <div class="is-flex is-justify-content-space-between" >
                {html !{
                    match self.edit_specs {
                        true => html!{
                            <div>
                              <button class="button is-success" onclick=self.link.callback(|_| Msg::EditSpecs(false))> {"back"} </button>
                              // <button class="button is-warning" onclick=self.link.callback(|_| Msg::EditSpecs(false))> {"cancel"} </button>
                            </div>
                        },
                        false => html!{
                            <div>
                                <button class="button is-info" onclick=self.link.callback(|_| Msg::EditSpecs(true)) > {"add/edit"} </button>
                            </div>
                        },
                    }
                }}
                </div>
                { self.fieldset_add_specs(self.edit_specs) }
                <br/>
                { self.fieldset_specs() }
            </>},
            // Show interface for manage Access
            Menu::Access => html!{<>
                <h4 id="updated-represents" class="title is-4">{"Access"}</h4>
                { self.fieldset_manage_access() }
            </>},
            // Show interface for remove company
            Menu::RemoveCompany => html!{<>
                <h4 id="remove-company" class="title is-4">{"Delete company"}</h4>
                {self.fieldset_remove_company()}
            </>},
        }
    }

    fn fieldset_company(&self) -> Html {
        let oninput_orgname =
            self.link.callback(|ev: InputData| Msg::UpdateOrgname(ev.value));
        let oninput_shortname =
            self.link.callback(|ev: InputData| Msg::UpdateShortname(ev.value));
        let oninput_inn =
            self.link.callback(|ev: InputData| Msg::UpdateInn(ev.value));
        let oninput_email =
            self.link.callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_description =
            self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let oninput_phone =
            self.link.callback(|ev: InputData| Msg::UpdatePhone(ev.value));
        let oninput_address =
            self.link.callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_site_url =
            self.link.callback(|ev: InputData| Msg::UpdateSiteUrl(ev.value));
        // let oninput_time_zone =
        //     self.link.callback(|ev: InputData| Msg::UpdateTimeZone(ev.value));

        let onchange_region_id = self.link.callback(|ev: ChangeData| {
            Msg::UpdateRegionId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            })
        });
        let onchange_company_type_id = self.link.callback(|ev: ChangeData| {
            Msg::UpdateCompanyTypeId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            })
        });

        html!{<>
            // first column
            {self.fileset_generator(
                "orgname", "Orgname", "Orgname",
                self.request_company.orgname.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                oninput_orgname.clone()
            )}

            // second column
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "shortname", "Shortname", "Shortname",
                        self.request_company.shortname.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_shortname.clone()
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "inn", "Inn", "Inn",
                        self.request_company.inn.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_inn.clone()
                    )}
                </div>
            </div>

            // third column
            <div class="columns">
                <div class="column">
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
                                    html!{<option value={x.company_type_id.to_string()}>{&x.name}</option>}
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "site_url", "Site", "Site",
                        self.request_company.site_url.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_site_url.clone()
                    )}
                </div>
            </div>

            // fourth column
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "email", "Email", "Email",
                        self.request_company.email.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_email.clone()
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "phone", "Phone", "Phone",
                        self.request_company.phone.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_phone.clone()
                    )}
                </div>
            </div>

            // fifth column
            <div class="columns">
                <div class="column">
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
                                    html!{<option value={x.region_id.to_string()}>{&x.region}</option>}
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "address", "Address", "Address",
                        self.request_company.address.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_address.clone()
                    )}
                </div>
            </div>

            // sixth column
            {self.fileset_generator(
                "description", "Description", "Description",
                self.request_company.description.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                oninput_description.clone()
            )}
        </>}
    }

    fn fieldset_update_favicon(&self) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::GetCurrentData);

        html!{
            <UpdateFaviconBlock
                company_uuid = self.company_uuid.clone()
                callback=callback_update_favicon
            />
        }
    }

    fn fieldset_certificates(&self) -> Html {
        match &self.current_data {
            Some(current_data) => html!{
                <CompanyCertificatesCard
                    certificates = current_data.company_certificates.clone()
                    show_cert_btn = true
                    download_btn = false
                    manage_btn = true
                />
            },
            None => html!{
                <div class="notification is-info">
                    <span>{"Not fount certificates."}</span>
                </div>
            },
        }
    }

    fn fieldset_add_certificate(&self) -> Html {
        let company_uuid = self
            .current_data
            .as_ref()
            .map(|company| company.uuid.to_string())
            .unwrap_or_default();

        let callback_upload_cert = self.link.callback(|_| Msg::GetCurrentData);

        html!{
            <AddCompanyCertificateCard
                company_uuid = company_uuid
                callback=callback_upload_cert
            />
        }
    }

    fn fieldset_represents(&self) -> Html {
        match self.current_data {
            Some(ref data) => html!{
                <CompanyRepresents
                    show_manage_btn = true
                    list = data.company_represents.clone()
                />
            },
            None => html!{
                <div class="notification is-info">
                    <span>{"Company don't have representations."}</span>
                </div>
            },
        }
    }

    fn fieldset_manage_access(&self) -> Html {
        let onsubmit_update_access = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestChangeAccess
        });

        html!{
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

    fn fieldset_access(&self) -> Html {
        let onchange_type_access_id = self.link.callback(|ev: ChangeData| {
            Msg::UpdateTypeAccessId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            })
        });

        html!{
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
                                    html!{<option value={x.type_access_id.to_string()}>{&x.name}</option>}
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </fieldset>
            </fieldset>
        }
    }

    fn fieldset_remove_company(&self) -> Html {
        let onclick_delete_company = self.link.callback(|_| Msg::RequestRemoveCompany);

        html!{<>
            <span id="remove-company" class="tag is-info is-light">
              {format!("Company delete: {}", self.get_result_remove_company)}
            </span>
            <br/>
            <div class="notification is-danger">
                <span><strong>{"Warning: "}</strong>{"this removed all data related with company, it cannot be canceled!"}</span>
            </div>
            <br/>
            <button
                id="button-delete-company"
                class="button is-danger"
                onclick=onclick_delete_company>
                { "Delete" }
            </button>
        </>}
    }

    fn fieldset_add_specs(&self, show_ipt: bool) -> Html {
        let ipt_ref = self.ipt_ref.clone();
        let company_id = self.current_data.as_ref().unwrap().uuid.clone();
        let company_specs = self.current_data.as_ref().unwrap().company_specs.clone();
        let specs = self
            .search_specs
            .iter()
            .map(|x| Spec::from(x))
            .collect::<Vec<Spec>>();
        html!{
          <div hidden=!show_ipt>
            <article class=classes!(String::from("panel is-primary")) style="margin-top: 16px;">
              <p class="panel-heading">
                {"Specs Search"}
              </p>
              // <p class="panel-tabs">
              //   <a class="is-active">All</a>
              //   <a>Public</a>
              //   <a>Private</a>
              //   <a>Sources</a>
              //   <a>Forks</a>
              // </p>
              <div class="panel-block">
                <p class=classes!(String::from("control has-icons-left"),if self.specs_search_loading {
                  String::from("is-loading")
                } else {
                  String::new()
                }) >
                  <input ref=ipt_ref oninput=self.link.callback(|ev: InputData| Msg::SetIptTimer(ev.value)) class="input is-rounded" type="text" placeholder="Rounded input" />
                  <span class="icon is-left">
                    <i class="fas fa-search" aria-hidden="true"></i>
                  </span>
                </p>
              </div>
              <div class="panel-block">
            //   <SpecsTags
            //   show_delete_btn = !self.edit_specs
            //   company_uuid = company.uuid.clone()
            //   specs = company.company_specs.clone()
            // />
                <SearchSpecsTags
                  company_specs=company_specs
                  company_uuid = company_id
                  specs = specs.clone()
                />
              </div>
            </article>
          </div>
              // <div class="field" hidden=!show_ipt>
              //   <p class="control has-icons-left has-icons-right is-loading">
              //   <input ref=ipt_ref oninput=self.link.callback(|ev: InputData| Msg::SetIptTimer(ev.value)) class="input is-rounded" type="text" placeholder="Rounded input" />
              //     <span class="icon is-small is-left">
              //       <i class="fas fa-search"></i>
              //     </span>
              //   </p>
              // </div>
        }
    }

    fn fieldset_specs(&self) -> Html {
        match &self.current_data {
            Some(company) => html!{<div>
                <span>{"company specs: "}</span>
                <SpecsTags
                  show_delete_btn = !self.edit_specs
                  company_uuid = company.uuid.clone()
                  specs = company.company_specs.clone()
                />
            </div>},
            None => html!{},
        }
    }
}
