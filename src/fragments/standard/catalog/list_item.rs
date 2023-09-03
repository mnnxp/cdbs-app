use yew::{agent::Bridged, classes, html, Bridge, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::fragments::{
    list_errors::ListErrors,
    switch_icon::res_btn,
};
use crate::types::ShowStandardShort;
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::standard::{
    AddStandardFav, add_standard_fav,
    DeleteStandardFav, delete_standard_fav,
};

pub enum Msg {
    OpenStandard,
    TriggerFav,
    AddFav,
    DelFav,
    GetFavResult(String),
    ResponseError(Error),
    Ignore,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowStandardShort,
    pub show_list: bool,
}

pub struct ListItemStandard {
    error: Option<Error>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
    is_followed: bool,
}

impl Component for ListItemStandard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let is_followed = props.data.is_followed;
        Self {
            error: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            props,
            is_followed,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::OpenStandard => {
                // Redirect to profile page
                self.router_agent.send(ChangeRoute(AppRoute::ShowStandard(
                    self.props.data.uuid.to_string()
                ).into()));
            },
            Msg::TriggerFav => {
                match &self.is_followed {
                    true => link.send_message(Msg::DelFav),
                    false => link.send_message(Msg::AddFav),
                }
            },
            Msg::AddFav => {
                let standard_uuid = self.props.data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(AddStandardFav::build_query(add_standard_fav::Variables{
                        standard_uuid
                    })).await;
                    link.send_message(Msg::GetFavResult(res.unwrap()));
                });
            },
            Msg::DelFav => {
                let standard_uuid = self.props.data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteStandardFav::build_query(delete_standard_fav::Variables{
                      standard_uuid
                    })).await;
                    link.send_message(Msg::GetFavResult(res.unwrap()));
                });
            },
            Msg::GetFavResult(res) => {
                let target_key = match &self.is_followed {
                    true => "deleteStandardFav",
                    false => "addStandardFav",
                };
                match resp_parsing(res, target_key) {
                    Ok(result) => {
                        debug!("Fav result: {:?}", result);
                        if result {
                            self.is_followed = !self.is_followed;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list || self.props.data.uuid != props.data.uuid {
            self.props.show_list = props.show_list;
            self.is_followed = props.data.is_followed;
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
      html!{<>
        <ListErrors error=self.error.clone()/>
        {match self.props.show_list {
            true => { self.showing_in_list() },
            false => { self.showing_in_box() },
        }}
      </>}
    }
}

impl ListItemStandard {
    fn showing_in_list(&self) -> Html {
        let ShowStandardShort {
            classifier,
            name,
            description,
            specified_tolerance,
            publication_at,
            image_file,
            owner_company,
            standard_status,
            updated_at,
            // is_followed,
            ..
        } = &self.props.data;

        let show_standard_btn = self.link.callback(|_| Msg::OpenStandard);
        let trigger_fab_btn = self.link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec!["fa-bookmark"];
        let mut class_color_btn = "";
        match &self.is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #1872F0;";
            },
            false => class_res_btn.push("far"),
        }

        html!{
          <div class="box itemBox">
              <article class="media center-media">
                  <div class="media-left">
                    <figure class="image is-96x96">
                      <div class="top-tag" >{standard_status.name.to_string()}</div>
                      // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                      <img
                        src={image_file.download_url.clone()} alt="Image standard"
                        loading="lazy"
                      />
                    </figure>
                  </div>
                  <div class="media-content">
                    <div class="columns is-gapless" style="margin-bottom:0">
                      <div class="column">
                          { get_value_field(&142) } // classifier
                          <span class="id-box has-text-grey-light has-text-weight-bold">{classifier}</span>
                      </div>
                      <div class="column">
                          { get_value_field(&144) } // specified tolerance
                          <span class="id-box has-text-grey-light has-text-weight-bold">{specified_tolerance}</span>
                      </div>
                    </div>
                    <div class="columns" style="margin-bottom:0">
                        <div class="column">
                            <div class="has-text-weight-bold is-size-4">{name}</div>
                            <div class="overflow-title">
                                {match &description.len() {
                                    0..=50 => description.clone(),
                                    _ => format!("{:.*}...", 50, description),
                                }}
                            </div>
                            <div class="overflow-title">
                                { get_value_field(&141) } // owner
                                <span class="has-text-weight-bold">
                                    {format!("{} {}",
                                            &owner_company.shortname,
                                            &owner_company.company_type.shortname
                                    )}
                                </span>
                            </div>
                        </div>
                        <div class="column buttons is-one-quarter flexBox" >
                          {res_btn(
                            classes!("fas", "fa-eye"),
                            show_standard_btn,
                            String::new()
                          )}
                          {res_btn(
                            classes!(class_res_btn),
                            trigger_fab_btn,
                            class_color_btn.to_string()
                          )}
                        </div>
                    </div>
                    <div class="columns is-gapless" style="margin-bottom:0">
                        <div class="column">
                          {format!("{}: {:.*}", get_value_field(&155), 10, publication_at.to_string())}
                        </div>
                        <div class="column">
                          {format!("{}: {:.*}", get_value_field(&156), 10, updated_at.to_string())}
                        </div>
                    </div>
                  </div>
              </article>
            </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let ShowStandardShort {
            classifier,
            name,
            // specified_tolerance,
            // publication_at,
            image_file,
            owner_company,
            standard_status,
            // is_followed,
            ..
        } = self.props.data.clone();

        let show_standard_btn = self.link.callback(|_| Msg::OpenStandard);
        let trigger_fab_btn = self.link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec![];
        let mut class_color_btn = "";
        match &self.is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #1872F0;";
            },
            false => class_res_btn.push("far"),
        }
        class_res_btn.push("fa-bookmark");

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <div class="top-tag" >{standard_status.name.to_string()}</div>
                // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                <img
                  src={image_file.download_url.clone()} alt="Image standard"
                  loading="lazy"
                />
              </div>
              <div>
                { get_value_field(&142) } // classifier
                <span class="id-box has-text-grey-light has-text-weight-bold">{classifier}</span>
                <br/>
              </div>
              <div class="has-text-weight-bold is-size-4">{name}</div>
              <div class="overflow-title">
                { get_value_field(&141) } // owner
                  <span class="has-text-weight-bold">
                    {format!("{} {}",
                      &owner_company.shortname,
                      &owner_company.company_type.shortname
                    )}
                  </span>
                </div>
              <div class="btnBox">
                <button class="button is-light is-fullwidth has-text-weight-bold"
                    onclick=show_standard_btn>
                    { get_value_field(&143) } // Show standard
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
