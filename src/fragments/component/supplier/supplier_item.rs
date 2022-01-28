use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use chrono::NaiveDateTime;
use crate::gqls::make_query;
use crate::error::{Error, get_error};
use crate::fragments::{
    list_errors::ListErrors,
    company::ListItemCompany,
};
use crate::types::{UUID, Supplier, ShowCompanyShort};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct GetCompaniesShortList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct DeleteSuppliersComponent;

/// Company card for show data on component page
pub struct ComponentSupplierItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    company_data: Option<ShowCompanyShort>,
    open_company_info: bool,
    get_result_delete: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub supplier_data: Supplier,
    pub delete_supplier: Option<Callback<UUID>>,
}

#[derive(Clone)]
pub enum Msg {
    ShowCompanyCard,
    RequestCompanyData,
    RequestDeleteSupplier,
    ResponseError(Error),
    GetCompanyDataResult(String),
    GetDeleteSupplierResult(String),
    Ignore,
}

impl Component for ComponentSupplierItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ComponentSupplierItem {
            error: None,
            props,
            link,
            company_data: None,
            open_company_info: false,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ShowCompanyCard => {
                if self.company_data.is_none() {
                    link.send_message(Msg::RequestCompanyData);
                }
                self.open_company_info = !self.open_company_info;
            },
            Msg::RequestCompanyData => {
                let ipt_companies_arg = Some(get_companies_short_list::IptCompaniesArg {
                    companiesUuids: Some(vec![self.props.supplier_data.supplier.uuid.clone()]),
                    userUuid: None,
                    favorite: None,
                    supplier: None,
                    limit: Some(1),
                    offset: None,
                });
                spawn_local(async move {
                    let res = make_query(GetCompaniesShortList::build_query(get_companies_short_list::Variables {
                        ipt_companies_arg
                    })).await.unwrap();
                    link.send_message(Msg::GetCompanyDataResult(res));
                });
            },
            Msg::RequestDeleteSupplier => {
                let del_suppliers_component_data = delete_suppliers_component::DelSuppliersComponentData{
                    componentUuid: self.props.component_uuid.clone(),
                    companiesUuids: vec![self.props.supplier_data.supplier.uuid.clone()],
                };
                spawn_local(async move {
                    let res = make_query(DeleteSuppliersComponent::build_query(
                        delete_suppliers_component::Variables {
                            del_suppliers_component_data,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteSupplierResult(res));
                })
            },
            Msg::ResponseError(error) => self.error = Some(error),
            Msg::GetCompanyDataResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                // debug!("res value: {:#?}", res_value);

                match res_value.is_null() {
                    false => {
                        let result: Vec<ShowCompanyShort> = serde_json::from_value(res_value.get("companies").unwrap().clone()).unwrap();
                        // debug!("GetCompanyDataResult result: {:?}", result);
                        self.company_data = result.first().map(|x| x.clone());
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetDeleteSupplierResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res.get("deleteSuppliersComponent").unwrap().clone()).unwrap();
                        debug!("deleteSuppliersComponent: {:?}", result);
                        match &self.props.delete_supplier {
                            Some(delete_supplier) => {
                                if result > 0 {
                                    self.get_result_delete = true;
                                    delete_supplier.emit(self.props.supplier_data.supplier.uuid.clone());
                                };
                            },
                            None => self.get_result_delete = result > 0,
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.supplier_data.supplier.uuid == props.supplier_data.supplier.uuid &&
                self.props.supplier_data.description == props.supplier_data.description {
            false
        } else {
            self.props = props;
            self.get_result_delete = false;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_supplier_data_info = self.link
            .callback(|_| Msg::ShowCompanyCard);

        let onclick_delete_supplier = self.link
            .callback(|_| Msg::RequestDeleteSupplier);

        html!{<>
            <ListErrors error=self.error.clone()/>
            {self.show_modal_company_info()}
            <tr>
                <td>{self.props.supplier_data.supplier.shortname.clone()}</td>
                <td>{self.props.supplier_data.description.clone()}</td>
                <td><a onclick={onclick_supplier_data_info.clone()}>{"info"}</a></td>
                {match self.props.show_delete_btn {
                    true => html!{<td><a onclick={onclick_delete_supplier.clone()}>
                        <span class="icon" >
                          <i class="fa fa-trash" aria-hidden="true"></i>
                        </span>
                    </a></td>},
                    false => html!{},
                }}
            </tr>
        </>}
    }
}

impl ComponentSupplierItem {
    fn show_modal_company_info(&self) -> Html {
        let onclick_company_data_info = self.link
            .callback(|_| Msg::ShowCompanyCard);

        let class_modal = match &self.open_company_info {
            true => "modal is-active",
            false => "modal",
        };

        match &self.company_data {
            Some(data) => html!{<div class=class_modal>
              <div class="modal-background" onclick=onclick_company_data_info.clone() />
              // <div class="modal-content">
                  <div class="card">
                    <ListItemCompany
                        data = data.clone()
                        show_list = true
                      />
                  </div>
              // </div>
              <button class="modal-close is-large" aria-label="close" onclick=onclick_company_data_info />
            </div>},
            None => html!{},
        }
    }
}
