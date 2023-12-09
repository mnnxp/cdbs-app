use yew::{html, Component, ComponentLink, Html, ShouldRender, MouseEvent, Callback, Properties};
use web_sys;
use rand;

use crate::services::set_clipboard;


pub struct ShareLinkBtn {
    open_window: bool,
    link: ComponentLink<Self>,
    share_link: String,
    copyed: bool,
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
    Copyed(bool),
    Ignore,
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
            input_id: rand::random::<char>().to_string(),
            share_link,
            copyed: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::ShowShare => {
              self.open_window = !self.open_window;
              if !self.open_window {
                self.copyed = false
              }
            },
            Msg::Copyed(value) => self.copyed = value,
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let target = self.input_id.clone();
        let onclick_share_btn = self.link.callback(move |_| {
          set_clipboard(format!(".{}", target).as_str());
          Msg::ShowShare
        });
        html!{<>
            {self.share_window()}
            <button id="share-btn" class="button" onclick={onclick_share_btn} >
              <span class="icon is-small"><i class="fas fa-share" /></span>
            </button>
        </>}
    }
}

impl ShareLinkBtn {
    fn share_window(&self) -> Html {
        let onclick_share_btn = self.link.callback(|_| Msg::ShowShare);
        let class_modal = match &self.open_window {
            true => "modal is-active",
            false => "modal",
        };

        let oncopyed: Callback<MouseEvent> = self.link.callback(|_| Msg::Copyed(true));

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_share_btn.clone()} />
              <div class="modal-content">
                <div class="card column">
                  <div class="clipboardBox" > 
                        <input id={self.input_id.clone()} type="text" class="input is-link inputBox" readonly={true} value={self.share_link.clone()} />
                        <button class={format!("btn button is-info {}", self.input_id.clone())} onclick={oncopyed} data-clipboard-target={format!("#{}", self.input_id)}>
                          {if self.copyed { html!{
                            <>
                            {"Copyed"}<i class="copyIcon fas fa-check"></i>
                            </>
                          }} else { html!{{"Copy"}} }}
                        </button>
                      </div>
                </div>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick={onclick_share_btn} />
            </div>
        }
    }
}
