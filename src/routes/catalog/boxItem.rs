use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct BoxItem {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for BoxItem {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
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
        html! {
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <div class="top-tag" >{"standart"}</div>
                <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              </div>
              <div>
                {"manufactured by "}<span class="id-box has-text-grey-light has-text-weight-bold">{"Alphametall"}</span>
              </div>
              <div class="overflow-title has-text-weight-bold	is-size-4" >{"Opel C20XE 2.2 16v Turbo Engine"}</div>
                <div class="btnBox">
                  <button class="button is-light is-fullwidth has-text-weight-bold">{"Download"}</button>
                  <button class="button" style="margin-left: 8px;">
                    <span class="icon is-small">
                      <i class="fas fa-bookmark"></i>
                    </span>
                  </button>
                </div>
            </div>
          </div>
        }
    }
}
