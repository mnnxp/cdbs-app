use yew::{
    html, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, Keyword};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct DeleteStandardKeywords;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub standard_uuid: UUID,
    pub keyword: Keyword,
    pub style_tag: Option<String>,
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
                let standard_uuid = self.props.standard_uuid.clone();
                let keyword_id = self.props.keyword.id as i64;
                spawn_local(async move {
                    let ipt_standard_keywords_data = delete_standard_keywords::IptStandardKeywordsData{
                        standardUuid: standard_uuid,
                        keywordIds: vec![keyword_id],
                    };
                    let res = make_query(DeleteStandardKeywords::build_query(
                        delete_standard_keywords::Variables {
                            ipt_standard_keywords_data,
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
                        let result: usize = serde_json::from_value(res.get("deleteStandardKeywords").unwrap().clone()).unwrap();
                        debug!("deleteStandardKeywords: {:?}", result);
                        self.get_result_delete = result > 0;
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
        html! {<>
            <ListErrors error=self.error.clone()/>
            {match self.get_result_delete {
                true => html! {},
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
                true => html! {<a class="tag is-delete is-small is-light" onclick={onclick_delete_keyword} />},
                false => html! {},
            }}
          </div>
        </div>}
    }
}
