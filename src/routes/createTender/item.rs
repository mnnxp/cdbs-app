use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Item {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Item {
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
          <div class="box itemBox">
            <article class="media">
              <div class="media-left">
                <figure class="image is-96x96">
                  <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                </figure>
              </div>
              <div class="media-content">
                <div class="content">
                  <p>
                    <div class="level" style="margin-bottom:0" >
                      <span class="level-left id-box has-text-grey-light">{"5045001885"}</span>
                      <span class="level-right price-box has-text-right has-text-grey	has-text-weight-bold">{"360 000 $"}</span>
                    </div>
                    <strong>{"Opel C20XE 2.2 16v Turbo Engine"}</strong>
                    <br/>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean efficitur sit amet massa fringilla egestas. Nullam condimentum luctus turpis."}
                  </p>
                </div>
                <span class="icon-text">
                  <span class="icon">
                    <svg style="width:24px;height:24px" viewBox="0 0 24 24">
                      <path fill="currentColor" d="M12,11.5A2.5,2.5 0 0,1 9.5,9A2.5,2.5 0 0,1 12,6.5A2.5,2.5 0 0,1 14.5,9A2.5,2.5 0 0,1 12,11.5M12,2A7,7 0 0,0 5,9C5,14.25 12,22 12,22C12,22 19,14.25 19,9A7,7 0 0,0 12,2Z" />
                    </svg>
                  </span>
                  <span class=" has-text-grey	has-text-weight-bold is-size-6" >{"North Triston"}</span>
                </span>
              </div>
            </article>
          </div>
        }
    }
}
