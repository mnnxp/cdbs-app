use log::debug;
use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::company::represent::edit::EditCompanyRepresentModal;
use crate::services::{resp_parsing, unique_id};
use crate::fragments::buttons::ft_delete_pair_btn;
use crate::fragments::notification::show_notification;
use crate::types::{UUID, CompanyRepresentInfo, Region, RepresentationType};
use crate::services::get_value_field;
use crate::gqls::make_query;
use crate::gqls::company::{
    DeleteCompanyRepresent, delete_company_represent,
};

#[derive(Clone, Debug, Properties, PartialEq)]
pub(crate) struct Props {
    pub(crate) data: CompanyRepresentInfo,
    pub(crate) regions: Vec<Region>,
    pub(crate) represent_types: Vec<RepresentationType>,
    pub(crate) show_list: bool,
    pub(crate) show_manage: bool,
    pub(crate) on_update: Callback<CompanyRepresentInfo>,
    pub(crate) on_delete: Callback<UUID>,
}

pub(crate) enum Msg {
    RequestDeleteRepresent(bool),
    GetDeleteRepresentResult(String),
    ResponseError(Error),
    ClearError,
}

pub(crate) struct ListItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    get_confirm: String,
    is_deleted: bool,
    loading: bool,
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            get_confirm: String::new(),
            is_deleted: false,
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestDeleteRepresent(confirm) => {
                debug!("Request delete for uuid: {:?}", self.props.data.uuid);
                if !confirm || self.props.data.uuid.is_empty() || self.props.data.company_uuid.is_empty() {
                    self.get_confirm.clear();
                    return true
                }
                if self.get_confirm != self.props.data.uuid {
                    self.get_confirm = self.props.data.uuid.clone();
                    return true
                }
                self.loading = true;
                let company_uuid = self.props.data.company_uuid.clone();
                let company_represent_uuid = self.props.data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteCompanyRepresent::build_query(
                        delete_company_represent::Variables {
                            company_uuid,
                            company_represent_uuid,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteRepresentResult(res));
                });
            },
            Msg::GetDeleteRepresentResult(res) => {
                match resp_parsing(res, "deleteCompanyRepresent") {
                    Ok(result) => {
                        self.is_deleted = result;
                        if self.is_deleted {
                            self.props.on_delete.emit(self.props.data.uuid.clone());
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.loading = false;
            },
            Msg::ResponseError(err) => self.error = Some(err),
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
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
            {show_notification(
                get_value_field(&292),
                "is-success",
                self.is_deleted,
            )}
            {match self.props.show_list {
                true => self.showing_in_list(),
                false => self.showing_in_box(),
            }}
        </>}
    }
}

impl ListItem {
    fn showing_in_list(&self) -> Html {
        let CompanyRepresentInfo {
            region,
            representation_type,
            name,
            address,
            phone,
            ..
        } = &self.props.data;

        let text_column_class = match self.props.show_manage {
            true => "column is-8-tablet is-12-mobile",
            false => "column is-12",
        };

        html!{
            <div class="box p-3 mb-3">
                <div class="columns is-vcentered is-mobile is-multiline">
                    <div class={text_column_class}>
                        <div class="block mb-1">
                            <span class="title is-6 has-text-dark mr-3">{name}</span>
                            <span class="tag is-light is-small">{representation_type.representation_type.clone()}</span>
                        </div>
                        {match address.is_empty() {
                            true => html!{},
                            false => html!{
                                <div class="is-size-7 has-text-weight-semibold has-text-grey-dark mb-1">
                                    <span class="icon is-small mr-1"><i class="fas fa-map-marker-alt"></i></span>
                                    <span class="mr-2">{address}</span>
                                </div>
                            },
                        }}
                        <div class="is-flex is-flex-wrap-wrap is-gap-3 has-text-grey is-size-7 pt-1">
                            <span class="is-inline-block">
                                <span class="icon is-small mr-1"><i class="fas fa-globe-africa"></i></span>
                                <span class="mr-2">{format!("{}: {}", get_value_field(&27), region.region)}</span>
                            </span>
                            {match phone.is_empty() {
                                true => html!{},
                                false => html!{
                                    <span class="is-inline-block">
                                        <span class="icon is-small mr-1"><i class="fas fa-phone-alt"></i></span>
                                        <span class="mr-2">{phone}</span>
                                    </span>
                                },
                            }}
                        </div>
                    </div>
                    {match self.props.show_manage {
                        true => self.show_actions(),
                        false => html!{},
                    }}
                </div>
            </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let CompanyRepresentInfo {
            region,
            name,
            address,
            phone,
            ..
        } = &self.props.data;

        html!{
            <div class="column is-12-mobile is-6-tablet is-4-desktop is-3-widescreen">
            <div class="card">
                <div class="card-content p-4">
                    <div class="mb-1">
                        <h3 class="title is-6 overflow-title" title={name.clone()}>{name}</h3>
                    </div>
                    <div class="is-size-7 has-text-grey">
                        <div class="mb-1 overflow-title">
                            <span class="icon is-small mr-1"><i class="fas fa-globe"></i></span>
                            <span title={region.region.clone()}>{region.region.clone()}</span>
                        </div>
                        {match address.is_empty() {
                            true => html!{},
                            false => html!{
                                <div class="mb-1 overflow-title">
                                    <span class="icon is-small mr-1"><i class="fas fa-map-marker-alt"></i></span>
                                    <span title={address.clone()}>{address}</span>
                                </div>
                            },
                        }}
                        {match phone.is_empty() {
                            true => html!{},
                            false => html!{
                                <div class="overflow-title">
                                    <span class="icon is-small mr-1"><i class="fas fa-phone-alt"></i></span>
                                    <span title={phone.clone()}>{phone}</span>
                                </div>
                            },
                        }}
                    </div>
                    {match self.props.show_manage {
                        true => self.show_actions(),
                        false => html!{},
                    }}
                </div>
            </div>
            </div>
        }
    }

    fn show_actions(&self) -> Html {
        let callback_delete_represent = self.link.callback(|confirm| Msg::RequestDeleteRepresent(confirm));
        html!{
            {match self.props.show_list {
                true => html!{
                    <div class="column is-4-tablet is-12-mobile has-text-right-tablet">
                        <div class="buttons is-inline-block mb-0">
                            <EditCompanyRepresentModal
                                data={self.props.data.clone()}
                                regions={self.props.regions.clone()}
                                represent_types={self.props.represent_types.clone()}
                                on_update={self.props.on_update.clone()}
                            />
                            {ft_delete_pair_btn(
                                &unique_id("delete-represent-btn"),
                                callback_delete_represent,
                                self.get_confirm == self.props.data.uuid,
                                self.loading,
                                classes!("is-small"),
                            )}
                        </div>
                    </div>
                },
                false => html!{
                    <div class="columns mt-1">
                            <EditCompanyRepresentModal
                                data={self.props.data.clone()}
                                regions={self.props.regions.clone()}
                                represent_types={self.props.represent_types.clone()}
                                on_update={self.props.on_update.clone()}
                            />
                            {ft_delete_pair_btn(
                                &unique_id("delete-represent-btn"),
                                callback_delete_represent,
                                self.get_confirm == self.props.data.uuid,
                                self.loading,
                                classes!("is-small"),
                            )}
                    </div>
                },
            }}
        }
    }
}