mod list_item;

pub use list_item::ListItemStandard;
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
  UUID, ShowStandardShort, StandardsQueryArg
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct GetStandardsShortList;

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    GetList,
}

#[derive(PartialEq, Eq)]
pub enum ListState {
    List,
    Box,
}

pub struct CatalogStandards {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    show_type: ListState,
    list: Vec<ShowStandardShort>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_create_btn: bool,
    pub arguments: Option<StandardsQueryArg>,
}

impl Component for CatalogStandards {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            show_type: ListState::List,
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
                let ipt_standards_arg = match &self.props.arguments {
                    Some(ref arg) => Some(get_standards_short_list::IptStandardsArg {
                        standardsUuids: arg.standards_uuids.clone(),
                        companyUuid: arg.company_uuid.to_owned(),
                        favorite: arg.favorite,
                        limit: arg.limit,
                        offset: arg.offset,
                    }),
                    None => None,
                };
                spawn_local(async move {
                    let res = make_query(GetStandardsShortList::build_query(get_standards_short_list::Variables {
                        ipt_standards_arg
                    })).await.unwrap();
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
              let data: Value = serde_json::from_str(res.as_str()).unwrap();
              let res_value = data.as_object().unwrap().get("data").unwrap();

              debug!("res value: {:#?}", res_value);

              match res_value.is_null() {
                  false => {
                      let result: Vec<ShowStandardShort> = serde_json::from_value(res_value.get("standards").unwrap().clone()).unwrap();

                      debug!("UpdateList result: {:?}", result);

                      self.list = result;
                  },
                  true => self.error = Some(get_error(&data)),
              }
          },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This standard has no properties so we will always return "false".
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
            <div class="standardsBox" >
              <ListErrors error=self.error.clone()/>
              <div class="level" >
                <div class="level-left ">
                {match &self.props.show_create_btn {
                    true => html! {
                        <RouterAnchor<AppRoute> route=AppRoute::CreateStandard >
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

impl CatalogStandards {
    fn show_card(
        &self,
        show_standard: &ShowStandardShort,
    ) -> Html {
        html! {<ListItemStandard data={show_standard.clone()}
            show_list={self.show_type == ListState::List}
        />}
    }
}
