use crate::types::{DownloadFile};
// use web_sys::MouseEvent;
use yew::{
    classes, html, Component, ComponentLink, Html, Properties, ShouldRender,
};

pub struct ImgShowcase {
    props: Props,
    selected_img: usize,
    show_cert: bool,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub img_arr: Option<Vec<DownloadFile>>,
}

#[derive(Clone)]
pub enum Msg {
    SetSelectImg(usize),
    ShowCert,
    Ignore,
}

impl Component for ImgShowcase {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ImgShowcase {
            props,
            selected_img: 0,
            link,
            show_cert: false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetSelectImg(index) => {
                self.selected_img = index;
            }
            Msg::ShowCert => self.show_cert = !self.show_cert,
            Msg::Ignore => {}
        };
        true
    }

    fn view(&self) -> Html {
        let selected_img = self.selected_img;
        html! {
          <div class="showImg">
            <div class="itemBox">
              {for self.props.img_arr.as_ref().unwrap().iter().enumerate().map(|x|
                self.item_generator((x.0, x.1.clone()))
              )}
            </div>
            <div class="mainImgBox">
              <img onclick=self.link.callback(|_| Msg::ShowCert) src=self.props.img_arr.as_ref().unwrap()[selected_img].download_url.clone() alt="" srcset="" />
            </div>
            {self.modal_full_certificate()}
          </div>
        }
    }
}

impl ImgShowcase {
    fn item_generator(&self, file_item: (usize, DownloadFile)) -> Html {
        let (idx, file) = file_item;

        html!(
          <>
            <div class=classes!("item", if self.selected_img == idx { "active" } else { "" })
            onclick={self.link.callback(move |_| Msg::SetSelectImg(idx))}
            >
              <img src=file.download_url alt="" srcset="" />
            </div>
            
          </>
        )
    }

    fn modal_full_certificate(&self) -> Html {
        let onclick_show_cert = self.link.callback(|_| Msg::ShowCert);
        let img_src = self.props.img_arr.as_ref().unwrap()[self.selected_img].download_url.clone();

        let class_modal = match &self.show_cert {
            true => "modal is-active",
            false => "modal",
        };

        html! {
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_show_cert.clone() />
              <div class="modal-content">
                <p class="image is-4by3">
                  // <img src="https://bulma.io/images/placeholders/1280x960.png" alt="" />
                  <img
                    src={img_src}
                    loading="lazy"
                  />
                </p>
              </div>
              <button class="modal-close is-large" aria-label="close"></button>
            </div>
        }
    }
}
