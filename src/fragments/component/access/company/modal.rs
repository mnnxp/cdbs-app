use yew::{html, Component, Callback, ComponentLink, Html, Properties, ShouldRender, InputData, classes};
use yew::services::timeout::{TimeoutService, TimeoutTask};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use std::time::Duration;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::{ft_cancel_btn, ft_save_btn};
use crate::fragments::permission::PermissionLevelBlock;
use crate::services::{get_value_field, resp_parsing, truncate_uuid};
use crate::types::{CompanySearchResult, PermissionLevel, UUID};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    SearchCompaniesForAccess, search_companies_for_access,
    SetCompanyAccessComponent, set_company_access_component,
};

/// Add company access modal component
pub(crate) struct AddCompanyAccessModal {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    search_text: String,
    search_results: Vec<CompanySearchResult>,
    selected_company_uuid: Option<UUID>,
    selected_level: usize,
    debounce_timeout: Option<TimeoutTask>,
    search_loading: bool,
    adding: bool,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) component_uuid: UUID,
    pub(crate) existing_company_uuids: Vec<UUID>,
    pub(crate) permissions: Vec<PermissionLevel>,
    pub(crate) is_active: bool,
    pub(crate) on_close: Callback<()>,
    pub(crate) on_success: Callback<()>,
}

pub(crate) enum Msg {
    Open,
    Close,
    UpdateSearch(String),
    SearchCompanies,
    SearchResults(String),
    SelectCompany(UUID),
    UpdateLevel(usize),
    AddAccess,
    AddResult(String),
    ResponseError(Error),
    ClearError,
}

