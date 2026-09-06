use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use web_sys;

use crate::fragments::copy_button::CopyButton;
use crate::fragments::modal::ModalBlock;
use crate::services::{unique_id, LocaleKey};


pub struct ShareLinkBtn {
    open_window: bool,
    link: ComponentLink<Self>,
    share_link: String,
    input_id: String
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub share_link: String,
}

#[derive(Clone)]
pub enum Msg {
    ShowShare,
}

impl Component for ShareLinkBtn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let location = document.location().expect("document should have a location");
        let share_link = if props.share_link == String::new() {
          location.href().unwrap()
        } else {
          props.share_link
        };

        ShareLinkBtn {
            link,
            open_window: false,
            input_id: unique_id("share-link-btn"),
            share_link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowShare => self.open_window = !self.open_window,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_share_btn = self.link.callback(move |_| Msg::ShowShare);
        html!{<>
            {self.share_window()}
            <button id={format!("open-{}", self.input_id)} class="button" onclick={onclick_share_btn} title={LocaleKey::Share.get_value()}>
              <span class="icon is-small" style="color: #1872f0;"><i class="fas fa-share" /></span>
            </button>
        </>}
    }
}

impl ShareLinkBtn {
  fn share_window(&self) -> Html {
      let onclick_share_btn = self.link.callback(|_| Msg::ShowShare);
      html! {
          <ModalBlock
              modal_id="share-window"
              title=""
              is_active={self.open_window}
              on_close={onclick_share_btn}
              on_save={None}
              save_disabled={false}
          >
              <div class="box">
                    <div class="field has-addons">
                        <div class="control is-expanded">
                            <input
                                id={self.input_id.clone()}
                                type="text"
                                class="input is-link"
                                readonly={true}
                                value={self.share_link.clone()}
                            />
                        </div>
                        <div class="control">
                            <CopyButton
                                text={self.share_link.clone()}
                                show_text=true
                                reset_delay={3000}
                            />
                        </div>
                    </div>
              </div>
          </ModalBlock>
      }
  }
}
