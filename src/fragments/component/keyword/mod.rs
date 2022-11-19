mod item;
mod add;

pub use item::KeywordTagItem;
pub use add::AddKeywordsTags;

use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, Keyword};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub keywords: Vec<Keyword>,
    pub delete_keyword: Option<Callback<Keyword>>,
}

pub struct KeywordsTags {
    props: Props,
}

impl Component for KeywordsTags {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
                self.props.keywords.len() == props.keywords.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{
            <div id="keywords" class="field is-grouped is-grouped-multiline">
                {for self.props.keywords.iter().map(|keyword| {
                    html!{<KeywordTagItem
                        show_delete_btn = {self.props.show_delete_btn}
                        component_uuid = {self.props.component_uuid.clone()}
                        keyword = {keyword.clone()}
                        delete_keyword = {self.props.delete_keyword.clone()}
                        />}
                })}
            </div>
        }
    }
}
