mod item;
mod add;
mod add_item;

pub use item::KeywordTagItem;
pub use add::AddKeywordsTags;
pub use add_item::AddKeywordTagItem;

use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, Keyword};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub standard_uuid: UUID,
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
        if self.props.show_delete_btn == props.show_delete_btn {
            false
        } else {
            self.props.show_delete_btn = props.show_delete_btn;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="keywords" class="field is-grouped is-grouped-multiline">
                {for self.props.keywords.iter().map(|keyword| {
                    html! {<KeywordTagItem
                        show_delete_btn = self.props.show_delete_btn.clone()
                        standard_uuid = self.props.standard_uuid.clone()
                        keyword = keyword.clone()
                        delete_keyword = self.props.delete_keyword.clone()
                        />}
                })}
            </div>
        }
    }
}
