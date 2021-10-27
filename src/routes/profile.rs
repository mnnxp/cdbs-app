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
use crate::fragments::{list_errors::ListErrors, certificate::CertificateCard};
// use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_token};
use crate::types::{
    UUID, SelfUserInfo, UserInfo, SlimUser, Program, Region, Certificate
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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetUserDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetUserData;

/// Profile user with relate data
pub struct Profile {
    error: Option<Error>,
    self_profile: Option<SelfUserInfo>,
    profile: Option<UserInfo>,
    // current_profile: String,
    // task: Option<FetchTask>,
    // router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    programs: Vec<Program>,
    regions: Vec<Region>,
}

#[derive(Properties, Clone)]
pub struct Props {
    // pub current_route: Option<AppRoute>,
    pub username: String,
    pub current_user: Option<SlimUser>,
    // pub tab: ProfileTab,
}

#[derive(Clone)]
pub enum Msg {
    // Follow,
    // UnFollow,
    GetSelfData(String),
    GetUserData(String),
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
            self_profile: None,
            profile: None,
            // current_profile: String::new(),
            // task: None,
            // router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            programs: Vec::new(),
            regions: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // get username for request user data
        let target_username = self.props.username.to_string();

        // check get self data
        let get_self = matches!(
            &self.props.current_user,
            Some(cu) if cu.username == target_username
        );

        let link = self.link.clone();

        // // if open profile different with new
        // let change_profile = matches!(
        //     &self.props.current_user,
        //     Some(cu) if cu.username != self.current_profile
        // );

        if first_render && is_authenticated() {
            // // update current_profile
            // if let Some(cp) = &self.props.current_user {
            //     self.current_profile = cp.username.to_string();
            // }

            spawn_local(async move {
                match get_self {
                    true => {
                        let res = make_query(
                            GetSelfDataOpt::build_query(get_self_data_opt::Variables)
                        ).await.unwrap();

                        link.send_message(Msg::GetSelfData(res.clone()));
                        link.send_message(Msg::UpdateList(res));
                    },
                    false => {
                        let res = make_query(
                            GetUserDataOpt::build_query(get_user_data_opt::Variables {
                                username: Some(target_username),
                            })
                        ).await.unwrap();

                        link.send_message(Msg::GetUserData(res.clone()));
                        link.send_message(Msg::UpdateList(res));
                    },
                }
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();

        match msg {
            Msg::GetSelfData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                // clean profile data if get self user data
                self.profile = None;

                match res_value.is_null() {
                    false => {
                        let self_data: SelfUserInfo = serde_json::from_value(res_value.get("selfData").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("User self data: {:?}", self_data).as_ref());
                        self.self_profile = Some(self_data);
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
            },
            Msg::GetUserData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                // clean sef data if get data other user
                self.self_profile = None;

                match res_value.is_null() {
                    false => {
                        let user_data: UserInfo = serde_json::from_value(res_value.get("user").unwrap().clone()).unwrap();
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
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.regions =
                            serde_json::from_value(res_value.get("regions").unwrap().clone()).unwrap();
                        self.programs =
                            serde_json::from_value(res_value.get("programs").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("Update: {:?}", self.programs).as_ref());
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
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

        match (&self.self_profile, &self.profile) {
            (Some(self_data), _) => html! {
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
                                        </ul>
                                    </aside>
                                </div>
                                <div class="column">
                                    <div class="card">
                                      <div class="card-content">
                                        <div class="media">
                                          <div class="media-left">
                                            <figure class="image is-48x48">
                                              // <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                                              <img src={self_data.image_file.download_url.to_string()} alt="Favicon profile"/>
                                            </figure>
                                          </div>
                                          <div class="media-content">
                                            <p id="title-fl" class="title is-4">{
                                                format!("{} {}", self_data.firstname, self_data.lastname)
                                            }</p>
                                            <p id="subtitle-username" class="subtitle is-6">{
                                                format!("@{}", self_data.username)
                                            }</p>
                                          </div>
                                          <div class="media-right">
                                            <p class="subtitle is-6 left">
                                                // date formatting for show on page
                                                { format!("{:.*}", 19, self_data.updated_at.to_string()) }
                                                <br/>
                                                // for self user data not show button "following"
                                                <div class="media-right flexBox " >
                                                    <span class="icon is-small">
                                                      <i class="fas fa-bookmark"></i>
                                                      { format!(" {}", self_data.subscribers.to_string()) }
                                                    </span>
                                                </div>
                                            </p>
                                          </div>
                                        </div>

                                        <div id="description" class="content">
                                            { format!("{}", self_data.description) }
                                        </div>

                                        <div class="content">
                                            <span id="position">
                                              <i class="fas fa-briefcase"></i>
                                              { format!("Position: {}", self_data.position.to_string()) }
                                            </span>
                                            <br/>
                                            <span id="region">
                                              <i class="fas fa-map-marker-alt"></i>
                                              { format!("Region: {}", self_data.region.region.to_string()) }
                                            </span>
                                            <br/>
                                            <span id="program">
                                              <i class="fab fa-uncharted"></i>
                                              { format!("Working software: {}", self_data.program.name.to_string()) }
                                            </span>
                                        </div>

                                        <footer class="card-footer">
                                            <>{
                                                match self_data.certificates.is_empty() {
                                                    true => html!{},
                                                    false => {
                                                        html!{
                                                            // <p class="card-footer-item">
                                                            <>{
                                                                for self_data.certificates.iter().map(|cert| {
                                                                    let view_cert: Certificate = cert.into();
                                                                    html! {
                                                                        <CertificateCard certificate = view_cert />
                                                                    }
                                                                })
                                                            }</>
                                                            // </p>
                                                        }
                                                    },
                                                }
                                            }</>
                                        </footer>
                                    </div>
                                  </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            },
            (_, Some(user_data)) => html! {
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
                                        </ul>
                                    </aside>
                                </div>
                                <div class="column">
                                    // <h1 class="title">{ title }</h1>
                                    <div class="card">
                                      <div class="card-content">
                                        <div class="media">
                                          <div class="media-left">
                                            <figure class="image is-48x48">
                                              // <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                                              <img src={user_data.image_file.download_url.to_string()} alt="Favicon profile"/>
                                            </figure>
                                          </div>
                                          <div class="media-content">
                                            <p class="title is-4">{
                                                format!("{} {}", user_data.firstname, user_data.lastname)
                                            }</p>
                                            <p class="subtitle is-6">{
                                                format!("@{}", user_data.username)
                                            }</p>
                                          </div>
                                          <div class="media-right">
                                            <p class="subtitle is-6 left">
                                                // date formatting for show on page
                                                { format!("{:.*}", 19, user_data.updated_at.to_string()) }
                                                <br/>
                                                <div class="media-right flexBox " >
                                                    {
                                                        match user_data.is_followed {
                                                            true => html! {
                                                                <button class="button">
                                                                  <span class="icon is-small">
                                                                    <i class="fas fa-bookmark"></i>
                                                                  </span>
                                                                </button>
                                                            },
                                                            false => html! {
                                                                <button class="button">
                                                                  <span class="icon is-small">
                                                                    <i class="far fa-bookmark"></i>
                                                                  </span>
                                                                </button>
                                                            },
                                                        }
                                                    }
                                                  { format!(" {}", user_data.subscribers.to_string()) }
                                                </div>
                                            </p>
                                          </div>
                                        </div>

                                        <div id="description" class="content">
                                            { format!("{}", user_data.description) }
                                        </div>

                                        <div class="content">
                                            <span id="position">
                                              <i class="fas fa-briefcase"></i>
                                              { format!("Position: {}", user_data.position.to_string()) }
                                            </span>
                                            <br/>
                                            <span id="region">
                                              <i class="fas fa-map-marker-alt"></i>
                                              { format!("Region: {}", user_data.region.region.to_string()) }
                                            </span>
                                            <br/>
                                            <span id="program">
                                              <i class="fab fa-uncharted"></i>
                                              { format!("Working software: {}", user_data.program.name.to_string()) }
                                            </span>
                                        </div>

                                        <footer class="card-footer">
                                            <>{
                                                match user_data.certificates.is_empty() {
                                                    true => html!{},
                                                    false => {
                                                        html!{
                                                            // <p class="card-footer-item">
                                                            <>{
                                                                for user_data.certificates.iter().map(|cert| {
                                                                    let view_cert: Certificate = cert.into();
                                                                    html! {
                                                                        <CertificateCard certificate = view_cert />
                                                                    }
                                                                })
                                                            }</>
                                                            // </p>
                                                        }
                                                    },
                                                }
                                            }</>
                                        </footer>
                                      </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            },
            _ => html! {<div>
                <ListErrors error=self.error.clone()/>
                // <h1>{"Not data"}</h1>
            </div>},
        }
    }
}
