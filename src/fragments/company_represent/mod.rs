mod list_item;

use list_item::ListItem;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::CompanyRepresent;

pub enum Msg {
    SwitchShowType,
}

#[derive(PartialEq, Eq)]
pub enum ListState {
    List,
    Box,
}

pub struct CompanyRepresents {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    show_type: ListState,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_manage_btn: bool,
    pub list: Vec<CompanyRepresent>,
}

impl Component for CompanyRepresents {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            show_type: ListState::List,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::SwitchShowType => {
                match self.show_type {
                    ListState::Box => self.show_type = ListState::List,
                    _ => self.show_type = ListState::Box,
                }
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_manage_btn == props.show_manage_btn {
            // debug!("if change");
            false
        } else {
            self.props.show_manage_btn = props.show_manage_btn;
            // debug!("else change");
            true
        }
    }

    fn view(&self) -> Html {
        // let list = self.props.list.clone();

        let onclick_change_view = self
            .link
            .callback(|_|Msg::SwitchShowType);

        let class_for_icon: &str;
        let mut class_for_list = "";
        match self.show_type {
            ListState::Box => {
                class_for_icon = "fas fa-bars";
                class_for_list = "flex-box";
            },
            ListState::List => {
                class_for_icon = "fas fa-th-large";
            },
        };

        html! {
            <div class="representsBox" >
              <ListErrors error=self.error.clone()/>
              <div class="level" >
                <div class="level-left ">
                {match &self.props.show_manage_btn {
                    true => html! {
                        <RouterAnchor<AppRoute> route=AppRoute::CreateTender >
                          <button class="button is-info" >{"Create"}</button>
                        </RouterAnchor<AppRoute>>
                    },
                    false => html! {},
                }}
                </div>
                <div class="level-right">
                  <div class="select">
                    <select>
                      <option>{"Select dropdown"}</option>
                      <option>{"With options"}</option>
                    </select>
                  </div>
                  <button class="button" onclick={onclick_change_view} >
                    <span class={"icon is-small"}>
                      <i class={class_for_icon}></i>
                    </span>
                  </button>
                </div>
              </div>
              <div class={class_for_list}>
                {for self.props.list.iter().map(|x| self.show_card(&x))}
              </div>
            </div>
        }
    }
}

impl CompanyRepresents {
    fn show_card(
        &self,
        show_company_represent: &CompanyRepresent,
    ) -> Html {
        html! {
            <ListItem data={show_company_represent.clone()}
                show_list={self.show_type == ListState::List}
                />
        }
    }
}
