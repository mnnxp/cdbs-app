use yew::{
  prelude::*, agent::Bridged, html, Bridge, Component, ComponentLink,
  Html, Properties, ShouldRender,
};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
use crate::routes::AppRoute;
use crate::fragments::switch_icon::res_btn;
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

pub struct ListItem {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    username: String,
    props: Props,
}

impl Component for ListItem {
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
                self.router_agent.send(ChangeRoute(AppRoute::Profile(
                    self.username.to_string()
                ).into()));
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list {
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

impl ListItem {
    fn open_profile_page(
        &self,
        small_button: bool,
    ) -> Html {
        let onclick_open_profile = self
            .link
            .callback(|_| Msg::ShowProfile);

        match small_button {
            true => html! {
                {res_btn(
                    classes!(String::from("fas  fa-user-o")),
                    onclick_open_profile,
                    "".to_string()
                )}
            },
            false => html! {
                <button
                      class="button is-light is-fullwidth has-text-weight-bold"
                      onclick=onclick_open_profile >
                    {"Show profile"}
                </button>
            },
        }
    }

    fn showing_in_list(&self) -> Html {
        let ShowUserShort {
            firstname,
            lastname,
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
                      <div class="overflow-title has-text-weight-bold is-size-4" >
                        {format!("{} {}", firstname, lastname)}
                      </div>
                      <span class="overflow-title">{format!("@{}", username)}</span>
                    </div>
                    // <div class="overflow-title has-text-weight-bold	is-size-4" >{username}</div>
                  </p>
                </div>
              </div>
              <div class="media-right flexBox " >
                {self.open_profile_page(false)}
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

        html! {
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
              </div>
              <div class="overflow-title has-text-weight-bold is-size-4" >
                {format!("{} {}", firstname, lastname)}
              </div>
              <span class="overflow-title">{format!("@{}", username)}</span>
              // <div class="overflow-title has-text-weight-bold	is-size-4" >{username}</div>
            </div>
            // <div class="overflow-title has-text-weight-bold	is-size-4" >{username}</div>
            <div class="btnBox">
                {self.open_profile_page(false)}
            </div>
          </div>
        }
    }
}
