// use yew::services::fetch::FetchTask;
use yew::{
    // agent::Bridged, Bridge,Callback,
    html, Component, ComponentLink,
    FocusEvent, Html, InputData, ChangeData, Properties, ShouldRender,
};
use yew_router::service::RouteService;
// use yew_router::{
//     agent::RouteRequest::ChangeRoute,
//     prelude::*
// };
use chrono::NaiveDateTime;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::gqls::make_query;
use crate::error::{Error, get_error};
use crate::fragments::{
    list_errors::ListErrors,
    // company_certificate::CompanyCertificateCard,
    // company_add_certificate::AddCertificateCard,
    upload_favicon::UpdateFaviconCard,
};
// use crate::routes::AppRoute;
use crate::services::is_authenticated;
use crate::types::{
    UUID, SlimUser, CompanyUpdateInfo, CompanyInfo, Region,
    CompanyType, TypeAccessTranslateListInfo,
    // Certificate, CompanyCertificate,
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
    // Certificates,
    // Access,
    // Password,
    // RemoveCompany,
}

/// Update settings of the author or logout
pub struct CompanySettings {
    // auth: Auth,
    error: Option<Error>,
    request_company: CompanyUpdateInfo,
    // request_access: i64,
    // response: Callback<Result<usize, Error>>,
    // task: Option<FetchTask>,
    // router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    company_uuid: String,
    current_data: Option<CompanyInfo>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessTranslateListInfo>,
    company_types: Vec<CompanyType>,
    get_result_update: usize,
    // get_result_access: bool,
    // get_result_remove_profile: bool,
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
    // RequestChangeAccess,
    // RequestRemoveCompany,
    ResponseError(Error),
    // GetUpdateAccessResult(String),
    GetUpdateCompanyData(String),
    GetUpdateCompanyResult(String),
    // GetRemoveCompanyResult(String),
    // UpdateTypeAccessId(String)
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
            // auth: Auth::new(),
            error: None,
            request_company: CompanyUpdateInfo::default(),
            // request_access: 0,
            // response: link.callback(Msg::Response),
            // task: None,
            // router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            company_uuid: String::new(),
            current_data: None,
            regions: Vec::new(),
            types_access: Vec::new(),
            company_types: Vec::new(),
            get_result_update: 0,
            // get_result_access: false,
            // get_result_remove_profile: false,
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
            Msg::ResponseError(err) => {
                self.error = Some(err);
                // self.task = None;
            }
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
            }
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
        let onsubmit_update_profile = self.link.callback(|ev: FocusEvent| {
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
                                        // Show interface for change profile data
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
                                            <form onsubmit=onsubmit_update_profile>
                                                { self.fieldset_profile() }
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
        // let onclick_certificates = self.link
        //     .callback(|_| Msg::SelectMenu(
        //         Menu::Certificates
        //     ));
        // let onclick_access = self.link
        //     .callback(|_| Msg::SelectMenu(
        //         Menu::Access
        //     ));
        // let onclick_remove_profile = self.link
        //     .callback(|_| Msg::SelectMenu(
        //         Menu::RemoveCompany
        //     ));

        let mut active_company = "";
        let mut active_favicon = "";
        // let mut active_certificates = "";
        // let mut active_access = "";
        // let mut active_remove_company = "";

        match self.select_menu {
            Menu::Company => active_company = "is-active",
            Menu::UpdataFavicon => active_favicon = "is-active",
            // Menu::Certificates => active_certificates = "is-active",
            // Menu::Access => active_access = "is-active",
            // Menu::RemoveCompany => active_remove_company = "is-active",
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
                    // <li><a
                    //   id="certificates"
                    //   class=active_certificates
                    //   onclick=onclick_certificates>
                    //     { "Certificates" }
                    // </a></li>
                    // <li><a
                    //   id="access"
                    //   class=active_access
                    //   onclick=onclick_access>
                    //     { "Access" }
                    // </a></li>
                    // <li><a
                    //   id="remove-profile"
                    //   class=active_remove_profile
                    //   onclick=onclick_remove_profile>
                    //     { "Remove profile" }
                    // </a></li>
                </ul>
            </aside>
        }
    }

    fn fieldset_profile(
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
}
