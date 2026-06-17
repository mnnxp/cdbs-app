mod builder;
mod model;

pub(crate) use builder::MenuBuilder;
pub(crate) use model::{MenuItem, MenuItemTemplate};

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

/// Side menu component
pub(crate) struct SideMenu {
    props: Props,
}

/// Component properties
#[derive(Properties, Clone)]
pub(crate) struct Props {
    /// Array of menu items to display
    pub(crate) menu_arr: Vec<MenuItem>,
}

impl Component for SideMenu {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SideMenu { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="side-menu-wrapper">
                <nav class="side-menu">
                    <ul>
                        {for self.props.menu_arr.iter().map(|x| self.li_generator(x))}
                    </ul>
                </nav>
            </div>
        }
    }
}

impl SideMenu {
    /// Renders single menu item as <li> with nested <a>
    fn li_generator(&self, item: &MenuItem) -> Html {
        let hide_tag = item.count == 0;
        let mut final_class = item.item_class.clone();
        if item.is_active {
            final_class.push("active");
        }
        if item.is_extend {
            final_class.push("extend");
        }

        html! {
            <li class={final_class} onclick={item.action.clone()}>
                <a>
                    <span>{item.title}</span>
                    <div hidden={hide_tag}>
                        <span class="tag is-info is-small">{item.count}</span>
                    </div>
                    {for item.icon_classes.iter().map(|icon_class|
                        html!{ <i class={*icon_class}></i> }
                    )}
                </a>
            </li>
        }
    }
}