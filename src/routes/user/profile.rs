// use yew::services::fetch::FetchTask;
use chrono::NaiveDateTime;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::service::RouteService;
// use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::{
    certificate::CertificateCard,
    list_errors::ListErrors,
    catalog_user::CatalogUsers,
};
use crate::gqls::make_query;
// use crate::routes::AppRoute;
use crate::services::{is_authenticated, set_token};
use crate::types::{
    Certificate, Program, Region, SelfUserInfo, SlimUser, UserCertificate, UserInfo, UsersQueryArg,
    UUID,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct AddUserFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct DeleteUserFav;

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
    current_profile: String,
    // task: Option<FetchTask>,
    // router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    programs: Vec<Program>,
    regions: Vec<Region>,
    subscribers: usize,
    is_followed: bool,
    profile_tab: ProfileTab,
}

#[derive(Properties, Clone)]
pub struct Props {
    // pub current_route: Option<AppRoute>,
    // pub username: String,
    pub current_user: Option<SlimUser>,
    // pub tab: ProfileTab,
}

#[derive(Default, Debug)]
pub struct UserDataCard<'a> {
    pub image_file: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub username: &'a str,
    pub updated_at: String,
}

#[derive(Clone)]
pub enum Msg {
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    GetSelfData(String),
    GetUserData(String),
    UpdateList(String),
    ChangeTab(ProfileTab),
    Ignore,
    Logout,
}

#[derive(Clone, PartialEq)]
pub enum ProfileTab {
    // ByCompany,
    // ByCoponent,
    // ByStandard,
    // ByAuthor,
    Certificates,
    FavoriteUsers,
}

impl Component for Profile {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Profile {
            error: None,
            self_profile: None,
            profile: None,
            current_profile: String::new(),
            // task: None,
            // router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            programs: Vec::new(),
            regions: Vec::new(),
            subscribers: 0,
            is_followed: false,
            profile_tab: ProfileTab::Certificates,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // get username for request user data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_username = route_service
            .get_fragment()
            .trim_start_matches("#/@")
            .to_string();
        // get flag changing current profile in route
        let not_matches_username = target_username != self.current_profile;
        // debug!("self.current_profile {:#?}", self.current_profile);

