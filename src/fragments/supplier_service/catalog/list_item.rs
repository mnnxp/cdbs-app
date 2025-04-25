use yew::{agent::Bridged, classes, html, Bridge, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
// use log::debug;
use crate::error::Error;
use crate::fragments::switch_icon::res_fullwidth_btn;
use crate::routes::AppRoute;
use crate::fragments::{
    list_errors::ListErrors,
    switch_icon::res_btn,
};
use crate::types::{Pathname, ShowServiceShort};
use crate::services::content_adapter::DateDisplay;
use crate::services::get_value_field;

pub enum Msg {
    OpenService,
    ResponseError(Error),
    ClearError,
    Ignore,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowServiceShort,
    pub show_list: bool,
}

pub struct ListItemService {
    error: Option<Error>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for ListItemService {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::OpenService => {
                // Redirect to profile page
                self.router_agent.send(ChangeRoute(AppRoute::ShowService(
                    self.props.data.uuid.to_string()
                ).into()));
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list || self.props.data.uuid != props.data.uuid {
            self.props.show_list = props.show_list;
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
            {match self.props.show_list {
                true => { self.showing_in_list() },
                false => { self.showing_in_box() },
            }}
        </>}
    }
}

impl ListItemService {
    fn showing_in_list(&self) -> Html {
        let ShowServiceShort {
            name,
            description,
            owner_user,
            owner_company,
            service_status,
            files,
            updated_at,
            ..
        } = &self.props.data;
        let mut class_box = classes!("box", "itemBox");
        class_box.push(service_status.get_class_color());
        let onclick_open_service = self.link.callback(|_| Msg::OpenService);

        html!{
          <div class={class_box}>
              <article class="media center-media">
                  <div class="media-left">
                   <figure class="image is-96x96">
                      <div class="top-tag" >{service_status.name.to_string()}</div>
                      <img src={files.first().map(|x| x.download_url.clone()).unwrap_or_default()} loading="lazy" alt="Image" />
                    </figure>
                  </div>
                  <div class="media-content" onclick={onclick_open_service.clone()}>
                    <div class="columns is-gapless mb-0">
                        <div class="column">
                            {get_value_field(&118)}
                            <span class="id-box has-text-weight-bold">
                                {format!(" @{}", &owner_user.username)}
                            </span>
                        </div>
                        <div class="column">
                            {get_value_field(&158)} // provider's
                            <span class="has-text-weight-bold">
                                {format!(" {}", &owner_company.shortname)}
                            </span>
                        </div>
                        <div class="column">
                          <span class={"icon"} title={get_value_field(&156)}>
                            <i class="fas fa-edit"></i>
                          </span>
                          {updated_at.date_to_display()}
                        </div>
                    </div>
                    <div class="column fix-width mb-0 p-0">
                      <div class="overflow-title has-text-weight-bold is-size-4">{name}</div>
                      <div class="overflow-title">{description.clone()}</div>
                    </div>
                  </div>
                  <div class="column buttons flexBox p-0" >
                    {res_btn(
                      classes!("far", "fa-folder"),
                      onclick_open_service,
                      String::new(),
                      get_value_field(&378),
                      Pathname::Service(self.props.data.uuid.clone())
                    )}
                  </div>
              </article>
            </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let ShowServiceShort {
            name,
            owner_company,
            service_status,
            ..
        } = self.props.data.clone();

        let onclick_open_service = self.link.callback(|_| Msg::OpenService);

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <div class="top-tag" >{service_status.name.to_string()}</div>
                <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              </div>
              <div class="has-text-weight-bold is-size-4">{name}</div>
              <div class="overflow-title">
                {get_value_field(&141)} // owner
                  <span class="has-text-weight-bold">
                    {format!("{} {}",
                      &owner_company.shortname,
                      &owner_company.company_type.shortname
                    )}
                  </span>
                </div>
              <div class="btnBox">
                {res_fullwidth_btn(onclick_open_service, get_value_field(&378), Pathname::Service(self.props.data.uuid.clone()))}
                <div style="margin-left: 8px;">
                </div>
              </div>
            </div>
          </div>
        }
    }
}
