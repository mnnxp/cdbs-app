use web_sys::MouseEvent;
use yew::{Component, Callback, Context, html, Html, Properties, classes, Classes};

pub struct SideMenu {}

// #[derive(Properties, Clone, Debug, PartialEq)]
// pub struct MenuBox {
//     pub title: String,
//     pub children: Vec<MenuItem>,
// }

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub title: String,
    pub action: Callback<MouseEvent>,
    #[prop_or_default]
    pub item_class: Classes,
    #[prop_or_default]
    pub icon_classes: Vec<Classes>,
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
            icon_classes: vec![classes!("fas", "fa-certificate")],
            count: 0,
            is_active: false,
            is_extend: false,
        }
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
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

    fn create(ctx: &Context<Self>) -> Self {
        SideMenu {}
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <nav class="side-menu">
            <ul>
              {for ctx.props().menu_arr.as_ref().unwrap().iter().map(|x|
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
            count,
            is_active,
            is_extend,
            ..
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
          <li class={item_class} onclick={action}>
            <a>
              <span>{title}</span>
              <div hidden={hide_tag} >
                <span class="tag is-info is-small" >{count}</span>
              </div>
              {for item.icon_classes.iter().map(|icon_class|
                  html!{<i class={classes!(icon_class.clone())} style="margin-right: 0.1rem"></i>}
              )}
            </a>
          </li>
        )
    }
}
