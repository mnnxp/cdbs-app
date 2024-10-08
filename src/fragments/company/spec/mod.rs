mod item;
mod search;
pub use item::SpecTagItem;
pub use search::SearchSpecsTags;

use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
use crate::types::{UUID, Spec};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub company_uuid: UUID,
    pub specs: Vec<Spec>,
    pub delete_spec: Option<Callback<usize>>,
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
        if self.props.company_uuid == props.company_uuid &&
            self.props.show_manage_btn == props.show_manage_btn &&
                self.props.specs.len() == props.specs.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{
            <div id="specs" class="field is-grouped is-grouped-multiline">
                {for self.props.specs.iter().map(|spec| {
                    html!{<SpecTagItem
                        show_manage_btn={self.props.show_manage_btn}
                        active_info_btn={true}
                        company_uuid={self.props.company_uuid.clone()}
                        spec={spec.clone()}
                        is_added={true}
                        delete_spec={self.props.delete_spec.clone()}
                        />}
                })}
            </div>
        }
    }
}
