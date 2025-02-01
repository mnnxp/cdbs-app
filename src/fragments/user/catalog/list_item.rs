use yew::{agent::Bridged, html, Bridge, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};
use crate::routes::AppRoute;
use crate::fragments::switch_icon::res_fullwidth_btn;
use crate::services::get_value_field;
use crate::types::Pathname;
use super::ShowUserShort;

pub enum Msg {
    ShowProfile,
    Ignore,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowUserShort,
    pub show_list: bool,
}

pub struct ListItemUser {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    username: String,
    props: Props,
}

impl Component for ListItemUser {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            username: props.data.username.to_string(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowProfile => {
                // Redirect to profile page
                self.router_agent.send(ChangeRoute(AppRoute::Profile(self.username.to_string()).into()));
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list || self.props.data.uuid != props.data.uuid {
            self.props.show_list = props.show_list;
            self.username = props.data.username.to_string();
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
      match self.props.show_list {
        true => { self.showing_in_list() },
        false => { self.showing_in_box() },
      }
    }
}

impl ListItemUser {
    fn showing_in_list(&self) -> Html {
        let ShowUserShort {
            firstname,
            lastname,
            username,
            image_file,
            ..
        } = &self.props.data;
        let onclick_open_profile = self.link.callback(|_| Msg::ShowProfile);
        html!{
          <div class="box itemBox" onclick={onclick_open_profile.clone()}>
            <article class="media center-media">
              <div class="media-left">
                <figure class="image is-96x96">
                  <img
                    src={image_file.download_url.to_string()} alt="Favicon profile"
                    loading="lazy"
                  />
                </figure>
              </div>
              <div class="media-content" style="min-width: 0px;">
                <div class="content">
                  <p>
                    <div style="margin-bottom:0" >
                      <div class="overflow-title has-text-weight-bold is-size-4" >
                        {format!("{} {}", firstname, lastname)}
                      </div>
                      <span class="overflow-title">{format!("@{}", username)}</span>
                    </div>
                  </p>
                </div>
              </div>
              <div class="media-right flexBox " >
                {res_fullwidth_btn(onclick_open_profile, get_value_field(&261), Pathname::User(self.props.data.username.clone()))}
              </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let ShowUserShort {
            firstname,
            lastname,
            username,
            image_file,
            ..
        } = self.props.data.clone();
        let onclick_open_profile = self.link.callback(|_| Msg::ShowProfile);
        html!{
          <div class="boxItem"  onclick={onclick_open_profile.clone()}>
            <div class="innerBox" >
              <div class="imgBox" >
                <img
                    src={image_file.download_url.to_string()} alt="Favicon profile"
                    loading="lazy"
                />
              </div>
              <div class="overflow-title has-text-weight-bold is-size-4" >
                {format!("{} {}", firstname, lastname)}
              </div>
              <span class="overflow-title">{format!("@{}", username)}</span>
            </div>
            <div class="btnBox">
              {res_fullwidth_btn(onclick_open_profile, get_value_field(&261), Pathname::User(self.props.data.username.clone()))}
            </div>
          </div>
        }
    }
}
