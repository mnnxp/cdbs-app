use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use yew_router::prelude::*;
use web_sys::{InputEvent, Event};
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::routes::AppRoute::{Login, ShowCompany};
use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_logged_user, get_value_field};
use crate::types::{UUID, SlimUser, CompanyCreateInfo, Region, CompanyType, TypeAccessInfo};
use crate::gqls::make_query;
use crate::gqls::company::{
    GetCreateCompanyDataOpt, get_create_company_data_opt,
    RegisterCompany, register_company
};

/// Update settings of the author or logout
pub struct CreateCompany {
    error: Option<Error>,
    current_user_uuid: UUID,
    request_company: CompanyCreateInfo,
    regions: Vec<Region>,
    company_types: Vec<CompanyType>,
    types_access: Vec<TypeAccessInfo>,
}

#[derive(Properties, Clone, Debug, PartialEq)]
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

    fn create(ctx: &Context<Self>) -> Self {
        CreateCompany {
            error: None,
            current_user_uuid: ctx.props().current_user.as_ref().map(|x| x.uuid.clone()).unwrap_or_default(),
            request_company: CompanyCreateInfo::new(),
            regions: Vec::new(),
            company_types: Vec::new(),
            types_access: Vec::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if let None = get_logged_user() {
            // route to login page if not found token
            let navigator: Navigator = ctx.link().navigator().unwrap();
            navigator.replace(&Login);
        };

        if first_render {
            let link = ctx.link().clone();
            spawn_local(async move {
                let res = make_query(GetCreateCompanyDataOpt::build_query(
                    get_create_company_data_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::UpdateList(res));
            })
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
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
                        site_url,
                        time_zone,
                        region_id,
                        company_type_id,
                        type_access_id,
                    };
                    let res = make_query(RegisterCompany::build_query(register_company::Variables {
                        ipt_company_data
                    })).await;
                    link.send_message(Msg::GetCreateCompanyResult(res.unwrap()));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetCreateCompanyResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let company_uuid: UUID = serde_json::from_value(res.get("registerCompany").unwrap().clone()).unwrap();
                        debug!("Company uuid: {:?}", company_uuid);
                        // Redirect to company page
                        let navigator: Navigator = ctx.link().navigator().unwrap();
                        navigator.replace(&ShowCompany { uuid: company_uuid.clone() });
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::UpdateTypeAccessId(type_access_id) => {
                self.request_company.type_access_id = type_access_id.parse::<i64>().unwrap_or(1);
                debug!("Update: {:?}", type_access_id);
            },
            Msg::UpdateOrgname(orgname) => self.request_company.orgname = orgname,
            Msg::UpdateShortname(shortname) => self.request_company.shortname = shortname,
            Msg::UpdateInn(inn) => self.request_company.inn = inn,
            Msg::UpdateEmail(email) => self.request_company.email = email,
            Msg::UpdatePhone(phone) => self.request_company.phone = phone,
            Msg::UpdateDescription(description) => self.request_company.description = description,
            Msg::UpdateAddress(address) => self.request_company.address = address,
            Msg::UpdateSiteUrl(site_url) => self.request_company.site_url = site_url,
            Msg::UpdateTimeZone(time_zone) => self.request_company.time_zone = time_zone,
            Msg::UpdateRegionId(region_id) => {
                self.request_company.region_id = region_id.parse::<i64>().unwrap_or(1);
                debug!("Update: {:?}", region_id);
            },
            Msg::UpdateCompanyTypeId(type_id) => {
                self.request_company.company_type_id = type_id.parse::<i64>().unwrap_or(1);
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
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().current_user.as_ref().map(|x| x.uuid == self.current_user_uuid).unwrap_or_default() {
            false
        } else {
            self.current_user_uuid = ctx.props().current_user.as_ref().map(|x| x.uuid.clone()).unwrap_or_default();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_create_company = ctx.link().callback(|_| Msg::RequestCreateCompany);

        html!{
            <div class="settings-page">
                <ListErrors error={self.error.clone()}/>
                <div class="container page">
                    <div class="row">
                        <h1 class="title">{ get_value_field(&289) }</h1>
                        <div class="card column">
                            { self.fieldset_company(ctx.link()) }
                        </div>
                        <br/>
                        <button
                            id={"create-company"}
                            class={"button is-success is-medium is-fullwidth"}
                            type={"submit"}
                            disabled={false}
                            onclick={onclick_create_company}
                            >
                            { get_value_field(&45) } // Create
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateCompany {
    fn fileset_generator(
        &self,
        id: &str,
        label: &str,
        // placeholder: &str,
        icon_left: &str,
        value: String,
        oninput: Callback<InputEvent>,
    ) -> Html {
        let placeholder = label;
        let input_type = match id {
            "email" => "email",
            "password" => "password",
            _ => "text",
        };

        html!{
            <fieldset class="field">
                <label class="label">{label.to_string()}</label>
                {match icon_left.is_empty() {
                    true => html!{
                        <input
                            id={id.to_string()}
                            class={"input"}
                            type={input_type}
                            placeholder={placeholder.to_string()}
                            {value}
                            {oninput} />
                    },
                    false => html!{
                        <div class="control has-icons-left">
                            <input
                                id={id.to_string()}
                                class={"input"}
                                type={input_type}
                                placeholder={placeholder.to_string()}
                                {value}
                                {oninput} />
                            <span class="icon is-small is-left">
                              <i class={icon_left.to_string()}></i>
                            </span>
                        </div>
                    },
                }}
            </fieldset>
        }
    }

    fn fieldset_company(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_orgname = link.callback(|ev: InputEvent| Msg::UpdateOrgname(ev.input_type()));
        let oninput_shortname = link.callback(|ev: InputEvent| Msg::UpdateShortname(ev.input_type()));
        let oninput_inn = link.callback(|ev: InputEvent| Msg::UpdateInn(ev.input_type()));
        let oninput_email = link.callback(|ev: InputEvent| Msg::UpdateEmail(ev.input_type()));
        let oninput_description = link.callback(|ev: InputEvent| Msg::UpdateDescription(ev.input_type()));
        let oninput_phone = link.callback(|ev: InputEvent| Msg::UpdatePhone(ev.input_type()));
        let oninput_address = link.callback(|ev: InputEvent| Msg::UpdateAddress(ev.input_type()));
        let oninput_site_url = link.callback(|ev: InputEvent| Msg::UpdateSiteUrl(ev.input_type()));
        // let oninput_time_zone = link.callback(|ev: InputEvent| Msg::UpdateTimeZone(ev.input_type()));
        let onchange_region_id = link.callback(|ev: Event| {
            Msg::UpdateRegionId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default())
        });
        let onchange_company_type_id = link.callback(|ev: Event| {
            Msg::UpdateCompanyTypeId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default())
        });
        let onchange_type_access_id = link.callback(|ev: Event| {
            Msg::UpdateTypeAccessId(ev.current_target().map(|et| et.as_string().unwrap_or_default()).unwrap_or_default())
        });

        html!{<>
            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "orgname", get_value_field(&170), // Orgname
                        "",
                        self.request_company.orgname.clone(),
                        oninput_orgname
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "shortname", get_value_field(&171), // Shortname
                        "",
                        self.request_company.shortname.clone(),
                        oninput_shortname
                    )}
                </div>
            </div>

            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "inn", get_value_field(&81), // Inn
                        "",
                        self.request_company.inn.clone(),
                        oninput_inn
                    )}
                </div>
                <div class="column">
                    <fieldset class="field">
                        <label class="label">{ get_value_field(&51) }</label> // Company type
                        <div class="control">
                            <div class="select">
                              <select
                                  id="company_type"
                                  select={self.request_company.company_type_id.to_string()}
                                  onchange={onchange_company_type_id}
                                  >
                                { for self.company_types.iter().map(|x|
                                    html!{
                                        <option value={x.company_type_id.to_string()}
                                              selected={x.company_type_id as i64 == self.request_company.company_type_id} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </div>
            </div>

            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "email", get_value_field(&22), // Email
                        "fas fa-at",
                        self.request_company.email.clone(),
                        oninput_email
                    )}
                </div>
                <div class="column">
                    {self.fileset_generator(
                        "phone", get_value_field(&56), // Phone
                        "fas fa-phone",
                        self.request_company.phone.clone(),
                        oninput_phone
                    )}
                </div>
            </div>

            <div class="columns">
                <div class="column">
                    <fieldset class="field">
                        <label class="label">{ get_value_field(&27) }</label> // Region
                        <div class="control">
                            <div class="select">
                              <select
                                  id="region"
                                  select={self.request_company.region_id.to_string()}
                                  onchange={onchange_region_id}
                                  >
                                { for self.regions.iter().map(|x|
                                    html!{
                                        <option value={x.region_id.to_string()}
                                              selected={x.region_id as i64 == self.request_company.region_id} >
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
                        "fas fa-map-marker-alt",
                        self.request_company.address.clone(),
                        oninput_address
                    )}
                </div>
            </div>

            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "site_url", get_value_field(&66), // Site
                        "fas fa-link",
                        self.request_company.site_url.clone(),
                        oninput_site_url
                    )}
                </div>
                <div class="column">
                    <fieldset class="field">
                        <label class="label">{ get_value_field(&58) }</label> // Type access
                        <div class="control">
                            <div class="select">
                              <select
                                  id="types-access"
                                  select={self.request_company.type_access_id.to_string()}
                                  onchange={onchange_type_access_id}
                                  >
                                { for self.types_access.iter().map(|x|
                                    html!{
                                        <option value={x.type_access_id.to_string()}
                                              selected={x.type_access_id as i64 == self.request_company.type_access_id} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </fieldset>
                </div>
            </div>

            <fieldset class="field">
                <label class="label">{ get_value_field(&61) }</label>
                <textarea
                    id={"description"}
                    class={"textarea"}
                    type={"text"}
                    placeholder={ get_value_field(&61) }
                    value={self.request_company.description.clone()}
                    oninput={oninput_description} />
            </fieldset>
        </>}
    }
}
