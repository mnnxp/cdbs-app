use yew::{html, Component, ComponentLink, Html, ShouldRender};
// use log::debug;

pub struct ShareLinkBtn {
    open_window: bool,
    link: ComponentLink<Self>,
    share_link: String,
}

#[derive(Clone)]
pub enum Msg {
    ShowShare,
    Ignore,
}

impl Component for ShareLinkBtn {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ShareLinkBtn {
            link,
            open_window: false,
            share_link: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::ShowShare => self.open_window = !self.open_window,
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_share_btn = self.link.callback(|_| Msg::ShowShare);
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

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_share_btn.clone()} />
              <div class="modal-content">
                <div class="card column">
                  // window with cool share button (copy and sent to socials) here
                </div>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick={onclick_share_btn} />
            </div>
        }
    }
}
