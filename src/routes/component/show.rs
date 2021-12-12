use chrono::NaiveDateTime;
// use web_sys::MouseEvent;
// use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::{
    service::RouteService,
    // agent::RouteRequest::ChangeRoute,
    // prelude::*,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

// use crate::routes::AppRoute;
use crate::error::{get_error, Error};
use crate::fragments::{
    // switch_icon::res_btn,
    list_errors::ListErrors,
    // catalog_standard::CatalogStandards,
    component_file::FilesCard,
    component_modification::ModificationsTable,
    component_spec::SpecsTags,
    component_keyword::KeywordsTags,
};
use crate::gqls::make_query;
use crate::services::{
    is_authenticated,
    // get_logged_user
};
use crate::types::{
    UUID, ComponentInfo, SlimUser, DownloadFile,
    // ComponentsQueryArg,
};

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "./graphql/schema.graphql",
//     query_path = "./graphql/components.graphql",
//     response_derives = "Debug"
// )]
// struct GetComponentDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetComponentData;

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "./graphql/schema.graphql",
//     query_path = "./graphql/components.graphql",
//     response_derives = "Debug"
// )]
// struct ComponentFiles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct AddComponentFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct DeleteComponentFav;

/// Component with relate data
pub struct ShowComponent {
    error: Option<Error>,
    component: Option<ComponentInfo>,
    current_component_uuid: UUID,
    current_user_owner: bool,
    // task: Option<FetchTask>,
    // router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    show_full_description: bool,
    show_related_standards: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub component_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    RequestDownloadFiles,
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    ResponseError(Error),
    GetDownloadFilesResult(String),
    GetComponentData(String),
    ShowDescription,
    ShowStandardsList,
    OpenComponentOwner,
    OpenComponentSetting,
    Ignore,
}

