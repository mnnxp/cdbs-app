mod list_item;

pub use list_item::ListItem;

use yew::{Component, Context, html, html::Scope, Html, Properties};
use yew_router::prelude::Link;
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;
use crate::error::Error;
use crate::fragments::ListState;
use crate::fragments::list_errors::ListErrors;
use crate::routes::AppRoute::{self, CreateComponent};
use crate::types::{ComponentsQueryArg, ShowComponentShort, UUID};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::component::{
    GetComponentsShortList, get_components_short_list,
    AddComponentFav, add_component_fav,
    DeleteComponentFav, delete_component_fav,
};

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    AddFav(UUID),
    DelFav(UUID),
    GetList,
    ResponseError(Error),
}

pub struct CatalogComponents {
    error: Option<Error>,
    show_type: ListState,
    list: Vec<ShowComponentShort>,
    show_create_btn: bool,
    arguments: Option<ComponentsQueryArg>,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_create_btn: bool,
    pub arguments: Option<ComponentsQueryArg>,
}

impl Component for CatalogComponents {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            show_type: ListState::get_from_storage(),
            list: Vec::new(),
            show_create_btn: ctx.props().show_create_btn,
            arguments: ctx.props().arguments.clone(),
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
                let ipt_components_arg = match &ctx.props().arguments {
                    Some(ref arg) => Some(get_components_short_list::IptComponentsArg {
                        components_uuids: arg.components_uuids.clone(),
                        company_uuid: arg.company_uuid.to_owned(),
                        standard_uuid: arg.standard_uuid.to_owned(),
                        user_uuid: arg.user_uuid.to_owned(),
                        favorite: arg.favorite,
                        limit: arg.limit,
                        offset: arg.offset,
                    }),
                    None => None,
                };
                spawn_local(async move {
                    let res = make_query(GetComponentsShortList::build_query(
                        get_components_short_list::Variables { ipt_components_arg },
                    ))
                    .await
                    .unwrap();
                    debug!("GetList res: {:?}", res);
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
                self.list = resp_parsing(res, "components")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
            },
            Msg::AddFav(component_uuid) => {
                spawn_local(async move {
                    make_query(AddComponentFav::build_query(add_component_fav::Variables {
                        component_uuid,
                    }))
                    .await
                    .unwrap();
                    // debug!("AddFav res: {:?}", res);
                    link.send_message(Msg::GetList);
                });
            },
            Msg::DelFav(component_uuid) => {
                spawn_local(async move {
                    make_query(DeleteComponentFav::build_query(
                        delete_component_fav::Variables { component_uuid },
                    ))
                    .await
                    .unwrap();
                    // debug!("DelFav res: {:?}", res);
                    link.send_message(Msg::GetList);
                });
            },
            Msg::ResponseError(err) => self.error = Some(err),
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let flag_change = match (&self.arguments, &ctx.props().arguments) {
            (Some(self_arg), Some(arg)) => self_arg == arg,
            (None, None) => true,
            _ => false,
        };

        debug!("self_arg == arg: {}", flag_change);

        if self.show_create_btn == ctx.props().show_create_btn && flag_change {
            // debug!("if change");
            false
        } else {
            self.show_create_btn = ctx.props().show_create_btn;
            self.arguments = ctx.props().arguments.clone();
            ctx.link().send_message(Msg::GetList);
            // debug!("else change");
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_change_view = ctx.link().callback(|_| Msg::SwitchShowType);

        let (class_for_icon, class_for_list) = match self.show_type {
            ListState::Box => ("fas fa-bars", "flex-box"),
            ListState::List => ("fas fa-th-large", ""),
        };

        html! {
            <div class="componentsBox" >
              <ListErrors error={self.error.clone()}/>
              <div class="level" >
                <div class="level-left ">
                </div>
                <div class="level-right">
                    <div class="buttons">
                        {match &ctx.props().show_create_btn {
                          true => html!{
                              <Link<AppRoute> to={CreateComponent} classes="button is-info">
                                  { get_value_field(&45) } // Create
                              </Link<AppRoute>>
                          },
                          false => html!{},
                        }}
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
              </div>
              <div class={class_for_list}>
                {for self.list.iter().map(|x| self.show_card(ctx.link(), &x))}
              </div>
            </div>
        }
    }
}

impl CatalogComponents {
    fn show_card(
        &self,
        link: &Scope<Self>,
        show_comp: &ShowComponentShort
    ) -> Html {
        let target_uuid_add = show_comp.uuid.clone();
        let onclick_add_fav = link.callback(move |_| Msg::AddFav(target_uuid_add.clone()));
        let target_uuid_del = show_comp.uuid.clone();
        let onclick_del_fav = link.callback(move |_| Msg::DelFav(target_uuid_del.clone()));

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
