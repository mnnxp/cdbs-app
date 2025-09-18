use yew::{agent::Bridged, html, Properties, Bridge, Component, ComponentLink, Html, ShouldRender, InputData, ChangeData};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::fragments::markdown_edit::MarkdownEditCard;
use crate::fragments::type_access::TypeAccessBlock;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_create_btn;
use crate::services::{get_from_value, get_logged_user, get_value_field, get_value_response, resp_parsing, set_focus, set_history_back};
use crate::types::{UUID, ComponentCreateData, TypeAccessInfo, ActualStatus};
use crate::gqls::make_query;
use crate::gqls::component::{
    GetComponentDataOpt, get_component_data_opt,
    RegisterComponent, register_component,
    SetCompanyOwnerSupplier, set_company_owner_supplier,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub company_uuid: Option<UUID>,
}

/// Component with relate data
pub struct CreateComponent {
    error: Option<Error>,
    request_component: ComponentCreateData,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    actual_statuses: Vec<ActualStatus>,
    types_access: Vec<TypeAccessInfo>,
    disable_create_btn: bool,
    name_empty: bool,
}

#[derive(Clone)]
pub enum Msg {
    RequestManager,
    RequestCreateComponentData,
    RequestChangeOwnerSupplier(UUID, UUID),
    ResponseError(Error),
    GetListOpt(String),
    GetCreateComponentResult(String),
    GetUpdateSetSupplierResult(String, UUID),
    UpdateName(String),
    UpdateDescription(String),
    UpdateTypeAccessId(usize),
    UpdateActualStatusId(String),
    ClearError,
    Focuser,
    Ignore,
}

impl Component for CreateComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateComponent {
            error: None,
            request_component: ComponentCreateData::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            actual_statuses: Vec::new(),
            types_access: Vec::new(),
            disable_create_btn: false,
            name_empty: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if let None = get_logged_user() {
            set_history_back(Some(String::new()));
            // route to login page if not found token
            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
        };

