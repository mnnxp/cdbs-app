mod item;
mod search;

pub use item::SpecTagItem;
pub use search::SearchSpecsTags;

use yew::{Component, Context, html, Html, Properties, Callback};
// use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, Spec};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub company_uuid: UUID,
    pub specs: Vec<Spec>,
    pub delete_spec: Option<Callback<usize>>,
}

pub struct SpecsTags {
    company_uuid: UUID,
    show_manage_btn: bool,
    specs_len: usize,
}

impl Component for SpecsTags {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            company_uuid: ctx.props().company_uuid,
            show_manage_btn: ctx.props().show_manage_btn,
            specs_len: ctx.props().specs.len(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.company_uuid == ctx.props().company_uuid &&
            self.show_manage_btn == ctx.props().show_manage_btn &&
                self.specs_len == ctx.props().specs.len() {
            false
        } else {
            self.company_uuid = ctx.props().company_uuid;
            self.show_manage_btn = ctx.props().show_manage_btn;
            self.specs_len = ctx.props().specs.len();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div id="specs" class="field is-grouped is-grouped-multiline">
                {for ctx.props().specs.iter().map(|spec| {
                    html!{<SpecTagItem
                        show_manage_btn = {ctx.props().show_manage_btn}
                        active_info_btn = true
                        company_uuid = {ctx.props().company_uuid.clone()}
                        spec = {spec.clone()}
                        is_added = true
                        delete_spec = {ctx.props().delete_spec.clone()}
                        />}
                })}
            </div>
        }
    }
}
