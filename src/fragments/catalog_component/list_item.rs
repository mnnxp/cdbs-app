use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
use crate::routes::AppRoute;
use crate::fragments::switch_icon::res_btn;
use crate::types::ShowComponentShort;

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
            Msg::OpenComponent => {
                // Redirect to profile page
                self.router_agent.send(ChangeRoute(AppRoute::ShowComponent(
                    self.props.data.uuid.to_string()
                ).into()));
                // debug!("OpenComponent");
            },
            Msg::TriggerFav => {
                if !self.props.data.is_followed {
                    self.props.add_fav.emit("".to_string());
                } else {
                    self.props.del_fav.emit("".to_string());
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
            // uuid,
            name,
            description,
            owner_user,
            // type_access,
            // component_type,
            actual_status,
            is_followed,
            is_base,
            updated_at,
            licenses,
            // files,
            component_suppliers,
            ..
        } = &self.props.data;

        let onclick_open_component = self.link
            .callback(|_| Msg::OpenComponent);

        let trigger_fab_btn = self.link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec!["fa-bookmark"];
        let mut class_color_btn = "";
        match is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #3298DD;";
            },
            false => class_res_btn.push("far"),
        }

        html! {
          <div class="box itemBox">
            <article class="media center-media">
              <div class="media-left">
                <figure class="image is-96x96">
                  <div hidden={!is_base} class="top-tag" >{"standard"}</div>
                  // {match files.first() {
                  //     Some(f) => html!{<img src={f.download_url.clone()} alt="Image" />},
                  //     None => html!{<img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />},
                  // }}
                  <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                </figure>
              </div>
              <div class="media-content">
                <div class="columns is-gapless" style="margin-bottom:0">
                    <div class="column">
                        {match component_suppliers.first() {
                            Some(x) => html!{<>
                                {"supplier "}
                                <span class="id-box has-text-grey-light has-text-weight-bold">
                                  {x.supplier.shortname.clone()}
                                </span>
                            </>},
                            None => html!{<>
                                {"uploaded from "}
                                <span class="id-box has-text-grey-light has-text-weight-bold">
                                  {format!("@{}",&owner_user.username)}
                                </span>
                            </>},
                        }}
                    </div>
                    <div class="column">
                        {"actual status "}<span class="id-box has-text-grey-light has-text-weight-bold">{
                            actual_status.name.clone()
                        }</span>
                    </div>
                  </div>
                  <div class="columns" style="margin-bottom:0">
                      <div class="column">
                          <div class="overflow-title has-text-weight-bold is-size-4">{name}</div>
                          <div class="overflow-title">
                            {match &description.len() {
                                0..=50 => description.clone(),
                                _ => format!("{:.*}...", 50, description),
                            }}
                          </div>
                      </div>
                      <div class="column is-one-quarter flexBox" >
                          {res_btn(classes!(String::from("fas fa-cloud-download-alt")),
                              onclick_open_component,
                              "".to_string())}
                          {res_btn(
                              classes!(class_res_btn),
                              trigger_fab_btn,
                              class_color_btn.to_string()
                          )}
                      </div>
                  </div>
                  <div class="columns" style="margin-bottom:0">
                      <div class="column">
                        // {format!("Component type: {}", component_type.component_type.to_string())}
                        {match licenses.first() {
                            Some(l) => html!{format!("License: {}", &l.name)},
                            None => html!{},
                        }}
                        // <div class="tags">
                        //   {for licenses.iter().map(|x| html!{
                        //       <span class="tag is-info is-light">{x.name.clone()}</span>
                        //   })}
                        // </div>
                      </div>
                      <div class="column">
                        {format!("Updated at: {:.*}", 10, updated_at.to_string())}
                      </div>
                  </div>
                </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let ShowComponentShort {
            is_base,
            is_followed,
            name,
            ..
        } = self.props.data.clone();
        let onclick_open_component = self.link
            .callback(|_| Msg::OpenComponent);

        let trigger_fab_btn = self.link
            .callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec![];
        let mut class_color_btn = "";
        match is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #3298DD;";
            },
            false => class_res_btn.push("far"),
        }
        class_res_btn.push("fa-bookmark");

        html! {
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <div class="top-tag" hidden={!is_base} >{"standart"}</div>
                <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              </div>
              <div>
                {"manufactured by "}<span class="id-box has-text-grey-light has-text-weight-bold">{"Alphametall"}</span>
              </div>
              <div class="overflow-title has-text-weight-bold	is-size-4" >{name}</div>
                <div class="btnBox">
                  <button class="button is-light is-fullwidth has-text-weight-bold"
                        onclick={onclick_open_component} >
                    {"Open"}
                  </button>
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
