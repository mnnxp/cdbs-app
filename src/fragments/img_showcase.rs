use std::collections::BTreeMap;
use yew::{Component, Context, html, html::Scope, Html, Properties, classes};
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::services::image_detector;
use crate::types::{DownloadFile, UUID};

pub struct ImgShowcase {
    object_uuid: UUID,
    file_arr_len: usize,
    selected_img: usize,
    show_image: bool,
    img_arr: BTreeMap<usize, DownloadFile>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub object_uuid: UUID,
    pub file_arr: Vec<DownloadFile>,
}

#[derive(Clone)]
pub enum Msg {
    ParsingFiles,
    SetImgArr(BTreeMap<usize, DownloadFile>),
    SetSelectImg(usize),
    ShowImg,
    Ignore,
}

impl Component for ImgShowcase {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ImgShowcase {
            object_uuid: ctx.props().object_uuid,
            file_arr_len: ctx.props().file_arr.len(),
            selected_img: 0,
            show_image: false,
            img_arr: BTreeMap::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::ParsingFiles);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::ParsingFiles => {
                let file_arr = ctx.props().file_arr.clone();
                spawn_local(async move {
                    let mut img_arr: BTreeMap<usize, DownloadFile> = BTreeMap::new();
                    let mut key = 0;

                    for file_data in &file_arr {
                        if image_detector(&file_data.filename) {
                            img_arr.insert(key, file_data.clone());
                            key += 1;
                        }
                    }

                    link.send_message(Msg::SetImgArr(img_arr));
                });
            },
            Msg::SetImgArr(img_arr) => self.img_arr = img_arr,
            Msg::SetSelectImg(index) => self.selected_img = index,
            Msg::ShowImg => self.show_image = !self.show_image,
            Msg::Ignore => {}
        };
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.object_uuid == ctx.props().object_uuid &&
              self.file_arr_len == ctx.props().file_arr.len() {
            debug!("no change: {:?}", ctx.props().file_arr.len());
            false
        } else {
            self.object_uuid == ctx.props().object_uuid;
            self.file_arr_len == ctx.props().file_arr.len();
            ctx.link().send_message(Msg::ParsingFiles);
            debug!("change: {:?}", ctx.props().file_arr.len());
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_show_image = ctx.link().callback(|_| Msg::ShowImg);

        match self.img_arr.is_empty() {
            true => html!{<div style="padding-left: 0.75rem;" />}, // <-- not found images for display
            false => html!{
                <div class="column is-one-quarter show-img-box">
                    <div class="showImg">
                      <div class="outBox">
                        <div class="itemBox">
                          {for self.img_arr.iter().map(|x|
                            {self.item_generator(ctx.link(), x.clone())}
                          )}
                        </div>
                      </div>
                      <div class="mainImgBox">
                          {match self.img_arr.get(&self.selected_img) {
                              Some(img_data) => html!{
                                  <img onclick={onclick_show_image} src={img_data.download_url.clone()} alt="" srcset="" />
                              },
                              None => html!{}, // <-- not found image for display
                          }}
                      </div>
                      {self.modal_full_image(ctx.link())}
                    </div>
                </div>
            }
        }
    }
}

impl ImgShowcase {
    fn item_generator(
        &self,
        link: &Scope<Self>,
        img_item: (&usize, &DownloadFile)
    ) -> Html {
        let (idx, image) = img_item;
        let saved_idx = *idx;
        let onclick_select_img = link.callback(move |_| Msg::SetSelectImg(saved_idx));
        let mut classes_img = classes!("item");
        if self.selected_img == saved_idx {
            classes_img.push("active")
        };

        html!(
            <div class={classes_img} onclick={onclick_select_img}>
              <img src={image.download_url.clone()} alt="" srcset="" />
            </div>
        )
    }

    fn modal_full_image(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_show_image = link.callback(|_| Msg::ShowImg);
        let class_modal = match &self.show_image {
            true => "modal is-active",
            false => "modal",
        };

        match self.img_arr.get(&self.selected_img) {
            Some(img_data) => html! {
                <div class={class_modal}>
                  <div class="modal-background" onclick={onclick_show_image.clone()} />
                  <div class="modal-content">
                    <p class="image is-4by3">
                      // <img src="https://bulma.io/images/placeholders/1280x960.png" alt="" />
                      <img src={img_data.download_url.clone()} loading="lazy"/>
                    </p>
                  </div>
                  <button class="modal-close is-large" aria-label="close" onclick={onclick_show_image} />
                </div>
            },
            None => html!{}, // <-- not found image for shown in modal
        }
    }
}