impl Component for AddCompanyAccessModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            search_text: String::new(),
            search_results: Vec::new(),
            selected_company_uuid: None,
            selected_level: 3,
            debounce_timeout: None,
            search_loading: false,
            adding: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::Open => {
                self.selected_company_uuid = None;
                self.search_text.clear();
                self.search_results.clear();
            }
            Msg::Close => {
                self.props.on_close.emit(());
            }
            Msg::UpdateSearch(text) => {
                self.search_text = text;
                self.selected_company_uuid = None;

                if !self.search_text.is_empty() {
                    let cb_link = link.clone();
                    self.debounce_timeout = Some(TimeoutService::spawn(
                        Duration::from_millis(500),
                        cb_link.callback(|_| Msg::SearchCompanies),
                    ));
                } else {
                    self.search_results.clear();
                }
            }
            Msg::SearchCompanies => {
                if self.search_text.is_empty() {
                    return true
                }
                self.search_loading = true;
                let search = self.search_text.clone();
                let exclude_uuids = Some(self.props.existing_company_uuids.clone());
                let ipt_paginate = Some(search_companies_for_access::IptPaginate {
                    currentPage: 1,
                    perPage: 50,
                });
                spawn_local(async move {
                    let res = make_query(SearchCompaniesForAccess::build_query(
                        search_companies_for_access::Variables {
                            search,
                            exclude_uuids,
                            ipt_paginate,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::SearchResults(res));
                });
            }
            Msg::SearchResults(res) => {
                self.search_loading = false;
                match resp_parsing(res, "companies") {
                    Ok(companies) => self.search_results = companies,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::SelectCompany(uuid) => self.selected_company_uuid = Some(uuid),
            Msg::UpdateLevel(level) => self.selected_level = level,
            Msg::AddAccess => {
                if self.selected_level < 1 {
                    return true
                }
                if let Some(company_uuid) = &self.selected_company_uuid {
                    self.adding = true;
                    let var_set_company_access_component = set_company_access_component::IptCompanyAccessComponentData {
                        componentUuid: self.props.component_uuid.clone(),
                        companyUuid: company_uuid.clone(),
                        typeAccessId: self.selected_level as i64,
                    };
                    spawn_local(async move {
                        let res = make_query(SetCompanyAccessComponent::build_query(
                            set_company_access_component::Variables { var_set_company_access_component }
                        )).await.unwrap();
                        link.send_message(Msg::AddResult(res));
                    });
                }
            }
            Msg::AddResult(res) => {
                self.adding = false;
                match resp_parsing(res, "setCompanyAccessComponent") {
                    Ok(value) => {
                        debug!("Add company access returned {}", value);
                        if value {
                            self.props.on_success.emit(());
                            self.props.on_close.emit(());
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid && self.props.is_active == props.is_active {
            false
        } else {
            self.props = props;
            if self.props.is_active {
                self.link.send_message(Msg::Open)
            }
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let close_modal = self.link.callback(|_| Msg::Close);
        let class_modal = if self.props.is_active { "modal is-active" } else { "modal" };
        let mut class_input = classes!("control");
        if self.search_loading {
            class_input.push("is-loading");
        };

        html! {
            <div class={class_modal}>
                <div class="modal-background" onclick={close_modal.clone()} />
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{get_value_field(&488)}</p>
                    </header>
                    <section class="modal-card-body">
                        <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
                        <div class="columns is-multiline">
                            <div class="column is-half">
                                <div class="field">
                                    <label class="label">{get_value_field(&489)}</label>
                                    <div class={class_input}>
                                        <input
                                            class="input"
                                            type="text"
                                            placeholder={get_value_field(&482)}
                                            value={self.search_text.clone()}
                                            oninput={self.link.callback(|ev: InputData| Msg::UpdateSearch(ev.value))}
                                        />
                                    </div>
                                </div>
                            </div>
                            <div class="column is-half">
                                <div class="field">
                                    <label class="label">{get_value_field(&487)}</label>
                                    <PermissionLevelBlock
                                        change_cb={self.link.callback(|id| Msg::UpdateLevel(id))}
                                        permissions={self.props.permissions.clone()}
                                        selected={self.selected_level}
                                        preset={self.selected_level}
                                    />
                                </div>
                            </div>
                        </div>
                        <div class={"column"}>
                            {if !self.search_results.is_empty() {
                                html! {
                                    <div class="">
                                        {for self.search_results.iter().map(|company| {
                                            self.show_item(company.clone())
                                        })}
                                    </div>
                                }
                            } else if !self.search_text.is_empty() && !self.search_loading {
                                html! {
                                    <p class="help is-info">{get_value_field(&496)}</p>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    </section>
                    <footer class="modal-card-foot">
                        {ft_cancel_btn(&format!("cancel-add-company-{}", self.props.component_uuid), close_modal.clone(), classes!(""))}
                        {ft_save_btn(
                            &format!("add-company-access-{}", self.props.component_uuid),
                            self.link.callback(|_| Msg::AddAccess),
                            true,
                            self.selected_company_uuid.is_none() || self.adding,
                        )}
                    </footer>
                </div>
            </div>
        }
    }
}

impl AddCompanyAccessModal {
    fn show_item(&self, company: CompanySearchResult) -> Html {
        let company_uuid = company.uuid.clone();
        let onclick_company_item = self.link.callback(move |_| Msg::SelectCompany(company_uuid.clone()));
        let is_selected = self.selected_company_uuid.as_ref().map(|f| f == &company.uuid).unwrap_or_default();
        let mut class_item = classes!("is-flex", "is-align-items-center", "is-justify-content-space-between", "p-2", "is-clickable");
        if is_selected {
            class_item.push("has-background-primary-light");
        }

        html!{
            <div class={class_item} onclick={onclick_company_item}>
                <div class="is-flex is-align-items-center is-flex-grow-1">
                    <div class="image is-32x32 mr-2">
                        <img src={company.image_file.download_url.clone()} alt="Logo" />
                    </div>
                    <div class="is-flex-grow-1">
                        <div class="is-flex is-align-items-center is-flex-wrap-wrap">
                            <span class="has-text-weight-bold mr-2">{company.shortname}</span>
                            <span class="is-size-7 has-text-grey">{truncate_uuid(&company.uuid)}</span>
                        </div>
                        {if !company.inn.is_empty() {
                            html! { <p class="is-size-7 has-text-grey mt-1">{"INN: "}{company.inn}</p> }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>
            </div>
        }
    }
}