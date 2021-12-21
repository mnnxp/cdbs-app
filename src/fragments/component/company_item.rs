use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
// use log::debug;
use chrono::NaiveDateTime;
use crate::gqls::make_query;
use crate::error::{Error, get_error};
use crate::fragments::{
    list_errors::ListErrors,
    catalog_company::ListItemCompany,
};
use crate::types::{UUID, Supplier, ShowCompanyShort};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct GetCompaniesShortList;

/// Company card for show data on component page
pub struct ComponentCompanyItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    company_data: Option<ShowCompanyShort>,
    open_company_info: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub supplier_data: Supplier,
}

#[derive(Clone)]
pub enum Msg {
    ShowCompanyCard,
    RequestCompanyData,
    GetCompanyDataResult(String),
    Ignore,
}

impl Component for ComponentCompanyItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ComponentCompanyItem {
            error: None,
            props,
            link,
            company_data: None,
            open_company_info: false,
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
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props.supplier_data.supplier.uuid == props.supplier_data.supplier.uuid {
            true => false,
            false => {
                self.props = props;
                true
            },
        }
    }

    fn view(&self) -> Html {
        let onclick_supplier_data_info = self.link
            .callback(|_| Msg::ShowCompanyCard);

        html!{<>
            <ListErrors error=self.error.clone()/>
            {self.show_modal_company_info()}
            <tr>
                <td>{self.props.supplier_data.supplier.shortname.clone()}</td>
                <td>{self.props.supplier_data.description.clone()}</td>
                <td><a onclick={onclick_supplier_data_info.clone()}>{"info"}</a></td>
            </tr>
        </>}
    }
}

impl ComponentCompanyItem {
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
              <div class="modal-content">
                  <div class="card">
                    <ListItemCompany
                        data = data.clone()
                        show_list = true
                      />
                  </div>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick=onclick_company_data_info />
            </div>},
            None => html!{},
        }
    }
}
