mod item;
mod add;

pub use item::KeywordTagItem;
pub use add::AddKeywordsTags;

use yew::{Component, Context, html, Html, Properties, Callback};
// use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, Keyword};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_delete_btn: bool,
    pub standard_uuid: UUID,
    pub keywords: Vec<Keyword>,
    pub delete_keyword: Option<Callback<Keyword>>,
}

pub struct KeywordsTags {
    standard_uuid: UUID,
    keywords_len: usize,
}

impl Component for KeywordsTags {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            standard_uuid: ctx.props().standard_uuid,
            keywords_len: ctx.props().keywords.len(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.standard_uuid == ctx.props().standard_uuid &&
                self.keywords_len == ctx.props().keywords.len() {
            false
        } else {
            self.standard_uuid = ctx.props().standard_uuid;
            self.keywords_len = ctx.props().keywords.len();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div id="keywords" class="field is-grouped is-grouped-multiline">
                {for ctx.props().keywords.iter().map(|keyword| {
                    html!{<KeywordTagItem
                        show_delete_btn = {ctx.props().show_delete_btn}
                        standard_uuid = {ctx.props().standard_uuid.clone()}
                        keyword = {keyword.clone()}
                        delete_keyword = {ctx.props().delete_keyword.clone()}
                        />}
                })}
            </div>
        }
    }
}
