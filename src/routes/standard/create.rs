use yew::{
    agent::Bridged, html, Bridge, Component, Properties,
    ComponentLink, Html, ShouldRender, InputData, ChangeData
};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use chrono::NaiveDateTime;
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_create_btn;
use crate::services::{get_from_value, get_logged_user, get_value_field, get_value_response, resp_parsing, set_history_back};
use crate::types::{
    UUID, StandardCreateData, SlimUser, TypeAccessInfo,
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
    request_standard: StandardCreateData,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    company_list: Vec<ShowCompanyShort>,
    standard_statuses: Vec<StandardStatus>,
    types_access: Vec<TypeAccessInfo>,
    disable_create_btn: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
}

#[derive(Clone)]
pub enum Msg {
    RequestManager,
    RequestCreateStandardData,
    GetListOpt(String),
    GetCreateStandardResult(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdatePublicationAt(String),
    UpdateTypeAccessId(String),
    UpdateCompanyUuid(String),
    UpdateStandardStatusId(String),
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for CreateStandard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateStandard {
            error: None,
            request_standard: StandardCreateData::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            company_list: Vec::new(),
            standard_statuses: Vec::new(),
            types_access: Vec::new(),
            disable_create_btn: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let logged_user_uuid = match get_logged_user() {
            Some(cu) => cu.uuid,
            None => {
                set_history_back(Some(String::new()));
                // route to login page if not found token
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                String::new()
            },
        };

        if first_render {
            let link = self.link.clone();

            spawn_local(async move {
                let ipt_companies_arg = get_standard_data_opt::IptCompaniesArg{
                    companiesUuids: None,
                    userUuid: Some(logged_user_uuid),
                    favorite: None,
                    supplier: None,
                };
                let res = make_query(GetStandardDataOpt::build_query(get_standard_data_opt::Variables {
                    ipt_companies_arg
                })).await.unwrap();

                link.send_message(Msg::GetListOpt(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestManager => {
                let mut flag = true;
                // checking have data
                if self.request_standard.company_uuid.is_empty() {
                    match self.company_list.first() {
                        Some(company) => self.request_standard.company_uuid = company.uuid.clone(),
                        None => {
                            debug!("company_list is none: {:?}", self.company_list);
                            flag = false;
                        },
                    }
                }
                if self.request_standard.name.is_empty() {
                    debug!("name is empty: {:?}", self.request_standard.name);
                    flag = false;
                }
                if self.request_standard.description.is_empty() {
                    debug!("description is empty: {:?}", self.request_standard.description);
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
                        name,
                        description,
                        publication_at,
                        company_uuid,
                        type_access_id,
                        standard_status_id,
                    } = request_standard;
                    let ipt_standard_data = register_standard::IptStandardData {
                        parentStandardUuid: parent_standard_uuid,
                        name,
                        description,
                        publicationAt: publication_at,
                        companyUuid: company_uuid,
                        typeAccessId: type_access_id as i64,
                        standardStatusId: standard_status_id as i64,
                    };
                    let res = make_query(RegisterStandard::build_query(register_standard::Variables {
                        ipt_standard_data
                    })).await.unwrap();
                    link.send_message(Msg::GetCreateStandardResult(res));
                })
            },
            Msg::GetListOpt(res) => {
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.company_list = get_from_value(value, "companies").unwrap_or_default();
                        self.standard_statuses = get_from_value(value, "standardStatuses").unwrap_or_default();
                        self.types_access = get_from_value(value, "typesAccess").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetCreateStandardResult(res) => {
                match resp_parsing::<UUID>(res, "registerStandard") {
                    Ok(result) => {
                        debug!("registerStandard: {:?}", result);
                        // Redirect to setting standard page
                        if result.is_empty() {
                            return true;
                        }
                        self.router_agent.send(
                            ChangeRoute(AppRoute::StandardSettings(result).into())
                        );
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            // items request create main standard data
            Msg::UpdateName(data) => self.request_standard.name = data,
            Msg::UpdateDescription(data) => self.request_standard.description = data,
            Msg::UpdatePublicationAt(data) => {
                let date = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", data), "%Y-%m-%d %H:%M:%S");
                debug!("new date: {:?}", date);
                if let Ok(dt) = date {
                    self.request_standard.publication_at = dt;
                }
            },
            Msg::UpdateTypeAccessId(data) =>
                self.request_standard.type_access_id = data.parse::<usize>().unwrap_or_default(),
            Msg::UpdateCompanyUuid(data) => self.request_standard.company_uuid = data,
            Msg::UpdateStandardStatusId(data) =>
                self.request_standard.standard_status_id = data.parse::<usize>().unwrap_or_default(),
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.current_user.as_ref().map(|x| &x.uuid) == props.current_user.as_ref().map(|x| &x.uuid) {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{
            <div class="standard-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                        <h1 class="title">{get_value_field(&291)}</h1>
                        {self.show_main_card()}
                        <br/>
                        {self.show_standard_params()}
                        <br/>
                        {self.show_manage_btn()}
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateStandard {
    fn show_main_card(&self) -> Html {
        let onchange_change_owner_company =
            self.link.callback(|ev: ChangeData| Msg::UpdateCompanyUuid(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
            }));
        let onchange_change_type_access =
            self.link.callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        html!{
            <div class="card">
              <div class="column">
                <div class="control">
                    <div class="media">
                        <div class="media-content">
                            <label class="label">{get_value_field(&223)}</label> // Owner company
                            <div class="select">
                              <select
                                  id="set-owner-company"
                                  select={self.request_standard.company_uuid.clone()}
                                  onchange={onchange_change_owner_company}
                                >
                              { for self.company_list.iter().map(|x|
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
                            <label class="label">{get_value_field(&114)}</label>
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
                <label class="label">{get_value_field(&110)}</label>
                <input
                    id="update-name"
                    class="input"
                    type="text"
                    placeholder={get_value_field(&110)}
                    value={self.request_standard.name.clone()}
                    oninput={oninput_name} />
                <label class="label">{get_value_field(&61)}</label>
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

    fn show_standard_params(&self) -> Html {
        let oninput_publication_at =
            self.link.callback(|ev: InputData| Msg::UpdatePublicationAt(ev.value));
        let onchange_standard_status_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateStandardStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        html!{
            <>
              <h2 class="has-text-weight-bold">{get_value_field(&229)}</h2> // Set standard characteristics
              <div class="card column">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{get_value_field(&149)}</td> // publication_at
                        <td><input
                            id="update-publication-at"
                            class="input"
                            type="date"
                            placeholder={get_value_field(&149)}
                            value={format!("{:.*}", 10, self.request_standard.publication_at.to_string())}
                            oninput={oninput_publication_at}
                            /></td>
                      </tr>
                      <tr>
                        <td>{get_value_field(&150)}</td> // standard_status
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
                    </tbody>
                  </table>
              </div>
            </>
        }
    }

    fn show_manage_btn(&self) -> Html {
        let onclick_create_changes = self.link.callback(|_| Msg::RequestManager);

        {match self.company_list.is_empty() {
            true => html!{},
            false => ft_create_btn(
                "create-standard",
                "is-medium".into(),
                onclick_create_changes,
                self.disable_create_btn,
            ),
        }}
    }
}
