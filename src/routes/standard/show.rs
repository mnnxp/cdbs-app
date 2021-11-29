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
    // switch_icon::res_btn,
    list_errors::ListErrors,
    // catalog_component::CatalogComponents,
    // spec::SpecsTags,
};
use crate::gqls::make_query;
use crate::services::{
    is_authenticated,
    // get_logged_user
};
use crate::types::{
    UUID, StandardInfo, SlimUser, // ShowUserShort, ShowCompanyShort,
    // ShowFileForDownload, DownloadFile, Spec, Keyword, Region,
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
    OpenCompanyOwnerStandard,
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
            Msg::OpenCompanyOwnerStandard => {
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
        let show_description_btn = self.link
            .callback(|_| Msg::ShowDescription);

        match &self.standard {
            Some(standard_data) => html! {
                <div class="standard-page">
                    <ListErrors error=self.error.clone()/>
                    <div class="container page">
                        <div class="row">
                            <div class="card">
                              <div class="columns">
                                <div class="column is-one-quarter">
                                  <img class="imgBox" src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                                </div>
                                <div class="column">
                                  {"classifier "} <span class="id-box has-text-grey-light has-text-weight-bold">{
                                      standard_data.classifier.to_string()
                                  }</span>
                                  // <h1>{"Standard"}</h1>
                                  <div class="has-text-weight-bold is-size-4">{
                                      standard_data.name.clone()
                                  }</div>
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
                                  //   &standard_data.owner_company.shortname,
                                  //   &standard_data.owner_company.company_type.shortname
                                  // )}
                                </div>
                              </div>
                            </div>
                            <div class="columns">
                              <div class="column">
                                <div class="card">
                                  <h2>{"Data"}</h2>
                                </div>
                              </div>
                              <div class="column">
                                <div class="card">
                                  <h2>{"Files"}</h2>
                                </div>
                              </div>
                            </div>
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

impl ShowStandard {}
