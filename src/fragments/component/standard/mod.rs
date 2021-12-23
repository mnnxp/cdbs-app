mod standard_item;

pub use standard_item::ComponentStandardItem;

use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, ShowStandardShort};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub component_standards: Vec<ShowStandardShort>,
    // pub delete_standard: Option<Callback<UUID>>,
}

pub struct ComponentStandardsCard {
    props: Props,
    link: ComponentLink<Self>,
    standard_uuids: BTreeSet<UUID>,
}

#[derive(Clone)]
pub enum Msg {
    DeleteComponentStandard(UUID),
    Ignore,
}

impl Component for ComponentStandardsCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut standard_uuids: BTreeSet<UUID> = BTreeSet::new();

        for standard in props.component_standards.clone() {
            standard_uuids.insert(standard.uuid.clone());
        };

        Self {
            props,
            link,
            standard_uuids,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteComponentStandard(standard_uuid) => {
                self.standard_uuids.remove(&standard_uuid);
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
                self.props.component_standards.len() == props.component_standards.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_delete_standard = self.link
            .callback(|value: UUID| Msg::DeleteComponentStandard(value));

        html! {
            html!{<div class="card">
              <table class="table is-fullwidth">
                <tbody>
                   <th>{"Classifier"}</th>
                   <th>{"Specified tolerance"}</th>
                   <th>{"Action"}</th>
                   {match self.props.show_delete_btn {
                       true => html!{<th>{"Delete"}</th>},
                       false => html!{},
                   }}
                   {for self.props.component_standards.iter().map(|data| {
                       match self.standard_uuids.get(&data.uuid) {
                           Some(_) => html!{<ComponentStandardItem
                               show_delete_btn = self.props.show_delete_btn
                               component_uuid = self.props.component_uuid.clone()
                               standard_data = data.clone()
                               delete_standard = Some(onclick_delete_standard.clone())
                             />},
                           None => html!{},
                       }
                   })}
                </tbody>
              </table>
            </div>}
        }
    }
}
