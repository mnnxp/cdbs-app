mod supplier_item;

pub use supplier_item::ComponentSupplierItem;

use std::collections::BTreeSet;
use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, Supplier, ShowCompanyShort};
use crate::services::get_value_field;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]

struct SetCompanyOwnerSupplier;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]

struct AddComponentSupplier;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComponentSuppliers;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub component_suppliers: Vec<Supplier>,
    pub supplier_list: Vec<ShowCompanyShort>,
    pub is_base: bool,
    // pub delete_company: Option<Callback<UUID>>,
}

pub struct ComponentSuppliersCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    company_uuids: BTreeSet<UUID>,
    component_suppliers: Vec<Supplier>,
    request_set_supplier_uuid: UUID,
    request_set_supplier_description: String,
    hide_set_supplier_modal: bool,
    // get_result_supplier: bool,
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
    UpdateSetSupplier(String),
    UpdateSupplierDescription(String),
    ChangeHideSetSupplier,
    ClearError,
    Ignore,
}

impl Component for ComponentSuppliersCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut company_uuids: BTreeSet<UUID> = BTreeSet::new();

        for company in props.component_suppliers.iter() {
            company_uuids.insert(company.supplier.uuid.clone());
        };

        let request_set_supplier_uuid =
            props.supplier_list.first().map(|s| s.uuid.clone()).unwrap_or_default();

        let component_suppliers = props.component_suppliers.clone();

        Self {
            error: None,
            props,
            link,
            company_uuids,
            component_suppliers,
            request_set_supplier_uuid,
            request_set_supplier_description: String::new(),
            hide_set_supplier_modal: true,
            // get_result_supplier: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::DeleteComponentCompany(company_uuid) => {
                self.company_uuids.remove(&company_uuid);
            },
            Msg::RequestChangeOwnerSupplier => {
                let ipt_supplier_component_data = set_company_owner_supplier::IptSupplierComponentData{
                    componentUuid: self.props.component_uuid.clone(),
                    companyUuid: self.request_set_supplier_uuid.clone(),
                    description: self.request_set_supplier_description.clone(),
                };
                spawn_local(async move {
                    let res = make_query(SetCompanyOwnerSupplier::build_query(
                        set_company_owner_supplier::Variables { ipt_supplier_component_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateSetSupplierResult(res));
                })
            },
            Msg::RequestAddSupplier => {
                let ipt_supplier_component_data = add_component_supplier::IptSupplierComponentData{
                    componentUuid: self.props.component_uuid.clone(),
                    companyUuid: self.request_set_supplier_uuid.clone(),
                    description: self.request_set_supplier_description.clone(),
                };
                spawn_local(async move {
                    let res = make_query(AddComponentSupplier::build_query(
                        add_component_supplier::Variables { ipt_supplier_component_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateAddSupplierResult(res));
                })
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
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("setCompanyOwnerSupplier").unwrap().clone()).unwrap();
                        debug!("setCompanyOwnerSupplier: {:?}", result);
                        self.hide_set_supplier_modal = result;
                        // self.get_result_supplier = result;
                        link.send_message(Msg::RequestComponentSuppliers);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUpdateAddSupplierResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("addComponentSupplier").unwrap().clone()).unwrap();
                        debug!("addComponentSupplier: {:?}", result);
                        self.hide_set_supplier_modal = result;
                        // self.get_result_supplier = result;
                        link.send_message(Msg::RequestComponentSuppliers);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetComponentSuppliersResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<Supplier> =
                            serde_json::from_value(res_value.get("componentSuppliers").unwrap().clone()).unwrap();
                        debug!("componentSuppliers: {:?}", result);
                        self.component_suppliers = result;
                        self.company_uuids = BTreeSet::new();
                        for company in self.component_suppliers.iter() {
                            self.company_uuids.insert(company.supplier.uuid.clone());
                        };
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateSetSupplier(data) => self.request_set_supplier_uuid = data,
            Msg::UpdateSupplierDescription(data) => self.request_set_supplier_description = data,
            Msg::ChangeHideSetSupplier => self.hide_set_supplier_modal = !self.hide_set_supplier_modal,
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
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
            self.request_set_supplier_uuid =
                props.supplier_list.first().map(|s| s.uuid.clone()).unwrap_or_default();
            self.request_set_supplier_description = String::new();
            self.hide_set_supplier_modal = true;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {self.show_suppliers()}
        </>}
    }
}

