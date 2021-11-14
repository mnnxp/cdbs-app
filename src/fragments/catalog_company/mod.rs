mod list_item;

use list_item::ListItem;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use chrono::NaiveDateTime;
use crate::gqls::make_query;
use crate::routes::AppRoute;
use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{
  UUID, ShowCompanyShort, CompaniesQueryArg
};

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
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct AddCompanyFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct DeleteCompanyFav;


pub enum Msg {
    AddOne,
    SwitchShowType,
    UpdateList(String),
    AddFav(UUID),
    DelFav(UUID),
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
    value: i64,
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
            value: 0,
            show_type: ListState::List,
            list: Vec::new()
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render {
            spawn_local(async move {
                link.send_message(Msg::GetList);
            });
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
            },
            Msg::SwitchShowType => {
                match self.show_type {
                    ListState::Box => self.show_type = ListState::List,
                    _ => self.show_type = ListState::Box,
                }
            },
            Msg::GetList => {
                let arguments = match &self.props.arguments {
                    Some(ref arg) => Some(get_companies_short_list::IptCompaniesArg {
                        companiesUuids: arg.companies_uuids.clone(),
                        userUuid: arg.user_uuid.to_owned(),
                        favorite: arg.favorite,
                        limit: arg.limit,
                        offset: arg.offset,
                    }),
                    None => None,
                };
                spawn_local(async move {
                    let res = make_query(GetCompaniesShortList::build_query(
                        get_companies_short_list::Variables {
                            arguments
                    })).await.unwrap();
                    debug!("GetList res: {:?}", res);
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
              let data: Value = serde_json::from_str(res.as_str()).unwrap();
              let res_value = data.as_object().unwrap().get("data").unwrap();

              debug!("res value: {:#?}", res_value);

              match res_value.is_null() {
                  false => {
                      let result: Vec<ShowCompanyShort> = serde_json::from_value(res_value.get("companies").unwrap().clone()).unwrap();

                      debug!("UpdateList result: {:?}", result);

                      self.list = result;
                  },
                  true => {
                      self.error = Some(get_error(&data));
                  },
              }
          },
          Msg::AddFav(company_uuid) => {
              spawn_local(async move {
                  let res = make_query(AddCompanyFav::build_query(add_company_fav::Variables{
                    company_uuid
                  })).await.unwrap();
                  debug!("AddFav res: {:?}", res);
                  link.send_message(Msg::GetList);
              });
          },
          Msg::DelFav(company_uuid) => {
              spawn_local(async move {
                  let res = make_query(DeleteCompanyFav::build_query(delete_company_fav::Variables{
                    company_uuid
                  })).await.unwrap();
                  debug!("DelFav res: {:?}", res);
                  link.send_message(Msg::GetList);
              });
          }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This company has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        // let list = self.list.clone();

        let onclick_change_view = self
            .link
            .callback(|_|Msg::SwitchShowType);

        let class_for_icon: &str;
        let mut class_for_list = "";
        match self.show_type {
            ListState::Box => {
                class_for_icon = "fas fa-bars";
                class_for_list = "flex-box";
            },
            ListState::List => {
                class_for_icon = "fas fa-th-large";
            },
        };

        html! {
            <div class="companiesBox" >
              <ListErrors error=self.error.clone()/>
              <div class="level" >
                <div class="level-left ">
                {match &self.props.show_create_btn {
                    true => html! {
                        <RouterAnchor<AppRoute> route=AppRoute::CreateTender >
                          <button class="button is-info" >{"Create"}</button>
                        </RouterAnchor<AppRoute>>
                    },
                    false => html! {},
                }}
                </div>
                <div class="level-right">
                  <div class="select">
                    <select>
                      <option>{"Select dropdown"}</option>
                      <option>{"With options"}</option>
                    </select>
                  </div>
                  <button class="button" onclick={onclick_change_view} >
                    <span class={"icon is-small"}>
                      <i class={class_for_icon}></i>
                    </span>
                  </button>
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
        let target_uuid_add = show_company.uuid.clone();
        let target_uuid_del = show_company.uuid.clone();

        let onclick_add_fav = self.link.callback(move |_|
                Msg::AddFav(target_uuid_add.clone())
            );

        let onclick_del_fav = self.link.callback(move |_|
                Msg::DelFav(target_uuid_del.clone())
            );

        html! {
            <ListItem data={show_company.clone()}
                show_list={self.show_type == ListState::List}
                // triggerFav={triggerFav}
                add_fav={onclick_add_fav}
                del_fav={onclick_del_fav}
                />
        }
    }
}
