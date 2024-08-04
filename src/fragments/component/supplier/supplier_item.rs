use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::fragments::{
    buttons::ft_delete_small_btn,
    list_errors::ListErrors,
    company::ListItemCompany,
};
use crate::services::resp_parsing;
use crate::types::{UUID, Supplier, ShowCompanyShort};
use crate::gqls::{
    make_query,
    company::{GetCompaniesShortList, get_companies_short_list},
    component::{DeleteSuppliersComponent, delete_suppliers_component},
};

/// Company card for show data on component page
pub struct ComponentSupplierItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    company_data: Option<ShowCompanyShort>,
    open_company_info: bool,
    get_result_delete: bool,
    get_confirm: UUID,
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
    ClearError,
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
            get_confirm: String::new(),
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
                let supplier_uuid = self.props.supplier_data.supplier.uuid.clone();
                if self.get_confirm == supplier_uuid {
                    let del_suppliers_component_data = delete_suppliers_component::DelSuppliersComponentData{
                        componentUuid: self.props.component_uuid.clone(),
                        companiesUuids: vec![supplier_uuid],
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteSuppliersComponent::build_query(
                            delete_suppliers_component::Variables {
                                del_suppliers_component_data,
                            }
                        )).await.unwrap();
                        link.send_message(Msg::GetDeleteSupplierResult(res));
                    })
                } else {
                    self.get_confirm = supplier_uuid;
                }
            },
            Msg::ResponseError(error) => self.error = Some(error),
            Msg::GetCompanyDataResult(res) => {
                match resp_parsing::<Vec<ShowCompanyShort>>(res, "companies") {
                    Ok(result) => self.company_data = result.first().map(|x| x.clone()),
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetDeleteSupplierResult(res) => {
                match resp_parsing::<usize>(res, "deleteSuppliersComponent") {
                    Ok(result) => {
                        debug!("deleteSuppliersComponent: {:?}", result);
                        self.get_result_delete = result > 0;
                        if self.get_result_delete {
                            if let Some(delete_supplier) = &self.props.delete_supplier {
                                delete_supplier.emit(self.props.supplier_data.supplier.uuid.clone());
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ClearError => self.error = None,
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
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_supplier_data_info = self.link.callback(|_| Msg::ShowCompanyCard);
        let onclick_delete_supplier = self.link.callback(|_| Msg::RequestDeleteSupplier);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
            {self.show_modal_company_info()}
            <tr>
                <td>{self.props.supplier_data.supplier.shortname.clone()}</td>
                <td>{self.props.supplier_data.description.clone()}</td>
                <td><a onclick={onclick_supplier_data_info}>
                    <span class="icon" >
                        <i class="fas fa-info" aria-hidden="true"></i>
                    </span>
                </a></td>
                {match self.props.show_delete_btn {
                    true => html!{<td>
                        {ft_delete_small_btn(
                            "component-supplier-delete",
                            onclick_delete_supplier,
                            self.get_confirm == self.props.supplier_data.supplier.uuid,
                        )}
                    </td>},
                    false => html!{},
                }}
            </tr>
        </>}
    }
}

impl ComponentSupplierItem {
    fn show_modal_company_info(&self) -> Html {
        let onclick_company_data_info = self.link.callback(|_| Msg::ShowCompanyCard);
        let class_modal = match &self.open_company_info {
            true => "modal is-active",
            false => "modal",
        };

        match &self.company_data {
            Some(data) => html!{<div class={class_modal}>
              <div class="modal-background" onclick={onclick_company_data_info.clone()} />
              // <div class="modal-content">
                  <div class="card">
                    <ListItemCompany
                        data={data.clone()}
                        show_list={true}
                      />
                  </div>
              // </div>
              <button class="modal-close is-large" aria-label="close" onclick={onclick_company_data_info} />
            </div>},
            None => html!{},
        }
    }
}
