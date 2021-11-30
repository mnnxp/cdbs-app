use chrono::NaiveDateTime;
// use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::{get_error, Error};
use crate::fragments::{
    switch_icon::res_btn,
    list_errors::ListErrors,
    // catalog_component::CatalogComponents,
    file::FilesCard,
    standard_spec::SpecsTags,
    standard_keyword::KeywordsTags,
};
use crate::gqls::make_query;
use crate::services::{
    is_authenticated,
    // get_logged_user
};
use crate::types::{
    UUID, StandardInfo, SlimUser, // ShowUserShort, ShowStandardShort,
    // ShowFileInfo, DownloadFile, Spec, Keyword, Region,
};

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "./graphql/schema.graphql",
//     query_path = "./graphql/standards.graphql",
//     response_derives = "Debug"
// )]
// struct GetStandardDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct GetStandardData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct AddStandardFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct DeleteStandardFav;

/// Standard with relate data
pub struct ShowStandard {
    error: Option<Error>,
    standard: Option<StandardInfo>,
    current_standard_uuid: UUID,
    current_user_owner: bool,
    // task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    show_full_description: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub standard_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    GetStandardData(String),
    ShowDescription,
    OpenStandardOwnerStandard,
    OpenSettingStandard,
    Ignore,
}

impl Component for ShowStandard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ShowStandard {
            error: None,
            standard: None,
            current_standard_uuid: String::new(),
            current_user_owner: false,
            // task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            show_full_description: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // get standard uuid for request standard data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_standard_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/standard/")
            .to_string();
        // get flag changing current standard in route
        let not_matches_standard_uuid = target_standard_uuid != self.current_standard_uuid;
        // debug!("self.current_standard_uuid {:#?}", self.current_standard_uuid);

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_standard_uuid) && is_authenticated() {
            // update current_standard_uuid for checking change standard in route
            self.current_standard_uuid = target_standard_uuid.to_string();

            spawn_local(async move {
                let res = make_query(GetStandardData::build_query(get_standard_data::Variables {
                    standard_uuid: target_standard_uuid,
                })).await.unwrap();

                link.send_message(Msg::GetStandardData(res.clone()));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();

        match msg {
            Msg::Follow => {
                let link = self.link.clone();
                let standard_uuid_string = self.standard.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(AddStandardFav::build_query(add_standard_fav::Variables {
                        standard_uuid: standard_uuid_string,
                    })).await.unwrap();

                    link.send_message(Msg::AddFollow(res.clone()));
                })
            }
            Msg::AddFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("addStandardFav").unwrap().clone())
                                .unwrap();

                        if result {
                            self.subscribers += 1;
                            self.is_followed = true;
                        }
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::UnFollow => {
                let link = self.link.clone();
                let standard_uuid_string = self.standard.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(DeleteStandardFav::build_query(delete_standard_fav::Variables {
                        standard_uuid: standard_uuid_string,
                    }))
                    .await
                    .unwrap();

                    link.send_message(Msg::DelFollow(res.clone()));
                })
            }
            Msg::DelFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("deleteStandardFav").unwrap().clone())
                                .unwrap();

                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::GetStandardData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let standard_data: StandardInfo =
                            serde_json::from_value(res_value.get("standard").unwrap().clone()).unwrap();
                        debug!("Standard data: {:?}", standard_data);

                        self.subscribers = standard_data.subscribers;
                        self.is_followed = standard_data.is_followed;
                        self.current_standard_uuid = standard_data.uuid.clone();
                        if let Some(user) = &self.props.current_user {
                            self.current_user_owner = standard_data.owner_user.uuid == user.uuid;
                        }
                        // description length check for show
                        self.show_full_description = standard_data.description.len() < 250;
                        self.standard = Some(standard_data);
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::ShowDescription => {
                self.show_full_description = !self.show_full_description;
            }
            Msg::OpenStandardOwnerStandard => {
                if let Some(standard_data) = &self.standard {
                    // Redirect to owner standard page
                    self.router_agent.send(ChangeRoute(AppRoute::Profile(
                        standard_data.owner_user.username.to_string()
                    ).into()));
                }
            }
            Msg::OpenSettingStandard => {
                // if let Some(standard_data) = &self.standard {
                //     // Redirect to owner standard page
                //     self.router_agent.send(ChangeRoute(AppRoute::StandardSettings(
                //         standard_data.uuid.to_string()
                //     ).into()));
                // }
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        match &self.standard {
            Some(standard_data) => html! {
                <div class="standard-page">
                    <ListErrors error=self.error.clone()/>
                    <div class="container page">
                        <div class="row">
                            <div class="card">
                              {self.show_main_card(standard_data)}
                            </div>
                            <div class="columns">
                              {self.show_standard_params(standard_data)}
                              {self.show_standard_files(standard_data)}
                            </div>
                            {self.show_standard_specs(standard_data)}
                            <br/>
                            {self.show_standard_keywords(standard_data)}
                        </div>
                    </div>
                </div>
            },
            None => html! {<div>
                <ListErrors error=self.error.clone()/>
                // <h1>{"Not data"}</h1>
            </div>},
        }
    }
}

