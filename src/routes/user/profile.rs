// use yew::services::fetch::FetchTask;
use chrono::NaiveDateTime;
use web_sys::MouseEvent;
// use yew::services::ConsoleService;
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::service::RouteService;
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::{
    user_certificate::CertificateCard,
    list_errors::ListErrors,
    catalog_user::CatalogUsers,
    catalog_component::CatalogComponents,
    catalog_company::CatalogCompanies,
    catalog_standard::CatalogStandards,
};
use crate::gqls::make_query;
// use crate::routes::AppRoute;
use crate::services::{is_authenticated, get_logged_user};
use crate::types::{
    UUID, Certificate, Program, Region, SelfUserInfo, SlimUser, UserCertificate,
    UserInfo, UsersQueryArg, ComponentsQueryArg, CompaniesQueryArg, StandardsQueryArg,
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
    current_user_uuid: UUID,
    current_username: String,
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
}

#[derive(Clone, PartialEq)]
pub enum ProfileTab {
    Certificates,
    Components,
    Companies,
    FavoriteComponents,
    FavoriteCompanies,
    FavoriteStandards,
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
            current_user_uuid: String::new(),
            current_username: String::new(),
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
        let not_matches_username = target_username != self.current_username;
        // debug!("self.current_username {:?}", self.current_username);

