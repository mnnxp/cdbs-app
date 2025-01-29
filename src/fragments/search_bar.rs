use yew::{
    classes, html, Component, ComponentLink, Html, Properties, ShouldRender, InputData
};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::gqls::make_query;
use crate::gqls::component::{SearchByComponents, search_by_components};
use log::debug;
use crate::fragments::list_errors::ListErrors;
use crate::error::Error;
use crate::services::{resp_parsing, get_value_field};
use crate::types::ShowComponentShort;
use web_sys::KeyboardEvent;
use yew::services::timeout::{TimeoutService, TimeoutTask};
use std::time::Duration;

#[derive(PartialEq)]
pub enum RequestStatus {
    None,
    Loading,
    Success,
    Error,
}

pub struct SearchBar {
    error: Option<Error>,
    link: ComponentLink<Self>,
    search_value: String,
    menu_arr: Vec<ShowComponentShort>,
    request_status: RequestStatus,
    is_focused: bool,
    debounce_timeout: Option<TimeoutTask>,
    focus_timeout: Option<TimeoutTask>,
}

#[derive(Properties, Clone)]
pub struct Props {}

#[derive(Clone)]
pub enum Msg {
    InputSearch(String),
    GetSearchByComponentsResult(String),
    Search,
    Ignore,
    SetFocus(bool),
    KeyPress(KeyboardEvent),
    SetFocusAfterDelay(bool),
    AutoSearch,
    ResponseError(Error),
    ClearError,
}

impl Default for search_by_components::IptSearchArg {
  fn default() -> Self {
      search_by_components::IptSearchArg {
          search: "".to_string(),
          byKeywords: false,
          byParams: false,
          bySpecs: false,
          companyUuid: None,
          favorite: false,
          standardUuid: None,
          userUuid: None,
      }
  }
}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            search_value: "".to_string(),
            menu_arr: vec![],
            request_status: RequestStatus::None,
            is_focused: false,
            debounce_timeout: None,
            focus_timeout: None
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::InputSearch(value) => {
                self.search_value = value;
                
                self.debounce_timeout = None;

                if !self.search_value.is_empty() {
                    let timeout = TimeoutService::spawn(
                        Duration::from_millis(1000),
                        link.callback(|_| Msg::AutoSearch)
                    );
                    self.debounce_timeout = Some(timeout);
                }
            },
            Msg::AutoSearch => {
                self.debounce_timeout = None;
                if !self.search_value.is_empty() {
                    link.send_message(Msg::Search);
                }
            },
            Msg::Search => {
                self.request_status = RequestStatus::Loading;
                let ipt_arg = search_by_components::IptSearchArg::default();
                let ipt_search_arg = search_by_components::IptSearchArg {
                  search: self.search_value.clone(),
                  ..ipt_arg
                };

                spawn_local(async move {
                  let res = make_query(SearchByComponents::build_query(search_by_components::Variables {
                    ipt_search_arg
                  })).await.unwrap();

                  debug!("search result: {:?}", res);

                  link.send_message(Msg::GetSearchByComponentsResult(res));
                });
                debug!("search: {:?}", self.search_value.clone());
            },
            Msg::GetSearchByComponentsResult(res) => {
                debug!("search result: {:?}", res);
                match resp_parsing::<Vec<ShowComponentShort>>(res, "searchByComponents") {
                    Ok(search_result) => {
                        self.menu_arr = search_result;
                        self.request_status = RequestStatus::Success;
                    },
                    Err(err) => {
                        self.menu_arr.clear();
                        self.request_status = RequestStatus::Error;
                        link.send_message(Msg::ResponseError(err))
                    },
                }
            },
            Msg::Ignore => {},
            Msg::SetFocus(focused) => {
                let timeout = TimeoutService::spawn(
                  Duration::from_millis(200),
                  link.callback(move |_| Msg::SetFocusAfterDelay(focused))
                );
                self.focus_timeout = Some(timeout);
            },
            Msg::SetFocusAfterDelay(focused) => {
                self.is_focused = focused;
                debug!("Input focus state: {}", focused);
            },
            Msg::KeyPress(event) => {
                if event.key() == "Enter" {
                    link.send_message(Msg::Search);
                }
            }
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        
        true
    }

    fn view(&self) -> Html {
        let show_dropdown = if self.request_status == RequestStatus::Success && self.is_focused { "is-active" } else { "" };
        let is_loading = if self.request_status == RequestStatus::Loading { "is-loading" } else { "" };
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html! {
          <div class="field has-addons is-relative">
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <div class={classes!("control", "has-icons-left", "has-icons-right", is_loading)} style="width: 100%;">
              <input class="input" style="width: 100%;" oninput={self.link.callback(|ev: InputData| Msg::InputSearch(ev.value))} onfocus={self.link.callback(|_| Msg::SetFocus(true))} onblur={self.link.callback(|_| Msg::SetFocus(false))} onkeypress={self.link.callback(|e: KeyboardEvent| Msg::KeyPress(e))} type="email" placeholder="Input Search" />
              <span class="icon is-small is-left">
                <i class="fas fa-search fa-xs"></i>
              </span>
            </div>
            <div class={"control"}>
              <button class="button is-info search-button" onclick={self.link.callback(|_| Msg::Search)}>
                {get_value_field(&349)} // Search
              </button>
            </div>
            <div class={classes!("dropdown", "is-absolute", show_dropdown)}>
              <div class="dropdown-menu" id="component-dropdown-menu" role="menu">
                <div class="dropdown-content">
                  {
                    if self.request_status == RequestStatus::Success && self.menu_arr.is_empty() {
                        html! {
                            <div class="dropdown-item has-text-grey">
                                <span class="icon-text">
                                    <span class="icon">
                                        <i class="fas fa-search-minus"></i>
                                    </span>
                                    <span>{get_value_field(&350)}</span> // No results
                                </span>
                            </div>
                        }
                    } else {
                        html! {
                            {for self.menu_arr.iter().map(|x| {
                                let uuid = x.uuid.clone();
                                html!{
                                    <a 
                                        href={format!("#/component/{}", uuid)} 
                                        class="dropdown-item"
                                    > 
                                        {x.name.clone()} 
                                    </a>
                                }
                            })}
                        }
                    }
                  }
                </div>
              </div>
            </div>
          </div>
        }
    }
}
