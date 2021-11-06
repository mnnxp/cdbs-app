mod boxItem;
mod listItem;

use crate::routes::AppRoute;

use boxItem::BoxItem;
use listItem::ListItem;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::{Value};
use crate::types::*;
// use chrono::NaiveDateTime;
use crate::gqls::make_query;
use serde::{Deserialize, Serialize};
use crate::types::{
  UUID, SelfUserInfo, UserInfo, SlimUser, Program, Region, Certificate, UserCertificate, NaiveDateTime
};


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetRawList;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShowedComponent {
  uuid : UUID,
  name: String,
  description: String,
  typeAccessId: usize,
  isFollowed: bool,
  isBase: bool,
  updatedAt: NaiveDateTime,
  componentSuppliers: Vec<Supplier>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Supplier{
  supplier:SlimCompany,
  componentUuid: UUID,
  description: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlimCompany{
  uuid: UUID,
  shortname: String,
  isSupplier: bool
}

pub struct Catalog {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    showType: ListState,
    list: Vec<ShowedComponent>
}

impl Component for Catalog {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            showType: ListState::List,
            list: Vec::new()
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render {
            spawn_local(async move {
                let res = make_query(GetRawList::build_query(get_raw_list::Variables)).await.unwrap();
                crate::yewLog!(res);
                link.send_message(Msg::UpdateList(res));
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
            }
            Msg::SwitchShowType => {
                match self.showType {
                    ListState::Box => self.showType = ListState::List,
                    _ => self.showType = ListState::Box,
                }
            }
            Msg::GetList => {
              spawn_local(async move {
                  let res = make_query(GetRawList::build_query(get_raw_list::Variables)).await.unwrap();
                  crate::yewLog!(res);
                  link.send_message(Msg::UpdateList(res));
              });
            }
            Msg::UpdateList(res) => {
              let data: Value = serde_json::from_str(res.as_str()).unwrap();
              let res = data.as_object().unwrap().get("data").unwrap().get("components").unwrap();
              self.list  = serde_json::from_value(res.clone()).unwrap();
              crate::yewLog!(res);crate::yewLog!(self.list);
              // self.regions =
              //     serde_json::from_value(res.get("regions").unwrap().clone()).unwrap();
              // self.programs =
              //     serde_json::from_value(res.get("programs").unwrap().clone()).unwrap();
              // ConsoleService::info(format!("Update: {:?}", self.programs).as_ref());
          }
          Msg::AddFav(component_uuid) => {
            spawn_local(async move {
                let res = make_query(AddComponentFav::build_query(add_component_fav::Variables{
                  component_uuid
                })).await.unwrap();
                crate::yewLog!(res);
                link.send_message(Msg::GetList);
            });
          }
          Msg::DelFav(component_uuid) => {
            spawn_local(async move {
                let res = make_query(DeleteComponentFav::build_query(delete_component_fav::Variables{
                  component_uuid
                })).await.unwrap();
                crate::yewLog!(res);
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
        let list = self.list.clone();
        html! {
            <div class="tendersBox" >
              <div class="level" >
                <div class="level-left ">
                <RouterAnchor<AppRoute> route=AppRoute::CreateTender >
                  <button class="button is-info" >{"Create"}</button>
                </RouterAnchor<AppRoute>>
                </div>
                <div class="level-right">
                  <div class="select">
                    <select>
                      <option>{"Select dropdown"}</option>
                      <option>{"With options"}</option>
                    </select>
                  </div>
                  <button class="button" onclick={self.link.callback(|_|Msg::SwitchShowType)} >
                    <span class="icon is-small">
                      <i class=classes!("fas", if self.showType == ListState::Box { "fa-bars" } else { "fa-th-large" } )></i>
                    </span>
                  </button>
                </div>
              </div>
              <div class=classes!( if self.showType == ListState::Box { "flex-box" } else { " " } ) >
                {for list.iter().map(|x| {
                  let backup = x.clone();
                  let uuid = x.uuid.clone();
                  let uuid1 = x.uuid.clone();
                  // let addMsg = Msg::AddFav(uuid.clone());
                  // let delMsg = Msg::DelFav(uuid.clone());
                  // let addEnv = move |ev: MouseEvent| {ev.prevent_default(); addMsg };
                  // let delEnv = move |ev: MouseEvent| {ev.prevent_default(); delMsg };
                  // let triggerFav = self.link.callback( );
                  html! { <ListItem data={x.clone()} showList={self.showType == ListState::List} 
                    // triggerFav={triggerFav} 
                    addFav={self.link.callback(move |_| Msg::AddFav(uuid.clone()) )}
                    delFav={self.link.callback(move |_| Msg::DelFav(uuid1.clone()) )}
                   /> }
                })}
              </div>
            </div>
        }
    }
}
