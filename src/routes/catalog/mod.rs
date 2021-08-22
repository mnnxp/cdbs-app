mod boxItem;
mod listItem;

use boxItem::BoxItem;
use listItem::ListItem;
use yew::prelude::*;

pub enum Msg {
    AddOne,
    SwitchShowType,
}

#[derive(PartialEq, Eq)]
pub enum ListState {
    List,
    Box,
}

pub struct Catalog {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    showType: ListState,
}

impl Component for Catalog {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            showType: ListState::List,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SwitchShowType => {
                match self.showType {
                    ListState::Box => self.showType = ListState::List,
                    _ => self.showType = ListState::Box,
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="tendersBox" >
              <div class="level" >
                <div class="level-left ">
                  <button class="button is-info">{"Create"}</button>
                </div>
                <div class="level-right">
                  <div class="select">
                    <select>
                      <option>{"Select dropdown"}</option>
                      <option>{"With options"}</option>
                    </select>
                  </div>
                  <button class="button" onclick={self.link.callback(|_|Msg::SwitchShowType)} >
                    <span class="icon is-small">
                      {if self.showType == ListState::Box {
                        html!{
                          <i class="fas fa-bars"></i>
                        }
                      }else {
                        html!{
                          <i class="fas fa-th-large"></i>
                        }
                      }}
                    </span>
                  </button>
                </div>
              </div>
              <div class=classes!( if self.showType == ListState::Box { "flex-box" } else { " " } ) >
                {vec![1;3].iter().map(|_| {
                  if self.showType == ListState::List {
                    html! { <ListItem/> }
                  }else{
                    html! { <BoxItem/> }
                  }
                }).collect::<Html>()}
              </div>
            </div>
        }
    }
}
