mod heads;
mod item;

pub use heads::ModificationTableHeads;
pub use item::ModificationTableItem;

use std::collections::HashMap;
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use crate::types::{UUID, ComponentModificationInfo, Param};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub modifications: Vec<ComponentModificationInfo>,
    pub delete_modification: Option<Callback<UUID>>,
}

pub struct ModificationsTable {
    props: Props,
    link: ComponentLink<Self>,
    component_uuid: UUID,
    collect_heads: Vec<Param>,
    collect_items: Vec<HashMap<usize, String>>,
    collect_columns: HashMap<usize, String>,
}

pub enum Msg {
    ParseParams
}

impl Component for ModificationsTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            component_uuid: String::new(),
            collect_heads: Vec::new(),
            collect_items: Vec::new(),
            collect_columns: HashMap::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::ParseParams);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();

        match msg {
            Msg::ParseParams => {
                let mut first = true;
                let mut set_heads: Vec<usize> = vec![0];
                let mut collect_heads: Vec<Param> = Vec::new();

                for modification in &self.props.modifications {
                    if first {
                        self.component_uuid = modification.component_uuid.clone();
                        first = false;
                    }

                    self.collect_columns.clear();
                    self.collect_columns.insert(
                        0, // for test
                        modification.modification_name.clone(),
                    );
                    for modification_param in &modification.modification_params {
                        let mut flag = true;
                        debug!("modification_param: {:?}", modification_param.param);
                        for head_id in &set_heads {
                            if head_id == &modification_param.param.param_id {
                                debug!("head: {:?}", modification_param.param.param_id);
                                flag = false;
                                break;
                            }
                        }
                        if flag {
                            set_heads.push(modification_param.param.param_id);
                            collect_heads.push(modification_param.param.clone());
                        }
                        self.collect_columns.insert(
                            modification_param.param.param_id,
                            modification_param.value.clone(),
                        );
                        debug!("collect_heads: {:?}", collect_heads);
                    }
                    debug!("collect_columns: {:?}", self.collect_columns);
                    self.collect_items.push(self.collect_columns.clone());
                }
                debug!("collect_items: {:?}", self.collect_items);
                self.collect_heads = collect_heads;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_manage_btn == props.show_manage_btn &&
                self.props.modifications.len() == props.modifications.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html! {<div class="card">
            <div class="table-container">
              <table class="table is-fullwidth">
                <ModificationTableHeads
                  component_uuid = self.component_uuid.clone()
                  params = self.collect_heads.clone()
                />

                {for self.collect_items.iter().map(|item| {
                  html! {<ModificationTableItem
                      component_uuid = self.component_uuid.clone()
                      collect_heads = self.collect_heads.clone()
                      collect_item = item.clone()
                      />}
                 })}
              </table>
            </div>
        </div>}
    }
}
