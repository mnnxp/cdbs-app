use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct ListItem {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for ListItem {
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
          <article class="media center-media">
            <div class="media-left">
              <figure class="image is-96x96">
                <div class="top-tag" >{"standart"}</div>
                <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              </figure>
            </div>
            <div class="media-content" style="min-width: 0px;">
              <div class="content">
                <p>
                  <div style="margin-bottom:0" >
                    {"manufactured by "} <span class="id-box has-text-grey-light has-text-weight-bold">{"Alphametall"}</span>
                  </div>
                  <div class="overflow-title has-text-weight-bold	is-size-4" >{"Opel C20XE 2.2 16v Turbo Engine"}</div>
                  <div>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean efficitur sit amet massa fringilla egestas. Nullam condimentum luctus turpis."}</div>
                </p>
              </div>
            </div>
            <div class="media-right flexBox " >
              <button class="button">
                <span class="icon is-small">
                  <i class="fas fa-cloud-download-alt"></i>
                </span>
              </button>
              <button class="button  is-info">
                <span class="icon is-small">
                  <i class="fas fa-cloud-download-alt"></i>
                </span>
              </button>
              <button class="button">
                <span class="icon is-small">
                  <i class="far fa-bookmark"></i>
                </span>
              </button>
              <button class="button">
                <span class="icon is-small">
                  <i class="fas fa-bookmark"></i>
                </span>
              </button>
            </div>
          </article>
        </div>
          }
    }
}
