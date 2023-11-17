use crate::services::image_detector;
use crate::types::{DownloadFile, UUID};
use log::debug;
use std::collections::BTreeMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::{
    classes, html, Callback, Children, Component, ComponentLink, Html, Properties, ShouldRender,
};

pub struct Modal {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub show_modal: bool,
    #[prop_or_default]
    pub children: Children,
    pub onclose: Callback<MouseEvent>,
}

#[derive(Clone)]
pub enum Msg {
    IsActive(bool),
    Ignore,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Modal { props }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let active_menu = if self.props.show_modal {
            "is-active"
        } else {
            ""
        };

        html! {
          <div id="modal-js" class={format!("modal {}", active_menu)}>
            <div class="modal-background" onclick={self.props.onclose.clone()}></div>

            <div class="modal-content">
              <div class="box">
              { for self.props.children.iter() }
              </div>
            </div>

            <button class="modal-close is-large" aria-label="close" onclick={self.props.onclose.clone()}></button>
          </div>
        }
    }
}
