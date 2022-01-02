use web_sys::MouseEvent;
use yew::{
    classes, html, Callback, Classes, Component, ComponentLink, Html, Properties, ShouldRender,
};

pub struct SideMenu {
    props: Props,
    link: ComponentLink<Self>,
}

// #[derive(Properties, Clone)]
// pub struct MenuBox {
//     pub title: String,
//     pub children: Vec<MenuItem>,
// }

#[derive(Properties, Clone)]
pub struct MenuItem {
    pub title: String,
    pub action: Callback<MouseEvent>,
    #[prop_or_default]
    pub icon_class: Classes,
    pub count: usize,
    pub is_active: bool,
    pub is_extend: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    // pub current_route: Option<AppRoute>,
    // pub username: String,
    pub menu_arr: Option<Vec<MenuItem>>,
    // pub tab: ProfileTab,
}

#[derive(Clone)]
pub enum Msg {
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    GetSelfData(String),
    GetUserData(String),
    UpdateList(String),
    Ignore,
}

impl Component for SideMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SideMenu { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
          <nav class="side-menu">
            <ul>
              { for self.props.menu_arr.as_ref().unwrap().iter().map(|x|self.li_generator(x)) }
            </ul>
          </nav>
        }
    }
}

impl SideMenu {
    fn li_generator(&self, item: &MenuItem) -> Html {
        let MenuItem {
            title,
            action,
            icon_class,
            count,
            is_active,
            is_extend,
        } = item.clone();
        let show_tag = count == 0;

        html!(
          <li class=classes!( if is_active {"active"} else {""}, if is_extend {"extend"} else {""} ) onclick=action>
            <a>
              <span>{title}</span>
              <span hidden=show_tag>
                <span class="tag is-info is-small" >{count}</span>
              </span>
              <i class=classes!(icon_class.clone())></i>
            </a>
          </li>
        )
    }
}
