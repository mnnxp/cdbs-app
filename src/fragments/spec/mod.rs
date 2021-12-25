mod item;

pub use item::SpecTagItem;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, Spec};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub company_uuid: UUID,
    pub specs: Vec<Spec>,
}

pub struct SpecsTags {
    props: Props,
}

impl Component for SpecsTags {
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
        html!{
            <div id="specs" class="tags">
                {for self.props.specs.iter().map(|spec| {
                    html!{<SpecTagItem
                        show_delete_btn = self.props.show_delete_btn.clone()
                        company_uuid = self.props.company_uuid.clone()
                        spec = spec.clone()
                        />}
                })}
            </div>
        }
    }
}
