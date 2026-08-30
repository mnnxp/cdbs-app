use web_sys::MouseEvent;
use yew::{classes, Callback, Classes, Properties};
use crate::services::LocaleKey;

/// Menu item component properties
#[derive(Properties, Clone)]
pub(crate) struct MenuItem {
    pub(crate) title: &'static str,
    pub(crate) action: Callback<MouseEvent>,
    #[prop_or_default]
    pub(crate) item_class: Classes,
    #[prop_or_default]
    pub(crate) icon_classes: &'static [&'static [&'static str]],
    pub(crate) count: usize,
    #[prop_or(false)]
    pub(crate) is_active: bool,
    #[prop_or(false)]
    pub(crate) is_extend: bool,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            title: "",
            action: Callback::noop(),
            item_class: classes!("has-background-white"),
            icon_classes: &[&["fas", "fa-certificate"]],
            count: 0,
            is_active: false,
            is_extend: false,
        }
    }
}

/// Menu item configuration template
pub(crate) struct MenuItemTemplate<T> {
    pub(crate) lk_title: LocaleKey,
    pub(crate) icon_classes: &'static [&'static [&'static str]],
    pub(crate) tab: T,
    pub(crate) custom_class: Option<&'static str>,
}