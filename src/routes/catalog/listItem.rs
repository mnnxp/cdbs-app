use yew::prelude::*;
use crate::fragments::switch_icon::resBtn;
use super::ShowedComponent;

pub enum Msg {
    AddOne,
    TriggerFav
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowedComponent,
    pub showList: bool,
    // pub triggerFav: Callback<MouseEvent>,
    pub addFav: Callback<String>,
    pub delFav : Callback<String>,
}

pub struct ListItem {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    props: Props
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                crate::yewLog!(self.value);
            }
            Msg::TriggerFav => {
                if !self.props.data.isFollowed {
                    self.props.addFav.emit("".to_string());
                } else {
                    self.props.delFav.emit("".to_string());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        if self.props.showList != props.showList || self.props.data.isFollowed != props.data.isFollowed {
            self.props.showList = props.showList;
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
      // let clickEvent = self.link.ca;
        let ShowedComponent { description, isBase, isFollowed, name, .. } = self.props.data.clone();
        let Props { addFav, delFav, .. } = self.props.clone();

        if self.props.showList {
          html! {
            <div class="box itemBox">
          <article class="media center-media">
            <div class="media-left">
              <figure class="image is-96x96">
                <div hidden={!isBase} class="top-tag" >{"standard"}</div>
                <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              </figure>
            </div>
            <div class="media-content" style="min-width: 0px;">
              <div class="content">
                <p>
                  <div style="margin-bottom:0" >
                    {"manufactured by "} <span class="id-box has-text-grey-light has-text-weight-bold">{"Alphametall"}</span>
                  </div>
                  <div class="overflow-title has-text-weight-bold	is-size-4" >{name}</div>
                  <div>{description}</div>
                </p>
              </div>
            </div>
            <div class="media-right flexBox " >
              {resBtn(classes!(String::from("fas fa-cloud-download-alt")), self.link.callback(|_| Msg::AddOne ), "".to_string())}
              // <SwitchIcon callback={BtnItem{class: String::from("fas fa-cloud-download-alt"),clickEvent:self.link.callback(|_| Msg::AddOne )}} />
              // <button class="button  is-info">
              //   <span class="icon is-small">
              //     <i class="fas fa-cloud-download-alt"></i>
              //   </span>
              // </button>
              {resBtn(classes!( if isFollowed {"fas"} else {"far"} , "fa-bookmark"), self.link.callback(|_| Msg::TriggerFav ), if isFollowed {
                "color: #3298DD;".to_string()
              } else {
                "".to_string()
              })}
              // <button class="button">
              //   <span class="icon is-small">
              //     <i class="fas fa-bookmark"></i>
              //   </span>
              // </button>
            </div>
          </article>
        </div>
          }
        } else {
          html! {
            <div class="boxItem" >
              <div class="innerBox" >
                <div class="imgBox" >
                  <div class="top-tag" hidden={!isBase} >{"standart"}</div>
                  <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                </div>
                <div>
                  {"manufactured by "}<span class="id-box has-text-grey-light has-text-weight-bold">{"Alphametall"}</span>
                </div>
                <div class="overflow-title has-text-weight-bold	is-size-4" >{name}</div>
                  <div class="btnBox">
                    <button class="button is-light is-fullwidth has-text-weight-bold">{"Download"}</button>
                    <div style="margin-left: 8px;">
                      {resBtn(classes!( if isFollowed {"fas"} else {"far"} , "fa-bookmark"), self.link.callback(|_| Msg::TriggerFav ), if isFollowed {
                        "color: #3298DD;".to_string()
                      } else {
                        "".to_string()
                      })}
                    </div>
                  </div>
              </div>
            </div>
          } 
        }
    }
}
