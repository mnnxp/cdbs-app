use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use log::debug;
use crate::types::{UUID, Param};
use crate::services::get_value_field;
use crate::services::content_adapter::Markdownable;

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
        debug!("change self.props.params, old: {:?}, new: {:?}", self.props.params, props.params);
        if self.props.component_uuid == props.component_uuid && self.props.params == props.params {
            false
        } else {
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
            <th>{get_value_field(&111)}</th>
            <th>{"\u{2116}"}</th> // Numero sign №
            <th>{get_value_field(&176)}</th> // Modification name
            {for self.props.params.iter().map(|head| {
                html!{<th title={get_value_field(&210)}>{head.paramname.to_markdown()}</th>}
            })}
            {match self.props.show_new_column {
                true => html!{<th title={get_value_field(&130)}>{get_value_field(&117)}</th>}, // add
                false => html!{},
            }}
        </>}
    }
}
