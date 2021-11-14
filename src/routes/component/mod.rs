// mod catalog;
// mod item;

// pub use catalog::CatalogComponents;

// use item::Item;
use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct ComponentTemp {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    // link: ComponentLink<Self>,
    value: i64,
}

impl Component for ComponentTemp {
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
            <div class="card">
              <div class="card-content">
                <div class="content">
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
                        </label>
                      </div>
                      {"Максимальный размер файла 4 mb. Максимальный обьединенный размер
                      файла 25 mb. Допустимые форматы PDF, JPEG, PNG, TIF."}
                      <p>
                        {"For more information please read the Community Values and
                        Guidelines."}
                      </p>
                    </div>
                  </div>
                  <div class="columns is-mobile">
                    <div class="column">
                      <div class="is-size-7">{"Component Name"}</div>
                      <input class="input" type="text" placeholder="Text input" />
                    </div>
                  </div>
                  <div class="columns is-mobile">
                    <div class="column">
                      <div class="is-size-7">{"Description"}</div>
                      <textarea
                        class="textarea"
                        placeholder="e.g. Hello world"
                      ></textarea>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div>
              <div class="columns is-mobile">
                <div class="column">
                  <div class="tags are-large">
                    <span class="tag">{"All"}</span>
                  </div>
                </div>
              </div>
              <div class="card">
                <table class="table is-fullwidth has-text-centered">
                  <thead>
                    <tr>
                      <th><abbr title="Position">{"ASA"}</abbr></th>
                      <th>{"Pitch (mm)"}</th>
                      <th><abbr title="Played">{"B (mm)"}</abbr></th>
                      <th><abbr title="Won">{"D1 (mm)"}</abbr></th>
                      <th><abbr title="Drawn">{"H (mm)"}</abbr></th>
                      <th>{"L (mm)"}</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr>
                      <th>{"1"}</th>
                      <td>
                        <a
                          href="https://en.wikipedia.org/wiki/Leicester_City_F.C."
                          title="Leicester City F.C."
                          >{"Leicester City"}</a
                        >
                        <strong>{"(C)"}</strong>
                      </td>
                      <td>{"38"}</td>
                      <td>{"23"}</td>
                      <td>{"12"}</td>
                      <td>{"3"}</td>
                    </tr>
                    <tr class="is-selected">
                      <th>{"2"}</th>
                      <td>
                        <a
                          href="https://en.wikipedia.org/wiki/Manchester_City_F.C."
                          title="Manchester City F.C."
                          >{"Manchester City"}</a>
                      </td>
                      <td>{"38"}</td>
                      <td>{"19"}</td>
                      <td>{"9"}</td>
                      <td>{"10"}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
            <div class="columns is-mobile">
              <div class="column">
                <div class="tags are-large">
                  <span class="tag">{"Characteristics"}</span>
                </div>
                <div class="card">
                  <div class="columns">
                    <div class="column is-flex is-justify-content-space-between">
                      <span>{"asd"}</span><span>{"asd"}</span>
                    </div>
                  </div>
                  <div class="columns">
                    <div class="column is-flex is-justify-content-center">
                      <button
                        class="button is-light has-text-black-bis has-text-weight-bold"
                      >
                        {"Show More"}
                      </button>
                    </div>
                  </div>
                </div>
              </div>
              <div class="column">
                <div class="tags are-large">
                  <span class="tag">{"Documents"}</span>
                </div>
                <div class="card">
                  <div class="columns">
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
                        </label>
                      </div>
                      {"Максимальный размер файла 4 mb. Максимальный обьединенный размер
                      файла 25 mb. Допустимые форматы PDF, JPEG, PNG, TIF."}
                      <p>
                        {"For more information please read the Community Values and
                        Guidelines."}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div class="columns is-mobile">
              <div class="column">
                <div class="tags are-large">
                  <span class="tag">{"Categories"}</span>
                </div>
                <div class="card">
                  <div class="columns">
                    <div class="column">
                      <div class="card">
                        <div class="field">
                          <p class="control has-icons-left">
                            <input
                              class="input"
                              type="password"
                              placeholder="Password"
                            />
                            <span class="icon is-small is-left">
                              <i class="fas fa-lock"></i>
                            </span>
                          </p>
                        </div>
                        <div>
                          <span class="tag is-light">{"Light"}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div class="columns is-mobile">
              <div class="column">
                <button class="button is-info" >{"Save"}</button>
                <button class="button">{"Cancel"}</button>
              </div>
            </div>
          </div>
        }
    }
}
