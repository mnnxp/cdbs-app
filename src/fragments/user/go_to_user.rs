use yew::{agent::Bridged, html, Bridge, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};
use crate::routes::AppRoute;
use crate::services::content_adapter::ContentDisplay;
use crate::types::{Pathname, ShowUserShort};

pub enum Msg {
  ShowOwnerUserCard,
  Ignore,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowUserShort,
}

pub struct GoToUser {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for GoToUser {
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
          Msg::ShowOwnerUserCard => self.router_agent.send(ChangeRoute(AppRoute::Profile(self.props.data.username.clone()).into())),
          Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if self.props.data.uuid == props.data.uuid {
        false
      } else {
        self.props = props;
        true
      }
    }

    fn view(&self) -> Html {
      let onclick_open_owner = self.link.callback(|_| Msg::ShowOwnerUserCard);
      html!{
        <a class={"id-box has-text-grey-light has-text-weight-bold"}
            onclick={onclick_open_owner.clone()}
            href={Pathname::User(self.props.data.username.clone()).get_pathname()}
            >
          {self.props.data.to_display()}
        </a>
      }
    }
}