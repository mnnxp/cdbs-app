use web_sys::MouseEvent;
use yew::{classes, html, Callback, Html};
use crate::services::get_value_field;
use super::SideMenu;
use super::model::{MenuItem, MenuItemTemplate};

/// Side menu component with declarative menu builder
///
/// Provides a reusable side menu and a trait for building menus from static configuration.
///
/// # Usage
///
/// 1. Define menu items using `MenuItemTemplate`
/// 2. Implement `MenuBuilder` for your component
/// 3. Call `render_menu()` in your view
///
/// # Example
///
/// ```
/// use crate::fragments::side_menu::{MenuBuilder, MenuItemTemplate};
///
/// impl MenuBuilder for MyComponent {
///     type TabType = MyTab;
///
///     fn menu_config() -> &'static [MenuItemTemplate<MyTab>] {
///         &[
///             // Single icon
///             MenuItemTemplate {
///                 title_key: 100,
///                 icon_classes: &[&["fas", "fa-home"]],
///                 tab: MyTab::Home,
///                 custom_class: None,
///             },
///             // Two icons (e.g., regular + bookmark)
///             MenuItemTemplate {
///                 title_key: 101,
///                 icon_classes: &[&["fas", "fa-star"], &["fas", "fa-bookmark"]],
///                 tab: MyTab::Favorites,
///                 custom_class: None,
///             },
///         ]
///     }
///
///     fn is_active(&self, tab: &MyTab) -> bool { self.current_tab == *tab }
///     fn get_count(&self, _tab: &MyTab) -> usize { 0 }
///     fn is_extend(&self, _tab: &MyTab) -> bool { false }
///     fn get_action(&self, tab: &MyTab) -> Callback<MouseEvent> {
///         self.link.callback(move |_| Msg::SelectTab(tab.clone()))
///     }
/// }
///
/// // In view:
/// html! { self.render_menu() }
/// ```
pub(crate) trait MenuBuilder {
    type TabType: Clone + PartialEq + 'static;

    /// Returns static array of menu item templates
    fn menu_config() -> &'static [MenuItemTemplate<Self::TabType>];

    /// Returns true if tab is currently selected (highlights the item)
    fn is_active(&self, tab: &Self::TabType) -> bool;

    /// Returns badge number (return 0 to hide badge)
    fn get_count(&self, tab: &Self::TabType) -> usize;

    /// Returns true if tab is in extended state (adds "extend" CSS class)
    fn is_extend(&self, tab: &Self::TabType) -> bool;

    /// Returns click handler for the menu item
    fn get_action(&self, tab: &Self::TabType) -> Callback<MouseEvent>;

    /// Builds a vector of MenuItem from static configuration
    fn build_menu(&self) -> Vec<MenuItem> {
        Self::menu_config()
            .iter()
            .map(|template| MenuItem {
                title: get_value_field(&template.title_key),
                action: self.get_action(&template.tab),
                icon_classes: template.icon_classes,
                count: self.get_count(&template.tab),
                is_active: self.is_active(&template.tab),
                is_extend: self.is_extend(&template.tab),
                item_class: classes!(template.custom_class.unwrap_or("has-background-white")),
            })
            .collect()
    }

    /// Renders the side menu component with current configuration
    fn render_menu(&self) -> Html {
        html!{ <SideMenu menu_arr={self.build_menu()} /> }
    }
}