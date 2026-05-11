use yew::{classes, html, Callback, ChangeData, Classes, Html};
use crate::types::{PermissionLevel, TypeAccessInfo};

impl TypeAccessInfo {
    /// Returns html code with access name and corresponding icon (icon+name)
    pub(crate) fn get_with_icon(&self) -> Html {
        let class_icon = match self.type_access_id {
            3 => "fa fa-globe",
            1 => "fas fa-lock",
            _ => "fas fa-shield-alt",
        };
        html!{<>
            <span class="icon is-small"><i class={class_icon}></i></span>
            {" "}{self.name.clone()}
        </>}
    }
}

impl PermissionLevel  {
    /// Renders a single access level in span with appropriate icon (icon+name)
    pub(crate) fn render_access_level_icon(&self) -> Html {
        html!{<>
            <span class="icon is-small"><i class={self.get_icon_for_level()}></i></span>
            {" "}{self.name.clone()}
        </>}
    }

    /// Renders a single access level tag with appropriate styling
    pub(crate) fn render_access_level_tag(&self, span_classes: Classes) -> Html {
        html!{
            <span class={classes!("tag", "is-light", span_classes)}>
                <span class="icon is-small mr-1">
                    <i class={self.get_icon_for_level()} />
                </span>
                {" "}{self.name.clone()}
            </span>
        }
    }

    pub(crate) fn get_icon_for_level(&self) -> &'static str {
        match self.type_access_id {
            1 => "fas fa-tools",
            2 => "fas fa-pen",
            3 => "fas fa-eye",
            _ => "fas fa-question-circle",
        }
    }

    /// Renders a select dropdown with permission level options
    pub(crate) fn render_permission_select(
        permissions: &[Self],
        current_id: usize,
        onchange_level: Callback<ChangeData>,
    ) -> Html {
        html! {
            <div class="select is-small">
                <select
                    value={current_id.to_string()}
                    onchange={onchange_level}
                >
                    {for permissions.iter().map(|permission| {
                        let type_access_id = permission.type_access_id;
                        html! {
                            <option value={type_access_id.to_string()} selected={type_access_id == current_id}>
                                {permission.name.clone()}
                            </option>
                        }
                    })}
                </select>
            </div>
        }
    }
}