        // check get self data
        let get_self = matches!(
            get_logged_user(),
            Some(cu) if cu.username == target_username
        );

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_username) && is_authenticated() {
            // update current_username for checking change profile in route
            self.current_username = target_username.to_string();

            spawn_local(async move {
                match get_self {
                    true => {
                        let res = make_query(GetSelfDataOpt::build_query(
                            get_self_data_opt::Variables
                        )).await.unwrap();

                        link.send_message(Msg::GetSelfData(res.clone()));
                        link.send_message(Msg::UpdateList(res));
                    }
                    false => {
                        let ipt_get_user_arg = get_user_data_opt::IptGetUserArg{
                            userUuid: None,
                            username: Some(target_username)
                        };
                        let res = make_query(GetUserDataOpt::build_query(get_user_data_opt::Variables{
                            ipt_get_user_arg
                        })).await.unwrap();

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
                let user_uuid = self.profile.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(AddUserFav::build_query(add_user_fav::Variables {
                        user_uuid,
                    })).await.unwrap();

                    link.send_message(Msg::AddFollow(res));
                })
            }
            Msg::AddFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(
                            res_value.get("addUserFav").unwrap().clone()
                        ).unwrap();

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
                let user_uuid = self.profile.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(DeleteUserFav::build_query(delete_user_fav::Variables {
                        user_uuid,
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

                // debug!("res_value: {:?}", res_value);

                match res_value.is_null() {
                    false => {
                        let self_data: SelfUserInfo =
                            serde_json::from_value(res_value.get("selfData").unwrap().clone())
                                .unwrap();
                        debug!("User self data: {:?}", self_data);

                        self.subscribers = self_data.subscribers.to_owned();
                        self.current_user_uuid = self_data.uuid.to_owned();
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
                        debug!("User data: {:?}", user_data);

                        self.subscribers = user_data.subscribers.to_owned();
                        self.is_followed = user_data.is_followed.to_owned();
                        self.current_user_uuid = user_data.uuid.to_owned();
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
                        debug!("Update: {:?}", self.programs);
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
                                <div style="display: flex;padding: 10px;padding-top: 20px;border-top: 5px dashed;">
                                { self.show_profile_action() }
                                // <hr/>
                                <div class="card-relate-data" style="flex:1;" >
                                    {match self.profile_tab {
                                        ProfileTab::Certificates => {
                                            self.view_certificates(&self_data.certificates)
                                        },
                                        ProfileTab::Components => {
                                            self.view_components(&self_data.uuid)
                                        },
                                        ProfileTab::Companies => {
                                            self.view_companies(&self_data.uuid)
                                        },
                                        ProfileTab::FavoriteComponents => {
                                            self.view_favorite_components(None)
                                        },
                                        ProfileTab::FavoriteCompanies => {
                                            self.view_favorite_companies(None)
                                        },
                                        ProfileTab::FavoriteStandards => {
                                            self.view_favorite_standards()
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
                                <div style="display: flex;padding: 10px;">
                                  { self.show_profile_action() }
                                  <div class="card-relate-data" style="flex:1;">
                                      {match self.profile_tab {
                                          ProfileTab::Certificates => {
                                              self.view_certificates(&user_data.certificates)
                                          },
                                          ProfileTab::Components => {
                                              self.view_components(&user_data.uuid)
                                          },
                                          ProfileTab::Companies => {
                                              self.view_companies(&user_data.uuid)
                                          },
                                          ProfileTab::FavoriteComponents => {
                                              self.view_favorite_components(Some(user_data.uuid.clone()))
                                          },
                                          ProfileTab::FavoriteCompanies => {
                                              self.view_favorite_companies(Some(user_data.uuid.clone()))
                                          },
                                          _ => html! {},
                                      }}
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

impl Profile {
    fn view_card(&self) -> Html {
        let UserDataCard {
            // image_file,
            firstname,
            lastname,
            username,
            updated_at,
            ..
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
                <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                // <img src={image_file.to_string()} alt="Favicon profile"/>
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
        let onclick_certificates = self
            .link
            .callback(|_| Msg::ChangeTab(ProfileTab::Certificates));

        let onclick_components = self
            .link
            .callback(|_| Msg::ChangeTab(ProfileTab::Components));

        let onclick_companies = self
            .link
            .callback(|_| Msg::ChangeTab(ProfileTab::Companies));

        let onclick_fav_components = self
            .link
            .callback(|_| Msg::ChangeTab(ProfileTab::FavoriteComponents));

        let onclick_fav_companies = self
            .link
            .callback(|_| Msg::ChangeTab(ProfileTab::FavoriteCompanies));

        let onclick_fav_standards = self
            .link
            .callback(|_| Msg::ChangeTab(ProfileTab::FavoriteStandards));

        let onclick_fav_users = self
            .link
            .callback(|_| Msg::ChangeTab(ProfileTab::FavoriteUsers));

        let mut active_certificates = "";
        let mut active_components = "";
        let mut active_companies = "";
        let mut active_fav_components = "";
        let mut active_fav_companies = "";
        let mut active_fav_standards = "";
        let mut active_fav_users = "";

        match &self.profile_tab {
            ProfileTab::Certificates => active_certificates = "is-active",
            ProfileTab::Components => active_components = "is-active",
            ProfileTab::Companies => active_companies = "is-active",
            ProfileTab::FavoriteComponents => active_fav_components = "is-active",
            ProfileTab::FavoriteCompanies => active_fav_companies = "is-active",
            ProfileTab::FavoriteStandards => active_fav_standards = "is-active",
            ProfileTab::FavoriteUsers => active_fav_users = "is-active",
        }

        fn li_generator(class:&'static str, onclick: Callback<MouseEvent>, info:String, number: usize) -> Html {
          let show_tag = number==0;

          html!(
            <li>
              <a class={class} onclick=onclick style="display: flex;justify-content: space-between;" >
                <span>{info}</span>
                <span hidden=show_tag>
                  <span class="tag is-info is-small" >{number}</span>
                </span>
              </a>
            </li>
          )
        }

        html! {
            <div class="card" style="padding: 10px;margin-right: 18px;" >
                // <ul>
                <aside class="menu">
                    {match &self.self_profile {
                        Some(ref self_data) => html! {<>
                            <p class="menu-label">
                              {"General"}
                            </p>
                            <ul class="menu-list">
                                {li_generator(active_certificates, onclick_certificates, "Certificates".to_string(), self_data.certificates.len())}
                            </ul>
                            <p class="menu-label">
                              {"Components"}
                            </p>
                            <ul class="menu-list">
                                {li_generator(active_components, onclick_components, "all".to_string(), self_data.components_count)}
                                {li_generator(active_fav_components, onclick_fav_components, "fav".to_string(), self_data.fav_components_count)}
                            </ul>
                            <p class="menu-label">
                              {"Companies"}
                            </p>
                            <ul class="menu-list">
                                {li_generator(active_companies, onclick_companies, "all".to_string(), self_data.companies_count)}
                                {li_generator(active_fav_companies, onclick_fav_companies, "fav".to_string(), self_data.fav_companies_count)}
                            </ul>
                            <p class="menu-label">
                              {"Other Fav"}
                            </p>
                            <ul class="menu-list">
                                {li_generator(active_fav_standards, onclick_fav_standards, "standards".to_string(), self_data.fav_standards_count)}
                                {li_generator(active_fav_users, onclick_fav_users, "users".to_string(), self_data.fav_users_count)}
                            </ul>
                        </>},
                        None => html! {<>
                            <p class="menu-label">
                              {"General"}
                            </p>
                            <ul class="menu-list">
                                {li_generator(active_certificates, onclick_certificates, "Certificates".to_string(), 0)}
                            </ul>
                            <p class="menu-label">
                              {"Components"}
                            </p>
                            <ul class="menu-list">
                                {li_generator(active_components, onclick_components, "all".to_string(), 0)}
                                {li_generator(active_fav_components, onclick_fav_components, "fav".to_string(), 0)}
                            </ul>
                            <p class="menu-label">
                              {"Companies"}
                            </p>
                            <ul class="menu-list">
                                {li_generator(active_companies, onclick_companies, "all".to_string(), 0)}
                                {li_generator(active_fav_companies, onclick_fav_companies, "fav".to_string(), 0)}
                            </ul>
                        </>},
                    }
                }
                // </ul>
                </aside>
            </div>
        }
    }

    fn view_content(
        &self,
        description: &str,
        position: &str,
        region: &str,
        program: &str
    ) -> Html {
        html! {
            <div class="columns">
                <div class="column">
                    <div id="description" class="content">
                      { format!("{}", description) }
                    </div>
                    <br/>
                </div>
                <div class="column">
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
            </div>
        }
    }

    fn view_certificates(
        &self,
        certificates: &[UserCertificate]
    ) -> Html {
        match certificates.is_empty() {
            true => html! {},
            false => {
                html! {
                    // <p class="card-footer-item">
                    <footer class="card-footer is-flex is-flex-wrap-wrap">{
                        for certificates.iter().map(|cert| {
                            let view_cert: Certificate = cert.into();
                            html! {
                                <CertificateCard
                                    certificate = view_cert
                                    show_cert_btn = true
                                    download_btn = false
                                    change_btn = false
                                 />
                            }
                        })
                    }</footer>
                    // </p>
                }
            }
        }
    }

    fn view_favorite_components(
        &self,
        user_uuid: Option<UUID>,
    ) -> Html {
        html! {
            <CatalogComponents
                show_create_btn = self.self_profile.is_some()
                arguments = ComponentsQueryArg::set_favorite(user_uuid)
            />
        }
    }

    fn view_components(
        &self,
        user_uuid: &UUID,
    ) -> Html {
        html! {
            <CatalogComponents
                show_create_btn = self.self_profile.is_some()
                arguments = ComponentsQueryArg::set_user_uuid(user_uuid)
            />
        }
    }

    fn view_favorite_companies(
        &self,
        user_uuid: Option<UUID>,
    ) -> Html {
        html! {
            <CatalogCompanies
                show_create_btn = self.self_profile.is_some()
                arguments = CompaniesQueryArg::set_favorite(user_uuid)
            />
        }
    }

    fn view_companies(
        &self,
        user_uuid: &UUID,
    ) -> Html {
        html! {
            <CatalogCompanies
                show_create_btn = self.self_profile.is_some()
                arguments = CompaniesQueryArg::set_user_uuid(user_uuid)
            />
        }
    }

    fn view_favorite_standards(&self) -> Html {
        html! {
            <CatalogStandards
                show_create_btn = false
                arguments = StandardsQueryArg::set_favorite()
            />
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