impl ComponentSuppliersCard {
    fn show_suppliers(&self) -> Html {
        let onclick_delete_supplier = self.link
            .callback(|value: UUID| Msg::DeleteComponentCompany(value));

        let onclick_action_btn = self.link
            .callback(|_| Msg::ChangeHideSetSupplier);

        html!{<div class="card column">
          <table class="table is-fullwidth">
            <tbody>
               <th>{ get_value_field(&109) }</th> // Company
               <th>{ get_value_field(&61) }</th> // Description
               <th>{ get_value_field(&111) }</th> // Action
               {match self.props.show_delete_btn {
                   true => html!{<th>{ get_value_field(&135) }</th>}, // Delete
                   false => html!{},
               }}
               {for self.component_suppliers.iter().map(|data| {
                   match self.company_uuids.get(&data.supplier.uuid) {
                       Some(_) => html!{<ComponentSupplierItem
                           show_delete_btn = self.props.show_delete_btn
                           component_uuid = self.props.component_uuid.clone()
                           supplier_data = data.clone()
                           delete_supplier = Some(onclick_delete_supplier.clone())
                         />},
                       None => html!{},
                   }
               })}
            </tbody>
          </table>
          {match self.props.is_base {
              true => self.modal_add_supplier(),
              false => self.modal_set_owner_supplier(),
          }}
          <button
                id="set-supplier-component"
                class="button is-fullwidth"
                onclick={onclick_action_btn} >
              <span class="icon" >
                  <i class="fas fa-plus" aria-hidden="true"></i>
              </span>
              <span>{ get_value_field(&166) }</span> // Add supplier for component
          </button>
        </div>}
    }

    fn modal_set_owner_supplier(&self) -> Html {
        let onclick_set_owner_supplier = self.link
            .callback(|_| Msg::RequestChangeOwnerSupplier);

        let onclick_hide_modal = self.link
            .callback(|_| Msg::ChangeHideSetSupplier);

        let onchange_select_set_supplier = self.link
            .callback(|ev: ChangeData| Msg::UpdateSetSupplier(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
          }));

        let oninput_supplier_description = self.link
            .callback(|ev: InputData| Msg::UpdateSupplierDescription(ev.value));

        let class_modal = match &self.hide_set_supplier_modal {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_hide_modal.clone() />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{ get_value_field(&167) }</p> // Set owner supplier
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <label class="label">{ get_value_field(&168) }</label> // Select supplier
                        <div class="select">
                          <select
                              id="set-main-supplier"
                              select={self.request_set_supplier_uuid.clone()}
                              onchange=onchange_select_set_supplier
                            >
                          { for self.props.supplier_list.iter().map(|x|
                              html!{
                                  <option value={x.uuid.to_string()}
                                        selected={x.uuid == self.request_set_supplier_uuid} >
                                      {&x.shortname}
                                  </option>
                              }
                          )}
                          </select>
                        </div>
                        <br/>
                        <label class="label">{ get_value_field(&169) }</label> // Supplier description
                        <textarea
                            id="update-description"
                            class="textarea"
                            type="text"
                            placeholder=get_value_field(&169)
                            value={self.request_set_supplier_description.clone()}
                            oninput=oninput_supplier_description
                            />
                        <br/>
                        <button
                            id="supplier-component"
                            class="button is-fullwidth"
                            disabled={self.request_set_supplier_uuid.is_empty()}
                            onclick={onclick_set_owner_supplier} >
                            { get_value_field(&210) } // Apply
                        </button>
                    </section>
                  </div>
                </div>
              </div>
        }
    }

    fn modal_add_supplier(&self) -> Html {
        let onclick_add_supplier = self.link
            .callback(|_| Msg::RequestAddSupplier);

        let onclick_hide_modal = self.link
            .callback(|_| Msg::ChangeHideSetSupplier);

        let onchange_select_add_supplier = self.link
            .callback(|ev: ChangeData| Msg::UpdateSetSupplier(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
          }));

        let oninput_supplier_description = self.link
            .callback(|ev: InputData| Msg::UpdateSupplierDescription(ev.value));

        let class_modal = match &self.hide_set_supplier_modal {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_hide_modal.clone() />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{ get_value_field(&123) }</p> // Add a supplier for the component
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <label class="label">{ get_value_field(&79) }</label> // Select a supplier
                        <div class="select">
                          <select
                              id="set-main-supplier"
                              select={self.request_set_supplier_uuid.clone()}
                              onchange=onchange_select_add_supplier
                            >
                          { for self.props.supplier_list.iter().map(|x|
                              html!{
                                  <option value={x.uuid.to_string()}
                                        selected={x.uuid == self.request_set_supplier_uuid} >
                                      {&x.shortname}
                                  </option>
                              }
                          )}
                          </select>
                        </div>
                    </section>
                    <textarea
                        id="update-description"
                        class="textarea"
                        // rows="10"
                        type="text"
                        placeholder=get_value_field(&169)
                        value={self.request_set_supplier_description.clone()}
                        oninput=oninput_supplier_description
                        />
                    <button
                        id="supplier-component"
                        class="button is-fullwidth"
                        disabled={self.request_set_supplier_uuid.is_empty()}
                        onclick={onclick_add_supplier} >
                        { get_value_field(&117) } // Add
                    </button>
                  </div>
                </div>
              </div>
        }
    }
}
