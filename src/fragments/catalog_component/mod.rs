// mod box_item;
mod list_item;

use crate::routes::AppRoute;

// use box_item::BoxItem;
use list_item::ListItem;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
// use chrono::NaiveDateTime;
use crate::gqls::make_query;
// use serde::{Deserialize, Serialize};
use crate::error::{Error, get_error};
use crate::types::{
  UUID, NaiveDateTime, ShowComponentShort, ComponentsQueryArg
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetComponentsShortList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct AddComponentFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct DeleteComponentFav;


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

pub struct CatalogComponents {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    value: i64,
    show_type: ListState,
    list: Vec<ShowComponentShort>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_create_btn: bool,
    pub arguments: Option<ComponentsQueryArg>,
}

impl Component for CatalogComponents {
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
                    Some(ref arg) => Some(get_components_short_list::IptComponentsArg {
                        componentsUuids: arg.components_uuids.clone(),
                        companyUuid: arg.company_uuid.to_owned(),
                        standardUuid: arg.standard_uuid.to_owned(),
                        userUuid: arg.user_uuid.to_owned(),
                        favorite: arg.favorite,
                        limit: arg.limit,
                        offset: arg.offset,
                    }),
                    None => None,
                };
                spawn_local(async move {
                    let res = make_query(GetComponentsShortList::build_query(
                        get_components_short_list::Variables {
                            arguments
                    })).await.unwrap();
                    debug!("GetList res: {:?}", res);
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
              let data: Value = serde_json::from_str(res.as_str()).unwrap();
              let res_value = data.as_object().unwrap().get("data").unwrap();

              match res_value.is_null() {
                  false => {
                      let result: Vec<ShowComponentShort> = serde_json::from_value(res_value.get("components").unwrap().clone()).unwrap();

                      debug!("UpdateList result: {:?}", result);

                      self.list = result;
                  },
                  true => {
                      self.error = Some(get_error(&data));
                  },
              }
          },
          Msg::AddFav(component_uuid) => {
              spawn_local(async move {
                  let res = make_query(AddComponentFav::build_query(add_component_fav::Variables{
                    component_uuid
                  })).await.unwrap();
                  debug!("AddFav res: {:?}", res);
                  link.send_message(Msg::GetList);
              });
          },
          Msg::DelFav(component_uuid) => {
              spawn_local(async move {
                  let res = make_query(DeleteComponentFav::build_query(delete_component_fav::Variables{
                    component_uuid
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

        html! {
            <div class="tendersBox" >
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

impl CatalogComponents {
    fn show_card(
        &self,
        show_comp: &ShowComponentShort,
    ) -> Html {
        let target_uuid_add = show_comp.uuid.clone();
        let target_uuid_del = show_comp.uuid.clone();

        let onclick_add_fav = self.link.callback(move |_|
                Msg::AddFav(target_uuid_add.clone())
            );

        let onclick_del_fav = self.link.callback(move |_|
                Msg::DelFav(target_uuid_del.clone())
            );

        html! {
            <ListItem data={show_comp.clone()}
                show_list={self.show_type == ListState::List}
                // triggerFav={triggerFav}
                add_fav={onclick_add_fav}
                del_fav={onclick_del_fav}
                />
        }
    }
}
