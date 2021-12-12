use std::collections::HashMap;
use yew::{
    html, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
// use log::debug;
use crate::types::{UUID, Param};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub component_uuid: UUID,
    pub collect_heads: Vec<Param>,
    pub collect_item: HashMap<usize, String>,
}

pub struct ModificationTableItem {
    props: Props,
}

impl Component for ModificationTableItem {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.show_modification_row()
    }
}

impl ModificationTableItem {
    fn show_modification_row(&self) -> Html {
        html!{<tr>
            {match self.props.collect_item.get(&0) {
                Some(value) => html!{<td>{value.clone()}</td>},
                None => html!{<td></td>},
            }}
            {for self.props.collect_heads.iter().map(|param| {
                match self.props.collect_item.get(&param.param_id) {
                    Some(value) => html!{<td>{value.clone()}</td>},
                    None => html!{<td></td>},
                }
            })}
        </tr>}
    }
}
