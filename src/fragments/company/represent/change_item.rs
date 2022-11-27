use yew::{Component, Callback, Context, html, html::Scope, Html, Properties, Event};
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::services::{is_authenticated, get_value_field};
use crate::fragments::list_errors::ListErrors;
use crate::error::{Error, get_error};
use crate::types::{UUID, Region, RepresentationType, CompanyRepresentInfo, CompanyRepresentUpdateInfo};
use crate::gqls::make_query;
use crate::gqls::company::{
    GetRepresentDataOpt, get_represent_data_opt,
    UpdateCompanyRepresent, update_company_represent,
    DeleteCompanyRepresent, delete_company_represent,
};

pub enum Msg {
    RequestUpdateRepresent,
    RequestDeleteRepresent,
    GetUpdateResult(String),
    GetDeleteRepresentResult(String),
    UpdateRegionId(String),
    UpdateRepresentationTypeId(String),
    UpdateName(String),
    UpdateAddress(String),
    UpdatePhone(String),
    UpdateList(String),
    ClearError,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub data: CompanyRepresentInfo,
}

pub struct ChangeItem {
    error: Option<Error>,
    company_uuid: UUID,
    company_represent_uuid: UUID,
    request_update: CompanyRepresentUpdateInfo,
    get_result_update: usize,
    regions: Vec<Region>,
    represent_types: Vec<RepresentationType>,
    get_result_delete: bool,
}

impl Component for ChangeItem {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            company_uuid: ctx.props().data.company_uuid.clone(),
            company_represent_uuid: ctx.props().data.uuid.clone(),
            request_update: CompanyRepresentUpdateInfo::default(),
            get_result_update: 0,
            regions: Vec::new(),
            represent_types: Vec::new(),
            get_result_delete: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let link = ctx.link().clone();

        if first_render && is_authenticated() && !self.company_uuid.is_empty() {
            self.request_update = CompanyRepresentUpdateInfo {
                region_id: Some(ctx.props().data.region.region_id as i64),
                representation_type_id: Some(ctx.props().data.representation_type.representation_type_id as i64),
                name: Some(ctx.props().data.name.clone()),
                address: Some(ctx.props().data.address.clone()),
                phone: Some(ctx.props().data.phone.clone()),
            };

            spawn_local(async move {
                let res = make_query(GetRepresentDataOpt::build_query(
                    get_represent_data_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::UpdateList(res));
            })
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestUpdateRepresent => {
                debug!("Update company represent: {:?}", &self.request_update);
                let company_uuid = self.company_uuid.clone();
                let company_represent_uuid = self.company_represent_uuid.clone();
                let ipt_update_company_represent_data = update_company_represent::IptUpdateCompanyRepresentData {
                    regionId: self.request_update.region_id.clone(),
                    representationTypeId: self.request_update.representation_type_id.clone(),
                    name: self.request_update.name.clone(),
                    address: self.request_update.address.clone(),
                    phone: self.request_update.phone.clone(),
                };
                spawn_local(async move {
                    let res = make_query(UpdateCompanyRepresent::build_query(
                        update_company_represent::Variables {
                            company_uuid,
                            company_represent_uuid,
                            ipt_update_company_represent_data,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateResult(res));
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
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteRepresentResult(res));
                })
            },
            Msg::GetUpdateResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.get_result_update = serde_json::from_value(
                            res_value.get("updateCompanyRepresent").unwrap().clone()
                        ).unwrap();
                        debug!("Update company represent: {:?}", self.get_result_update);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetDeleteRepresentResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.get_result_delete = serde_json::from_value(
                            res_value.get("deleteCompanyRepresent").unwrap().clone()
                        ).unwrap();
                        debug!("Delete company represent: {:?}", self.get_result_delete);
                    },
                    true => self.error = Some(get_error(&data)),
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
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <br/>
            <div class="card">
                <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error)} />
                {match &self.get_result_delete {
                    true => html!{
                        <article class="message is-success">
                          <div class="message-header">
                            <p>{ get_value_field(&89) }</p>
                          </div>
                          <div class="message-body">
                            { get_value_field(&292) }
                          </div>
                        </article>
                    },
                    false => html!{<div class="column">
                        <label class="label">{ get_value_field(&215) }</label> // Change represent
                        {if self.get_result_update > 0 {
                            html!{
                                <span id="tag-info-update-represent" class="tag is-info is-light">
                                    // Data updated! Change rows:
                                    {format!("{} {}", get_value_field(&213), self.get_result_update)}
                                </span>
                            }
                        } else { html!{} }}
                        {self.change_represent_block(ctx.link(), ctx.props())}
                        {self.show_manage_buttons(ctx.link())}
                    </div>}
                }}
            </div>
        </>}
    }
}

impl ChangeItem {
    fn fileset_generator(
        &self,
        id: &str,
        label: &str,
        // placeholder: &str,
        value: String,
        oninput: Callback<Event>,
    ) -> Html {
        let placeholder = label;
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
                    oninput={oninput} ></@>
            </fieldset>
        }
    }

    fn change_represent_block(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let oninput_region_id =
            link.callback(|ev: Event| Msg::UpdateRegionId(match ev {
              Event::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let oninput_representation_type_id =
            link.callback(|ev: Event| Msg::UpdateRepresentationTypeId(match ev {
              Event::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let oninput_name = link.callback(|ev: Event| Msg::UpdateName(ev.value));
        let oninput_address = link.callback(|ev: Event| Msg::UpdateAddress(ev.value));
        let oninput_phone = link.callback(|ev: Event| Msg::UpdatePhone(ev.value));

        html!{<>
            {self.fileset_generator(
                "name", get_value_field(&110), // Name
                self.request_update.name.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                oninput_name
            )}
            // <div class="column">
            // </div>

            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "phone", get_value_field(&56), // Phone
                        self.request_update.phone.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_phone
                    )}
                </div>
                <div class="column">
                    <fieldset class="field">
                        <label class="label">{ get_value_field(&216) }</label> // Representation type
                        <div class="control">
                            <div class="select">
                              <select
                                  id="representation_type_id"
                                  select={props.data.representation_type.representation_type_id.to_string()}
                                  onchange={oninput_representation_type_id}
                                  >
                                { for self.represent_types.iter().map(|x|
                                    html!{
                                        <option value={x.representation_type_id.to_string()}
                                              selected={x.representation_type_id == props.data.representation_type.representation_type_id} >
                                            {&x.representation_type}
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
                <fieldset class="field">
                    <label class="label">{ get_value_field(&27) }</label> // Region
                    <div class="control">
                        <div class="select">
                          <select
                              id="region_id"
                              select={props.data.region.region_id.to_string()}
                              onchange={oninput_region_id}
                              >
                            { for self.regions.iter().map(|x|
                                html!{
                                    <option value={x.region_id.to_string()}
                                          selected={x.region_id == props.data.region.region_id} >
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
                        self.request_update.address.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                        oninput_address
                    )}
                </div>
            </div>
        </>}
    }

    fn show_manage_buttons(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_change_represent = link.callback(|_| Msg::RequestUpdateRepresent);
        let onclick_delete_represent = link.callback(|_| Msg::RequestDeleteRepresent);

        html!{<div class="columns">
            <div class="column">
                <button id={"btn-delete-represent"}
                    class="button is-danger is-fullwidth"
                    onclick={onclick_delete_represent}>
                    { get_value_field(&135) }
                </button>
            </div>
            <div class="column">
                <button id={"btn-change-represent"}
                    class="button is-fullwidth"
                    onclick={onclick_change_represent}>
                    { get_value_field(&46) }
                </button>
            </div>
        </div>}
    }
}
