use yew::{Component, Context, html, html::Scope, Html, Properties, classes};
use yew_router::prelude::*;
use crate::routes::AppRoute::Profile;
use crate::fragments::switch_icon::res_btn;
use crate::services::get_value_field;
use super::{UUID, ShowUserShort};

pub enum Msg {
    ShowProfile,
    Ignore,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub data: ShowUserShort,
    pub show_list: bool,
}

pub struct ListItemUser {
    user_uuid: UUID,
    username: String,
    show_list: bool,
}

impl Component for ListItemUser {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user_uuid: ctx.props().data.uuid.clone(),
            username: ctx.props().data.username.clone(),
            show_list: ctx.props().show_list,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowProfile => {
                // Redirect to profile page
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Profile { username: self.username.to_string() });
            },
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.show_list == ctx.props().show_list ||
            self.user_uuid == ctx.props().data.uuid {
            false
        } else {
            self.show_list = ctx.props().show_list;
            self.user_uuid = ctx.props().data.uuid.clone();
            self.username = ctx.props().data.username.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
      match ctx.props().show_list {
        true => { self.showing_in_list(ctx.link(), ctx.props()) },
        false => { self.showing_in_box(ctx.link(), ctx.props()) },
      }
    }
}

impl ListItemUser {
    fn open_profile_page(
        &self,
        link: &Scope<Self>,
        small_button: bool,
    ) -> Html {
        let onclick_open_profile = link.callback(|_| Msg::ShowProfile);

        match small_button {
            true => html!{
                {res_btn(
                    classes!(String::from("fas  fa-user-o")),
                    onclick_open_profile,
                    String::new()
                )}
            },
            false => html!{
                <button
                      class="button is-light is-fullwidth has-text-weight-bold"
                      onclick={onclick_open_profile} >
                    { get_value_field(&261) }
                </button>
            },
        }
    }

    fn showing_in_list(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let ShowUserShort {
            firstname,
            lastname,
            username,
            image_file,
            ..
        } = props.data.clone();

        html!{
          <div class="box itemBox">
            <article class="media center-media">
              <div class="media-left">
                <figure class="image is-96x96">
                  // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
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
                    // <div class="overflow-title has-text-weight-bold	is-size-4" >{username}</div>
                  </p>
                </div>
              </div>
              <div class="media-right flexBox " >
                {self.open_profile_page(link, false)}
              </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let ShowUserShort {
            firstname,
            lastname,
            username,
            image_file,
            ..
        } = props.data.clone();

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                <img
                    src={image_file.download_url.to_string()} alt="Favicon profile"
                    loading="lazy"
                />
              </div>
              <div class="overflow-title has-text-weight-bold is-size-4" >
                {format!("{} {}", firstname, lastname)}
              </div>
              <span class="overflow-title">{format!("@{}", username)}</span>
              // <div class="overflow-title has-text-weight-bold	is-size-4" >{username}</div>
            </div>
            // <div class="overflow-title has-text-weight-bold	is-size-4" >{username}</div>
            <div class="btnBox">
                {self.open_profile_page(link, false)}
            </div>
          </div>
        }
    }
}