impl ShowStandard {
    fn show_main_card(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        let show_description_btn = self.link
            .callback(|_| Msg::ShowDescription);

        html!{
            <div class="columns">
              <div class="column is-one-quarter">
                <img class="imgBox" src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              </div>
              <div class="column">
                {"classifier "} <span class="id-box has-text-grey-light has-text-weight-bold">{
                    standard_data.classifier.clone()
                }</span>
                // <h1>{"Standard"}</h1>
                <div class="media">
                    <div class="is-size-4">{
                        standard_data.name.clone()
                    }</div>
                    <div class="buttons flexBox">
                        {self.show_setting_btn()}
                        {self.show_followers_btn()}
                        {self.show_share_btn()}
                    </div>
                </div>
                <div class="standard-description">{
                    match self.show_full_description {
                        true => html!{<>
                          {standard_data.description.clone()}
                          {match standard_data.description.len() {
                              250.. => html!{<>
                                <br/>
                                <button class="button is-white"
                                    onclick=show_description_btn
                                  >{"See less"}</button>
                              </>},
                              _ => html!{},
                          }}
                        </>},
                        false => html!{<>
                          {format!("{:.*}", 200, standard_data.description)}
                          <br/>
                          <button class="button is-white"
                              onclick=show_description_btn
                            >{"See more"}</button>
                        </>},
                    }
                }</div>
                // <span class="id-box has-text-grey-light has-text-weight-bold">{"design by "} </span>
                // {format!("{} {}",
                //   &standard_data.owner_standard.shortname,
                //   &standard_data.owner_standard.standard_type.shortname
                // )}
              </div>
            </div>
        }
    }

    fn show_standard_params(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2>{"Сharacteristics"}</h2>
              <div class="card">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{"classifier"}</td>
                        <td>{standard_data.classifier.clone()}</td>
                      </tr>
                      <tr>
                        <td>{"specified_tolerance"}</td>
                        <td>{standard_data.specified_tolerance.clone()}</td>
                      </tr>
                      <tr>
                        <td>{"technical_committee"}</td>
                        <td>{standard_data.technical_committee.clone()}</td>
                      </tr>
                      <tr>
                        <td>{"publication_at"}</td>
                        <td>{format!("{:.*}", 10, standard_data.publication_at.to_string())}</td>
                      </tr>
                      <tr>
                        <td>{"standard_status"}</td>
                        <td>{standard_data.standard_status.name.clone()}</td>
                      </tr>
                      <tr>
                        <td>{"region"}</td>
                        <td>{standard_data.region.region.clone()}</td>
                      </tr>
                    </tbody>
                  </table>
              </div>
            </div>
        }
    }

    fn show_standard_files(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2>{"Files"}</h2>
              <FilesCard
                  show_delete_btn = false
                  files = standard_data.standard_files.clone()
                />
            </div>
        }
    }

    fn show_standard_specs(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{<>
              <h2>{"Specs"}</h2>
              <div class="card">
                <SpecsTags
                    show_delete_btn = false
                    standard_uuid = standard_data.uuid.clone()
                    specs = standard_data.standard_specs.clone()
                  />
              </div>
        </>}
    }

    fn show_standard_keywords(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{<>
              <h2>{"Keywords"}</h2>
              <div class="card">
                <KeywordsTags
                    show_delete_btn = false
                    standard_uuid = standard_data.uuid.clone()
                    keywords = standard_data.standard_keywords.clone()
                  />
              </div>
        </>}
    }

    fn show_followers_btn(&self) -> Html {
        let class_fav = match self.is_followed {
            true => "fas fa-bookmark",
            false => "far fa-bookmark",
        };

        let onclick_following = match self.is_followed {
            true => self.link.callback(|_| Msg::UnFollow),
            false => self.link.callback(|_| Msg::Follow),
        };

        html! {<>
            <div class="media-right flexBox" >
              <button
                  id="following-button"
                  class="button"
                  onclick=onclick_following >
                <span class="icon is-small">
                  <i class={class_fav}></i>
                </span>
              </button>
            </div>
            { format!(" {}", &self.subscribers) }
        </>}
    }

    fn show_share_btn(&self) -> Html {
        html! {
            <div class="media-right flexBox" >
              <button
                  id="share-button"
                  class="button" >
                <span class="icon is-small">
                  <i class="far fa-share"></i>
                </span>
              </button>
            </div>
        }
    }

    fn show_setting_btn(&self) -> Html {
        let onclick_setting_standard_btn = self.link
            .callback(|_| Msg::OpenSettingStandard);

        match &self.current_user_owner {
            true => {res_btn(classes!(
                String::from("fa fa-cog")),
                onclick_setting_standard_btn,
                String::new())},
            false => html!{},
        }
    }
}
