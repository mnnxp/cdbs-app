mod supplier_item;
pub use supplier_item::ComponentSupplierItem;

use std::collections::BTreeSet;
use std::time::Duration;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html, InputData, classes};
use yew::services::timeout::{TimeoutService, TimeoutTask};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::buttons::ft_add_btn;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::modal::ModalBlock;
use crate::types::{UUID, Supplier, ShowCompanyShort};
use crate::services::{LocaleKey, resp_parsing, unique_id, truncate_uuid};
use crate::gqls::make_query;
use crate::gqls::component::{
    SetCompanyOwnerSupplier, set_company_owner_supplier,
    AddComponentSupplier, add_component_supplier,
    ComponentSuppliers, component_suppliers,
};
use crate::gqls::company::{
    GetCompaniesShortList, get_companies_short_list,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub component_suppliers: Vec<Supplier>,
    pub supplier_list: Vec<ShowCompanyShort>,
    pub is_base: bool,
}

pub struct ComponentSuppliersCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    company_uuids: BTreeSet<UUID>,
    component_suppliers: Vec<Supplier>,
    request_set_supplier_uuid: Option<UUID>,
    request_set_supplier_description: String,
    hide_set_supplier_modal: bool,
    search_text: String,
    search_results: Vec<ShowCompanyShort>,
    debounce_timeout: Option<TimeoutTask>,
    search_loading: bool,
}

#[derive(Clone)]
pub enum Msg {
    DeleteComponentCompany(UUID),
    RequestChangeOwnerSupplier,
    RequestAddSupplier,
    RequestComponentSuppliers,
    GetUpdateSetSupplierResult(String),
    GetUpdateAddSupplierResult(String),
    GetComponentSuppliersResult(String),
    SearchCompanies,
    SearchResults(String),
    UpdateSearch(String),
    SelectSupplier(UUID),
    UpdateSupplierDescription(String),
    ToggleSetSupplierModal,
    ResponseError(Error),
    ClearError,
}

