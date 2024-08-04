mod list_item;
pub use list_item::ListItemCompany;

use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use yew_router::prelude::RouterAnchor;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{list_errors::ListErrors, list_empty::ListEmpty};
use crate::types::{ShowCompanyShort, CompaniesQueryArg};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::company::{GetCompaniesShortList, get_companies_short_list};
use crate::fragments::ListState;

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    GetList,
    ResponseError(Error),
    ClearError,
}

pub struct CatalogCompanies {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
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
                let ipt_companies_arg = match &self.props.arguments {
                    Some(ref arg) => Some(get_companies_short_list::IptCompaniesArg {
                        companiesUuids: arg.companies_uuids.clone(),
                        userUuid: arg.user_uuid.to_owned(),
                        favorite: arg.favorite,
                        supplier: arg.supplier,
                        limit: arg.limit,
                        offset: arg.offset,
                    }),
                    None => None,
                };
                spawn_local(async move {
                    let res = make_query(GetCompaniesShortList::build_query(
                        get_companies_short_list::Variables {
                            ipt_companies_arg
                    })).await.unwrap();
                    debug!("GetList res: {:?}", res);
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
              match resp_parsing(res, "companies") {
                  Ok(result) => self.list = result,
                  Err(err) => link.send_message(Msg::ResponseError(err)),
              }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let flag_change = match (&self.props.arguments, &props.arguments) {
            (Some(self_arg), Some(arg)) => self_arg == arg,
            (None, None) => true,
            _ => false,
        };
        // debug!("self_arg == arg: {}", flag_change);
        if self.props.show_create_btn == props.show_create_btn && flag_change {
            // debug!("if change");
            false
        } else {
            self.props.show_create_btn = props.show_create_btn;
            self.props.arguments = props.arguments;
            self.link.send_message(Msg::GetList);
            // debug!("else change");
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_change_view = self.link.callback(|_|Msg::SwitchShowType);
        let (class_for_icon, class_for_list) = match self.show_type {
            ListState::Box => ("fas fa-bars", "flex-box"),
            ListState::List => ("fas fa-th-large", ""),
        };

        html!{
            <div class="companiesBox" >
              <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
              <div class="level" >
                <div class="level-left">
                </div>
                <div class="level-right">
                    <div class="buttons">
                        {match &self.props.show_create_btn {
                            true => html!{
                                <RouterAnchor<AppRoute> route={AppRoute::CreateCompany} classes="button is-info">
                                    {get_value_field(&289)} // Create company
                                </RouterAnchor<AppRoute>>
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

impl CatalogCompanies {
    fn show_card(&self, show_company: &ShowCompanyShort) -> Html {
        html!{
            <ListItemCompany
                data={show_company.clone()}
                show_list={self.show_type == ListState::List}
                />
        }
    }
}
