mod supplier_item;
pub use supplier_item::ComponentSupplierItem;

use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html, ChangeData, InputData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::buttons::ft_add_btn;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::modal::ModalBlock;
use crate::types::{UUID, Supplier, ShowCompanyShort};
use crate::services::{LocaleKey, resp_parsing, unique_id};
use crate::gqls::make_query;
use crate::gqls::component::{
    SetCompanyOwnerSupplier, set_company_owner_supplier,
    AddComponentSupplier, add_component_supplier,
    ComponentSuppliers, component_suppliers,
};

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
            Msg::UpdateSetSupplier(data) => self.request_set_supplier_uuid = data,
            Msg::UpdateSupplierDescription(data) => self.request_set_supplier_description = data,
            Msg::ChangeHideSetSupplier => self.hide_set_supplier_modal = !self.hide_set_supplier_modal,
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
        let onclick_action_btn = self.link.callback(|_| Msg::ChangeHideSetSupplier);

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
        let onclick_hide_modal = self.link.callback(|_| Msg::ChangeHideSetSupplier);
        let onchange_select_set_supplier = self.link.callback(|ev: ChangeData| {
            Msg::UpdateSetSupplier(match ev {
                ChangeData::Select(el) => el.value(),
                _ => String::new(),
            })
        });
        let oninput_supplier_description = self.link.callback(|ev: InputData| Msg::UpdateSupplierDescription(ev.value));
        let select_id = unique_id("set-main-supplier");
        let description_id = unique_id("update-description");
        html! {
            <ModalBlock
                modal_id={modal_id}
                title={lk_title.get_value()}
                is_active={!self.hide_set_supplier_modal}
                on_close={onclick_hide_modal}
                on_save={Some(self.link.callback(move |_| on_save_msg.clone()))}
                save_disabled={self.request_set_supplier_uuid.is_empty()}
            >
                <div class="field">
                    <label for={select_id.clone()} class="label">{lk_label.get_value()}</label>
                    <div class="control">
                        <div class="select is-fullwidth">
                            <select id={select_id} onchange={onchange_select_set_supplier}>
                                { for self.props.supplier_list.iter().map(|x| {
                                    html! {
                                        <option value={x.uuid.to_string()}
                                                selected={x.uuid == self.request_set_supplier_uuid} >
                                            { x.shortname.clone() }
                                        </option>
                                    }
                                })}
                            </select>
                        </div>
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
}
