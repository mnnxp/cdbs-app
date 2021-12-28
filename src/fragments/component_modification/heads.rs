use yew::{
    html, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
// use log::debug;
use crate::types::{UUID, Param};

#[derive(Clone, Debug, Properties)]
pub struct Props {
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
        if self.props.component_uuid == props.component_uuid &&
          self.props.params.len() == props.params.len() {
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
            <th>{"modification"}</th>
            {for self.props.params.iter().map(|head| {
                html!{<th>{head.paramname.clone()}</th>}
            })}
        </>}
    }
}
