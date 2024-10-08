mod item;
mod search;
pub use item::SpecTagItem;
pub use search::SearchSpecsTags;

use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
use crate::services::get_value_field;
use crate::types::{UUID, Spec};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub standard_uuid: UUID,
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
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.standard_uuid == props.standard_uuid &&
            self.props.show_manage_btn == props.show_manage_btn &&
                self.props.specs.len() == props.specs.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        match self.props.show_manage_btn {
            true => self.specs(),
            false => html!{
                <div class="card">
                    <header class="card-header">
                        <p class="card-header-title">{get_value_field(&104)}</p> // Catalogs
                    </header>
                    <div class="card-content">
                        <div class="content">
                            {self.specs()}
                        </div>
                    </div>
                </div>
            }
        }
    }
}

impl SpecsTags {
    fn specs(&self) -> Html {
        html!{
            <div id="specs" class="field is-grouped is-grouped-multiline">
                {for self.props.specs.iter().map(|spec| {
                    html!{<SpecTagItem
                        show_manage_btn={self.props.show_manage_btn}
                        active_info_btn={true}
                        standard_uuid={self.props.standard_uuid.clone()}
                        spec={spec.clone()}
                        is_added={true}
                        delete_spec={self.props.delete_spec.clone()}
                        />}
                })}
            </div>
        }
    }
}