use yew::{html, Component, ComponentLink, Html, ShouldRender, Callback, Properties};

use crate::types::PermissionLevel;

pub(crate) struct PermissionLevelBlock {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) change_cb: Callback<usize>,
    pub(crate) permissions: Vec<PermissionLevel>,
    pub(crate) selected: usize,
    pub(crate) preset: Option<usize>,
}

#[derive(Clone)]
pub(crate) enum Msg {
    UpdatePermissionId(usize),
}

impl Component for PermissionLevelBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PermissionLevelBlock {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdatePermissionId(value) => self.props.change_cb.emit(value),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.selected == props.selected &&
            self.props.permissions.len() == props.permissions.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{
            <div class={"control"}>
                {for self.props.permissions.iter().map(|permission| {
                    self.show_permission_item(permission)
                })}
            </div>
        }
    }
}

impl PermissionLevelBlock {
    fn show_permission_item(&self, permission: &PermissionLevel) -> Html {
        let type_access_id = permission.type_access_id;
        let onclick_permission_level = self.link.callback(move |_| Msg::UpdatePermissionId(type_access_id));
        let onchange_permission_level = self.link.callback(move |_| Msg::UpdatePermissionId(type_access_id));
        let is_checked = match self.props.selected {
            0 => permission.type_access_id == self.props.preset.unwrap_or_default(),
            _ => permission.type_access_id == self.props.selected,
        };
        html!{
            <div class={"is-block"}>
                <input
                    type={"radio"}
                    name={"permission"}
                    value={permission.type_access_id.to_string()}
                    onchange={onchange_permission_level}
                    checked={is_checked}
                />
                <span onclick={onclick_permission_level}>
                    {" "}{permission.render_access_level_icon()}
                </span>
            </div>
        }
    }
}