        if first_render {
            let link = self.link.clone();

            spawn_local(async move {
                let res = make_query(GetComponentDataOpt::build_query(
                    get_component_data_opt::Variables
                )).await.unwrap();

                link.send_message(Msg::GetListOpt(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestManager => {
                self.disable_create_btn = true;
                // checking have data
                if self.request_component.name.is_empty() {
                    debug!("name is empty: {:?}", self.request_component.name);
                    self.name_empty = true;
                    self.disable_create_btn = false;
                }
                if self.name_empty {
                    link.send_message(Msg::Focuser);
                }
                if self.disable_create_btn {
                    link.send_message(Msg::RequestCreateComponentData);
                }
            },
            Msg::RequestCreateComponentData => {
                let ipt_component_data = register_component::IptComponentData {
                    parentComponentUuid: self.request_component.parent_component_uuid.clone(),
                    name: self.request_component.name.clone(),
                    description: self.request_component.description.clone(),
                    typeAccessId: self.request_component.type_access_id as i64,
                    componentTypeId: self.request_component.component_type_id as i64,
                    actualStatusId: self.request_component.actual_status_id as i64,
                    isBase: self.request_component.is_base,
                };
                spawn_local(async move {
                    let res = make_query(RegisterComponent::build_query(register_component::Variables {
                        ipt_component_data
                    })).await.unwrap();
                    link.send_message(Msg::GetCreateComponentResult(res));
                })
            },
            Msg::RequestChangeOwnerSupplier(component_uuid, company_uuid) => {
                let ipt_supplier_component_data = set_company_owner_supplier::IptSupplierComponentData{
                    componentUuid: component_uuid.clone(),
                    companyUuid: company_uuid,
                    description: String::new(), // self.request_set_supplier_description.clone(),
                };
                spawn_local(async move {
                    let res = make_query(SetCompanyOwnerSupplier::build_query(
                        set_company_owner_supplier::Variables { ipt_supplier_component_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateSetSupplierResult(res, component_uuid));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetListOpt(res) => {
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.actual_statuses = get_from_value(value, "componentActualStatuses").unwrap_or_default();
                        self.types_access = get_from_value(value, "typesAccess").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetCreateComponentResult(res) => {
                match resp_parsing::<UUID>(res, "registerComponent") {
                    Ok(component_uuid) => {
                        debug!("registerComponent: {:?}", component_uuid);
                        if component_uuid.is_empty() {
                            return true
                        }
                        if let Some(company_uuid) = &self.props.company_uuid {
                            link.send_message(Msg::RequestChangeOwnerSupplier(component_uuid, company_uuid.clone()));
                            return true
                        }
                        // Redirect to setting component page
                        self.router_agent.send(
                            ChangeRoute(AppRoute::ComponentSettings(component_uuid).into())
                        );
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateSetSupplierResult(res, component_uuid) => {
                match resp_parsing::<bool>(res, "setCompanyOwnerSupplier") {
                    Ok(result) => {
                        debug!("setCompanyOwnerSupplier: {:?}", result);
                        self.router_agent.send(
                            ChangeRoute(AppRoute::ComponentSettings(component_uuid).into())
                        );
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            // items request create main component data
            Msg::UpdateName(data) => {
                self.request_component.name = data;
                self.name_empty = false;
            },
            Msg::UpdateDescription(data) => self.request_component.description = data,
            Msg::UpdateTypeAccessId(value) => self.request_component.type_access_id = value,
            Msg::UpdateActualStatusId(data) =>
                self.request_component.actual_status_id = data.parse::<usize>().unwrap_or_default(),
            Msg::ClearError => self.error = None,
            Msg::Focuser => set_focus("update-component-name"),
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_create_component = self.link.callback(|_| Msg::RequestManager);

        html!{
            <div class="component-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                        <h1 class="title">{get_value_field(&290)}</h1>
                        {self.show_main_card()}
                        <br/>
                        {ft_create_btn(
                            "create-component",
                            "is-medium".into(),
                            onclick_create_component,
                            self.disable_create_btn,
                        )}
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateComponent {
    fn show_main_card(&self) -> Html {
        let onchange_actual_status_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let onchange_type_access =
            self.link.callback(|value| Msg::UpdateTypeAccessId(value));
        let oninput_name =
            self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_description =
            self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let class_name = match self.name_empty {
            true => "input is-danger",
            false => "input",
        };

        html!{
            <div class="card">
                <div class="column">
                  <label class="title is-5" for="create-component-name">{get_value_field(&110)}</label>
                  <input
                      id="create-component-name"
                      class={class_name}
                      type="text"
                      placeholder={get_value_field(&110)}
                      value={self.request_component.name.clone()}
                      oninput={oninput_name} />
                </div>
                <MarkdownEditCard
                    id_tag={"create-component-description"}
                    title={get_value_field(&61)}
                    placeholder={String::new()}
                    raw_text={self.request_component.description.clone()}
                    oninput_text={oninput_description}
                    />
                <div class="column">
                    <div class="columns">
                        <div class="column">
                            <label class="label" for="create-component-actual-status">{get_value_field(&96)}</label>
                            <div class="select">
                              <select
                                  id="create-component-actual-status"
                                  select={self.request_component.actual_status_id.to_string()}
                                  onchange={onchange_actual_status_id}
                                  >
                                { for self.actual_statuses.iter().map(|x|
                                    html!{
                                        <option value={x.actual_status_id.to_string()}
                                              selected={x.actual_status_id == self.request_component.actual_status_id} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                        <div class="column">
                          <label class="label" for="type-access-block">{get_value_field(&58)}</label>
                            <TypeAccessBlock
                                change_cb={onchange_type_access}
                                types={self.types_access.clone()}
                                selected={self.request_component.type_access_id}
                            />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