        // check get self data
        let get_self = matches!(
            &self.props.current_user,
            Some(cu) if cu.username == target_username
        );

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_username) && is_authenticated() {
            // update current_profile for checking change profile in route
            self.current_profile = target_username.to_string();

            spawn_local(async move {
                match get_self {
                    true => {
                        let res =
                            make_query(GetSelfDataOpt::build_query(get_self_data_opt::Variables))
                                .await
                                .unwrap();

                        link.send_message(Msg::GetSelfData(res.clone()));
                        link.send_message(Msg::UpdateList(res));
                    }
                    false => {
                        let res =
                            make_query(GetUserDataOpt::build_query(get_user_data_opt::Variables {
                                username: Some(target_username),
                            }))
                            .await
                            .unwrap();

                        link.send_message(Msg::GetUserData(res.clone()));
                        link.send_message(Msg::UpdateList(res));
                    }
                }
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();

        match msg {
            Msg::Follow => {
                let link = self.link.clone();
                let user_uuid_string = self.profile.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(AddUserFav::build_query(add_user_fav::Variables {
                        user_uuid: user_uuid_string,
                    }))
                    .await
                    .unwrap();

                    link.send_message(Msg::AddFollow(res.clone()));
                })
            }
            Msg::AddFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("addUserFav").unwrap().clone())
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
                let user_uuid_string = self.profile.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(DeleteUserFav::build_query(delete_user_fav::Variables {
                        user_uuid: user_uuid_string,
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
                            serde_json::from_value(res_value.get("deleteUserFav").unwrap().clone())
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
            Msg::GetSelfData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                // clean profile data if get self user data
                self.profile = None;

                match res_value.is_null() {
                    false => {
                        let self_data: SelfUserInfo =
                            serde_json::from_value(res_value.get("selfData").unwrap().clone())
                                .unwrap();
                        ConsoleService::info(format!("User self data: {:?}", self_data).as_ref());

                        self.subscribers = self_data.subscribers.to_owned();

                        self.self_profile = Some(self_data);
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::GetUserData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                // clean sef data if get data other user
                self.self_profile = None;

                match res_value.is_null() {
                    false => {
                        let user_data: UserInfo =
                            serde_json::from_value(res_value.get("user").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("User data: {:?}", user_data).as_ref());

                        self.subscribers = user_data.subscribers.to_owned();
                        self.is_followed = user_data.is_followed.to_owned();

                        self.profile = Some(user_data);
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::ChangeTab(set_tab) => {
                self.profile_tab = set_tab;
            }
            Msg::Ignore => {}
            Msg::Logout => {
                // Clear global token after logged out
                set_token(None);
                // Notify app to clear current user info
                // self.props.callback.emit(());
                // Redirect to home page
                // self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::UpdateList(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.regions =
                            serde_json::from_value(res_value.get("regions").unwrap().clone())
                                .unwrap();
                        self.programs =
                            serde_json::from_value(res_value.get("programs").unwrap().clone())
                                .unwrap();
                        ConsoleService::info(format!("Update: {:?}", self.programs).as_ref());
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
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
                            <div class="card">
                              <div class="card-content">
                                <div class="media">
                                  { self.view_card() }
                                </div>

                                <div class="content">
                                    { self.view_content(
                                        self_data.description.as_str(),
                                        self_data.position.as_str(),
                                        self_data.region.region.as_str(),
                                        self_data.program.name.as_str(),
                                    ) }
                                </div>
                                <hr/>
                                <div class="card-data">
                                    {match self.profile_tab {
                                        ProfileTab::Certificates => {
                                            self.view_certificates(&self_data.certificates)
                                        },
                                        ProfileTab::FavoriteUsers => {
                                            self.view_favorite_users()
                                        },
                                    }}
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
                            // <h1 class="title">{ title }</h1>
                            <div class="card">
                              <div class="card-content">
                                <div class="media">
                                  { self.view_card() }
                                </div>

                                <div class="content">
                                    { self.view_content(
                                        user_data.description.as_str(),
                                        user_data.position.as_str(),
                                        user_data.region.region.as_str(),
                                        user_data.program.name.as_str(),
                                    ) }
                                </div>

                                <footer class="card-footer">
                                    { self.view_certificates(&user_data.certificates) }
                                </footer>
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

impl Profile {
    fn view_card(&self) -> Html {
        let UserDataCard {
            image_file,
            firstname,
            lastname,
            username,
            updated_at,
        } = match (&self.self_profile, &self.profile) {
            (_, Some(ref other_data)) => UserDataCard {
                image_file: &other_data.image_file.download_url,
                firstname: &other_data.firstname,
                lastname: &other_data.lastname,
                username: &other_data.username,
                updated_at: format!("{:.*}", 19, &other_data.updated_at.to_string()),
            },
            (Some(ref self_data), _) => UserDataCard {
                image_file: &self_data.image_file.download_url,
                firstname: &self_data.firstname,
                lastname: &self_data.lastname,
                username: &self_data.username,
                updated_at: format!("{:.*}", 19, &self_data.updated_at.to_string()),
            },
            (None, None) => UserDataCard::default(),
        };

        html! {<>
            <div class="media-left">
              <figure class="image is-48x48">
                // <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                <img src={image_file.to_string()} alt="Favicon profile"/>
              </figure>
            </div>
            <div class="media-content">
              <p id="title-fl" class="title is-4">{
                  format!("{} {}", firstname, lastname)
              }</p>
              <p id="subtitle-username" class="subtitle is-6">{
                  format!("@{}", username)
              }</p>
            </div>
            <div class="media-right">
              <p class="subtitle is-6 left">
                  // date formatting for show on page
                  { updated_at }
                  <br/>
                  // for self user data not show button "following"
                  <div class="media-right flexBox " >
                    { self.show_profile_followers() }
                  </div>
              </p>
            </div>
        </>}
    }

    fn show_profile_followers(&self) -> Html {
        html! {<>
            {match &self.profile {
                Some(_) => {
                    let class_fav = match self.is_followed {
                        true => "fas fa-bookmark",
                        false => "far fa-bookmark",
                    };

                    let onclick_following = match self.is_followed {
                        true => self.link.callback(|_| Msg::UnFollow),
                        false => self.link.callback(|_| Msg::Follow),
                    };

                    html! {
                        // for self user data not show button "following"
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
                    }
                },
                None => html!{},
            }}
            { format!(" {}", &self.subscribers) }
        </>}
    }

    fn show_profile_action(&self) -> Html {
        let onclick_fav_users = self
            .link
            .callback(|_| Msg::ChangeTab(ProfileTab::FavoriteUsers));

        match &self.self_profile {
            Some(ref self_data) => html! {<>
                <div class="columns">
                    <div class="column">
                        <span>{ "objects count" }</span>
                        <br/>
                        { format!("companies: {}", self_data.companies_count.to_string()) }
                        <br/>
                        { format!("components: {}", self_data.components_count.to_string()) }
                        <br/>
                        { format!("standards: {}", self_data.standards_count.to_string()) }
                    </div>
                    <div class="column">
                        <span>{ "favorites count" }</span>
                        <br/>
                        { format!("companies: {}", self_data.fav_companies_count.to_string()) }
                        <br/>
                        { format!("components: {}", self_data.fav_components_count.to_string()) }
                        <br/>
                        { format!("standards: {}", self_data.fav_standards_count.to_string()) }
                        <br/>
                        <a onclick=onclick_fav_users>
                            { format!("users: {}", self_data.fav_users_count.to_string()) }
                        </a>
                    </div>
                    // <div class="column"></div>
                </div>
            </>},
            None => html! {},
        }
    }

    fn view_content(&self, description: &str, position: &str, region: &str, program: &str) -> Html {
        html! {<>
            <div id="description" class="content">
              { format!("{}", description) }
            </div>
            <div class="columns">
                <div class="column">
                    <br/>
                    <span id="position">
                      <i class="fas fa-briefcase"></i>
                      { format!("Position: {}", position) }
                    </span>
                    <br/>
                    <span id="region">
                      <i class="fas fa-map-marker-alt"></i>
                      { format!("Region: {}", region) }
                    </span>
                    <br/>
                    <span id="program">
                      <i class="fab fa-uncharted"></i>
                      { format!("Working software: {}", program) }
                    </span>
                </div>
                <div class="column">
                    { self.show_profile_action() }
                </div>
            </div>
        </>}
    }

    fn view_certificates(&self, certificates: &[UserCertificate]) -> Html {
        match certificates.is_empty() {
            true => html! {},
            false => {
                html! {
                    // <p class="card-footer-item">
                    <>{
                        for certificates.iter().map(|cert| {
                            let view_cert: Certificate = cert.into();
                            html! {
                                <CertificateCard
                                    certificate = view_cert
                                    show_cert_btn = true
                                    download_btn = false
                                    change_btn = false
                                    company_uuid = None
                                 />
                            }
                        })
                    }</>
                    // </p>
                }
            }
        }
    }

    fn view_favorite_users(&self) -> Html {
        html! {
            <CatalogUsers
                arguments = UsersQueryArg::set_favorite()
            />
        }
    }
}
