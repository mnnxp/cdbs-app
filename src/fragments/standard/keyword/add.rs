use yew::{Component, Context, html, Html, Properties};
use yew::html::{Scope, TargetCast};
use web_sys::{InputEvent, KeyboardEvent, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::standard::{KeywordsTags, KeywordTagItem};
use crate::types::{UUID, Keyword};
use crate::services::{get_value_field, resp_parsing, resp_parsing_item};
use crate::gqls::make_query;
use crate::gqls::standard::{
    GetStandardKeywords, get_standard_keywords,
    AddStandardKeywordsByNames, add_standard_keywords_by_names,
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub standard_uuid: UUID,
    pub standard_keywords: Vec<Keyword>,
}

pub struct AddKeywordsTags {
    error: Option<Error>,
    standard_uuid: UUID,
    standard_keywords: Vec<Keyword>,
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
    RequestGetStandardKeywords,
    GetAddKeywordsResult(String),
    GetStandardKeywordsResult(String),
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            standard_uuid: ctx.props().standard_uuid.clone(),
            standard_keywords: ctx.props().standard_keywords.clone(),
            ipt_index: 0,
            ipt_keyword: String::new(),
            add_keywords: Vec::new(),
            new_keywords: Vec::new(),
            bad_keyword: false,
            request_add_keyword: 0,
        }
    }

    // fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {}

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::GetString(keyword) => {
                self.ipt_keyword = keyword.clone();
                self.bad_keyword = self.ipt_keyword.len() > 9;
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
                let keyword = self.ipt_keyword.clone();
                match keyword.find(|c| (c == ' ') || (c == ',')) {
                    None => (),
                    Some(1) => {
                        if keyword.len() < 11 {
                            self.add_keywords.push(Keyword {
                                id: self.ipt_index,
                                keyword: keyword.trim().to_string()
                            });
                            self.ipt_keyword = String::new();
                            self.ipt_index += 1;
                        };
                        link.send_message(Msg::RequestAddKeywords)
                    },
                    Some(_) => {
                        for k in keyword.split(|c| c == ' ' || c == ',') {
                            if k.len() < 11 {
                                self.add_keywords.push(Keyword {
                                    id: self.ipt_index,
                                    keyword: k.trim().to_string()
                                });
                                self.ipt_keyword = String::new();
                                self.ipt_index += 1;
                            }
                        };
                        debug!("Keywords: {:?}", self.add_keywords.len());
                        link.send_message(Msg::RequestAddKeywords)
                    },
                }
            },
            Msg::RequestGetStandardKeywords => {
                let standard_uuid = ctx.props().standard_uuid.clone();
                spawn_local(async move {
                    let ipt_standard_keywords_arg = get_standard_keywords::IptStandardKeywordsArg{
                        standard_uuid,
                        limit: None,
                        offset: None,
                    };
                    let res = make_query(GetStandardKeywords::build_query(get_standard_keywords::Variables {
                        ipt_standard_keywords_arg
                    })).await.unwrap();
                    link.send_message(Msg::GetStandardKeywordsResult(res));
                })
            },
            Msg::GetStandardKeywordsResult(res) => {
                let result: Vec<Keyword> = resp_parsing(res, "standardKeywords")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if self.standard_keywords.is_empty() {
                    for k_res in &result {
                        match self.new_keywords.iter().find(|k| k.keyword == k_res.keyword) {
                            Some(dup) => debug!("dup {:?}", dup),
                            None => self.new_keywords.push(k_res.clone()),
                        }
                    }
                    return true
                }
                for k_res in &result {
                    match self.standard_keywords.iter().find(|k| k.keyword == k_res.keyword) {
                        Some(k) => debug!("k_res.keyword {:?} != k_props.keyword {:?}", k_res.keyword, k),
                        None => {
                            match self.new_keywords.iter().find(|k| k.keyword == k_res.keyword) {
                                Some(dup) => debug!("dup {:?}", dup),
                                None => self.new_keywords.push(k_res.clone()),
                            }
                        },
                    }
                }
                // self.rendered(false);
            },
            Msg::RequestAddKeywords => {
                let standard_uuid = ctx.props().standard_uuid.clone();
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
                        let ipt_standard_keywords_names = add_standard_keywords_by_names::IptStandardKeywordsNames{
                            standard_uuid,
                            keywords,
                        };
                        let res = make_query(AddStandardKeywordsByNames::build_query(add_standard_keywords_by_names::Variables {
                            ipt_standard_keywords_names
                        })).await.unwrap();
                        link.send_message(Msg::GetAddKeywordsResult(res));
                    })
                }
            },
            Msg::GetAddKeywordsResult(res) => {
                self.request_add_keyword = resp_parsing_item(res, "addStandardKeywordsByNames")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if self.request_add_keyword > 0 {
                    link.send_message(Msg::RequestGetStandardKeywords);
                }
            },
            Msg::HideNotification => self.bad_keyword = false,
            Msg::DeleteCurrentKeyword(keyword_id) => {
                let mut props_keywords: Vec<Keyword> = Vec::new();
                for k in self.standard_keywords.iter() {
                    if k.id == keyword_id {
                        props_keywords.push(Keyword::default());
                    } else {
                        props_keywords.push(k.clone());
                    }
                }
                self.standard_keywords = props_keywords;
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

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.standard_uuid == ctx.props().standard_uuid {
            false
        } else {
            self.standard_uuid = ctx.props().standard_uuid.clone();
            self.standard_keywords = ctx.props().standard_keywords.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            <br/>
            {self.add_standard_keyword(ctx.link())}
        </>}
    }
}

impl AddKeywordsTags {
    fn add_standard_keyword(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_parse_keyword = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::GetString(input.value())
        });
        let onkeypress_parse_keyword =
            link.callback(|ev: KeyboardEvent| {
                debug!("ev.key(): {:?}, ev.key(): {:?}", ev.key_code(), ev.key());
                match ev.key_code() {
                    13 => Msg::PressKeyEnter,
                    _ => Msg::Ignore,
                }
            });
        let onclick_hide_notification = link.callback(|_| Msg::HideNotification);
        let onclick_del_new_keyword =
            link.callback(|value: Keyword| Msg::DeleteNewKeyword(value.keyword));
        let onclick_del_old_keyword =
            link.callback(|value: Keyword| Msg::DeleteCurrentKeyword(value.id));

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
                  {"Keywords must be less than 10 symbols"}
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
                         show_delete_btn = {true}
                         standard_uuid = {self.standard_uuid.clone()}
                         keyword = {keyword.clone()}
                         style_tag = {Some("is-success".to_string())}
                         delete_keyword = {Some(onclick_del_new_keyword.clone())}
                        />}
                      }
                  })}
                </div>
            </div>
           <div class="panel-block">
               <KeywordsTags
                  show_delete_btn = {true}
                  standard_uuid = {self.standard_uuid.clone()}
                  keywords = {self.standard_keywords.clone()}
                  delete_keyword = {Some(onclick_del_old_keyword.clone())}
                 />
           </div>
        </>}
    }
}
