use yew::{agent::Bridged, Callback, classes, html, Bridge, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use crate::fragments::switch_icon::res_fullwidth_btn;
use crate::services::content_adapter::DateDisplay;
use crate::services::get_value_field;
use crate::routes::AppRoute;
use crate::fragments::{
    buttons::ft_follow_btn,
    switch_icon::res_btn,
};
use crate::types::{Pathname, ShowComponentShort};

pub enum Msg {
    OpenComponent,
    TriggerFav,
    Ignore,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowComponentShort,
    pub show_list: bool,
    pub add_fav: Callback<String>,
    pub del_fav : Callback<String>,
}

pub struct ListItem {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenComponent => {
                // Redirect to profile page
                self.router_agent.send(ChangeRoute(AppRoute::ShowComponent(
                    self.props.data.uuid.to_string()
                ).into()));
                // debug!("OpenComponent");
            },
            Msg::TriggerFav => {
                if !self.props.data.is_followed {
                    self.props.add_fav.emit(String::new());
                } else {
                    self.props.del_fav.emit(String::new());
                }
            },
            Msg::Ignore => (),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list || self.props.data.is_followed != props.data.is_followed || self.props.data.uuid != props.data.uuid {
            self.props.show_list = props.show_list;
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
      match self.props.show_list {
        true => self.showing_in_list(),
        false => self.showing_in_box(),
      }
    }
}

impl ListItem {
    fn showing_in_list(&self) -> Html {
        let ShowComponentShort {
            name,
            description,
            updated_at,
            ..
        } = &self.props.data;

        let onclick_open_component = self.link.callback(|_| Msg::OpenComponent);
        let trigger_fav_btn = self.link.callback(|_| Msg::TriggerFav);

        html!{
          <div class="box itemBox componentListItem">
            <article class="media center-media">
              <div class="media-left" onclick={onclick_open_component.clone()}>
                <figure class="image is-96x96">
                  <img src={self.props.data.image_file.download_url.clone()} alt="Image" />
                </figure>
              </div>
              <div class="media-content">
                  <div class="columns is-gapless" style="margin-bottom:0" onclick={onclick_open_component.clone()}>
                    <div class="column">
                      {self.show_owner()}
                    </div>
                    <div class="column is-2">
                      <span class={"icon"} title={get_value_field(&156)}>
                        <i class="fas fa-edit"></i>
                      </span>
                      {updated_at.date_to_display()}
                    </div>
                    <div class="column is-4">
                      <span class="id-box has-text-weight-bold" title={get_value_field(&159)}>
                        {self.props.data.actual_status.name.clone()}
                      </span>
                    </div>
                  </div>
                  <div class="columns" style="margin-bottom:0">
                      <div class="column" onclick={onclick_open_component.clone()}>
                          <div class="overflow-title has-text-weight-bold is-size-4">{name}</div>
                          <div class="overflow-title">
                            {match &description.len() {
                                0..=50 => description.clone(),
                                _ => format!("{:.*}...", 70, description),
                            }}
                          </div>
                      </div>
                      <div class="column buttons is-one-fifth flexBox" >
                          {res_btn(
                            classes!("far", "fa-folder"),
                            onclick_open_component,
                            String::new(),
                            get_value_field(&315),
                            Pathname::Component(self.props.data.uuid.clone())
                          )}
                          {ft_follow_btn(
                            trigger_fav_btn,
                            self.props.data.is_followed,
                            String::new(),
                          )}
                      </div>
                  </div>
                </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let onclick_open_component = self.link.callback(|_| Msg::OpenComponent);
        let trigger_fav_btn = self.link.callback(|_| Msg::TriggerFav);

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              <div onclick={onclick_open_component.clone()}>
                <div class="imgBox">
                  <img src={self.props.data.image_file.download_url.clone()} alt="Image" />
                </div>
                <p>{self.show_owner()}</p>
                <p class="overflow-title has-text-weight-bold is-size-4">
                  {self.props.data.name.clone()}
                </p>
              </div>
              <div class="btnBox">
                {res_fullwidth_btn(onclick_open_component, get_value_field(&161), Pathname::Component(self.props.data.uuid.clone()))}
                <div style="margin-left: 8px;">
                  {ft_follow_btn(
                      trigger_fav_btn,
                      self.props.data.is_followed,
                      String::new(),
                  )}
                </div>
              </div>
            </div>
          </div>
        }
    }

    fn show_owner(&self) -> Html {
        match &self.props.data.component_suppliers.first() {
            Some(x) => html!{<>
                // supplier / manufactured by
                <span class="id-box has-text-weight-bold" title={get_value_field(&158)}>
                  {x.supplier.shortname.clone()}
                </span>
            </>},
            None => html!{<>
                // user uploaded
                <span class="id-box has-text-weight-bold" title={get_value_field(&118)}>
                  {format!("@{}",&self.props.data.owner_user.username)}
                </span>
            </>},
        }
    }
}
