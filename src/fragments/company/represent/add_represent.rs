use yew::{html, Component, Callback, ComponentLink, Html, InputData, ChangeData, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::services::{is_authenticated, get_value_field, resp_parsing, get_value_response, get_from_value};
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_create_btn;
use crate::error::Error;
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
    ResponseError(Error),
    ClearData,
    ClearError,
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
                let ipt_company_represent_data = register_company_represent::IptCompanyRepresentData {
                    companyUuid: self.props.company_uuid.clone(),
                    regionId: self.request_register.region_id as i64,
                    representationTypeId: self.request_register.representation_type_id as i64,
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
                match resp_parsing(res, "registerCompanyRepresent") {
                    Ok(result) => self.get_result_register = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("Register company represent: {:?}", self.get_result_register);
            },
            Msg::UpdateRegionId(region_id) =>
                self.request_register.region_id = region_id.parse::<usize>().unwrap_or_default(),
            Msg::UpdateRepresentationTypeId(representation_type_id) =>
                self.request_register.representation_type_id = representation_type_id.parse::<usize>().unwrap_or_default(),
            Msg::UpdateName(name) => self.request_register.name = name,
            Msg::UpdateAddress(address) => self.request_register.address = address,
            Msg::UpdatePhone(phone) => self.request_register.phone = phone,
            Msg::UpdateList(res) => {
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.regions = get_from_value(value, "regions").unwrap_or_default();
                        self.represent_types = get_from_value(value, "companyRepresentTypes").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearData => {
                self.error = None;
                self.request_register = RegisterCompanyRepresentInfo::default();
                self.get_result_register = false;
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props = props;
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_hide_notification = self.link.callback(|_| Msg::ClearData);

        html!{<div class="card">
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {match &self.get_result_register {
                true => html!{
                    <article class="message is-success">
                      <div class="message-header">
                        <p>{ get_value_field(&89) }</p>
                        <button class="delete" aria-label="close" onclick=onclick_hide_notification.clone() />
                      </div>
                      <div class="message-body">
                        { get_value_field(&293) }
                      </div>
                    </article>
                },
                false => html!{<div class="column">
                    <label class="label">{ get_value_field(&230) }</label> // New represent
                    {self.new_represent_block()}
                    {self.show_manage_buttons()}
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
        oninput: Callback<InputData>,
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
                    oninput=oninput ></@>
            </fieldset>
        }
    }

    fn new_represent_block(&self) -> Html {
        let oninput_region_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateRegionId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let oninput_representation_type_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateRepresentationTypeId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_address = self.link.callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_phone = self.link.callback(|ev: InputData| Msg::UpdatePhone(ev.value));

        html!{<>
            {self.fileset_generator(
                "name", get_value_field(&110), // Name
                self.request_register.name.clone(),
                oninput_name
            )}
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
                                  onchange=oninput_representation_type_id
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
                                  onchange=oninput_region_id
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

    fn show_manage_buttons(&self) -> Html {
        let onclick_clear_data = self.link.callback(|_| Msg::ClearData);
        let onclick_create_represent = self.link.callback(|_| Msg::RequestRegisterRepresent);

        html!{<div class="columns">
            <div class="column">
                <button id={"btn-clear-represent"}
                    class="button is-fullwidth is-warning"
                    onclick=onclick_clear_data>
                    { get_value_field(&88) }
                </button>
            </div>
            <div class="column">
                {ft_create_btn(
                    "btn-new-represent",
                    "".into(),
                    onclick_create_represent,
                    false,
                )}
            </div>
        </div>}
    }
}
