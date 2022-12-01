// use yew_agent::Bridge;
use yew::{Component, Context, html, html::Scope, Html, Properties};
use yew_router::prelude::*;
use web_sys::{InputEvent, Event};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use chrono::NaiveDateTime;
use log::debug;
use crate::routes::AppRoute::{Login, StandardSettings};
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_logged_user, get_value_field};
use crate::types::{
    UUID, StandardCreateData, SlimUser, Region, TypeAccessInfo,
    ShowCompanyShort, StandardStatus,
};
use crate::gqls::make_query;
use crate::gqls::standard::{
    GetStandardDataOpt, get_standard_data_opt,
    RegisterStandard, register_standard,
};

/// Standard with relate data
pub struct CreateStandard {
    error: Option<Error>,
    current_user_uuid: UUID,
    request_standard: StandardCreateData,
    // router_agent: Box<dyn Bridge<AppRoute>>,
    supplier_list: Vec<ShowCompanyShort>,
    standard_statuses: Vec<StandardStatus>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessInfo>,
    disable_create_btn: bool,
    // get_result_created_standard: UUID,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub current_user: Option<SlimUser>,
}

#[derive(Clone)]
pub enum Msg {
    RequestManager,
    RequestCreateStandardData,
    GetListOpt(String),
    GetCreateStandardResult(String),
    UpdateClassifier(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdateSpecifiedTolerance(String),
    UpdateTechnicalCommittee(String),
    UpdatePublicationAt(String),
    UpdateTypeAccessId(String),
    UpdateCompanyUuid(String),
    UpdateStandardStatusId(String),
    UpdateRegionId(String),
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for CreateStandard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        CreateStandard {
            error: None,
            current_user_uuid: ctx.props().current_user.as_ref().map(|x| x.uuid.clone()).unwrap_or_default(),
            request_standard: StandardCreateData::new(),
            // router_agent: AppRoute::bridge(ctx.link().callback(|_| Msg::Ignore)),
            supplier_list: Vec::new(),
            standard_statuses: Vec::new(),
            regions: Vec::new(),
            types_access: Vec::new(),
            disable_create_btn: false,
            // get_result_created_standard: String::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let logged_user_uuid = match get_logged_user() {
            Some(cu) => cu.uuid,
            None => {
                // route to login page if not found token
                // self.router_agent.send(Login);
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Login);
                String::new()
            },
        };

