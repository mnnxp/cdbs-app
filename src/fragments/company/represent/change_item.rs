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
    CompanyRepresentInfo, CompanyRepresentUpdateInfo
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
struct UpdateCompanyRepresent;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct DeleteCompanyRepresent;

pub enum Msg {
    RequestUpdateRepresent,
    RequestDeleteRepresent,
    ResponseError(Error),
    GetUpdateResult(String),
    GetDeleteRepresentResult(String),
    UpdateRegionId(String),
    UpdateRepresentationTypeId(String),
    UpdateName(String),
    UpdateAddress(String),
    UpdatePhone(String),
    UpdateList(String),
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: CompanyRepresentInfo,
}

pub struct ChangeItem {
    error: Option<Error>,
    company_uuid: UUID,
    company_represent_uuid: UUID,
    request_update: CompanyRepresentUpdateInfo,
    props: Props,
    link: ComponentLink<Self>,
    get_result_update: usize,
    regions: Vec<Region>,
    represent_types: Vec<RepresentationType>,
    get_result_delete: bool,
}

impl Component for ChangeItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            company_uuid: props.data.company_uuid.clone(),
            company_represent_uuid: props.data.uuid.clone(),
            request_update: CompanyRepresentUpdateInfo::default(),
            props,
            link,
            get_result_update: 0,
            regions: Vec::new(),
            represent_types: Vec::new(),
            get_result_delete: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();

        if first_render && is_authenticated() && !self.company_uuid.is_empty() {
            self.request_update = CompanyRepresentUpdateInfo {
                region_id: Some(self.props.data.region.region_id as i64),
                representation_type_id: Some(self.props.data.representation_type.representation_type_id as i64),
                name: Some(self.props.data.name.clone()),
                address: Some(self.props.data.address.clone()),
                phone: Some(self.props.data.phone.clone()),
            };

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
            Msg::RequestUpdateRepresent => {
                debug!("Update company represent: {:?}", &self.request_update);
                let company_uuid = self.company_uuid.clone();
                let company_represent_uuid = self.company_represent_uuid.clone();
                let request_update = self.request_update.clone();
                spawn_local(async move {
                    let CompanyRepresentUpdateInfo {
                        region_id,
                        representation_type_id,
                        name,
                        address,
                        phone,
                    } = request_update;
                    let ipt_update_company_represent_data = update_company_represent::IptUpdateCompanyRepresentData {
                        regionId: region_id,
                        representationTypeId: representation_type_id,
                        name,
                        address,
                        phone,
                    };
                    let res = make_query(UpdateCompanyRepresent::build_query(
                        update_company_represent::Variables {
                            company_uuid,
                            company_represent_uuid,
                            ipt_update_company_represent_data,
                        }
                    )).await;
                    link.send_message(Msg::GetUpdateResult(res.unwrap()));
                })
            },
            Msg::RequestDeleteRepresent => {
                debug!("Update company represent: {:?}", &self.request_update);
                let company_uuid = self.company_uuid.clone();
                let company_represent_uuid = self.company_represent_uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteCompanyRepresent::build_query(
                        delete_company_represent::Variables {
                            company_uuid,
                            company_represent_uuid,
                        }
                    )).await;
                    link.send_message(Msg::GetDeleteRepresentResult(res.unwrap()));
                })
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
                // self.task = None;
            },
            Msg::GetUpdateResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res_value.get("updateCompanyRepresent").unwrap().clone()).unwrap();
                        debug!("Update company represent: {:?}", result);
                        self.get_result_update = result;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::GetDeleteRepresentResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res_value.get("deleteCompanyRepresent").unwrap().clone()).unwrap();
                        debug!("Delete company represent: {:?}", result);
                        self.get_result_delete = result;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
            Msg::UpdateRegionId(region_id) => {
                self.request_update.region_id = Some(region_id.parse::<i64>().unwrap_or_default());
            },
            Msg::UpdateRepresentationTypeId(representation_type_id) => {
                self.request_update.representation_type_id = Some(representation_type_id.parse::<i64>().unwrap_or_default());
            },
            Msg::UpdateName(name) => {
                self.request_update.name = Some(name);
            },
            Msg::UpdateAddress(address) => {
                self.request_update.address = Some(address);
            },
            Msg::UpdatePhone(phone) => {
                self.request_update.phone = Some(phone);
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
                    true => {
                        self.error = Some(get_error(&data));
                    },
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
        html!{<>
            <ListErrors error=self.error.clone()/>
            {match &self.get_result_delete {
                true => html!{<div class="card">
                    <article class="message is-success">
                      <div class="message-header">
                        <p>{ "Success" }</p>
                      </div>
                      <div class="message-body">
                        { "This representative removed!" }
                      </div>
                    </article>
                </div>},
                false => html!{<div class="card">
                  {self.show_data_for_change()}
                  {self.show_btn_delete()}
                </div>}
            }}
        </>}
    }
}

impl ChangeItem {
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

        let onclick_change_represent = self
            .link
            .callback(|_| Msg::RequestUpdateRepresent);

        html!{<>
            {if self.get_result_update > 0 {
                html!{<span id="tag-info-update-represent" class="tag is-info is-light">
                 { format!("Data updated! Change rows: {}", self.get_result_update) }
                </span>}
            } else { html!{} }}

            // without columns
            <fieldset class="field">
                <label class="label">{"name"}</label>
                <input
                    id="name"
                    class="input"
                    type="text"
                    placeholder="name"
                    value={self.request_update.name
                        .as_ref()
                        .map(|x| x.to_string())
                        .unwrap_or_default()}
                    oninput=oninput_name />
            </fieldset>
            <fieldset class="field">
                <label class="label">{"address"}</label>
                <input
                    id="address"
                    class="input"
                    type="text"
                    placeholder="address"
                    value={self.request_update.address
                        .as_ref()
                        .map(|x| x.to_string())
                        .unwrap_or_default()}
                    oninput=oninput_address />
            </fieldset>
            <fieldset class="field">
                <label class="label">{"phone"}</label>
                <input
                    id="phone"
                    class="input"
                    type="text"
                    placeholder="phone"
                    value={self.request_update.phone
                        .as_ref()
                        .map(|x| x.to_string())
                        .unwrap_or_default()}
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
                                  select={self.props.data.region.region_id.to_string()}
                                  onchange=oninput_region_id
                                  >
                                { for self.regions.iter().map(|x|
                                    match self.props.data.region.region_id == x.region_id {
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
                                  select={self.props.data.representation_type.representation_type_id.to_string()}
                                  onchange=oninput_representation_type_id
                                  >
                                { for self.represent_types.iter().map(|x|
                                    match self.props.data.representation_type.representation_type_id == x.representation_type_id {
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
            <a id={ format!("btn-change-represent-{}", &self.props.data.uuid) }
                class="button"
                onclick=onclick_change_represent>
                { "Change" }
            </a>
        </>}
    }

    fn show_btn_delete(
        &self,
    ) -> Html {
        let onclick_delete_represent = self
            .link
            .callback(|_| Msg::RequestDeleteRepresent);

        html!{<a id={ format!(
            "btn-delete-represent-{}", &self.props.data.uuid) }
            class="button"
            onclick=onclick_delete_represent>
            { "Delete" }
        </a>}
    }
}
