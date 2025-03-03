use yew::{Html, html};
use crate::types::TypeAccessInfo;

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