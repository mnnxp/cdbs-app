mod add_represent;
mod change_item;
mod list_item;

pub use add_represent::AddCompanyRepresentCard;
use list_item::ListItem;
use change_item::ChangeItem;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
// use log::debug;
use crate::error::Error;
use crate::fragments::{list_errors::ListErrors, list_empty::ListEmpty};
use crate::types::CompanyRepresentInfo;
use crate::fragments::ListState;

pub enum Msg {
    SwitchShowType,
    ClearError,
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
    pub list: Vec<CompanyRepresentInfo>,
}

impl Component for CompanyRepresents {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            show_type: ListState::get_from_storage(),
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
                ListState::set_to_storage(&self.show_type);
            },
            Msg::ClearError => self.error = None,
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
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{
            <div class="representsBox">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
                {match &self.props.show_manage_btn {
                    true => html!{<>
                        {for self.props.list.iter().map(|represent|
                            html!{<ChangeItem data={represent.clone()} />}
                        )}
                    </>},
                    false => self.show_card(),
                }}
            </div>
        }
    }
}

impl CompanyRepresents {
    fn show_card(&self) -> Html {
        let onclick_change_view = self.link.callback(|_|Msg::SwitchShowType);
        let (class_for_icon, class_for_list) = match self.show_type {
            ListState::Box => ("fas fa-bars", "flex-box"),
            ListState::List => ("fas fa-th-large", ""),
        };

        html!{<>
            <div class="level" >
              <div class="level-left ">
              </div>
              <div class="level-right">
                <button class="button" onclick={onclick_change_view} >
                  <span class={"icon is-small"}>
                    <i class={class_for_icon}></i>
                  </span>
                </button>
              </div>
            </div>
            {if self.props.list.is_empty() {
                html!{<ListEmpty />}
            } else { html!{
                <div class={class_for_list}>
                    {for self.props.list.iter().map(|represent|
                        html!{
                            <ListItem
                                data={represent.clone()}
                                show_list={self.show_type == ListState::List}
                            />
                        }
                    )}
                </div>
            }}}
        </>}
    }
}
