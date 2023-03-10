use yew::{
    html, Callback, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, Keyword};
use crate::gqls::make_query;
use crate::gqls::component::{DeleteComponentKeywords, delete_component_keywords};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub keyword: Keyword,
    pub style_tag: Option<String>,
    pub delete_keyword: Option<Callback<Keyword>>,
}

pub struct KeywordTagItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    get_result_delete: bool,
}

pub enum Msg {
    RequestDeleteKeyword,
    ResponseError(Error),
    GetDeleteKeywordResult(String),
}

impl Component for KeywordTagItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            error: None,
            props,
            link,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDeleteKeyword => {
                let component_uuid = self.props.component_uuid.clone();
                let keyword_id = self.props.keyword.id as i64;
                spawn_local(async move {
                    let ipt_component_keywords_data = delete_component_keywords::IptComponentKeywordsData{
                        componentUuid: component_uuid,
                        keywordIds: vec![keyword_id],
                    };
                    let res = make_query(DeleteComponentKeywords::build_query(
                        delete_component_keywords::Variables {
                            ipt_component_keywords_data,
                        }
                    )).await;
                    link.send_message(Msg::GetDeleteKeywordResult(res.unwrap()));
                })
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
            },
            Msg::GetDeleteKeywordResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res.get("deleteComponentKeywords").unwrap().clone()).unwrap();
                        debug!("deleteComponentKeywords: {:?}", result);
                        match &self.props.delete_keyword {
                            Some(delete_keyword) => {
                                if result > 0 {
                                    self.get_result_delete = true;
                                    delete_keyword.emit(self.props.keyword.clone());
                                };
                            },
                            None => self.get_result_delete = result > 0,
                        }
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{<>
            <ListErrors error=self.error.clone()/>
            {match self.get_result_delete {
                true => html!{},
                false => self.show_keyword(),
            }}
        </>}
    }
}

impl KeywordTagItem {
    fn show_keyword(
        &self,
    ) -> Html {
        let onclick_delete_keyword = self
            .link
            .callback(|_| Msg::RequestDeleteKeyword);

        let style_tag = match &self.props.style_tag {
            Some(style) => format!("tag is-light {}", style),
            None => "tag is-light".to_string(),
        };

        html!{<div class="control">
          <div class="tags has-addons">
            <span class={style_tag}>{self.props.keyword.keyword.clone()}</span>
            {match &self.props.show_delete_btn {
                true => html!{<a class="tag is-delete is-small is-light" onclick={onclick_delete_keyword} />},
                false => html!{},
            }}
          </div>
        </div>}
    }
}
