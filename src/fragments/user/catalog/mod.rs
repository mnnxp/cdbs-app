mod list_item;

pub use list_item::ListItemUser;

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::gqls::make_query;
use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowUserShort, UsersQueryArg};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetUsersShortList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct AddUserFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct DeleteUserFav;

pub enum Msg {
    AddOne,
    SwitchShowType,
    UpdateList(String),
    GetList
}

#[derive(PartialEq, Eq)]
pub enum ListState {
    List,
    Box,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub arguments: Option<UsersQueryArg>,
}

pub struct CatalogUsers {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    value: i64,
    show_type: ListState,
    list: Vec<ShowUserShort>
}

impl Component for CatalogUsers {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            value: 0,
            show_type: ListState::Box,
            list: Vec::new()
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render {
            link.send_message(Msg::GetList);
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
                let ipt_users_arg = match &self.props.arguments {
                    Some(ref arg) => Some(get_users_short_list::IptUsersArg {
                        usersUuids: arg.users_uuids.clone(),
                        subscribers: arg.subscribers,
                        favorite: arg.favorite,
                        limit: arg.limit,
                        offset: arg.offset,
                    }),
                    None => None,
                };
                spawn_local(async move {
                    let res = make_query(GetUsersShortList::build_query(get_users_short_list::Variables {
                        ipt_users_arg
                    })).await.unwrap();
                    debug!("users query: {}", res);
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
              let data: Value = serde_json::from_str(res.as_str()).unwrap();
              let res_value = data.as_object().unwrap().get("data").unwrap();

              match res_value.is_null() {
                  false => {
                      let result: Vec<ShowUserShort> = serde_json::from_value(res_value.get("users").unwrap().clone()).unwrap();

                      debug!("users list: {:?}", result);

                      self.list = result;
                  },
                  true => {
                      self.error = Some(get_error(&data));
                  },
              }
          }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
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

        html!{
            <div class="usersBox" >
              <ListErrors error=self.error.clone()/>
              <div class="level" >
                <div class="level-left ">
                </div>
                <div class="level-right">
                  // <div class="select">
                  //   <select>
                  //     <option>{"Select dropdown"}</option>
                  //     <option>{"With options"}</option>
                  //   </select>
                  // </div>
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

impl CatalogUsers {
    fn show_card(
        &self,
        show_comp: &ShowUserShort,
    ) -> Html {
        html!{
            <ListItemUser data={show_comp.clone()}
                show_list={self.show_type == ListState::List}
                />
        }
    }
}
