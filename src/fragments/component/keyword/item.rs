use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, Keyword};
use crate::gqls::make_query;
use crate::gqls::component::{DeleteComponentKeywords, delete_component_keywords};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub keyword: Keyword,
    pub style_tag: Option<String>,
    pub delete_keyword: Option<Callback<Keyword>>,
}

pub struct KeywordTagItem {
    error: Option<Error>,
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
    fn create(ctx: &Context<Self>) -> Self {

        Self {
            error: None,
            get_result_delete: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestDeleteKeyword => {
                let component_uuid = ctx.props().component_uuid.clone();
                let keyword_id = ctx.props().keyword.id as i64;
                spawn_local(async move {
                    let ipt_component_keywords_data = delete_component_keywords::IptComponentKeywordsData{
                        component_uuid,
                        keyword_ids: vec![keyword_id],
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
                        match &ctx.props().delete_keyword {
                            Some(delete_keyword) => {
                                if result > 0 {
                                    self.get_result_delete = true;
                                    delete_keyword.emit(ctx.props().keyword.clone());
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

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<>
            <ListErrors error={self.error.clone()}/>
            {match self.get_result_delete {
                true => html!{},
                false => self.show_keyword(ctx.link(), ctx.props()),
            }}
        </>}
    }
}

impl KeywordTagItem {
    fn show_keyword(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_delete_keyword = link.callback(|_| Msg::RequestDeleteKeyword);

        let style_tag = match &props.style_tag {
            Some(style) => format!("tag is-light {}", style),
            None => "tag is-light".to_string(),
        };

        html!{<div class="control">
          <div class="tags has-addons">
            <span class={style_tag}>{props.keyword.keyword.clone()}</span>
            {match &props.show_delete_btn {
                true => html!{
                    <a class="tag is-delete is-small is-light" onclick={onclick_delete_keyword} />
                },
                false => html!{},
            }}
          </div>
        </div>}
    }
}
