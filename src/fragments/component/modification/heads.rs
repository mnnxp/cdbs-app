use yew::{Component, Context, html, Html, Properties};
use log::debug;
use crate::types::{UUID, Param};
use crate::services::get_value_field;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_new_column: bool,
    pub component_uuid: UUID,
    pub params: Vec<Param>,
}

pub struct ModificationTableHeads {
    component_uuid: UUID,
}

impl Component for ModificationTableHeads {
    type Message = ();
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            component_uuid: ctx.props().component_uuid,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.component_uuid == ctx.props().component_uuid {
            debug!("no change ctx.props().params: {:?}", ctx.props().params);
            false
        } else {
            debug!("change ctx.props().params: {:?}", ctx.props().params);
            self.component_uuid = ctx.props().component_uuid;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.show_modification_head(ctx.props())
    }
}

impl ModificationTableHeads {
    fn show_modification_head(
        &self,
        props: &Properties,
    ) -> Html {
        html!{<>
            {match props.show_new_column {
                true => html!{<th>{ get_value_field(&111) }</th>}, // Action
                false => html!{<th>{ get_value_field(&115) }</th>}, // Action | files
            }}
            <th>{ get_value_field(&116) }</th> // modification
            {for props.params.iter().map(|head| {
                html!{<th>{ head.paramname.clone() }</th>}
            })}
            {match props.show_new_column {
                true => html!{<th>{ get_value_field(&117) }</th>}, // add
                false => html!{},
            }}
        </>}
    }
}
