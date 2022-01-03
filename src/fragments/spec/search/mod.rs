mod item;

pub use item::SpecTagItem;

use log::debug;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
// use crate::error::{get_error, Error};
use crate::types::{Spec, UUID};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    // pub show_delete_btn: bool,
    pub company_specs: Vec<Spec>,
    pub company_uuid: UUID,
    pub specs: Vec<Spec>,
    // pub is_added: bool,
}

pub struct SearchSpecsTags {
    props: Props,
}

impl Component for SearchSpecsTags {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        debug!("SearchSpecsTags::create {:?}", props.specs);
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html!{
            <div id="specs" class="tags search_res_box">
                {for self.props.specs.iter().map(|spec| {
                    html!{<SpecTagItem
                        // show_delete_btn = self.props.show_delete_btn
                        company_uuid = self.props.company_uuid.clone()
                        spec = spec.clone()
                        is_added = self.props.company_specs.iter().any(|x| x.spec_id == spec.clone().spec_id)
                        />}
                })}
            </div>
        }
    }
}
