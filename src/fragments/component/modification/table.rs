use std::collections::HashMap;
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use crate::types::{UUID, ComponentModificationInfo, Param};
use super::{heads::ModificationTableHeads, item::ModificationTableItem};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub modifications: Vec<ComponentModificationInfo>,
    pub select_modification_uuid: UUID,
    pub callback_select_modification: Option<Callback<UUID>>,
    pub callback_new_modification_param: Option<Callback<UUID>>,
    pub numero_offset: usize,
}

pub struct ModificationsTable {
    props: Props,
    link: ComponentLink<Self>,
    component_uuid: UUID,
    collect_heads: Vec<Param>,
    collect_items: Vec<(UUID, HashMap<usize, String>)>,
    collect_columns: HashMap<usize, String>,
}

pub enum Msg {
    RebuildTable,
    ParseParams,
    SelectModification(UUID),
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
        debug!("Modification table for component uuid {:?}", self.component_uuid);
        if first_render {
            debug!("First bulild modifications table");
            self.link.send_message(Msg::RebuildTable);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RebuildTable => {
                self.component_uuid = match self.props.modifications.first().map(|x| x.component_uuid.clone()) {
                    Some(component_uuid) => component_uuid,
                    None => String::new(),
                };
                self.collect_items.clear();
                self.collect_columns.clear();
                if self.component_uuid.len() == 36 {
                    self.link.send_message(Msg::ParseParams);
                }
            },
            Msg::ParseParams => {
                let mut set_heads: Vec<usize> = vec![0];
                let mut collect_heads: Vec<Param> = Vec::new();
                for modification in &self.props.modifications {
                    self.collect_columns.clear();
                    self.collect_columns.insert(
                        0, modification.modification_name.clone(),
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
                    self.collect_items.push((
                        modification.uuid.clone(),
                        self.collect_columns.clone()
                    ));
                }
                debug!("collect_items: {:?}", self.collect_items);
                self.collect_heads = collect_heads;
            },
            Msg::SelectModification(modification_uuid) => {
                debug!("Callback TABLE, modification uuid: {:?} (Show modifications)",
                    self.props.select_modification_uuid
                );
                if let Some(select_modification) = &self.props.callback_select_modification {
                    select_modification.emit(modification_uuid);
                }
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        debug!("Show modifications TABLE, self.props uuid: {:?}\nprops uuid: {:?}",
            self.props.select_modification_uuid,
            props.select_modification_uuid
        );
        if self.props.select_modification_uuid == props.select_modification_uuid &&
        self.props.modifications.len() == props.modifications.len() &&
        self.props.numero_offset == props.numero_offset {
            if self.props.callback_new_modification_param.is_some() {
                // need further verification of the change
                let mut modification_name = &String::new();
                let mut old_params = &Vec::new();
                for old_m in &self.props.modifications {
                    if old_m.uuid == self.props.select_modification_uuid {
                        modification_name = &old_m.modification_name;
                        old_params = &old_m.modification_params;
                        break;
                    }
                }
                for new_m in &props.modifications {
                    if new_m.uuid == props.select_modification_uuid {
                        if modification_name == &new_m.modification_name &&
                           old_params.len() == new_m.modification_params.len() {
                            break;
                        }
                        // name or parameters have changed - update is required
                        self.props = props;
                        self.link.send_message(Msg::RebuildTable);
                        return true
                    }
                }
            }
            false
        } else {
            self.props = props;
            self.link.send_message(Msg::RebuildTable);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_select_modification = self.link.callback(|value: UUID| Msg::SelectModification(value));
        html!{
            <div class="content">
                <div class="table-container">
                    <table class="table is-fullwidth">
                        <ModificationTableHeads
                            show_new_column={self.props.callback_new_modification_param.is_some()}
                            component_uuid={self.component_uuid.clone()}
                            params={self.collect_heads.clone()}
                            />
                        {for self.collect_items.iter().enumerate().map(|(numer, (modification_uuid, item))| {
                            html!{<ModificationTableItem
                                show_manage_btn={self.props.callback_new_modification_param.is_some()}
                                modification_uuid={modification_uuid.clone()}
                                collect_heads={self.collect_heads.clone()}
                                collect_item={item.clone()}
                                select_item={&self.props.select_modification_uuid == modification_uuid}
                                callback_new_modification_param={self.props.callback_new_modification_param.clone()}
                                callback_select_modification={onclick_select_modification.clone()}
                                ordinal_indicator={self.props.numero_offset+numer}
                            />}
                        })}
                    </table>
                </div>
            </div>
        }
    }
}
