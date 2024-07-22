use yew::{html, Component, Callback, ComponentLink, Html, InputData, ChangeData, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::services::{is_authenticated, get_value_field, resp_parsing, get_value_response, get_from_value};
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::{ft_delete_btn, ft_save_btn};
use crate::error::Error;
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
    ResponseError(Error),
    ClearError,
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
    get_confirm: UUID,
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
            get_confirm: String::new(),
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
                if self.get_confirm == self.company_represent_uuid {
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
                } else {
                    self.get_confirm = self.company_represent_uuid.clone();
                }
            },
            Msg::GetUpdateResult(res) => {
                match resp_parsing(res, "updateCompanyRepresent") {
                    Ok(result) => self.get_result_update = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("Update company represent: {:?}", self.get_result_update);
            },
            Msg::GetDeleteRepresentResult(res) => {
                match resp_parsing(res, "deleteCompanyRepresent") {
                    Ok(result) => self.get_result_delete = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("Delete company represent: {:?}", self.get_result_delete);
            },
            Msg::UpdateRegionId(region_id) =>
                self.request_update.region_id = Some(region_id.parse::<i64>().unwrap_or_default()),
            Msg::UpdateRepresentationTypeId(representation_type_id) =>
                self.request_update.representation_type_id = Some(representation_type_id.parse::<i64>().unwrap_or_default()),
            Msg::UpdateName(name) => self.request_update.name = Some(name),
            Msg::UpdateAddress(address) => self.request_update.address = Some(address),
            Msg::UpdatePhone(phone) => self.request_update.phone = Some(phone),
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

        html!{<>
            <br/>
            <div class="card">
                <ListErrors error=self.error.clone() clear_error=onclick_clear_error />
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
                        {self.change_represent_block()}
                        {self.show_manage_buttons()}
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
        value: String,
        oninput: Callback<InputData>,
    ) -> Html {
        let placeholder = label;
        let input_tag = "input";

        html!{
            <fieldset class="field">
                <label class="label">{label.to_string()}</label>
                <@{input_tag}
                    id={id.to_string()}
                    class={input_tag}
                    type={"text"}
                    placeholder={placeholder.to_string()}
                    value={value}
                    oninput=oninput ></@>
            </fieldset>
        }
    }

    fn change_represent_block(&self) -> Html {
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
                self.request_update.name.as_ref().map(|x| x.to_string()).unwrap_or_default(),
                oninput_name
            )}
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
                                  select={self.props.data.representation_type.representation_type_id.to_string()}
                                  onchange=oninput_representation_type_id
                                  >
                                { for self.represent_types.iter().map(|x|
                                    html!{
                                        <option value={x.representation_type_id.to_string()}
                                              selected={x.representation_type_id == self.props.data.representation_type.representation_type_id} >
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
                              select={self.props.data.region.region_id.to_string()}
                              onchange=oninput_region_id
                              >
                            { for self.regions.iter().map(|x|
                                html!{
                                    <option value={x.region_id.to_string()}
                                          selected={x.region_id == self.props.data.region.region_id} >
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

    fn show_manage_buttons(&self) -> Html {
        let onclick_change_represent = self.link.callback(|_| Msg::RequestUpdateRepresent);
        let onclick_delete_represent = self.link.callback(|_| Msg::RequestDeleteRepresent);

        html!{<div class="columns">
            <div class="column">
                {ft_delete_btn(
                    "btn-delete-represent",
                    onclick_delete_represent,
                    self.get_confirm == self.company_represent_uuid,
                    false
                )}
            </div>
            <div class="column">
                {ft_save_btn(
                    "btn-change-represent",
                    onclick_change_represent,
                    true,
                    false
                )}
            </div>
        </div>}
    }
}
