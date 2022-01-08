use web_sys::MouseEvent;
use yew::{
    classes, html, Callback, Classes, Component, ComponentLink, Html, Properties, ShouldRender,
};

pub struct SideMenu {
    props: Props,
    // link: ComponentLink<Self>,
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
    pub item_class: Classes,
    #[prop_or_default]
    pub icon_class: Classes,
    pub count: usize,
    pub is_active: bool,
    pub is_extend: bool,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            action: Callback::noop(),
            item_class: classes!("has-background-white"),
            icon_class: classes!("fas", "fa-certificate"),
            count: 0,
            is_active: false,
            is_extend: false,
        }
    }
}

#[derive(Properties, Clone)]
pub struct Props {
    pub menu_arr: Option<Vec<MenuItem>>,
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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SideMenu { props }
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
              {for self.props.menu_arr.as_ref().unwrap().iter().map(|x|
                  self.li_generator(x)
              )}
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
            item_class,
            icon_class,
            count,
            is_active,
            is_extend,
        } = item.clone();
        let hide_tag = count == 0;

        let mut item_class = item_class;

        if is_active {
            item_class.push("active");
        }

        if is_extend {
            item_class.push("extend");
        }

        html!(
          <li class=item_class onclick=action>
            <a>
              <span>{title}</span>
              {match hide_tag {
                  true => html!{},
                  false => html!{
                      <div style="display: inline-flex;" >
                        <span class="tag is-info is-small" >{count}</span>
                      </div>
                  },
              }}
              <i class=classes!(icon_class.clone())></i>
            </a>
          </li>
        )
    }
}
