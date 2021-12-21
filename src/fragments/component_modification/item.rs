use std::collections::HashMap;
use yew::{
    html, Callback, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
// use log::debug;
use crate::types::{UUID, Param};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub modification_uuid: UUID,
    pub collect_heads: Vec<Param>,
    pub collect_item: HashMap<usize, String>,
    pub select_item: bool,
    pub callback_select_modification: Option<Callback<UUID>>,
}

pub struct ModificationTableItem {
    props: Props,
    link: ComponentLink<Self>,
    modification_uuid: UUID,
    select_item: bool,
}

pub enum Msg {
    SelectModification,
}

impl Component for ModificationTableItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let modification_uuid = props.modification_uuid.clone();
        let select_item = props.select_item;
        Self {
            props,
            link,
            modification_uuid,
            select_item,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectModification => {
                if let Some(select_modification) = &self.props.callback_select_modification {
                    select_modification.emit(self.props.modification_uuid.clone());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.modification_uuid == props.modification_uuid &&
              self.select_item == props.select_item {
            false
        } else {
            self.modification_uuid = props.modification_uuid.clone();
            self.select_item = props.select_item;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        self.show_modification_row()
    }
}

impl ModificationTableItem {
    fn show_modification_row(&self) -> Html {
        let onclick_select_modification = self.link
            .callback(|_| Msg::SelectModification);

        let class_style = match &self.props.select_item {
            true => "is-selected",
            false => "",
        };

        html!{<tr class={class_style}>
            {match self.props.collect_item.get(&0) {
                Some(value) => html!{<td>
                    <a onclick={onclick_select_modification}>
                        {match &self.props.select_item {
                            true => html!{<>
                                <span>{"click for "}</span>
                                <span class="icon">
                                  <i class="fab fa-info"></i>
                                </span>
                            </>},
                            false => html!{value.clone()},
                        }}
                    </a>
                </td>},
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
