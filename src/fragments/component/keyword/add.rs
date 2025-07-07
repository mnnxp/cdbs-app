use yew::{html, Component, ComponentLink, InputData, KeyboardEvent, Html, Properties, ShouldRender};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::{
    list_errors::ListErrors,
    component::{KeywordsTags, KeywordTagItem},
};
use crate::types::{UUID, Keyword};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::component::{
    GetComponentKeywords, get_component_keywords,
    AddComponentKeywordsByNames, add_component_keywords_by_names,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub component_keywords: Vec<Keyword>,
    pub component_uuid: UUID,
}

pub struct AddKeywordsTags {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    ipt_index: usize,
    ipt_keyword: String,
    add_keywords: Vec<Keyword>,
    new_keywords: Vec<Keyword>,
    bad_keyword: bool,
    request_add_keyword: usize,
}

pub enum Msg {
    GetString(String),
    PressKeyEnter,
    ParseKeywords,
    RequestAddKeywords,
    RequestGetComponentKeywords,
    GetAddKeywordsResult(String),
    GetComponentKeywordsResult(String),
    HideNotification,
    DeleteCurrentKeyword(usize),
    DeleteNewKeyword(String),
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for AddKeywordsTags {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            ipt_index: 0,
            ipt_keyword: String::new(),
            add_keywords: Vec::new(),
            new_keywords: Vec::new(),
            bad_keyword: false,
            request_add_keyword: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::GetString(keyword) => {
                self.ipt_keyword = keyword.clone();
                if !self.ipt_keyword.is_empty() {
                    link.send_message(Msg::ParseKeywords)
                }
            },
            Msg::PressKeyEnter => {
                // debug!("PressKeyEnter: {:?}", self.ipt_keyword);
                self.ipt_keyword += ",";
                link.send_message(Msg::ParseKeywords)
            },
            Msg::ParseKeywords => {
                // debug!("ParseKeywords: {:?}", self.ipt_keyword);
                Keyword::parse_keywords(
                    self.ipt_keyword.clone(),
                    &mut self.ipt_index,
                    &mut self.ipt_keyword,
                    &mut self.add_keywords,
                    &mut self.bad_keyword
                );
                debug!("Keywords: {:?}", self.add_keywords.len());
                link.send_message(Msg::RequestAddKeywords)
            },
            Msg::RequestGetComponentKeywords => {
                let component_uuid = self.props.component_uuid.clone();
                spawn_local(async move {
                    let res = make_query(GetComponentKeywords::build_query(get_component_keywords::Variables {
                        component_uuid
                    })).await.unwrap();
                    link.send_message(Msg::GetComponentKeywordsResult(res));
                })
            },
            Msg::GetComponentKeywordsResult(res) => {
                match resp_parsing::<Vec<Keyword>>(res, "componentKeywords") {
                    Ok(result) => {
                        debug!("GetComponentKeywords before: {:?}", result);
                        if self.props.component_keywords.is_empty() {
                            for k_res in &result {
                                match self.new_keywords.iter().find(|k| k.keyword == k_res.keyword) {
                                    Some(dup) => debug!("dup {:?}", dup),
                                    None => self.new_keywords.push(k_res.clone()),
                                }
                            }
                        } else {
                            for k_res in &result {
                                match self.props.component_keywords.iter().find(|k| k.keyword == k_res.keyword) {
                                    Some(k) => debug!("k_res.keyword {:?} != k_props.keyword {:?}", k_res.keyword, k),
                                    None => {
                                        match self.new_keywords.iter().find(|k| k.keyword == k_res.keyword) {
                                            Some(dup) => debug!("dup {:?}", dup),
                                            None => self.new_keywords.push(k_res.clone()),
                                        }
                                    },
                                }
                            }
                        }
                        debug!("GetComponentKeywords after: {:?}", self.new_keywords);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::RequestAddKeywords => {
                let component_uuid = self.props.component_uuid.clone();
                let mut keywords: Vec<String> = Vec::new();
                for value in self.add_keywords.iter() {
                    if !value.keyword.is_empty() {
                        keywords.push(value.keyword.clone())
                    }
                }
                debug!("Keywords: {:?}", keywords);
                self.add_keywords = Vec::new();
                if keywords.len() > 0 {
                    spawn_local(async move {
                        let ipt_component_keywords_names = add_component_keywords_by_names::IptComponentKeywordsNames{
                            componentUuid: component_uuid,
                            keywords,
                        };
                        let res = make_query(AddComponentKeywordsByNames::build_query(add_component_keywords_by_names::Variables {
                            ipt_component_keywords_names
                        })).await.unwrap();
                        link.send_message(Msg::GetAddKeywordsResult(res));
                    })
                }
            },
            Msg::GetAddKeywordsResult(res) => {
                match resp_parsing(res, "addComponentKeywordsByNames") {
                    Ok(result) => {
                        debug!("request_add_keyword: {:?}", result);
                        self.request_add_keyword = result;
                        if result > 0 {
                            link.send_message(Msg::RequestGetComponentKeywords);
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::HideNotification => self.bad_keyword = false,
            Msg::DeleteCurrentKeyword(keyword_id) => {
                let mut props_keywords: Vec<Keyword> = Vec::new();
                for k in self.props.component_keywords.iter() {
                    if k.id == keyword_id {
                        props_keywords.push(Keyword::default());
                    } else {
                        props_keywords.push(k.clone());
                    }
                }
                self.props.component_keywords = props_keywords;
            },
            Msg::DeleteNewKeyword(keyword) => {
                // debug!("self.new_keywords before delete: {:?}", self.new_keywords);
                // self.new_keywords.retain(|k| k != &keyword);
                let mut new_keywords_empty = true;
                let mut new_keywords: Vec<Keyword> = Vec::new();
                for k in &self.new_keywords {
                    if k.keyword == keyword {
                        new_keywords.push(Keyword::default());
                    } else {
                        new_keywords.push(k.clone());
                        new_keywords_empty = false;
                    }
                }
                if new_keywords_empty {
                    self.new_keywords = Vec::new();
                } else {
                    self.new_keywords = new_keywords;
                }
                debug!("self.new_keywords after delete: {:?}", self.new_keywords);
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{
            <div class="card">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&105)}</p> // Keywords
                </header>
                <div class="card-content">
                    <div class="content">
                        {self.add_component_keyword()}
                    </div>
                </div>
            </div>
        }
    }
}

impl AddKeywordsTags {
    fn add_component_keyword(&self) -> Html {
        let oninput_parse_keyword = self.link.callback(|ev: InputData| Msg::GetString(ev.value));
        let onkeypress_parse_keyword =
            self.link.callback(|ev: KeyboardEvent| {
                debug!("ev.key(): {:?}, ev.key(): {:?}", ev.key_code(), ev.key());
                match ev.key_code() {
                    13 => Msg::PressKeyEnter,
                    _ => Msg::Ignore,
                }
            });
        let onclick_hide_notification = self.link.callback(|_| Msg::HideNotification);
        let onclick_del_new_keyword =
            self.link.callback(|value: Keyword| Msg::DeleteNewKeyword(value.keyword));
        let onclick_del_old_keyword =
            self.link.callback(|value: Keyword| Msg::DeleteCurrentKeyword(value.id));

        html!{<>
            <div class="panel-block">
                <input
                    oninput={oninput_parse_keyword}
                    onkeypress={onkeypress_parse_keyword}
                    class="input"
                    type="text"
                    value={self.ipt_keyword.clone()}
                    placeholder={get_value_field(&193)} // Emter keywords separated by spaces or commas
                  />
            </div>
           {match self.bad_keyword {
               true => html!{<div class="notification is-danger">
                  <button class="delete" onclick={onclick_hide_notification}></button>
                  {get_value_field(&243)} // Keywords must be less...
               </div>},
               false => html!{}
           }}
           <div class="panel-block">
               <div id="new-keywords" class="field is-grouped is-grouped-multiline">
                 {for self.new_keywords.iter().map(|keyword| {
                   if keyword.keyword.is_empty() {
                      html!{}
                   } else {
                      html!{<KeywordTagItem
                         show_delete_btn={true}
                         component_uuid={self.props.component_uuid.clone()}
                         keyword={keyword.clone()}
                         style_tag={Some("is-success".to_string())}
                         delete_keyword={Some(onclick_del_new_keyword.clone())}
                        />}
                      }
                  })}
                </div>
            </div>
           <div class="panel-block">
               <KeywordsTags
                  show_delete_btn={true}
                  component_uuid={self.props.component_uuid.clone()}
                  keywords={self.props.component_keywords.clone()}
                  delete_keyword={Some(onclick_del_old_keyword.clone())}
                 />
           </div>
        </>}
    }
}
