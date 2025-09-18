use std::time::Duration;
use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender, InputData};
use yew::services::timeout::{TimeoutService, TimeoutTask};
use web_sys::KeyboardEvent;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::fragments::search::SearchArg;
use crate::fragments::{
    list_errors::ListErrors,
    component::CatalogComponents,
    responsive::resizer,
};
use crate::services::{resp_parsing, get_value_field};
use crate::types::{ShowComponentShort, ComponentsQueryArg};
use crate::gqls::make_query;
use crate::gqls::component::{SearchByComponents, search_by_components};

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
    has_props: bool,
    search_arg: SearchArg,
    found_components: Vec<ShowComponentShort>,
    request_status: RequestStatus,
    is_focused: bool,
    debounce_timeout: Option<TimeoutTask>,
    focus_timeout: Option<TimeoutTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub search_arg: Option<SearchArg>,
}

#[derive(Clone)]
pub enum Msg {
    InputSearch(String),
    GetSearchByComponentsResult(String),
    Search,
    SetFocus(bool),
    KeyPress(KeyboardEvent),
    SetFocusAfterDelay(bool),
    AutoSearch,
    ResponseError(Error),
    ClearError,
}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            has_props: props.search_arg.is_some(),
            search_arg: props.search_arg.unwrap_or_default(),
            found_components: vec![],
            request_status: RequestStatus::None,
            is_focused: false,
            debounce_timeout: None,
            focus_timeout: None
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && !self.search_arg.search.is_empty() {
            self.link.send_message(Msg::Search);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::InputSearch(value) => {
                self.search_arg.search = value;
                self.debounce_timeout = None;
                if !self.search_arg.search.is_empty() {
                    let timeout = TimeoutService::spawn(
                        Duration::from_millis(1000),
                        link.callback(|_| Msg::AutoSearch)
                    );
                    self.debounce_timeout = Some(timeout);
                }
            },
            Msg::AutoSearch => {
                self.debounce_timeout = None;
                if !self.search_arg.search.is_empty() {
                    link.send_message(Msg::Search);
                }
            },
            Msg::Search => {
                self.request_status = RequestStatus::Loading;
                if !self.search_arg.search.is_empty() {
                    let ipt_search_arg = search_by_components::IptSearchArg::get_ipt(&self.search_arg);
                    spawn_local(async move {
                        let res = make_query(SearchByComponents::build_query(search_by_components::Variables {
                            ipt_search_arg
                        })).await.unwrap();

                        debug!("search result: {:?}", res);

                        link.send_message(Msg::GetSearchByComponentsResult(res));
                    });
                    debug!("search: {:?}", self.search_arg.search);
                } else {
                    self.request_status = RequestStatus::Success;
                    self.found_components.clear();
                }
            },
            Msg::GetSearchByComponentsResult(res) => {
                debug!("search result: {:?}", res);
                match resp_parsing::<Vec<ShowComponentShort>>(res, "searchByComponents") {
                    Ok(search_result) => {
                        self.found_components = search_result;
                        self.request_status = RequestStatus::Success;
                    },
                    Err(err) => {
                        self.found_components.clear();
                        self.request_status = RequestStatus::Error;
                        link.send_message(Msg::ResponseError(err))
                    },
                }
            },
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {

        if props.search_arg.is_none() {
          return false;
        }

        let flag = props.search_arg
            .as_ref()
            .map(|p_arg| self.search_arg.partial_comparison(p_arg))
            .unwrap_or(false);

        debug!("flag: {:?}, {:?}, {}", props.search_arg.clone().unwrap(), self.search_arg, flag);

        if flag && self.has_props == props.search_arg.is_some() {
            false
        } else {
            if let Some(sa) = props.search_arg {
                let search = self.search_arg.search.clone();
                self.search_arg = sa;
                self.search_arg.search = search;
                self.has_props = true;
            } else {
                self.has_props = false;
            }
            self.link.send_message(Msg::Search);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html! {<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {self.show_input_block()}
            {match self.has_props {
                true => self.result_area(),
                false => html!{},
            }}
        </>}
    }
}

impl SearchBar {
    fn result_area(&self) -> Html {
        let (arguments, component_list) = match self.search_arg.search.is_empty() {
            true => (Some(ComponentsQueryArg::set_by_arg(&self.search_arg)), None),
            false => (None, Some(self.found_components.clone())),
        };
        html!{
            <div class={"search-result-list"}>
                <div class={"columns is-mobile"}>
                    <div class={"column is-flex"}>
                        <div id={"search-result-list-items"} class="card-relate-data" style={resizer("search-result-list", 1)}>
                            <CatalogComponents
                                show_create_btn={false}
                                arguments={arguments}
                                component_list={component_list}
                                />
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn show_input_block(&self) -> Html {
        let is_loading = if self.request_status == RequestStatus::Loading { "is-loading" } else { "" };
        let mut bar_class = classes!("field", "is-relative");
        if self.has_props { bar_class.push(vec!["has-addons", "column", "p-0", "m-0", "is-three-quarters"]); }
        html! {
            <div class={bar_class}>
                <div class={classes!("control", "has-icons-left", "has-icons-right", is_loading)} style={"width: 100%;"}>
                <input class={"input"} style={"width: 100%;"}
                    oninput={self.link.callback(|ev: InputData| Msg::InputSearch(ev.value))}
                    onfocus={self.link.callback(|_| Msg::SetFocus(true))}
                    onblur={self.link.callback(|_| Msg::SetFocus(false))}
                    onkeypress={self.link.callback(|e: KeyboardEvent| Msg::KeyPress(e))}
                    placeholder={get_value_field(&351)} // Enter search text
                    value={self.search_arg.search.clone()}
                    />
                <span class={"icon is-small is-left"}>
                    <i class={"fas fa-search fa-xs"}></i>
                </span>
                </div>
                {match self.has_props {
                    true => html!{
                        <div class={"control"}>
                            <button class="button is-info search-button" onclick={self.link.callback(|_| Msg::Search)}>
                                {get_value_field(&349)}
                            </button>
                        </div>
                    },
                    false => self.show_dropdown(),
                }}
            </div>
        }
    }

    fn show_dropdown(&self) -> Html {
        let show_dropdown = if self.request_status == RequestStatus::Success && self.is_focused { "is-active" } else { "" };
        html! {
            <div class={classes!("dropdown", "is-absolute", show_dropdown)}>
              <div class={"dropdown-menu"} id={"component-dropdown-menu"} role={"menu"}>
                <div class={"dropdown-content"}>
                  {
                    if self.request_status == RequestStatus::Success && self.found_components.is_empty() {
                        html! {
                            <div class={"dropdown-item has-text-grey"}>
                                <span class={"icon-text"}>
                                    <span class={"icon"}>
                                        <i class={"fas fa-search-minus"}></i>
                                    </span>
                                    <span>{get_value_field(&350)}</span> // No results
                                </span>
                            </div>
                        }
                    } else {
                        html! {
                            {for self.found_components.iter().map(|x| {
                                html!{
                                    <a href={format!("#/component/{}", x.uuid)} class={"dropdown-item"}>
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
        }
    }
}
