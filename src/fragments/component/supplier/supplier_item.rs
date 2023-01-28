use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::company::ListItemCompany;
use crate::types::{UUID, Supplier, ShowCompanyShort};
use crate::gqls::make_query;
use crate::gqls::{
    company::{GetCompaniesShortList, get_companies_short_list},
    component::{DeleteSuppliersComponent, delete_suppliers_component},
};
use crate::services::{resp_parsing, resp_parsing_item};

/// Company card for show data on component page
pub struct ComponentSupplierItem {
    error: Option<Error>,
    company_data: Option<ShowCompanyShort>,
    supplier_uuid: UUID,
    supplier_description: String,
    open_company_info: bool,
    get_result_delete: bool,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub supplier_data: Supplier,
    #[prop_or_default]
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            supplier_uuid: ctx.props().supplier_data.supplier.uuid.clone(),
            supplier_description: ctx.props().supplier_data.description.clone(),
            company_data: None,
            open_company_info: false,
            get_result_delete: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::ShowCompanyCard => {
                if self.company_data.is_none() {
                    link.send_message(Msg::RequestCompanyData);
                }
                self.open_company_info = !self.open_company_info;
            },
            Msg::RequestCompanyData => {
                let ipt_companies_arg = Some(get_companies_short_list::IptCompaniesArg {
                    companies_uuids: Some(vec![ctx.props().supplier_data.supplier.uuid.clone()]),
                    user_uuid: None,
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
                    component_uuid: ctx.props().component_uuid.clone(),
                    companies_uuids: vec![ctx.props().supplier_data.supplier.uuid.clone()],
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
                let result: Vec<ShowCompanyShort> = resp_parsing(res, "companies")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                self.company_data = result.first().map(|x| x.clone());
            },
            Msg::GetDeleteSupplierResult(res) => {
                let result: usize  = resp_parsing_item(res, "deleteSuppliersComponent")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                match &ctx.props().delete_supplier {
                    Some(delete_supplier) => {
                        if result > 0 {
                            self.get_result_delete = true;
                            delete_supplier.emit(ctx.props().supplier_data.supplier.uuid.clone());
                        }
                    },
                    None => self.get_result_delete = result > 0,
                }
            },
            Msg::Ignore => {}
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.supplier_uuid == ctx.props().supplier_data.supplier.uuid &&
                self.supplier_description == ctx.props().supplier_data.description {
            false
        } else {
            self.get_result_delete = false;
            self.supplier_uuid = ctx.props().supplier_data.supplier.uuid.clone();
            self.supplier_description = ctx.props().supplier_data.description.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_supplier_data_info = ctx.link().callback(|_| Msg::ShowCompanyCard);
        let onclick_delete_supplier = ctx.link().callback(|_| Msg::RequestDeleteSupplier);

        html!{<>
            <ListErrors error={self.error.clone()}/>
            {self.show_modal_company_info(ctx.link())}
            <tr>
                <td>{ctx.props().supplier_data.supplier.shortname.clone()}</td>
                <td>{ctx.props().supplier_data.description.clone()}</td>
                <td><a onclick={onclick_supplier_data_info.clone()}>
                    <span class="icon" >
                        <i class="fas fa-info" aria-hidden="true"></i>
                    </span>
                </a></td>
                {match ctx.props().show_delete_btn {
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
    fn show_modal_company_info(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_company_data_info = link.callback(|_| Msg::ShowCompanyCard);
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
                        data = {data.clone()}
                        show_list = true
                      />
                  </div>
              // </div>
              <button class="modal-close is-large" aria-label="close" onclick={onclick_company_data_info} />
            </div>},
            None => html!{},
        }
    }
}
