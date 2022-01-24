use std::collections::HashMap;
use yew::{
    classes, html, Component, ComponentLink, Html, Properties, ShouldRender,
};
// use web_sys::MouseEvent;
use crate::services::filter_images;
use crate::types::DownloadFile;

pub struct ImgShowcase {
    props: Props,
    selected_img: usize,
    show_image: bool,
    link: ComponentLink<Self>,
    img_arr: HashMap<usize, DownloadFile>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub file_arr: Vec<DownloadFile>,
}

#[derive(Clone)]
pub enum Msg {
    SetSelectImg(usize),
    ShowImg,
    Ignore,
}

impl Component for ImgShowcase {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut img_arr: HashMap<usize, DownloadFile> = HashMap::new();
        let mut key = 0;
        for file_data in &props.file_arr {
            if filter_images(&file_data.filename) {
                img_arr.insert(key, file_data.clone());
                key += 1;
            }
        }

        ImgShowcase {
            props,
            selected_img: 0,
            link,
            show_image: false,
            img_arr,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.file_arr.len() == props.file_arr.len() &&
              self.props.file_arr.last().map(|x| &x.uuid) == props.file_arr.last().map(|x| &x.uuid) {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetSelectImg(index) => self.selected_img = index,
            Msg::ShowImg => self.show_image = !self.show_image,
            Msg::Ignore => {}
        };
        true
    }

    fn view(&self) -> Html {
        let onclick_show_image = self.link.callback(|_| Msg::ShowImg);

        match self.img_arr.is_empty() {
            true => html!{<div style="padding-left: 0.75rem;" />}, // <-- not found images for display
            false => html!{
                <div class="column is-one-quarter show-img-box">
                    <div class="showImg">
                      <div class="outBox">
                        <div class="itemBox">
                          {for self.img_arr.iter().map(|x|
                            {self.item_generator(x.clone())}
                          )}
                        </div>
                      </div>
                      <div class="mainImgBox">
                          {match self.img_arr.get(&self.selected_img) {
                              Some(img_data) => html!{<img onclick=onclick_show_image src=img_data.download_url.clone() alt="" srcset="" />},
                              None => html!{}, // <-- not found image for display
                          }}
                      </div>
                      {self.modal_full_image()}
                    </div>
                </div>
            }
        }
    }
}

impl ImgShowcase {
    fn item_generator(
        &self,
        img_item: (&usize, &DownloadFile)
    ) -> Html {
        let (idx, image) = img_item;
        let saved_idx = *idx;
        let onclick_select_img = self.link.callback(move |_| Msg::SetSelectImg(saved_idx));

        let mut classes_img = classes!("item");
        if self.selected_img == saved_idx { classes_img.push("active") };

        html!(
            <div class={classes_img}
                onclick={onclick_select_img}
            >
              <img src=image.download_url.clone() alt="" srcset="" />
            </div>
        )
    }

    fn modal_full_image(&self) -> Html {
        let onclick_show_image = self.link.callback(|_| Msg::ShowImg);

        let class_modal = match &self.show_image {
            true => "modal is-active",
            false => "modal",
        };

        match self.img_arr.get(&self.selected_img) {
            Some(img_data) => html! {
                <div class=class_modal>
                  <div class="modal-background" onclick=onclick_show_image.clone() />
                  <div class="modal-content">
                    <p class="image is-4by3">
                      // <img src="https://bulma.io/images/placeholders/1280x960.png" alt="" />
                      <img
                        src={img_data.download_url.clone()}
                        loading="lazy"
                      />
                    </p>
                  </div>
                  <button class="modal-close is-large" aria-label="close" onclick=onclick_show_image />
                </div>
            },
            None => html!{}, // <-- not found image for shown in modal
        }
    }
}
