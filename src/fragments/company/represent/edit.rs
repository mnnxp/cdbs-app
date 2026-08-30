use yew::{classes, html, Callback, ChangeData, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::modal::ModalBlock;
use crate::services::{LocaleKey, resp_parsing};
use crate::fragments::buttons::ft_custom_btn;
use crate::fragments::notification::show_notification;
use crate::types::{Region, RepresentationType, CompanyRepresentInfo, CompanyRepresentUpdateInfo};
use crate::gqls::make_query;
use crate::gqls::company::{
    UpdateCompanyRepresent, update_company_represent,
};

use super::represent_form::{render_represent_form, FormCallbacks};

pub(crate) enum Msg {
    ShowEditCompanyRepresent,
    PrepareData,
    RequestUpdateRepresent,
    GetUpdateResult(String),
    UpdateRegionId(String),
    UpdateRepresentationTypeId(String),
    UpdateName(String),
    UpdateAddress(String),
    UpdatePhone(String),
    ResponseError(Error),
    ClearData,
    ClearError,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub(crate) struct Props {
    pub(crate) data: CompanyRepresentInfo,
    pub(crate) regions: Vec<Region>,
    pub(crate) represent_types: Vec<RepresentationType>,
    pub(crate) on_update: Callback<CompanyRepresentInfo>,
}

pub(crate) struct EditCompanyRepresentModal {
    error: Option<Error>,
    request_update: CompanyRepresentUpdateInfo,
    props: Props,
    link: ComponentLink<Self>,
    get_result_update: usize,
    is_active: bool,
    loading: bool,
}

impl Component for EditCompanyRepresentModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_update: CompanyRepresentUpdateInfo::default(),
            props,
            link,
            get_result_update: 0,
            is_active: false,
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::ShowEditCompanyRepresent => {
                self.is_active = !self.is_active;
                if self.is_active {
                    self.link.send_message(Msg::PrepareData);
                }
            },
            Msg::PrepareData => {
                debug!("Prepare represent data: {:?}", self.props.data.name);
                self.request_update = CompanyRepresentUpdateInfo {
                    region_id: Some(self.props.data.region.region_id as i64),
                    representation_type_id: Some(self.props.data.representation_type.representation_type_id as i64),
                    name: Some(self.props.data.name.clone()),
                    address: Some(self.props.data.address.clone()),
                    phone: Some(self.props.data.phone.clone()),
                };
            },
            Msg::RequestUpdateRepresent => {
                self.loading = true;
                debug!("Update company represent: {:?}", &self.request_update);
                let company_uuid = self.props.data.company_uuid.clone();
                let company_represent_uuid = self.props.data.uuid.clone();
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
            Msg::GetUpdateResult(res) => {
                match resp_parsing(res, "updateCompanyRepresent") {
                    Ok(result) => {
                        self.get_result_update = result;
                        if self.get_result_update <= 0 {
                            return true
                        }
                        let data_region_id = self.request_update.region_id.unwrap_or(self.props.data.region.region_id as i64);
                        let data_type_id = self.request_update.representation_type_id.unwrap_or(self.props.data.representation_type.representation_type_id as i64);
                        let new_region = self.props.regions
                            .iter()
                            .find(|r| r.region_id == data_region_id as usize)
                            .cloned()
                            .unwrap_or_default();
                        let new_type = self.props.represent_types
                            .iter()
                            .find(|t| t.representation_type_id == data_type_id as usize)
                            .cloned()
                            .unwrap_or_default();
                        let updated_info = self.request_update.to_info(
                            self.props.data.uuid.clone(),
                            self.props.data.company_uuid.clone(),
                            new_region,
                            new_type,
                            self.props.data.name.clone(),
                            self.props.data.address.clone(),
                            self.props.data.phone.clone(),
                        );
                        self.props.on_update.emit(updated_info);
                        self.is_active = false;
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.loading = false;
                debug!("Update company represent: {:?}", self.get_result_update);
            },
            Msg::UpdateRegionId(region_id) =>
                self.request_update.region_id = Some(region_id.parse::<i64>().unwrap_or_default()),
            Msg::UpdateRepresentationTypeId(representation_type_id) =>
                self.request_update.representation_type_id = Some(representation_type_id.parse::<i64>().unwrap_or_default()),
            Msg::UpdateName(name) => self.request_update.name = Some(name),
            Msg::UpdateAddress(address) => self.request_update.address = Some(address),
            Msg::UpdatePhone(phone) => self.request_update.phone = Some(phone),
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearData => {
                self.error = None;
                self.is_active = false;
                self.get_result_update = 0;
                self.loading = false;
                self.request_update = CompanyRepresentUpdateInfo::default();
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
        let callback_event_edit_represent = self.link.callback(|_| Msg::ShowEditCompanyRepresent);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
            {show_notification(
                &format!("{} {}", LocaleKey::DataUpdatedChangeRows.get_value(), self.get_result_update),
                "is-success",
                self.get_result_update > 0,
            )}
            {ft_custom_btn(
                "edit-represent-btn",
                LocaleKey::Edit.get_value(),
                classes!("button", "is-info", "is-light", "is-fullwidth", "is-small"),
                "fas fa-pencil-alt",
                callback_event_edit_represent,
                false
            )}
            {self.modal_card()}
        </>}
    }
}

impl EditCompanyRepresentModal {
    fn modal_card(&self) -> Html {
        html! {
            <ModalBlock
                modal_id="edit-represent"
                title={LocaleKey::ChangeRepresent.get_value()}
                is_active={self.is_active}
                on_close={self.link.callback(|_| Msg::ShowEditCompanyRepresent)}
                on_cancel={self.link.callback(|_| Msg::ClearData)}
                on_save={Some(self.link.callback(|_| Msg::RequestUpdateRepresent))}
                save_disabled={self.loading}
            >
                { self.edit_represent_block() }
            </ModalBlock>
        }
    }

    fn edit_represent_block(&self) -> Html {
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
            &self.request_update,
            callbacks,
            &self.props.regions,
            self.props.data.region.region_id,
            &self.props.represent_types,
            self.props.data.representation_type.representation_type_id,
            self.loading,
        )
    }
}