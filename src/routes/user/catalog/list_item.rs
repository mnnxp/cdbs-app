use yew::prelude::*;
use crate::fragments::switch_icon::res_btn;
use super::ShowUserShort;

pub enum Msg {
    AddOne,
    TriggerFav
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowUserShort,
    pub show_list: bool,
}

pub struct ListItem {
    // link: ComponentLink<Self>,
    props: Props
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            // link,
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        if self.props.show_list != props.show_list {
            self.props.show_list = props.show_list;
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
      // let clickEvent = self.link.ca;
      // let Props { add_fav, del_fav, .. } = self.props.clone();

      match self.props.show_list {
        true => { self.showing_in_list() },
        false => { self.showing_in_box() },
      }
    }
}

impl ListItem {
    fn showing_in_list(&self) -> Html {
        let ShowUserShort {
            username,
            image_file,
            ..
        } = &self.props.data;

        html! {
          <div class="box itemBox">
            <article class="media center-media">
              <div class="media-left">
                <figure class="image is-96x96">
                  <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
                </figure>
              </div>
              <div class="media-content" style="min-width: 0px;">
                <div class="content">
                  <p>
                    <div style="margin-bottom:0" >
                      {username}
                    </div>
                    // <div class="overflow-title has-text-weight-bold	is-size-4" >{username}</div>
                  </p>
                </div>
              </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let ShowUserShort {
            username,
            image_file,
            ..
        } = self.props.data.clone();

        html! {
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
              </div>
              {username}
              // <div class="overflow-title has-text-weight-bold	is-size-4" >{username}</div>
            </div>
          </div>
        }
    }
}
