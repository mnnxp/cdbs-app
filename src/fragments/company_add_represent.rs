use yew::{
    html, Component, ComponentLink,
    Html, InputData, ChangeData, Properties, ShouldRender,
};
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::gqls::make_query;
use crate::services::is_authenticated;
use crate::fragments::list_errors::ListErrors;
use crate::error::{Error, get_error};
use crate::types::{
    UUID, Region, RepresentationType,
    RegisterCompanyRepresentInfo
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct GetRepresentDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct RegisterCompanyRepresent;

pub enum Msg {
    RequestRegisterRepresent,
    ResponseError(Error),
    GetRegisterResult(String),
    UpdateRegionId(String),
    UpdateRepresentationTypeId(String),
    UpdateName(String),
    UpdateAddress(String),
    UpdatePhone(String),
    UpdateList(String),
}

pub struct AddCompanyRepresentCard {
    error: Option<Error>,
    request_register: RegisterCompanyRepresentInfo,
    props: Props,
    link: ComponentLink<Self>,
    regions: Vec<Region>,
    represent_types: Vec<RepresentationType>,
    get_result_register: bool,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub company_uuid: UUID,
}

impl Component for AddCompanyRepresentCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_register: RegisterCompanyRepresentInfo::default(),
            props,
            link,
            regions: Vec::new(),
            represent_types: Vec::new(),
            get_result_register: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();

        if first_render && is_authenticated() {
            spawn_local(async move {
                let res = make_query(GetRepresentDataOpt::build_query(
                    get_represent_data_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::UpdateList(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestRegisterRepresent => {
                debug!("Register company represent: {:?}", &self.request_register);
                let request_register = RegisterCompanyRepresentInfo {
                    company_uuid: self.props.company_uuid.clone(),
                    region_id: self.request_register.region_id,
                    representation_type_id: self.request_register.representation_type_id,
                    name: self.request_register.name.clone(),
                    address: self.request_register.address.clone(),
                    phone: self.request_register.phone.clone(),
                };
                spawn_local(async move {
                    let RegisterCompanyRepresentInfo {
                        company_uuid,
                        region_id,
                        representation_type_id,
                        name,
                        address,
                        phone,
                    } = request_register;
                    let ipt_company_represent_data = register_company_represent::IptCompanyRepresentData {
                        companyUuid: company_uuid,
                        regionId: region_id as i64,
                        representationTypeId: representation_type_id as i64,
                        name,
                        address,
                        phone,
                    };
                    let res = make_query(RegisterCompanyRepresent::build_query(register_company_represent::Variables {
                        ipt_company_represent_data
                    })).await.unwrap();
                    link.send_message(Msg::GetRegisterResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetRegisterResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res_value.get("registerCompanyRepresent").unwrap().clone()).unwrap();
                        debug!("Register company represent: {:?}", result);
                        self.get_result_register = result;
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::UpdateRegionId(region_id) => {
                self.request_register.region_id = region_id.parse::<usize>().unwrap_or_default();
            },
            Msg::UpdateRepresentationTypeId(representation_type_id) => {
                self.request_register.representation_type_id = representation_type_id.parse::<usize>().unwrap_or_default();
            },
            Msg::UpdateName(name) => {
                self.request_register.name = name;
            },
            Msg::UpdateAddress(address) => {
                self.request_register.address = address;
            },
            Msg::UpdatePhone(phone) => {
                self.request_register.phone = phone;
            },
            Msg::UpdateList(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();
                match res_value.is_null() {
                    false => {
                        // debug!("Result: {:#?}", res_value.clone);
                        self.regions =
                            serde_json::from_value(res_value.get("regions").unwrap().clone()).unwrap();
                        self.represent_types =
                            serde_json::from_value(res_value.get("companyRepresentTypes").unwrap().clone()).unwrap();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props = props;
        false
    }

    fn view(&self) -> Html {
        html! {<>
            <ListErrors error=self.error.clone()/>
            {match &self.get_result_register {
                true => html!{<div class="card">
                    <article class="message is-success">
                      <div class="message-header">
                        <p>{ "Success" }</p>
                      </div>
                      <div class="message-body">
                        { "This representative created!" }
                      </div>
                    </article>
                </div>},
                false => html!{<div class="card">
                  {self.show_data_for_change()}
                </div>}
            }}
        </>}
    }
}

impl AddCompanyRepresentCard {
    fn show_data_for_change(
        &self,
    ) -> Html {
        let oninput_region_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateRegionId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let oninput_representation_type_id = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateRepresentationTypeId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_address = self
            .link
            .callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_phone = self
            .link
            .callback(|ev: InputData| Msg::UpdatePhone(ev.value));

        let onclick_create_represent = self
            .link
            .callback(|_| Msg::RequestRegisterRepresent);

        html! {<>
            // without columns
            <fieldset class="field">
                <label class="label">{"name"}</label>
                <input
                    id="name"
                    class="input"
                    type="text"
                    placeholder="name"
                    value={self.request_register.name.clone()}
                    oninput=oninput_name />
            </fieldset>
            <fieldset class="field">
                <label class="label">{"address"}</label>
                <input
                    id="address"
                    class="input"
                    type="text"
                    placeholder="address"
                    value={self.request_register.address.clone()}
                    oninput=oninput_address />
            </fieldset>
            <fieldset class="field">
                <label class="label">{"phone"}</label>
                <input
                    id="phone"
                    class="input"
                    type="text"
                    placeholder="phone"
                    value={self.request_register.phone.clone()}
                    oninput=oninput_phone />
            </fieldset>
            // two columns
            <fieldset class="columns">
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"region"}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="region_id"
                                  select={self.request_register.region_id.to_string()}
                                  onchange=oninput_region_id
                                  >
                                { for self.regions.iter().map(|x|
                                    match self.request_register.region_id == x.region_id {
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
                <fieldset class="column">
                    <fieldset class="field">
                        <label class="label">{"representation type"}</label>
                        <div class="control">
                            <div class="select">
                              <select
                                  id="representation_type_id"
                                  select={self.request_register.representation_type_id.to_string()}
                                  onchange=oninput_representation_type_id
                                  >
                                { for self.represent_types.iter().map(|x|
                                    match self.request_register.representation_type_id == x.representation_type_id {
                                        true => {
                                            html!{
                                                <option value={x.representation_type_id.to_string()} selected=true>{&x.representation_type}</option>
                                            }
                                        },
                                        false => {
                                            html!{
                                                <option value={x.representation_type_id.to_string()}>{&x.representation_type}</option>
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
            <a id={ format!("btn-change-represent-{}", &self.props.company_uuid) }
                class="button"
                onclick=onclick_create_represent>
                { "Create" }
            </a>
        </>}
    }
}