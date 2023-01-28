mod list_item;

pub use list_item::ListItemStandard;

use yew::{Component, Context, html, Html, Properties};
use yew_router::prelude::Link;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::routes::AppRoute::{self, CreateStandard};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{ShowStandardShort, StandardsQueryArg};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::standard::{GetStandardsShortList, get_standards_short_list};
use crate::fragments::ListState;

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    GetList,
    ResponseError(Error),
}

pub struct CatalogStandards {
    error: Option<Error>,
    show_type: ListState,
    list: Vec<ShowStandardShort>
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_create_btn: bool,
    pub arguments: Option<StandardsQueryArg>,
}

impl Component for CatalogStandards {
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
                let ipt_standards_arg = match &ctx.props().arguments {
                    Some(ref arg) => Some(get_standards_short_list::IptStandardsArg {
                        standards_uuids: arg.standards_uuids.clone(),
                        company_uuid: arg.company_uuid.to_owned(),
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
                self.list = resp_parsing(res, "standards")
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
        // This standard has no properties so we will always return "false".
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_change_view = ctx.link().callback(|_|Msg::SwitchShowType);
        let (class_for_icon, class_for_list) = match self.show_type {
            ListState::Box => ("fas fa-bars", "flex-box"),
            ListState::List => ("fas fa-th-large", ""),
        };

        html!{
            <div class="standardsBox" >
              <ListErrors error={self.error.clone()}/>
              <div class="level" >
                <div class="level-left">
                </div>
                <div class="level-right">
                    <div class="buttons">
                        {match &ctx.props().show_create_btn {
                            true => html!{
                                <Link<AppRoute> to={CreateStandard} classes="button is-info">
                                    { get_value_field(&45) } // Create
                                </Link<AppRoute>>
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

impl CatalogStandards {
    fn show_card(
        &self,
        show_standard: &ShowStandardShort,
    ) -> Html {
        html!{<ListItemStandard data={show_standard.clone()}
            show_list={self.show_type == ListState::List}
        />}
    }
}
