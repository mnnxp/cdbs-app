use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
use crate::routes::AppRoute;
use crate::fragments::switch_icon::res_btn;
use crate::types::ShowCompanyShort;

pub enum Msg {
    OpenCompany,
    TriggerFav,
    Ignore,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowCompanyShort,
    pub show_list: bool,
    pub add_fav: Callback<String>,
    pub del_fav : Callback<String>,
}

pub struct ListItem {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props
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
            Msg::OpenCompany => {
                // Redirect to profile page
                self.router_agent.send(ChangeRoute(AppRoute::ShowCompany(
                    self.props.data.uuid.to_string()
                ).into()));
            },
            Msg::TriggerFav => {
                if !self.props.data.is_followed {
                    self.props.add_fav.emit("".to_string());
                } else {
                    self.props.del_fav.emit("".to_string());
                }
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This company has no properties so we will always return "false".
        if self.props.show_list != props.show_list || self.props.data.is_followed != props.data.is_followed {
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
        let ShowCompanyShort {
            shortname,
            inn,
            description,
            // image_file,
            region,
            company_type,
            is_supplier,
            is_followed,
            updated_at,
            ..
        } = &self.props.data;

        let show_company_btn = self.link.callback(|_| Msg::OpenCompany);
        let trigger_fab_btn = self.link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec!["fa-bookmark"];
        let mut class_color_btn = "";
        match is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #3298DD;";
            },
            false => {
                class_res_btn.push("far");
            },
        }

        html! {
          <div class="box itemBox">
            <article class="media center-media">
              <div class="media-left">
                <figure class="image is-96x96">
                  <div hidden={!is_supplier} class="top-tag" >{"supplier"}</div>
                  <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                  // <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
                </figure>
              </div>
              <div class="media-content" style="min-width: 0px;">
                <div class="content">
                  <p>
                    <div style="margin-bottom:0" >
                      {"from "} <span class="id-box has-text-grey-light has-text-weight-bold">{region.region.to_string()}</span>
                    </div>
                    <div class="overflow-title has-text-weight-bold is-size-4">{shortname}</div>
                    <div class="overflow-title has-text-weight-bold">{description}</div>
                  </p>
                </div>
              </div>
              <div class="media-right overflow-title">
                  {format!("Updated at: {:.*}", 19, updated_at.to_string())}
                  <br/>
                  {format!("Type: {}", company_type.name.to_string())}
                  <br/>
                  {format!("Reg.№: {}", inn.to_string())}
              </div>
              <div class="media-right flexBox " >
                {res_btn(classes!(
                    String::from("fas fa-building")),
                    show_company_btn,
                    String::new())}
                {res_btn(
                    classes!(class_res_btn),
                    trigger_fab_btn,
                    class_color_btn.to_string()
                )}
              </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let ShowCompanyShort {
            shortname,
            // image_file,
            region,
            company_type,
            is_supplier,
            is_followed,
            ..
        } = self.props.data.clone();

        let show_company_btn = self.link.callback(|_| Msg::OpenCompany);
        let trigger_fab_btn = self.link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec![];
        let mut class_color_btn = "";
        match is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #3298DD;";
            },
            false => {
                class_res_btn.push("far");
            },
        }
        class_res_btn.push("fa-bookmark");

        html! {
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <div class="top-tag" hidden={!is_supplier} >{"supplier"}</div>
                <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                // <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
              </div>
              <div>
                {"from "}<span class="id-box has-text-grey-light has-text-weight-bold">{region.region.to_string()}</span>
              </div>
              <div class="overflow-title has-text-weight-bold is-size-4">{shortname}</div>
              <div class="overflow-title has-text-weight-bold">{company_type.shortname.to_string()}</div>
              <div class="btnBox">
                <button class="button is-light is-fullwidth has-text-weight-bold"
                    onclick=show_company_btn
                    >{"Show company"}</button>
                <div style="margin-left: 8px;">
                {res_btn(
                    classes!(class_res_btn),
                    trigger_fab_btn,
                    class_color_btn.to_string()
                )}
                </div>
              </div>
            </div>
          </div>
        }
    }
}