impl Component for ComponentSuppliersCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut company_uuids: BTreeSet<UUID> = BTreeSet::new();
        for company in props.component_suppliers.iter() {
            company_uuids.insert(company.supplier.uuid.clone());
        };
        let component_suppliers = props.component_suppliers.clone();

        Self {
            error: None,
            props,
            link,
            company_uuids,
            component_suppliers,
            request_set_supplier_uuid: None,
            request_set_supplier_description: String::new(),
            hide_set_supplier_modal: true,
            search_text: String::new(),
            search_results: Vec::new(),
            debounce_timeout: None,
            search_loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::DeleteComponentCompany(company_uuid) => {
                self.company_uuids.remove(&company_uuid);
            },
            Msg::RequestChangeOwnerSupplier => {
                if let Some(uuid) = &self.request_set_supplier_uuid {
                    let ipt_supplier_component_data = set_company_owner_supplier::IptSupplierComponentData{
                        componentUuid: self.props.component_uuid.clone(),
                        companyUuid: uuid.clone(),
                        description: self.request_set_supplier_description.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(SetCompanyOwnerSupplier::build_query(
                            set_company_owner_supplier::Variables { ipt_supplier_component_data }
                        )).await.unwrap();
                        link.send_message(Msg::GetUpdateSetSupplierResult(res));
                    })
                }
            },
            Msg::RequestAddSupplier => {
                if let Some(uuid) = &self.request_set_supplier_uuid {
                    let ipt_supplier_component_data = add_component_supplier::IptSupplierComponentData{
                        componentUuid: self.props.component_uuid.clone(),
                        companyUuid: uuid.clone(),
                        description: self.request_set_supplier_description.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(AddComponentSupplier::build_query(
                            add_component_supplier::Variables { ipt_supplier_component_data }
                        )).await.unwrap();
                        link.send_message(Msg::GetUpdateAddSupplierResult(res));
                    })
                }
            },
            Msg::RequestComponentSuppliers => {
                let component_uuid = self.props.component_uuid.clone();
                spawn_local(async move {
                    let res = make_query(ComponentSuppliers::build_query(
                        component_suppliers::Variables { component_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentSuppliersResult(res));
                })
            },
            Msg::GetUpdateSetSupplierResult(res) => {
                match resp_parsing::<bool>(res, "setCompanyOwnerSupplier") {
                    Ok(result) => {
                        debug!("setCompanyOwnerSupplier: {:?}", result);
                        self.hide_set_supplier_modal = result;
                        link.send_message(Msg::RequestComponentSuppliers);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateAddSupplierResult(res) => {
                match resp_parsing::<bool>(res, "addComponentSupplier") {
                    Ok(result) => {
                        debug!("addComponentSupplier: {:?}", result);
                        self.hide_set_supplier_modal = result;
                        link.send_message(Msg::RequestComponentSuppliers);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetComponentSuppliersResult(res) => {
                match resp_parsing::<Vec<Supplier>>(res, "componentSuppliers") {
                    Ok(result) => {
                        debug!("componentSuppliers: {:?}", result);
                        self.component_suppliers = result;
                        self.company_uuids = BTreeSet::new();
                        for company in self.component_suppliers.iter() {
                            self.company_uuids.insert(company.supplier.uuid.clone());
                        };
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::SearchCompanies => {
                if self.search_text.is_empty() {
                    return true
                }
                self.search_loading = true;
                let search = self.search_text.clone();
                let ipt_companies_arg = Some(get_companies_short_list::IptCompaniesArg {
                    companiesUuids: None,
                    userUuid: None,
                    favorite: None,
                    supplier: None,
                    search: Some(search),
                    excludeUuids: None,
                });
                let ipt_paginate = Some(get_companies_short_list::IptPaginate {
                    currentPage: 1,
                    perPage: 50,
                });
                spawn_local(async move {
                    let res = make_query(GetCompaniesShortList::build_query(
                        get_companies_short_list::Variables {
                            ipt_companies_arg,
                            ipt_paginate,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::SearchResults(res));
                });
            },
            Msg::SearchResults(res) => {
                self.search_loading = false;
                match resp_parsing::<Vec<ShowCompanyShort>>(res, "companies") {
                    Ok(companies) => self.search_results = companies,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateSearch(text) => {
                self.search_text = text;
                self.request_set_supplier_uuid = None;

                if !self.search_text.is_empty() {
                    let cb_link = link.clone();
                    self.debounce_timeout = Some(TimeoutService::spawn(
                        Duration::from_millis(500),
                        cb_link.callback(|_| Msg::SearchCompanies),
                    ));
                } else {
                    self.search_results.clear();
                }
            },
            Msg::SelectSupplier(uuid) => self.request_set_supplier_uuid = Some(uuid),
            Msg::UpdateSupplierDescription(data) => self.request_set_supplier_description = data,
            Msg::ToggleSetSupplierModal => {
                self.hide_set_supplier_modal = !self.hide_set_supplier_modal;
                if !self.hide_set_supplier_modal {
                    // Reset fields on open
                    self.search_text.clear();
                    self.search_results.clear();
                    self.request_set_supplier_uuid = None;
                    self.search_loading = false;
                    self.debounce_timeout = None;
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
             self.props.component_suppliers.len() == props.component_suppliers.len() &&
               self.props.supplier_list.len() == props.supplier_list.len() {
            false
        } else {
            self.company_uuids = BTreeSet::new();
            for company in props.component_suppliers.iter() {
                self.company_uuids.insert(company.supplier.uuid.clone());
            };
            self.hide_set_supplier_modal = true;
            self.search_text.clear();
            self.search_results.clear();
            self.request_set_supplier_uuid = None;
            self.request_set_supplier_description.clear();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_action_btn = self.link.callback(|_| Msg::ToggleSetSupplierModal);

        html!{
            <div class="card">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <header class="card-header">
                    <p class="card-header-title">{LocaleKey::ManageComponentSuppliers.get_value()}</p>
                </header>
                <div class="card-content">
                    <div class="content">
                        {self.show_suppliers()}
                    </div>
                    <footer class="card-footer">
                        {ft_add_btn(
                            "set-supplier-component",
                            LocaleKey::AddSupplierForComponentLabel.get_value(),
                            onclick_action_btn,
                            true,
                            false
                        )}
                    </footer>
                </div>
                {match self.props.is_base {
                    true => self.supplier_modal_form(
                        LocaleKey::AddSupplierForComponent,
                        LocaleKey::SelectSupplier,
                        "add-supplier",
                        Msg::RequestAddSupplier
                    ),
                    false => self.supplier_modal_form(
                        LocaleKey::SetOwnerSupplier,
                        LocaleKey::SelectSupplierLabel,
                        "set-owner-supplier",
                        Msg::RequestChangeOwnerSupplier
                    ),
                }}
            </div>
        }
    }
}

impl ComponentSuppliersCard {
    fn show_suppliers(&self) -> Html {
        let onclick_delete_supplier =
            self.link.callback(|value: UUID| Msg::DeleteComponentCompany(value));

        html!{
        <div class="table-container">
        <div class="content">
          <table class="table is-fullwidth">
            <thead>
            <tr>
                <th>{LocaleKey::Company.get_value()}</th>
                <th>{LocaleKey::Description.get_value()}</th>
                <th>{LocaleKey::Action.get_value()}</th>
                {match self.props.show_delete_btn {
                    true => html!{<th>{LocaleKey::Delete.get_value()}</th>},
                    false => html!{},
                }}
            </tr>
            </thead>
            <tbody>
               {for self.component_suppliers.iter().map(|data| {
                   match self.company_uuids.get(&data.supplier.uuid) {
                       Some(_) => html!{<ComponentSupplierItem
                           show_delete_btn={self.props.show_delete_btn}
                           component_uuid={self.props.component_uuid.clone()}
                           supplier_data={data.clone()}
                           delete_supplier={Some(onclick_delete_supplier.clone())}
                         />},
                       None => html!{},
                   }
               })}
            </tbody>
          </table>
        </div>
        </div>
        }
    }

    fn supplier_modal_form(&self, lk_title: LocaleKey, lk_label: LocaleKey, modal_id: &'static str, on_save_msg: Msg) -> Html {
        let onclick_hide_modal = self.link.callback(|_| Msg::ToggleSetSupplierModal);
        let oninput_search = self.link.callback(|ev: InputData| Msg::UpdateSearch(ev.value));
        let oninput_supplier_description = self.link.callback(|ev: InputData| Msg::UpdateSupplierDescription(ev.value));
        let description_id = unique_id("update-description");
        let mut class_input = classes!("control");
        if self.search_loading {
            class_input.push("is-loading");
        };

        html! {
            <ModalBlock
                modal_id={modal_id}
                title={lk_title.get_value()}
                is_active={!self.hide_set_supplier_modal}
                on_close={onclick_hide_modal}
                on_save={Some(self.link.callback(move |_| on_save_msg.clone()))}
                save_disabled={self.request_set_supplier_uuid.is_none()}
            >
                <div class="field">
                    <label class="label">{lk_label.get_value()}</label>
                    <div class={class_input}>
                        <input
                            class="input"
                            type="text"
                            placeholder={LocaleKey::SearchCompanies.get_value()}
                            value={self.search_text.clone()}
                            oninput={oninput_search}
                        />
                    </div>
                </div>
                <div class="field">
                    <div class="column">
                        {if !self.search_text.is_empty() {
                            // Show search results
                            if !self.search_results.is_empty() {
                                html! {
                                    <div class="">
                                        {for self.search_results.iter().map(|company| {
                                            self.show_item(company.clone())
                                        })}
                                    </div>
                                }
                            } else if !self.search_loading {
                                html! {
                                    <p class="help is-info">{LocaleKey::NoCompaniesFound.get_value()}</p>
                                }
                            } else {
                                html! {}
                            }
                        } else {
                            // Show owned companies
                            if !self.props.supplier_list.is_empty() {
                                html! {
                                    <div class="">
                                        {for self.props.supplier_list.iter().map(|company| {
                                            self.show_item(company.clone())
                                        })}
                                    </div>
                                }
                            } else {
                                html! {
                                    <p class="help is-info">{LocaleKey::NoCompaniesFound.get_value()}</p>
                                }
                            }
                        }}
                    </div>
                </div>
                <div class="field">
                    <label for={description_id.clone()} class="label">{LocaleKey::SupplierDescription.get_value()}</label>
                    <div class="control">
                        <textarea
                            id={description_id}
                            class="textarea"
                            placeholder={LocaleKey::SupplierDescription.get_value()}
                            value={self.request_set_supplier_description.clone()}
                            oninput={oninput_supplier_description}
                        />
                    </div>
                </div>
            </ModalBlock>
        }
    }

    fn show_item(&self, company: ShowCompanyShort) -> Html {
        let company_uuid = company.uuid.clone();
        let onclick_company_item = self.link.callback(move |_| Msg::SelectSupplier(company_uuid.clone()));
        let is_selected = self.request_set_supplier_uuid.as_ref().map(|f| f == &company.uuid).unwrap_or_default();
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
