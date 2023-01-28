mod list_item;

pub use list_item::ListItemUser;

use yew::{Component, Callback, Context, html, Html, Properties};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowUserShort, UsersQueryArg};
use crate::fragments::ListState;
use crate::gqls::make_query;
use crate::gqls::user::{
    GetUsersShortList, get_users_short_list
};
use crate::services::resp_parsing;

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    GetList,
    ResponseError(Error),
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub arguments: Option<UsersQueryArg>,
    pub callback_change: Option<Callback<bool>>,
}

pub struct CatalogUsers {
    error: Option<Error>,
    show_type: ListState,
    list: Vec<ShowUserShort>
}

impl Component for CatalogUsers {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            show_type: ListState::get_from_storage(),
            list: Vec::new()
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
                let ipt_users_arg = match &ctx.props().arguments {
                    Some(ref arg) => Some(get_users_short_list::IptUsersArg {
                        users_uuids: arg.users_uuids.clone(),
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
                self.list = resp_parsing(res, "users")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
            },
            Msg::ResponseError(err) => self.error = Some(err),
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_change_view = ctx.link().callback(|_|Msg::SwitchShowType);
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
              <ListErrors error={self.error.clone()}/>
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
                {for self.list.iter().map(|x| self.show_card(&x, ctx.props().callback_change.clone()))}
              </div>
            </div>
        }
    }
}

impl CatalogUsers {
    fn show_card(
        &self,
        show_comp: &ShowUserShort,
        callback_change: Option<Callback<bool>>,
    ) -> Html {
        html!{
            <ListItemUser
                data={show_comp.clone()}
                show_list={self.show_type == ListState::List}
                {callback_change}
            />
        }
    }
}