        if first_render {
            let link = ctx.link().clone();

            spawn_local(async move {
                let ipt_companies_arg = get_standard_data_opt::IptCompaniesArg{
                    companies_uuids: None,
                    user_uuid: Some(logged_user_uuid),
                    favorite: None,
                    supplier: Some(true),
                    limit: None,
                    offset: None,
                };
                let res = make_query(GetStandardDataOpt::build_query(get_standard_data_opt::Variables {
                    ipt_companies_arg
                })).await.unwrap();

                link.send_message(Msg::GetListOpt(res));
            })
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestManager => {
                let mut flag = true;
                // checking have data
                if self.request_standard.company_uuid.is_empty() {
                    debug!("company_uuid is empty: {:?}", self.request_standard.classifier);
                    match self.supplier_list.first() {
                        Some(company) => self.request_standard.company_uuid = company.uuid.clone(),
                        None => {
                            debug!("supplier_list is none: {:?}", self.supplier_list);
                            flag = false;
                        },
                    }
                }
                if self.request_standard.classifier.is_empty() {
                    debug!("classifier is empty: {:?}", self.request_standard.classifier);
                    flag = false;
                }
                if self.request_standard.name.is_empty() {
                    debug!("name is empty: {:?}", self.request_standard.name);
                    flag = false;
                }
                if self.request_standard.description.is_empty() {
                    debug!("description is empty: {:?}", self.request_standard.description);
                    flag = false;
                }
                if self.request_standard.specified_tolerance.is_empty() {
                    debug!("specified_tolerance is empty: {:?}", self.request_standard.specified_tolerance);
                    flag = false;
                }
                if self.request_standard.technical_committee.is_empty() {
                    debug!("technical_committee {:?}", self.request_standard.technical_committee);
                    flag = false;
                }

                if flag {
                    link.send_message(Msg::RequestCreateStandardData);
                }
            },
            Msg::RequestCreateStandardData => {
                let request_standard: StandardCreateData = self.request_standard.clone();

                spawn_local(async move {
                    let StandardCreateData {
                        parent_standard_uuid,
                        classifier,
                        name,
                        description,
                        specified_tolerance,
                        technical_committee,
                        publication_at,
                        company_uuid,
                        type_access_id,
                        standard_status_id,
                        region_id,
                    } = request_standard;
                    let ipt_standard_data = register_standard::IptStandardData {
                        parent_standard_uuid,
                        classifier,
                        name,
                        description,
                        specified_tolerance,
                        technical_committee,
                        publication_at,
                        company_uuid,
                        type_access_id: type_access_id as i64,
                        standard_status_id: standard_status_id as i64,
                        region_id: region_id as i64,
                    };
                    let res = make_query(RegisterStandard::build_query(register_standard::Variables {
                        ipt_standard_data
                    })).await.unwrap();
                    link.send_message(Msg::GetCreateStandardResult(res));
                })
            },
            Msg::GetListOpt(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.supplier_list = serde_json::from_value(
                            res_value.get("companies").unwrap().clone()
                        ).unwrap();
                        self.standard_statuses = serde_json::from_value(
                            res_value.get("standardStatuses").unwrap().clone()
                        ).unwrap();
                        self.regions = serde_json::from_value(
                            res_value.get("regions").unwrap().clone()
                        ).unwrap();
                        self.types_access = serde_json::from_value(
                            res_value.get("typesAccess").unwrap().clone()
                        ).unwrap();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetCreateStandardResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UUID = serde_json::from_value(
                            res_value.get("registerStandard").unwrap().clone()
                        ).unwrap();
                        debug!("registerStandard: {:?}", result);
                        // Redirect to setting standard page
                        if !result.is_empty() {
                            // self.get_result_created_standard = result;
                            // self.router_agent.send(StandardSettings { uuid: result });
                            let navigator: Navigator = ctx.link().navigator().unwrap();
                            navigator.replace(&StandardSettings { uuid: result });
                        }

                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            // items request create main standard data
            Msg::UpdateClassifier(data) => self.request_standard.classifier = data,
            Msg::UpdateName(data) => self.request_standard.name = data,
            Msg::UpdateDescription(data) => self.request_standard.description = data,
            Msg::UpdateSpecifiedTolerance(data) => self.request_standard.specified_tolerance = data,
            Msg::UpdateTechnicalCommittee(data) => self.request_standard.technical_committee = data,
            Msg::UpdatePublicationAt(data) => {
                let date = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", data), "%Y-%m-%d %H:%M:%S");
                debug!("new date: {:?}", date);
                if let Ok(dt) = date {
                    self.request_standard.publication_at = dt;
                }
            },
            Msg::UpdateTypeAccessId(data) =>
                self.request_standard.type_access_id = data.parse::<usize>().unwrap_or(1),
            Msg::UpdateCompanyUuid(data) => self.request_standard.company_uuid = data,
            Msg::UpdateStandardStatusId(data) =>
                self.request_standard.standard_status_id = data.parse::<usize>().unwrap_or(1),
            Msg::UpdateRegionId(data) =>
                self.request_standard.region_id = data.parse::<usize>().unwrap_or(1),
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
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
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{
            <div class="standard-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
                        <h1 class="title">{ get_value_field(&291) }</h1>
                        {self.show_main_card(ctx.link())}
                        <br/>
                        {self.show_standard_params(ctx.link())}
                        <br/>
                        {self.show_manage_btn(ctx.link())}
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateStandard {
    fn show_main_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        // let default_company_uuid = self.current_standard.as_ref().map(|x| x.owner_company.uuid.clone()).unwrap_or_default();
        let onchange_change_owner_company = link.callback(|ev: Event| {
            Msg::UpdateCompanyUuid(ev.current_target().map(|ev| ev.as_string().unwrap_or_default()).unwrap_or_default())
        });
        let onchange_change_type_access = link.callback(|ev: Event| {
            Msg::UpdateTypeAccessId(ev.current_target().map(|ev| ev.as_string().unwrap_or_default()).unwrap_or_default())
        });
        let oninput_name = link.callback(|ev: InputEvent| Msg::UpdateName(ev.input_type()));
        let oninput_description = link.callback(|ev: InputEvent| Msg::UpdateDescription(ev.input_type()));

        html!{
            <div class="card">
              <div class="column">
                <div class="control">
                    <div class="media">
                        <div class="media-content">
                            <label class="label">{ get_value_field(&223) }</label> // Owner company
                            <div class="select">
                              <select
                                  id="set-owner-company"
                                  select={self.request_standard.company_uuid.clone()}
                                  onchange={onchange_change_owner_company}
                                >
                              { for self.supplier_list.iter().map(|x|
                                  html!{
                                      <option value={x.uuid.to_string()}
                                            selected={x.uuid == self.request_standard.company_uuid} >
                                          {&x.shortname}
                                      </option>
                                  }
                              )}
                              </select>
                            </div>
                        </div>
                        <div class="media-right" style="margin-right: 1rem">
                            <label class="label">{ get_value_field(&114) }</label>
                            <div class="select">
                              <select
                                  id="set-type-access"
                                  select={self.request_standard.type_access_id.to_string()}
                                  onchange={onchange_change_type_access}
                                >
                              { for self.types_access.iter().map(|x|
                                  html!{
                                      <option value={x.type_access_id.to_string()}
                                            selected={x.type_access_id == self.request_standard.type_access_id} >
                                          {&x.name}
                                      </option>
                                  }
                              )}
                              </select>
                            </div>
                        </div>
                    </div>
                </div>
                <label class="label">{ get_value_field(&110) }</label>
                <input
                    id="update-name"
                    class="input"
                    type="text"
                    placeholder={get_value_field(&110)}
                    value={self.request_standard.name.clone()}
                    oninput={oninput_name} />
                <label class="label">{ get_value_field(&61) }</label>
                <textarea
                    id="update-description"
                    class="textarea"
                    // rows="10"
                    type="text"
                    placeholder={get_value_field(&61)}
                    value={self.request_standard.description.clone()}
                    oninput={oninput_description} />
              </div>
            </div>
        }
    }

    fn show_standard_params(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_classifier = link.callback(|ev: InputEvent| Msg::UpdateClassifier(ev.input_type()));
        let oninput_specified_tolerance = link.callback(|ev: InputEvent| Msg::UpdateSpecifiedTolerance(ev.input_type()));
        let oninput_technical_committee = link.callback(|ev: InputEvent| Msg::UpdateTechnicalCommittee(ev.input_type()));
        let oninput_publication_at = link.callback(|ev: InputEvent| Msg::UpdatePublicationAt(ev.input_type()));
        let onchange_standard_status_id = link.callback(|ev: Event| {
            Msg::UpdateStandardStatusId(ev.current_target().map(|ev| ev.as_string().unwrap_or_default()).unwrap_or_default())
        });
        let onchange_region_id = link.callback(|ev: Event| {
            Msg::UpdateRegionId(ev.current_target().map(|ev| ev.as_string().unwrap_or_default()).unwrap_or_default())
        });

        html!{
            <>
              <h2 class="has-text-weight-bold">{ get_value_field(&229) }</h2> // Set standard characteristics
              <div class="card column">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{ get_value_field(&146) }</td> // classifier
                        <td><input
                            id="update-classifier"
                            class="input"
                            type="text"
                            placeholder={get_value_field(&146)}
                            value={self.request_standard.classifier.clone()}
                            oninput={oninput_classifier} /></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&147) }</td> // specified_tolerance
                        // <td>{self.request_standard.specified_tolerance.as_ref().map(|x| x.clone()).unwrap_or_default()}</td>
                        <td><input
                            id="update-specified-tolerance"
                            class="input"
                            type="text"
                            placeholder={get_value_field(&147)}
                            value={self.request_standard.specified_tolerance.clone()}
                            oninput={oninput_specified_tolerance} /></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&148) }</td> // technical_committee
                        <td><input
                            id="update-technical-committee"
                            class="input"
                            type="text"
                            placeholder={get_value_field(&148)}
                            value={self.request_standard.technical_committee.clone()}
                            oninput={oninput_technical_committee} /></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&149) }</td> // publication_at
                        <td><input
                            id="update-publication-at"
                            class="input"
                            type="date"
                            placeholder={get_value_field(&149)}
                            value={format!("{:.*}", 10,
                                self.request_standard.publication_at.to_string()
                            )}
                            oninput={oninput_publication_at}
                            /></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&150) }</td> // standard_status
                        <td><div class="control">
                            <div class="select">
                              <select
                                  id="standard-status-id"
                                  select={self.request_standard.standard_status_id.to_string()}
                                  onchange={onchange_standard_status_id}
                                  >
                                { for self.standard_statuses.iter().map(|x|
                                    html!{
                                        <option value={x.standard_status_id.to_string()}
                                              selected={x.standard_status_id == self.request_standard.standard_status_id} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&151) }</td> // region
                        <td><div class="select">
                              <select
                                  id="region"
                                  select={self.request_standard.region_id.to_string()}
                                  onchange={onchange_region_id}
                                  >
                                { for self.regions.iter().map(|x|
                                    html!{
                                        <option value={x.region_id.to_string()}
                                              selected={x.region_id == self.request_standard.region_id} >
                                            {&x.region}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </td>
                      </tr>
                    </tbody>
                  </table>
              </div>
            </>
        }
    }

    fn show_manage_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_create_changes = link.callback(|_| Msg::RequestManager);

        {match self.supplier_list.is_empty() {
            true => html!{},
            false => html!{
                <button
                    id="create-data"
                    class="button is-success is-medium is-fullwidth"
                    onclick={onclick_create_changes}
                    disabled={self.disable_create_btn} >
                    { get_value_field(&45) } // Create
                </button>
            },
        }}
    }
}