impl Component for ShowComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ShowComponent {
            error: None,
            component: None,
            current_component_uuid: String::new(),
            current_user_owner: false,
            // task: None,
            // router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            show_full_description: false,
            show_related_standards: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // get component uuid for request component data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_component_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/component/")
            .to_string();
        // get flag changing current component in route
        let not_matches_component_uuid = target_component_uuid != self.current_component_uuid;
        // debug!("self.current_component_uuid {:#?}", self.current_component_uuid);

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_component_uuid) && is_authenticated() {
            // update current_component_uuid for checking change component in route
            self.current_component_uuid = target_component_uuid.to_string();

            spawn_local(async move {
                let res = make_query(GetComponentData::build_query(get_component_data::Variables {
                    component_uuid: target_component_uuid,
                })).await.unwrap();

                link.send_message(Msg::GetComponentData(res.clone()));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFiles => {
                // let component_uuid = self.props.component_uuid.clone();
                // spawn_local(async move {
                //     let ipt_component_files_arg = component_files::IptComponentFilesArg{
                //         filesUuids: None,
                //         componentUuid: component_uuid,
                //     };
                //     let res = make_query(ComponentFiles::build_query(
                //         component_files::Variables {
                //             ipt_component_files_arg,
                //         }
                //     )).await;
                //     link.send_message(Msg::GetDownloadFilesResult(res.unwrap()));
                // })
            }
            Msg::Follow => {
                let component_uuid_string = self.component.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(AddComponentFav::build_query(add_component_fav::Variables {
                        component_uuid: component_uuid_string,
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
                            serde_json::from_value(res_value.get("addComponentFav").unwrap().clone())
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
                let component_uuid = self.component.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(DeleteComponentFav::build_query(delete_component_fav::Variables {
                        component_uuid,
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
                            serde_json::from_value(res_value.get("deleteComponentFav").unwrap().clone())
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
            Msg::ResponseError(err) => {
                self.error = Some(err);
            }
            Msg::GetDownloadFilesResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: Vec<DownloadFile> = serde_json::from_value(res.get("componentFiles").unwrap().clone()).unwrap();
                        debug!("componentFiles: {:?}", result);
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    },
                }
            }
            Msg::GetComponentData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let component_data: ComponentInfo =
                            serde_json::from_value(res_value.get("component").unwrap().clone()).unwrap();
                        debug!("Component data: {:?}", component_data);

                        self.subscribers = component_data.subscribers;
                        self.is_followed = component_data.is_followed;
                        self.current_component_uuid = component_data.uuid.clone();
                        if let Some(user) = &self.props.current_user {
                            self.current_user_owner = component_data.owner_user.uuid == user.uuid;
                        }
                        // description length check for show
                        self.show_full_description = component_data.description.len() < 250;
                        self.component = Some(component_data);
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::ShowDescription => {
                self.show_full_description = !self.show_full_description;
            }
            Msg::ShowStandardsList => {
                self.show_related_standards = !self.show_related_standards;
            }
            Msg::OpenComponentOwner => {
                // if let Some(component_data) = &self.component {
                //     // Redirect to owner component page
                //     self.router_agent.send(ChangeRoute(AppRoute::ShowCompany(
                //         component_data.owner_company.uuid.to_string()
                //     ).into()));
                // }
            }
            Msg::OpenComponentSetting => {
                // if let Some(component_data) = &self.component {
                //     // Redirect to page for change and update component
                //     self.router_agent.send(ChangeRoute(AppRoute::ComponentSettings(
                //         component_data.uuid.clone()
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
        match &self.component {
            Some(component_data) => html! {
                <div class="component-page">
                    <ListErrors error=self.error.clone()/>
                    <div class="container page">
                        <div class="row">
                            <div class="card">
                              {self.show_main_card(component_data)}
                            </div>
                            <br/>
                            {self.show_modifications_table(component_data)}
                            <br/>
                            <div class="columns">
                              {self.show_component_params(component_data)}
                              {self.show_component_files(component_data)}
                            </div>
                            {self.show_component_specs(component_data)}
                            <br/>
                            {self.show_component_keywords(component_data)}
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

impl ShowComponent {
    fn show_main_card(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        let onclick_open_owner_company = self.link
            .callback(|_| Msg::OpenComponentOwner);

        let show_description_btn = self.link
            .callback(|_| Msg::ShowDescription);

        html!{
            <div class="columns">
              <div class="column is-one-quarter">
                <img class="imgBox" src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              </div>
              <div class="column">
                <div class="media">
                    <div class="media-content">
                        {"uploaded from "}
                        <a class="id-box has-text-grey-light has-text-weight-bold"
                            onclick={onclick_open_owner_company} >
                          {format!("@{}",&component_data.owner_user.username)}
                        </a>
                    </div>
                    <div class="media-right" style="margin-right: 1rem">
                        {"type access "}<span class="id-box has-text-grey-light has-text-weight-bold">{
                            component_data.type_access.name.clone()
                        }</span>
                    </div>
                </div>
                // <h1>{"Component"}</h1>
                <div class="has-text-weight-bold is-size-4">{
                    component_data.name.clone()
                }</div>
                <div class="buttons flexBox">
                    // {self.show_related_standards_btn()}
                    // {self.show_download_btn()}
                    // {self.show_setting_btn()}
                    // {self.show_followers_btn()}
                    // {self.show_share_btn()}
                </div>
                <div class="component-description">{
                    match self.show_full_description {
                        true => html!{<>
                          {component_data.description.clone()}
                          {match component_data.description.len() {
                              250.. => html!{<>
                                <br/>
                                <button class="button is-white"
                                    onclick=show_description_btn >
                                  {"See less"}
                                </button>
                              </>},
                              _ => html!{},
                          }}
                        </>},
                        false => html!{<>
                          {format!("{:.*}", 200, component_data.description)}
                          <br/>
                          <button class="button is-white"
                              onclick=show_description_btn >
                            {"See more"}
                          </button>
                        </>},
                    }
                }</div>
              </div>
            </div>
        }
    }

    fn show_modifications_table(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<>
            <h2>{"Modification"}</h2>
            <ModificationsTable
                show_manage_btn = false
                modifications = component_data.component_modifications.clone()
                delete_modification = None
              />
        </>}
    }

    fn show_component_params(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2>{"Characteristic"}</h2>
              <div class="card">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{"actual_status"}</td>
                        <td>{component_data.actual_status.name.clone()}</td>
                      </tr>
                      <tr>
                        <td>{"component_type"}</td>
                        <td>{component_data.component_type.component_type.clone()}</td>
                      </tr>
                      <tr>
                        <td>{"technical_committee"}</td>
                        <td>{for component_data.licenses.iter().map(|x| html!{<td>{x.name.clone()}</td>})}</td>
                      </tr>
                      <tr>
                        <td>{"updated_at"}</td>
                        <td>{format!("{:.*}", 10, component_data.updated_at.to_string())}</td>
                      </tr>
                    </tbody>
                  </table>
              </div>
            </div>
        }
    }

    fn show_component_files(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2>{"Files"}</h2>
              <FilesCard
                  show_download_btn = true
                  show_delete_btn = false
                  component_uuid = component_data.uuid.clone()
                  files = component_data.files.clone()
                />
            </div>
        }
    }

    fn show_component_specs(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<>
              <h2>{"Specs"}</h2>
              <div class="card">
                <SpecsTags
                    show_manage_btn = false
                    component_uuid = component_data.uuid.clone()
                    specs = component_data.component_specs.clone()
                  />
              </div>
        </>}
    }

    fn show_component_keywords(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<>
              <h2>{"Keywords"}</h2>
              <div class="card">
                <KeywordsTags
                    show_delete_btn = false
                    component_uuid = component_data.uuid.clone()
                    keywords = component_data.component_keywords.clone()
                  />
              </div>
        </>}
    }
}
