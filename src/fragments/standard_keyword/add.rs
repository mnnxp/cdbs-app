use yew::{
    html, Component, ComponentLink, InputData, KeyboardEvent,
    Html, Properties, ShouldRender,
};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::{
    list_errors::ListErrors,
    standard_keyword::{KeywordsTags, KeywordTagItem},
};
use crate::gqls::make_query;
use crate::types::{UUID, Keyword};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct GetStandardKeywords;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct AddStandardKeywordsByNames;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub standard_keywords: Vec<Keyword>,
    pub standard_uuid: UUID,
}

pub struct AddKeywordsTags {
    error: Option<Error>,
    // current_keywords: Vec<Keyword>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            // current_keywords: props.standard_keywords.clone(),
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

    // fn rendered(&mut self, first_render: bool) {}

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

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
                let standard_uuid = self.props.standard_uuid.clone();
                spawn_local(async move {
                    let ipt_standard_keywords_arg = get_standard_keywords::IptStandardKeywordsArg{
                        standardUuid: standard_uuid,
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
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<Keyword> = serde_json::from_value(
                            res_value.get("standardKeywords").unwrap().clone()
                        ).unwrap();
                        debug!("GetStandardKeywords before: {:?}", result);
                        if self.props.standard_keywords.is_empty() {
                            for k_res in &result {
                                match self.new_keywords.iter().find(|k| k.keyword == k_res.keyword) {
                                    Some(dup) => debug!("dup {:?}", dup),
                                    None => self.new_keywords.push(k_res.clone()),
                                }
                            }
                        } else {
                            for k_res in &result {
                                match self.props.standard_keywords.iter().find(|k| k.keyword == k_res.keyword) {
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
                        debug!("GetStandardKeywords after: {:?}", self.new_keywords);
                        // self.rendered(false);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::RequestAddKeywords => {
                let standard_uuid = self.props.standard_uuid.clone();
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
                            standardUuid: standard_uuid,
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
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(
                            res_value.get("addStandardKeywordsByNames").unwrap().clone()
                        ).unwrap();
                        debug!("request_add_keyword: {:?}", result);
                        self.request_add_keyword = result;
                        if result > 0 {
                            link.send_message(Msg::RequestGetStandardKeywords);
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::HideNotification => self.bad_keyword = false,
            Msg::DeleteCurrentKeyword(keyword_id) => {
                let mut props_keywords: Vec<Keyword> = Vec::new();
                for k in self.props.standard_keywords.iter() {
                    if k.id == keyword_id {
                        props_keywords.push(Keyword::default());
                    } else {
                        props_keywords.push(k.clone());
                    }
                }
                self.props.standard_keywords = props_keywords;
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
        let onclick_clear_error = self.link
            .callback(|_| Msg::ClearError);

        html! {<>
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            <br/>
            {self.add_standard_keyword()}
        </>}
    }
}

impl AddKeywordsTags {
    fn add_standard_keyword(&self) -> Html {
        let oninput_parse_keyword = self.link
            .callback(|ev: InputData| Msg::GetString(ev.value));
        let onkeypress_parse_keyword = self.link
            .callback(|ev: KeyboardEvent| {
                debug!("ev.key(): {:?}, ev.key(): {:?}", ev.key_code(), ev.key());
                match ev.key_code() {
                    13 => Msg::PressKeyEnter,
                    _ => Msg::Ignore,
                }
            });
        let onclick_hide_notification = self.link
            .callback(|_| Msg::HideNotification);
        let onclick_del_new_keyword = self.link
            .callback(|value: Keyword| Msg::DeleteNewKeyword(value.keyword));
        let onclick_del_old_keyword = self.link
            .callback(|value: Keyword| Msg::DeleteCurrentKeyword(value.id));

        html! {<>
            <div class="panel-block">
                <input
                    oninput=oninput_parse_keyword
                    onkeypress=onkeypress_parse_keyword
                    class="input"
                    type="text"
                    value={self.ipt_keyword.clone()}
                    placeholder="Input keywords separated by spaces or commas"
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
                      html! {<KeywordTagItem
                         show_delete_btn = true
                         standard_uuid = self.props.standard_uuid.clone()
                         keyword = keyword.clone()
                         style_tag = Some("is-success".to_string())
                         delete_keyword = Some(onclick_del_new_keyword.clone())
                        />}
                      }
                  })}
                </div>
            </div>
           <div class="panel-block">
               <KeywordsTags
                  show_delete_btn = true
                  standard_uuid = self.props.standard_uuid.clone()
                  keywords = self.props.standard_keywords.clone()
                  delete_keyword = Some(onclick_del_old_keyword.clone())
                 />
           </div>
        </>}
    }
}
