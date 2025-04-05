use yew::{agent::Bridged, classes, html, Bridge, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::fragments::switch_icon::res_fullwidth_btn;
use crate::routes::AppRoute;
use crate::fragments::{
    buttons::ft_follow_btn,
    list_errors::ListErrors,
    switch_icon::res_btn,
};
use crate::types::{Pathname, ShowStandardShort};
use crate::services::content_adapter::DateDisplay;
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
    ClearError,
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
            Msg::ClearError => self.error = None,
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

impl ListItemStandard {
    fn showing_in_list(&self) -> Html {
        let ShowStandardShort {
            name,
            description,
            publication_at,
            image_file,
            owner_company,
            ..
        } = &self.props.data;

        let show_standard_btn = self.link.callback(|_| Msg::OpenStandard);
        let trigger_fav_btn = self.link.callback(|_| Msg::TriggerFav);

        html!{
          <div class="box itemBox">
              <article class="media center-media">
                  <div class="media-left">
                    <figure class="image is-96x96">
                      // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                      <img
                        src={image_file.download_url.clone()} alt="Image standard"
                        loading="lazy"
                      />
                    </figure>
                  </div>
                  <div class="media-content">
                    <div class="columns is-gapless mb-0" onclick={show_standard_btn.clone()}>
                        <div class="column">
                            {get_value_field(&141)} // owner
                            <span class="has-text-weight-bold">{owner_company.shortname.clone()}</span>
                        </div>
                        <div class="column">
                            {format!("{}: ", get_value_field(&155))}
                            <span class="id-box has-text-weight-bold">{publication_at.date_to_display()}</span>
                        </div>
                    </div>
                    <div class="columns mb-0">
                        <div class="column fix-width" onclick={show_standard_btn.clone()}>
                            <div class="has-text-weight-bold is-size-4">{name}</div>
                            <div class="overflow-title">{description.clone()}</div>
                        </div>
                    </div>
                    </div>
                    <div class="column buttons flexBox p-0" >
                        {res_btn(
                            classes!("far", "fa-folder"),
                            show_standard_btn,
                            String::new(),
                            get_value_field(&315),
                            Pathname::Standard(self.props.data.uuid.clone())
                        )}
                        {ft_follow_btn(
                            trigger_fav_btn,
                            self.is_followed,
                            String::new(),
                        )}
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
        let trigger_fav_btn = self.link.callback(|_| Msg::TriggerFav);

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
                {get_value_field(&142)} // classifier
                <span class="id-box has-text-weight-bold">{classifier}</span>
                <br/>
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
                {res_fullwidth_btn(show_standard_btn, get_value_field(&143), Pathname::Standard(self.props.data.uuid.clone()))}
                <div style="margin-left: 8px;">
                {ft_follow_btn(
                    trigger_fav_btn,
                    self.is_followed,
                    String::new(),
                )}
                </div>
              </div>
            </div>
          </div>
        }
    }
}
