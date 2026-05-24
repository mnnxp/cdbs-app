use yew::{html, Callback, ChangeData, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::modal::ModalBlock;
use crate::fragments::notification::show_notification;
use crate::services::{get_value_field, resp_parsing, unique_id};
use crate::fragments::buttons::ft_add_btn;
use crate::types::{UUID, CompanyRepresentInfo, Region, RegisterCompanyRepresentInfo, RepresentationType};
use crate::gqls::make_query;
use crate::gqls::company::{
    RegisterCompanyRepresent, register_company_represent,
};

use super::represent_form::{render_represent_form, FormCallbacks};

pub(crate) enum Msg {
    ShowAddCompanyRepresent,
    RequestRegisterRepresent,
    GetRegisterResult(String),
    UpdateRegionId(String),
    UpdateRepresentationTypeId(String),
    UpdateName(String),
    UpdateAddress(String),
    UpdatePhone(String),
    ResponseError(Error),
    ClearData,
    ClearError,
}

pub(crate) struct AddCompanyRepresentModal {
    error: Option<Error>,
    request_register: RegisterCompanyRepresentInfo,
    props: Props,
    link: ComponentLink<Self>,
    get_result_register: UUID,
    is_active: bool,
    loading: bool,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) regions: Vec<Region>,
    pub(crate) represent_types: Vec<RepresentationType>,
    pub(crate) on_add: Callback<CompanyRepresentInfo>,
}

impl Component for AddCompanyRepresentModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_register: RegisterCompanyRepresentInfo::default(),
            props,
            link,
            get_result_register: String::new(),
            is_active: false,
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ShowAddCompanyRepresent => {
                self.is_active = !self.is_active;
                if !self.is_active {
                    self.link.send_message(Msg::ClearData);
                }
            },
            Msg::RequestRegisterRepresent => {
                self.loading = true;
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
                });
            },
            Msg::GetRegisterResult(res) => {
                match resp_parsing(res, "registerCompanyRepresent") {
                    Ok(result) => {
                        self.get_result_register = result;
                        let selected_region = self.props.regions
                            .iter()
                            .find(|r| r.region_id == self.request_register.region_id)
                            .cloned()
                            .unwrap_or_default();
                        let selected_type = self.props.represent_types
                            .iter()
                            .find(|t| t.representation_type_id == self.request_register.representation_type_id)
                            .cloned()
                            .unwrap_or_default();
                        let new_represent_info = self.request_register.to_info(
                            self.get_result_register.clone(),
                            self.props.company_uuid.clone(),
                            selected_region,
                            selected_type,
                        );
                        debug!("Represent data: {:?}", new_represent_info);
                        self.props.on_add.emit(new_represent_info);
                        // clear flags and data
                        self.is_active = false;
                        self.request_register = RegisterCompanyRepresentInfo::default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.loading = false;
            },
            Msg::UpdateRegionId(region_id) =>
                self.request_register.region_id = region_id.parse::<usize>().unwrap_or_default(),
            Msg::UpdateRepresentationTypeId(representation_type_id) =>
                self.request_register.representation_type_id = representation_type_id.parse::<usize>().unwrap_or_default(),
            Msg::UpdateName(name) => self.request_register.name = name,
            Msg::UpdateAddress(address) => self.request_register.address = address,
            Msg::UpdatePhone(phone) => self.request_register.phone = phone,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearData => {
                self.error = None;
                self.is_active = false;
                self.loading = false;
                self.request_register = RegisterCompanyRepresentInfo::default();
                self.get_result_register.clear();
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let callback_event_add_represent = self.link.callback(|_| Msg::ShowAddCompanyRepresent);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
            {show_notification(
                get_value_field(&293),
                "is-success",
                !self.get_result_register.is_empty(),
            )}
            {ft_add_btn(
                &unique_id("new-represent-btn"),
                get_value_field(&230),
                callback_event_add_represent,
                false,
                self.props.represent_types.is_empty(),
            )}
            {self.modal_card()}
        </>}
    }
}

impl AddCompanyRepresentModal {
    fn modal_card(&self) -> Html {
        html! {
            <ModalBlock
                modal_id="new-represent"
                title={get_value_field(&230)}
                is_active={self.is_active}
                on_close={self.link.callback(|_| Msg::ShowAddCompanyRepresent)}
                on_cancel={Some(self.link.callback(|_| Msg::ClearData))}
                on_save={Some(self.link.callback(|_| Msg::RequestRegisterRepresent))}
                save_disabled={false}
            >
                { self.new_represent_block() }
            </ModalBlock>
        }
    }

    fn new_represent_block(&self) -> Html {
        let onchange_region =
            self.link.callback(|ev: ChangeData| Msg::UpdateRegionId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            }));
        let onchange_type =
            self.link.callback(|ev: ChangeData| Msg::UpdateRepresentationTypeId(match ev {
                ChangeData::Select(el) => el.value(),
                _ => "1".to_string(),
            }));
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_address = self.link.callback(|ev: InputData| Msg::UpdateAddress(ev.value));
        let oninput_phone = self.link.callback(|ev: InputData| Msg::UpdatePhone(ev.value));

        let callbacks = FormCallbacks {
            oninput_name,
            oninput_phone,
            oninput_address,
            onchange_region,
            onchange_type,
        };

        render_represent_form(
            &self.request_register,
            callbacks,
            &self.props.regions,
            self.request_register.region_id,
            &self.props.represent_types,
            self.request_register.representation_type_id,
            self.loading,
        )
    }
}
