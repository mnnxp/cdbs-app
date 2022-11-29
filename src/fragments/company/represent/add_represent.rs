use yew::{Component, Callback, Context, html, html::Scope, Html, Properties, Event};
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::services::{is_authenticated, get_value_field};
use crate::fragments::list_errors::ListErrors;
use crate::error::{Error, get_error};
use crate::types::{UUID, Region, RepresentationType, RegisterCompanyRepresentInfo};
use crate::gqls::make_query;
use crate::gqls::company::{
    GetRepresentDataOpt, get_represent_data_opt,
    RegisterCompanyRepresent, register_company_represent,
};

pub enum Msg {
    RequestRegisterRepresent,
    GetRegisterResult(String),
    UpdateRegionId(String),
    UpdateRepresentationTypeId(String),
    UpdateName(String),
    UpdateAddress(String),
    UpdatePhone(String),
    UpdateList(String),
    ClearData,
    ClearError,
}

pub struct AddCompanyRepresentCard {
    error: Option<Error>,
    request_register: RegisterCompanyRepresentInfo,
    regions: Vec<Region>,
    represent_types: Vec<RepresentationType>,
    get_result_register: bool,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub company_uuid: UUID,
}

impl Component for AddCompanyRepresentCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            request_register: RegisterCompanyRepresentInfo::default(),
            regions: Vec::new(),
            represent_types: Vec::new(),
            get_result_register: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let link = ctx.link().clone();

        if first_render && is_authenticated() {
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
            Msg::RequestRegisterRepresent => {
                debug!("Register company represent: {:?}", &self.request_register);
                let ipt_company_represent_data = register_company_represent::IptCompanyRepresentData {
                    company_uuid: ctx.props().company_uuid.clone(),
                    region_id: self.request_register.region_id as i64,
                    representation_type_id: self.request_register.representation_type_id as i64,
                    name: self.request_register.name.clone(),
                    address: self.request_register.address.clone(),
                    phone: self.request_register.phone.clone(),
                };
                spawn_local(async move {
                    let res = make_query(RegisterCompanyRepresent::build_query(
                        register_company_represent::Variables { ipt_company_represent_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetRegisterResult(res));
                })
            },
            Msg::GetRegisterResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res_value.get("registerCompanyRepresent").unwrap().clone()).unwrap();
                        debug!("Register company represent: {:?}", result);
                        self.get_result_register = result;
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateRegionId(region_id) =>
                self.request_register.region_id = region_id.parse::<usize>().unwrap_or_default(),
            Msg::UpdateRepresentationTypeId(representation_type_id) =>
                self.request_register.representation_type_id = representation_type_id.parse::<usize>().unwrap_or_default(),
            Msg::UpdateName(name) => self.request_register.name = name,
            Msg::UpdateAddress(address) => self.request_register.address = address,
            Msg::UpdatePhone(phone) => self.request_register.phone = phone,
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
            Msg::ClearData => {
                self.error = None;
                self.request_register = RegisterCompanyRepresentInfo::default();
                self.get_result_register = false;
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
        let onclick_hide_notification = ctx.link().callback(|_| Msg::ClearData);

        html!{<div class="card">
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            {match &self.get_result_register {
                true => html!{
                    <article class="message is-success">
                      <div class="message-header">
                        <p>{ get_value_field(&89) }</p>
                        <button class="delete" aria-label="close" onclick={onclick_hide_notification.clone()} />
                      </div>
                      <div class="message-body">
                        { get_value_field(&293) }
                      </div>
                    </article>
                },
                false => html!{<div class="column">
                    <label class="label">{ get_value_field(&230) }</label> // New represent
                    {self.new_represent_block(ctx.link())}
                    {self.show_manage_buttons(ctx.link())}
                </div>}
            }}
        </div>}
    }
}

impl AddCompanyRepresentCard {
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

    fn new_represent_block(
        &self,
        link: &Scope<Self>,
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
                self.request_register.name.clone(),
                oninput_name
            )}
            // <div class="column">
            // </div>

            <div class="columns">
                <div class="column">
                    {self.fileset_generator(
                        "phone", get_value_field(&56), // Phone
                        self.request_register.phone.clone(),
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
                                  select={self.request_register.representation_type_id.to_string()}
                                  onchange={oninput_representation_type_id}
                                  >
                                { for self.represent_types.iter().map(|x|
                                    html!{
                                        <option value={x.representation_type_id.to_string()}
                                              selected={x.representation_type_id == self.request_register.representation_type_id} >
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
                                  select={self.request_register.region_id.to_string()}
                                  onchange={oninput_region_id}
                                  >
                                { for self.regions.iter().map(|x|
                                    html!{
                                        <option value={x.region_id.to_string()}
                                              selected={x.region_id == self.request_register.region_id} >
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
                        self.request_register.address.clone(),
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
        let onclick_clear_data = link.callback(|_| Msg::ClearData);

        let onclick_create_represent = link.callback(|_| Msg::RequestRegisterRepresent);

        html!{<div class="columns">
            <div class="column">
                <button id={"btn-clear-represent"}
                    class="button is-fullwidth"
                    onclick={onclick_clear_data}>
                    { get_value_field(&88) }
                </button>
            </div>
            <div class="column">
                <button id={"btn-new-represent"}
                    class="button is-success is-fullwidth"
                    onclick={onclick_create_represent}>
                    { get_value_field(&45) } // Create
                </button>
            </div>
        </div>}
    }
}
