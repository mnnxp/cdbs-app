mod list_item;
pub use list_item::ListItemStandard;

use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use yew_router::prelude::RouterAnchor;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
// use log::debug;

use crate::fragments::buttons::ft_change_view_btn;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{list_errors::ListErrors, list_empty::ListEmpty};
use crate::types::{ShowStandardShort, StandardsQueryArg};
use crate::services::{LocaleKey, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::standard::{GetStandardsShortList, get_standards_short_list};
use crate::fragments::ListState;

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    GetList,
    ResponseError(Error),
    ClearError,
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
            show_type: ListState::get_from_storage(),
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
                ListState::set_to_storage(&self.show_type);
            },
            Msg::GetList => {
                let ipt_standards_arg = self.props.arguments.as_ref().map(|arg| {
                    get_standards_short_list::IptStandardsArg {
                        standardsUuids: arg.standards_uuids.clone(),
                        companyUuid: arg.company_uuid.to_owned(),
                        favorite: arg.favorite,
                    }
                });
                spawn_local(async move {
                    let res = make_query(GetStandardsShortList::build_query(get_standards_short_list::Variables {
                        ipt_standards_arg
                    })).await.unwrap();
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
              match resp_parsing::<Vec<ShowStandardShort>>(res, "standards") {
                Ok(result) => self.list = result,
                Err(err) => link.send_message(Msg::ResponseError(err)),
              }
          },
          Msg::ResponseError(err) => self.error = Some(err),
          Msg::ClearError => self.error = None,
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
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_change_view = self.link.callback(|_|Msg::SwitchShowType);

        html!{
            <div id="standards-box">
              <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
              <div class="level" >
                <div class="level-left">
                </div>
                <div class="level-right">
                    <div class="buttons">
                        {match &self.props.show_create_btn {
                            true => html!{
                                <RouterAnchor<AppRoute> route={AppRoute::CreateStandard} classes="button is-info">
                                    {LocaleKey::CreateStandard.get_value()}
                                </RouterAnchor<AppRoute>>
                            },
                            false => html!{},
                        }}
                        {ft_change_view_btn(onclick_change_view, &self.show_type)}
                    </div>
                </div>
              </div>
              {if self.list.is_empty() {
                html!{<ListEmpty />}
              } else { html!{
                <div class={self.show_type.get_container_class()}>
                  {for self.list.iter().map(|x| self.show_card(&x))}
                </div>
              }}}
            </div>
        }
    }
}

impl CatalogStandards {
    fn show_card(
        &self,
        show_standard: &ShowStandardShort,
    ) -> Html {
        html!{
            <ListItemStandard
                data={show_standard.clone()}
                show_list={self.show_type == ListState::List}
                />
        }
    }
}
