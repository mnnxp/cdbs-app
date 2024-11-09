mod list_item;
pub use list_item::ListItemUser;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::fragments::{ListState, list_errors::ListErrors, list_empty::ListEmpty};
use crate::services::resp_parsing;
use crate::types::{ShowUserShort, UsersQueryArg};
use crate::gqls::make_query;
use crate::gqls::user::{GetUsersShortList, get_users_short_list};

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    GetList,
    ResponseError(Error),
    ClearError,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub arguments: Option<UsersQueryArg>,
}

pub struct CatalogUsers {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
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
            show_type: ListState::get_from_storage(),
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
            Msg::SwitchShowType => {
                match self.show_type {
                    ListState::Box => self.show_type = ListState::List,
                    _ => self.show_type = ListState::Box,
                }
                ListState::set_to_storage(&self.show_type);
            },
            Msg::GetList => {
                let ipt_users_arg = match &self.props.arguments {
                    Some(ref arg) => Some(get_users_short_list::IptUsersArg {
                        usersUuids: arg.users_uuids.clone(),
                        subscribers: arg.subscribers,
                        favorite: arg.favorite,
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
                match resp_parsing(res, "users") {
                    Ok(result) => self.list = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("users list: {:?}", self.list);
            }
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
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
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_change_view = self.link.callback(|_|Msg::SwitchShowType);
        let class_for_icon: &str;
        let mut class_for_list = "";
        match self.show_type {
            ListState::Box => {
                class_for_icon = "fas fa-bars";
                class_for_list = "flex-box";
            },
            ListState::List => class_for_icon = "fas fa-th-large",
        };

        html!{
            <div class="usersBox" >
              <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
              <div class="level" >
                <div class="level-left ">
                </div>
                <div class="level-right">
                  <button class="button" onclick={onclick_change_view} >
                    <span class={"icon is-small"}>
                      <i class={class_for_icon}></i>
                    </span>
                  </button>
                </div>
              </div>
              {if self.list.is_empty() {
                html!{<ListEmpty />}
              } else { html!{
                <div class={class_for_list}>
                  {for self.list.iter().map(|x| self.show_card(&x))}
                </div>
              }}}
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
            <ListItemUser
                data={show_comp.clone()}
                show_list={self.show_type == ListState::List}
            />
        }
    }
}
