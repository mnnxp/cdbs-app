use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
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
    props: Props,
}

impl Component for ModificationTableHeads {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid {
            debug!("no change self.props.params: {:?}", self.props.params);
            false
        } else {
            debug!("change self.props.params: {:?}", self.props.params);
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        self.show_modification_head()
    }
}

impl ModificationTableHeads {
    fn show_modification_head(&self) -> Html {
        html!{<>
            {match self.props.show_new_column {
                true => html!{<th>{get_value_field(&111)}</th>}, // Action
                false => html!{<th>{get_value_field(&115)}</th>}, // Action | files
            }}
            <th>{get_value_field(&116)}</th> // modification
            {for self.props.params.iter().map(|head| {
                html!{<th>{ head.paramname.clone() }</th>}
            })}
            {match self.props.show_new_column {
                true => html!{<th>{get_value_field(&117)}</th>}, // add
                false => html!{},
            }}
        </>}
    }
}
