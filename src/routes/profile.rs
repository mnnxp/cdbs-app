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
                    <div class="container page">
                        <div class="row">
                            <div>
                                // <h1 class="title">{ title }</h1>
                                // <fieldset class="field">
                                    // <span class="tag is-info is-light">{
                                    //     match &self.profile {
                                    //         Some(data) => format!("Last updated: {}", data.updated_at),
                                    //         None => "Not data".to_string(),
                                    //     }
                                    // }</span>
                                // </fieldset>
                                <ListErrors error=self.error.clone()/>
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
                                    </div>

                                    <div class="content">
                                        { format!("{}", data.description) }
                                        <br/>
                                        <time datetime={ data.updated_at.to_string() }>{
                                            format!("Updated at: {:.*}", 19, data.updated_at.to_string())
                                        }</time>
                                    </div>

                                    <footer class="card-footer">
                                        <p class="card-footer-item">
                                          <span>
                                            { "View on" } <a href="https://twitter.com/codinghorror/status/506010907021828096">{"Twitter"}</a>
                                          </span>
                                        </p>
                                        <p class="card-footer-item">
                                          <span>
                                            { "Share on" } <a href="#">{"Facebook"}</a>
                                          </span>
                                        </p>
                                    </footer>
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
