// mod item;

// use item::Item;
use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct CreateTender {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    // link: ComponentLink<Self>,
    value: i64,
}

impl Component for CreateTender {
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
        html! {
          <div>
            <div class="is-size-6">{"Создание тендера на производство"}</div>
            <div class="is-size-3 has-text-weight-bold">
              {"Opel C20XE 2.2 16v Turbo Engine"}
            </div>
            <div class="is-size-7">{"Add description"}</div>
            <textarea class="textarea" placeholder="e.g. Hello world"></textarea>
            <div class="is-size-7">{"Место поставки"}</div>
            <input class="input" type="text" placeholder="Text input" />
            <div class="columns">
              <div class="column">
                <div class="columns is-mobile">
                  <div class="column">
                    <div class="is-size-7">{"Дата окончания приема заявок"}</div>
                    <input class="input" type="text" placeholder="Text input" />
                  </div>
                  <div class="column">
                    <div class="is-size-7">{"Начальная цена"}</div>
                    <input class="input" type="text" placeholder="Text input" />
                  </div>
                </div>
                <div class="columns is-mobile">
                  <div class="column">
                    <div class="file is-large is-boxed has-name">
                      <label
                        class="file-label"
                        style="width: 100%; text-align: center"
                      >
                        <input class="file-input" type="file" name="resume" />
                        <span class="file-cta">
                          <span class="file-icon">
                            <i class="fas fa-upload"></i>
                          </span>
                          <span class="file-label"> {"Large file…"} </span>
                        </span>
                        <span class="file-name" style="max-width: 100%">
                          {"Screen Shot 2017-07-29 at 15.54.25.png"}
                        </span>
                      </label>
                    </div>
                  </div>
                </div>
                <div class="columns is-mobile">
                  <div>
                    {"Максимальный размер файла 4 mb. Максимальный обьединенный размер
                    файла 25 mb. Допустимые форматы PDF, JPEG, PNG, TIF."}
                  </div>
                </div>
                <div class="columns is-mobile">
                  <div class="column">
                    <div class="is-size-7">{"Company"}</div>
                    <input class="input" type="text" placeholder="Text input" />
                  </div>
                  <div class="column">
                    <div class="is-size-7">{"phone number"}</div>
                    <input class="input" type="text" placeholder="Text input" />
                  </div>
                </div>
                <div class="columns is-mobile">
                  <div class="column">
                    <div class="is-size-7">{"Code"}</div>
                    <input class="input" type="text" placeholder="Text input" />
                  </div>
                  <div class="column">
                    <div class="is-size-7">{"e-mail_1"}</div>
                    <input class="input" type="text" placeholder="Text input" />
                  </div>
                </div>
                <div class="columns is-mobile">
                  <div class="column">
                    <div class="is-size-7">{"First /Second name"}</div>
                    <input class="input" type="text" placeholder="Text input" />
                  </div>
                  <div class="column">
                    <div class="is-size-7">{"e-mail_2"}</div>
                    <input class="input" type="text" placeholder="Text input" />
                  </div>
                </div>
                <div class="columns is-mobile">
                  <div class="column">
                    <button class="button is-info">{"Create Tender"}</button>
                    <button class="button">{"Cancel"}</button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        }
    }
}
