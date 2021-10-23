// use yew::services::fetch::FetchTask;
use yew::{
    html, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
// use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use chrono::NaiveDateTime;

use yew::services::ConsoleService;

use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use crate::gqls::make_query;

use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
// use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_token};
use crate::types::{
    UUID, UserInfo, SlimUser, Program, Region
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetSelfDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetSelfData;

/// Profile user with relate data
pub struct Profile {
    error: Option<Error>,
    profile: Option<UserInfo>,
    // task: Option<FetchTask>,
    // router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    programs: Vec<Program>,
    regions: Vec<Region>,
}

#[derive(Properties, Clone)]
pub struct Props {
    // pub callback: Callback<()>,
    pub username: String,
    pub current_user: Option<SlimUser>,
    // pub tab: ProfileTab,
}

#[derive(Clone)]
pub enum Msg {
    // Follow,
    // UnFollow,
    GetData(String),
    UpdateList(String),
    Ignore,
    Logout,
}

#[derive(Clone, PartialEq)]
pub enum ProfileTab {
    // ByCompany,
    // ByCoponent,
    // ByStandard,
    // ByAuthor,
    FavoritedBy,
}

impl Component for Profile {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Profile {
            error: None,
            profile: None,
            // task: None,
            // router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            programs: Vec::new(),
            regions: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render && is_authenticated() {
            spawn_local(async move {
                let res = make_query(
                    GetSelfDataOpt::build_query(get_self_data_opt::Variables)
                ).await.unwrap();
                link.send_message(Msg::GetData(res.clone()));
                link.send_message(Msg::UpdateList(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();

        match msg {
            Msg::GetData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let user_data: UserInfo = serde_json::from_value(res.get("selfData").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("User data: {:?}", user_data).as_ref());
                        self.profile = Some(user_data);
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
            },
            Msg::Ignore => {}
            Msg::Logout => {
                // Clear global token after logged out
                set_token(None);
                // Notify app to clear current user info
                // self.props.callback.emit(());
                // Redirect to home page
                // self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            },
            Msg::UpdateList(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();
                self.regions =
                    serde_json::from_value(res.get("regions").unwrap().clone()).unwrap();
                self.programs =
                    serde_json::from_value(res.get("programs").unwrap().clone()).unwrap();
                ConsoleService::info(format!("Update: {:?}", self.programs).as_ref());
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // let onsubmit = self.link.callback(|ev: FocusEvent| {
        //     ev.prevent_default();
        //     Msg::Request
        // });
        // let title = match &self.profile {
        //     Some(data) => format!("Profile {}", data.username),
        //     None => "Not data".to_string(),
        // };

        match &self.profile {
            Some(data) => html! {
                <div class="profile-page">
                    <ListErrors error=self.error.clone()/>
                    <div class="container page">
                        <div class="row">
                            <div class="columns">
                                <div class="column is-one-quarter">
                                    <aside class="menu">
                                        <p class="menu-label">
                                            {"Profile"}
                                        </p>
                                        <ul class="menu-list">
                                            <li><a>{"Components"}</a></li>
                                            <li><a>{"Standards"}</a></li>
                                            <li><a>{"Companies"}</a></li>
                                            // <li><a>{"Certificates"}</a></li>
                                            // <li><a>{"Balance"}</a></li>
                                            // <li>
                                            //     <a class="is-active">{"Metric"}</a>
                                            //     <ul>
                                            //         <li><a>{ format!("Subscribers: {}", data.subscribers.to_string()) }</a></li>
                                            //         <li><a>{ format!("Companies: {}", data.companies_count.to_string()) }</a></li>
                                            //         <li><a>{ format!("Components: {}", data.components_count.to_string()) }</a></li>
                                            //         <li><a>{ format!("Favorite companies: {}", data.fav_companies_count.to_string()) }</a></li>
                                            //         <li><a>{ format!("Favorite components: {}", data.fav_components_count.to_string()) }</a></li>
                                            //         <li><a>{ format!("Favorite standards: {}", data.fav_standards_count.to_string()) }</a></li>
                                            //         <li><a>{ format!("Favorite users: {}", data.fav_users_count.to_string()) }</a></li>
                                            //     </ul>
                                            // </li>
                                        </ul>
                                    </aside>
                                </div>
                                <div class="column">
                                    // <h1 class="title">{ title }</h1>
                                    // <fieldset class="field">
                                        // <span class="tag is-info is-light">{
                                        //     match &self.profile {
                                        //         Some(data) => format!("Last updated: {}", data.updated_at),
                                        //         None => "Not data".to_string(),
                                        //     }
                                        // }</span>
                                    // </fieldset>
                                    <div class="card">
                                      // <div class="card-image">
                                      //   <figure class="image is-4by3">
                                      //     <img src="https://bulma.io/images/placeholders/1280x960.png" alt="Placeholder image"/>
                                      //   </figure>
                                      // </div>
                                      <div class="card-content">
                                        <div class="media">
                                          <div class="media-left">
                                            <figure class="image is-48x48">
                                              <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                                            </figure>
                                          </div>
                                          <div class="media-content">
                                            <p class="title is-4">{
                                                format!("{} {}", data.firstname, data.lastname)
                                            }</p>
                                            <p class="subtitle is-6">{
                                                format!("@{}", data.username)
                                            }</p>
                                          </div>
                                          <div class="media-right">
                                            <p class="subtitle is-6 left">
                                                { format!("{:.*}", 19, data.updated_at.to_string()) }
                                                <br/>
                                                { format!("followers: {}", 0) } // todo!(make show followers of data.followers.to_string())
                                            </p>
                                          </div>
                                        </div>

                                        <div class="content">
                                            { format!("{}", data.description) }
                                        </div>

                                        <div class="content">
                                            // { format!("{}", data.description) }
                                            { format!("Position: {}", data.position.to_string()) }
                                            <br/>
                                            { format!("Region: {}", data.region.region.to_string()) }
                                            <br/>
                                            { format!("Working software: {}", data.program.name.to_string()) }
                                            // <br/>
                                            // { format!("{:#?}", data.certificates) }
                                        </div>

                                        // <footer class="card-footer">
                                        //     <p class="card-footer-item">
                                        //       <span>
                                        //         { format!("Position: {}", data.position.to_string()) }
                                        //       </span>
                                        //     </p>
                                        //     <p class="card-footer-item">
                                        //       <span>
                                        //         { format!("Region: {}", data.region.region.to_string()) }
                                        //       </span>
                                        //     </p>
                                        //     <p class="card-footer-item">
                                        //       <span>
                                        //         { format!("Working software: {}", data.program.name.to_string()) }
                                        //       </span>
                                        //     </p>
                                        // </footer>
                                      </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            },
            None => html! {<div>
                <ListErrors error=self.error.clone()/>

                <h1>{"Not data"}</h1>
            </div>},
        }
    }
}
