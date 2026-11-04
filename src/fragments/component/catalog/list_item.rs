use yew::{agent::Bridged, Callback, classes, html, Bridge, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use crate::fragments::switch_icon::res_fullwidth_btn;
use crate::services::content_adapter::{DateDisplay, Markdownable};
use crate::services::LocaleKey;
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
                if self.props.data.is_followed {
                    self.props.del_fav.emit(self.props.data.uuid.clone());
                } else {
                    self.props.add_fav.emit(self.props.data.uuid.clone());
                }
            },
            Msg::Ignore => (),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list == props.show_list && self.props.data == props.data {
            false
        } else {
            self.props = props;
            true
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
            type_access,
            created_at,
            updated_at,
            ..
        } = &self.props.data;

        let onclick_open_component = self.link.callback(|_| Msg::OpenComponent);
        let trigger_fav_btn = self.link.callback(|_| Msg::TriggerFav);
        let class_icon = "icon is-small has-text-grey mr-1";

        html!{
          <div class="box itemBox componentListItem">
            <article class="media center-media">
              <div class="media-left" onclick={onclick_open_component.clone()}>
                <figure class="image is-96x96">
                  <img src={self.props.data.image_file.download_url.clone()} alt="Image" />
                </figure>
              </div>
              <div class="media-content" onclick={onclick_open_component.clone()}>
                  <div class="columns is-gapless mb-0">
                    <div class="column">{self.show_owner()}</div>
                    <div class="column" title={LocaleKey::LifeCycleStage.get_value()}>
                        <span class="has-text-dark">{self.props.data.actual_status.name.clone()}</span>
                    </div>
                    <div class="column" title={LocaleKey::CreatedAtLabel.get_value()}>
                        <span class={class_icon}>
                            <i class="far fa-calendar-plus"></i>
                        </span>
                        <span class="has-text-dark">{created_at.date_to_display()}</span>
                    </div>
                    <div class="column" title={LocaleKey::UpdatedAt.get_value()}>
                      <span class={classes!(class_icon, "pl-1")}>
                        <i class="fas fa-edit"></i>
                      </span>
                      <span class="has-text-dark">{updated_at.date_to_display()}</span>
                    </div>
                    <div class="column" title={LocaleKey::TypeAccess.get_value()}>
                      {type_access.get_with_icon()}
                    </div>
                  </div>
                  <div class="column fix-width mb-0 p-0">
                    <div class="overflow-title has-text-weight-bold is-size-4">{name}</div>
                    <div class="overflow-title">{description.to_markdown_short()}</div>
                  </div>
                </div>
                <div class="buttons flexBox p-0" >
                    {res_btn(
                      classes!("far", "fa-folder"),
                      onclick_open_component,
                      String::new(),
                      LocaleKey::Show.get_value(),
                      Pathname::Component(self.props.data.uuid.clone())
                    )}
                    {ft_follow_btn(
                      trigger_fav_btn,
                      self.props.data.is_followed,
                      String::new(),
                    )}
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
                {res_fullwidth_btn(onclick_open_component, LocaleKey::Open.get_value(), Pathname::Component(self.props.data.uuid.clone()))}
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
        let class_icon = "icon is-small has-text-grey mr-1";
        let class_value = "has-text-dark has-text-weight-bold";
        match &self.props.data.component_suppliers.first() {
            Some(x) => html!{
              <span title={LocaleKey::SupplierLabel.get_value()}>
                // supplier / manufactured by
                <span class={class_icon}><i class="fas fa-building"></i></span>
                <span class={class_value}>
                    {x.supplier.shortname.clone()}
                </span>
              </span>
            },
            None => html!{
              <span title={LocaleKey::CreatedByUser.get_value()}>
                // user uploaded
                <span class={class_value}>
                    {format!("@{}", &self.props.data.owner_user.username)}
                </span>
              </span>
            },
        }
    }
}
