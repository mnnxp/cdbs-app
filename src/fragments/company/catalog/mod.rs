mod list_item;

pub use list_item::ListItemCompany;

use yew::{Component, Context, html, Html, Properties};
use yew_router::prelude::RouterAnchor;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;

use crate::routes::AppRoute::{self, CreateCompany};
use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{ShowCompanyShort, CompaniesQueryArg};
use crate::services::get_value_field;
use crate::gqls::make_query;
use crate::gqls::company::{GetCompaniesShortList, get_companies_short_list};
use crate::fragments::ListState;

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    GetList
}

pub struct CatalogCompanies {
    error: Option<Error>,
    show_type: ListState,
    list: Vec<ShowCompanyShort>,
    arguments: Option<CompaniesQueryArg>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_create_btn: bool,
    pub arguments: Option<CompaniesQueryArg>,
}

impl Component for CatalogCompanies {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            show_type: ListState::get_from_storage(),
            list: Vec::new(),
            arguments: ctx.props().arguments,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::GetList);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::SwitchShowType => {
                match self.show_type {
                    ListState::Box => self.show_type = ListState::List,
                    _ => self.show_type = ListState::Box,
                }
                ListState::set_to_storage(&self.show_type);
            },
            Msg::GetList => {
                let ipt_companies_arg = match &ctx.props().arguments {
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let flag_change = match (&self.arguments, &ctx.props().arguments) {
            (Some(self_arg), Some(arg)) => self_arg == arg,
            (None, None) => true,
            _ => false,
        };

        // debug!("self_arg == arg: {}", flag_change);

        if self.show_create_btn == ctx.props().show_create_btn && flag_change{
            // debug!("if change");
            false
        } else {
            self.show_create_btn = ctx.props().show_create_btn;
            self.arguments = ctx.props().arguments;
            ctx.link().send_message(Msg::GetList);
            // debug!("else change");
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_change_view = ctx.link().callback(|_|Msg::SwitchShowType);

        let (class_for_icon, class_for_list) = match self.show_type {
            ListState::Box => ("fas fa-bars", "flex-box"),
            ListState::List => ("fas fa-th-large", ""),
        };

        html!{
            <div class="companiesBox" >
              <ListErrors error={self.error.clone()}/>
              <div class="level" >
                <div class="level-left">
                </div>
                <div class="level-right">
                    <div class="buttons">
                        {match &ctx.props().show_create_btn {
                            true => html!{
                                <RouterAnchor<AppRoute> route={CreateCompany} classes="button is-info">
                                    { get_value_field(&45) } // Create
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
