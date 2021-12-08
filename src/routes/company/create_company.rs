use yew::{
    agent::Bridged, html, Bridge, Component, ComponentLink,
    FocusEvent, Html, InputData, ChangeData, Properties, ShouldRender,
};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::*
};
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::gqls::make_query;
use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
use crate::routes::AppRoute;
use crate::services::is_authenticated;
use crate::types::{
    UUID, SlimUser, CompanyCreateInfo, Region,
    CompanyType, TypeAccessInfo, SlimCompany,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct GetCreateCompanyDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct RegisterCompany;

/// Update settings of the author or logout
pub struct CreateCompany {
    error: Option<Error>,
    request_company: CompanyCreateInfo,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    regions: Vec<Region>,
    company_types: Vec<CompanyType>,
    types_access: Vec<TypeAccessInfo>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
}

pub enum Msg {
    RequestCreateCompany,
    ResponseError(Error),
    GetCreateCompanyResult(String),
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
    Ignore,
}

impl Component for CreateCompany {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateCompany {
            error: None,
            request_company: CompanyCreateInfo::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            regions: Vec::new(),
            company_types: Vec::new(),
            types_access: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();

        if first_render && is_authenticated() {
            spawn_local(async move {
                let res = make_query(
                    GetCreateCompanyDataOpt::build_query(get_create_company_data_opt::Variables)
                ).await.unwrap();
                link.send_message(Msg::UpdateList(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestCreateCompany => {
                let request_company = self.request_company.clone();
                spawn_local(async move {
                    let CompanyCreateInfo {
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
                        type_access_id,
                    } = request_company;
                    let ipt_company_data = register_company::IptCompanyData {
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
                        typeAccessId: type_access_id,
                    };
                    let res = make_query(RegisterCompany::build_query(register_company::Variables {
                        ipt_company_data
                    })).await;
                    link.send_message(Msg::GetCreateCompanyResult(res.unwrap()));
                })
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
                // self.task = None;
            },
            Msg::GetCreateCompanyResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let company_data: SlimCompany = serde_json::from_value(res.get("registerCompany").unwrap().clone()).unwrap();
                        debug!("Company data: {:?}", company_data);
                        self.router_agent.send(ChangeRoute(AppRoute::ShowCompany(
                            company_data.uuid.clone()
                        ).into()))
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::UpdateTypeAccessId(type_access_id) => {
                self.request_company.type_access_id = type_access_id.parse::<i64>().unwrap_or_default();
                debug!("Update: {:?}", type_access_id);
            },
            Msg::UpdateOrgname(orgname) => {
                self.request_company.orgname = orgname;
            },
            Msg::UpdateShortname(shortname) => {
                self.request_company.shortname = shortname;
            },
            Msg::UpdateInn(inn) => {
                self.request_company.inn = inn;
            },
            Msg::UpdateEmail(email) => {
                self.request_company.email = email;
            },
            Msg::UpdatePhone(phone) => {
                self.request_company.phone = phone;
            },
            Msg::UpdateDescription(description) => {
                self.request_company.description = description;
            },
            Msg::UpdateAddress(address) => {
                self.request_company.address = address;
            },
            Msg::UpdateSiteUrl(site_url) => {
                self.request_company.site_url = site_url;
            },
            Msg::UpdateTimeZone(time_zone) => {
                self.request_company.time_zone = time_zone;
            },
            Msg::UpdateRegionId(region_id) => {
                self.request_company.region_id = region_id.parse::<i64>().unwrap_or_default();
                debug!("Update: {:?}", region_id);
            },
            Msg::UpdateCompanyTypeId(type_id) => {
                self.request_company.company_type_id = type_id.parse::<i64>().unwrap_or_default();
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
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit_create_company = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::RequestCreateCompany
        });

        html! {
            <div class="settings-page">
                <ListErrors error=self.error.clone()/>
                <div class="container page">
                    <div class="row">
                        <div class="card">
                            <h1 class="title">{ "Create company" }</h1>
                            <form onsubmit=onsubmit_create_company>
                                { self.fieldset_company() }
                                <button
                                    id="create-company"
                                    class="button"
                                    type="submit"
                                    disabled=false>
                                    { "Create company" }
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateCompany {
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
        let onchange_type_access_id = self
              .link
              .callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
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
                            value={self.request_company.orgname.clone()}
                            oninput=oninput_orgname />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Shortname"}</label>
                        <input
                            id="shortname"
                            class="input"
                            type="text"
                            placeholder="shortname"
                            value={self.request_company.shortname.clone()}
                            oninput=oninput_shortname />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Inn"}</label>
                        <input
                            id="inn"
                            class="input"
                            type="text"
                            placeholder="inn"
                            value={self.request_company.inn.clone()}
                            oninput=oninput_inn />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Region"}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="region"
                                  select={self.request_company.region_id.to_string()}
                                  onchange=onchange_region_id
                                  >
                                { for self.regions.iter().map(|x|
                                    match self.request_company.region_id == x.region_id as i64 {
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
                            value={self.request_company.email.clone()}
                            oninput=oninput_email />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Phone"}</label>
                        <input
                            id="phone"
                            class="input"
                            type="text"
                            placeholder="phone"
                            value={self.request_company.phone.clone()}
                            oninput=oninput_phone />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Address"}</label>
                        <input
                            id="address"
                            class="input"
                            type="text"
                            placeholder="address"
                            value={self.request_company.address.clone()}
                            oninput=oninput_address />
                    </fieldset>
                    <fieldset class="field">
                        <label class="label">{"Site"}</label>
                        <input
                            id="site_url"
                            class="input"
                            type="text"
                            placeholder="site_url"
                            value={self.request_company.site_url.clone()}
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
                          select={self.request_company.company_type_id.to_string()}
                          onchange=onchange_company_type_id
                          >
                        { for self.company_types.iter().map(|x|
                            match self.request_company.company_type_id == x.company_type_id as i64{
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
                <label class="label">{"Type Access"}</label>
                <div class="control">
                    <div class="select">
                      <select
                          id="types-access"
                          select={self.request_company.type_access_id.to_string()}
                          onchange=onchange_type_access_id
                          >
                        { for self.types_access.iter().map(|x|
                            match self.request_company.type_access_id == x.type_access_id as i64{
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
            <fieldset class="field">
                <label class="label">{"Description"}</label>
                <textarea
                    id="description"
                    class="input"
                    type="description"
                    placeholder="description"
                    value={self.request_company.description.clone()}
                    oninput=oninput_description />
            </fieldset>
        </>}
    }
}
