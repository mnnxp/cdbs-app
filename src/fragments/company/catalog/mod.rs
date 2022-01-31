mod list_item;

pub use list_item::ListItemCompany;

use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use yew_router::prelude::RouterAnchor;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use chrono::NaiveDateTime;

use crate::gqls::make_query;
use crate::routes::AppRoute;
use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowCompanyShort, CompaniesQueryArg};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct GetCompaniesShortList;

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    GetList
}

#[derive(PartialEq, Eq)]
pub enum ListState {
    List,
    Box,
}

pub struct CatalogCompanies {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    show_type: ListState,
    list: Vec<ShowCompanyShort>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_create_btn: bool,
    pub arguments: Option<CompaniesQueryArg>,
}

impl Component for CatalogCompanies {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            show_type: ListState::Box,
            list: Vec::new()
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::GetList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::SwitchShowType => {
                match self.show_type {
                    ListState::Box => self.show_type = ListState::List,
                    _ => self.show_type = ListState::Box,
                }
            },
            Msg::GetList => {
                let ipt_companies_arg = match &self.props.arguments {
                    Some(ref arg) => Some(get_companies_short_list::IptCompaniesArg {
                        companiesUuids: arg.companies_uuids.clone(),
                        userUuid: arg.user_uuid.to_owned(),
                        favorite: arg.favorite,
                        supplier: arg.supplier,
                        limit: arg.limit,
                        offset: arg.offset,
                    }),
                    None => None,
                };
                spawn_local(async move {
                    let res = make_query(GetCompaniesShortList::build_query(
                        get_companies_short_list::Variables {
                            ipt_companies_arg
                    })).await.unwrap();
                    debug!("GetList res: {:?}", res);
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
              let data: Value = serde_json::from_str(res.as_str()).unwrap();
              let res_value = data.as_object().unwrap().get("data").unwrap();

              // debug!("res value: {:#?}", res_value);

              match res_value.is_null() {
                  false => {
                      let result: Vec<ShowCompanyShort> = serde_json::from_value(res_value.get("companies").unwrap().clone()).unwrap();
                      // debug!("UpdateList result: {:?}", result);
                      self.list = result;
                  },
                  true => self.error = Some(get_error(&data)),
              }
          },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let flag_change = match (&self.props.arguments, &props.arguments) {
            (Some(self_arg), Some(arg)) => self_arg == arg,
            (None, None) => true,
            _ => false,
        };

        // debug!("self_arg == arg: {}", flag_change);

        if self.props.show_create_btn == props.show_create_btn && flag_change {
            // debug!("if change");
            false
        } else {
            self.props.show_create_btn = props.show_create_btn;
            self.props.arguments = props.arguments;
            self.link.send_message(Msg::GetList);
            // debug!("else change");
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_change_view = self.link.callback(|_|Msg::SwitchShowType);

        let (class_for_icon, class_for_list) = match self.show_type {
            ListState::Box => ("fas fa-bars", "flex-box"),
            ListState::List => ("fas fa-th-large", ""),
        };

        html!{
            <div class="companiesBox" >
              <ListErrors error=self.error.clone()/>
              <div class="level" >
                <div class="level-left">
                </div>
                <div class="level-right">
                    <div class="buttons">
                        {match &self.props.show_create_btn {
                            true => html!{
                                <RouterAnchor<AppRoute> route=AppRoute::CreateCompany classes="button is-info">
                                    {"Create"}
                                </RouterAnchor<AppRoute>>
                            },
                            false => html!{},
                        }}
                        <button class="button" onclick={onclick_change_view} >
                          <span class={"icon is-small"}>
                            <i class={class_for_icon}></i>
                          </span>
                        </button>
                    </div>
                </div>
              </div>
              <div class={class_for_list}>
                {for self.list.iter().map(|x| self.show_card(&x))}
              </div>
            </div>
        }
    }
}

impl CatalogCompanies {
    fn show_card(
        &self,
        show_company: &ShowCompanyShort,
    ) -> Html {
        html!{<ListItemCompany
            data={show_company.clone()}
            show_list={self.show_type == ListState::List}
        />}
    }
}
