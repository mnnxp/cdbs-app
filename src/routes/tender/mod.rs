mod create_tender;
mod item;

pub use create_tender::CreateTender;

use item::Item;
use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Tenders {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    // link: ComponentLink<Self>,
    value: i64,
}

impl Component for Tenders {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            // link,
            value: 0
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
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html!{
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
                </div>
              </div>
              {vec![1;3].iter().map(|_| {
                html!{ <Item/> }
              }).collect::<Html>()}
            </div>
        }
    }
}
