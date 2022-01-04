// use yew::services::fetch::FetchTask;
use chrono::NaiveDateTime;
use web_sys::MouseEvent;
// use yew::services::ConsoleService;
use graphql_client::GraphQLQuery;
use log::debug;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::service::RouteService;

use crate::error::{get_error, Error};
use crate::fragments::{
    company::CatalogCompanies,
    component::CatalogComponents,
    list_errors::ListErrors,
    side_menu::{MenuItem, SideMenu},
    standard::CatalogStandards,
    user::CatalogUsers,
    user::CertificateCard,
};
use crate::gqls::make_query;
// use crate::routes::AppRoute;
use crate::services::{get_logged_user, is_authenticated};
use crate::types::{
    UserDataCard, Certificate, CompaniesQueryArg, ComponentsQueryArg, Program, Region, SelfUserInfo, SlimUser,
    StandardsQueryArg, UserCertificate, UserInfo, UsersQueryArg, UUID,
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
    extend_tab: Option<ProfileTab>,
    show_full_user_info: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    // pub current_route: Option<AppRoute>,
    // pub username: String,
    pub current_user: Option<SlimUser>,
    // pub tab: ProfileTab,
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
    ShowFullUserInfo,
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
            extend_tab: Some(ProfileTab::Certificates),
            show_full_user_info: true,
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
                        let res =
                            make_query(GetSelfDataOpt::build_query(get_self_data_opt::Variables))
                                .await
                                .unwrap();

                        link.send_message(Msg::GetSelfData(res.clone()));
                        link.send_message(Msg::UpdateList(res));
                    }
                    false => {
                        let ipt_get_user_arg = get_user_data_opt::IptGetUserArg {
                            userUuid: None,
                            username: Some(target_username),
                        };
                        let res =
                            make_query(GetUserDataOpt::build_query(get_user_data_opt::Variables {
                                ipt_get_user_arg,
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
                let user_uuid = self.profile.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(AddUserFav::build_query(add_user_fav::Variables {
                        user_uuid,
                    }))
                    .await
                    .unwrap();

                    link.send_message(Msg::AddFollow(res));
                })
            },
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
            },
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
            },
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
            },
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
            },
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
            },
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
            },
            Msg::ChangeTab(set_tab) => {
                self.profile_tab = set_tab.clone();
                if self.extend_tab.is_none() {
                    self.extend_tab = Some(set_tab);
                } else {
                    if self.extend_tab.clone().unwrap() != set_tab {
                        self.extend_tab = Some(set_tab);
                    } else {
                        self.extend_tab = None;
                    }
                }
            },
            Msg::ShowFullUserInfo => self.show_full_user_info = !self.show_full_user_info,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        match (&self.self_profile, &self.profile) {
            (Some(self_data), _) => self.self_user_card(self_data),
            (_, Some(user_data)) => self.other_user_card(user_data),
            _ => html!{<ListErrors error=self.error.clone()/>},
        }
    }
}

impl Profile {
    fn self_user_card(
        &self,
        self_data: &SelfUserInfo,
    ) -> Html {
        html! {
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
                                { self.view_user_info(
                                    self_data.description.as_str(),
                                    self_data.position.as_str(),
                                    self_data.region.region.as_str(),
                                    self_data.program.name.as_str(),
                                ) }
                            </div>
                        </div>
                        {self.self_user_relate_object(self_data)}
                      </div>
                    </div>
                </div>
            </div>
        }
    }

    fn self_user_relate_object(
        &self,
        self_data: &SelfUserInfo,
    ) -> Html {
        html!{<div class="card">
            <div class="columns is-mobile">
                <div class="column is-1">
                    { self.show_profile_action() }
                </div>
                // <hr/>
                <div class="column">
                    <div class="card-relate-data" style="flex:1;" >
                        {match self.profile_tab {
                            ProfileTab::Certificates => self.view_certificates(&self_data.certificates),
                            ProfileTab::Components => self.view_components(&self_data.uuid),
                            ProfileTab::Companies => self.view_companies(&self_data.uuid),
                            ProfileTab::FavoriteComponents => self.view_favorite_components(None),
                            ProfileTab::FavoriteCompanies => self.view_favorite_companies(None),
                            ProfileTab::FavoriteStandards => self.view_favorite_standards(),
                            ProfileTab::FavoriteUsers => self.view_favorite_users(),
                        }}
                    </div>
                </div>
            </div>
        </div>}
    }

    fn other_user_card(
        &self,
        user_data: &UserInfo,
    ) -> Html {
        html! {
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
                                { self.view_user_info(
                                    user_data.description.as_str(),
                                    user_data.position.as_str(),
                                    user_data.region.region.as_str(),
                                    user_data.program.name.as_str(),
                                ) }
                            </div>
                          </div>
                        </div>
                        {self.other_user_relate_object(user_data)}
                    </div>
                </div>
            </div>
        }
    }

    fn other_user_relate_object(
        &self,
        user_data: &UserInfo,
    ) -> Html {
        html!{<div class="card">
          { self.show_profile_action() }
          <div class="card-relate-data" style="flex:1;">
              {match self.profile_tab {
                  ProfileTab::Certificates => self.view_certificates(&user_data.certificates),
                  ProfileTab::Components => self.view_components(&user_data.uuid),
                  ProfileTab::Companies => self.view_companies(&user_data.uuid),
                  ProfileTab::FavoriteComponents => self.view_favorite_components(Some(user_data.uuid.clone())),
                  ProfileTab::FavoriteCompanies => self.view_favorite_companies(Some(user_data.uuid.clone())),
                  _ => html!{},
              }}
          </div>
        </div>}
    }

    fn view_card(&self) -> Html {
        let UserDataCard {
            // image_file,
            firstname,
            lastname,
            username,
            updated_at,
            ..
        } = match (&self.self_profile, &self.profile) {
            (_, Some(ref other_data)) => other_data.into(),
            (Some(ref self_data), _) => self_data.into(),
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

                    html!{
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

    fn cb_generator(&self, cb: ProfileTab) -> Callback<MouseEvent> {
        match cb {
            n => self.link.callback(move |_| Msg::ChangeTab(n.clone())),
        }
        // self.link.callback(move |_| Msg::ChangeTab(ProfileTab))
    }

    fn check_extend(&self, tab: ProfileTab) -> bool {
        match tab {
            n => {
                if self.extend_tab.is_some() {
                    self.extend_tab.clone().unwrap() == n.clone()
                } else {
                    false
                }
            }
        }
    }

    fn show_profile_action(&self) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            MenuItem {
                title: "Certificates".to_string(),
                action: self.cb_generator(ProfileTab::Certificates),
                count: if let Some(res) = self.self_profile.as_ref() {
                    res.certificates.len()
                } else {
                    0
                },
                icon_class: classes!("fas", "fa-certificate"),
                is_active: self.profile_tab == ProfileTab::Certificates,
                is_extend: self.check_extend(ProfileTab::Certificates),
            },
            MenuItem {
                title: "COMPONENTS all".to_string(),
                action: self.cb_generator(ProfileTab::Components),
                count: if let Some(res) = self.self_profile.as_ref() {
                    res.certificates.len()
                } else {
                    0
                },
                icon_class: classes!("fas", "fa-certificate"),
                is_active: self.profile_tab == ProfileTab::Components,
                is_extend: self.check_extend(ProfileTab::Components),
            },
            MenuItem {
                title: "COMPONENTS fav".to_string(),
                action: self.cb_generator(ProfileTab::FavoriteComponents),
                count: if let Some(res) = self.self_profile.as_ref() {
                    res.certificates.len()
                } else {
                    0
                },
                icon_class: classes!("fas", "fa-certificate"),
                is_active: self.profile_tab == ProfileTab::FavoriteComponents,
                is_extend: self.check_extend(ProfileTab::FavoriteComponents),
            },
            // company MenuItem
            MenuItem {
                title: "COMPANIES all".to_string(),
                action: self.cb_generator(ProfileTab::Companies),
                count: if let Some(res) = self.self_profile.as_ref() {
                    res.certificates.len()
                } else {
                    0
                },
                icon_class: classes!("fas", "fa-certificate"),
                is_active: self.profile_tab == ProfileTab::Companies,
                is_extend: self.check_extend(ProfileTab::Companies),
            },
            // company fav MenuItem
            MenuItem {
                title: "COMPANIES fav".to_string(),
                action: self.cb_generator(ProfileTab::FavoriteCompanies),
                count: if let Some(res) = self.self_profile.as_ref() {
                    res.certificates.len()
                } else {
                    0
                },
                icon_class: classes!("fas", "fa-certificate"),
                is_active: self.profile_tab == ProfileTab::FavoriteCompanies,
                is_extend: self.check_extend(ProfileTab::FavoriteCompanies),
            },
            // standards MenuItem
            MenuItem {
                title: "STANDARDS all".to_string(),
                action: self.cb_generator(ProfileTab::FavoriteStandards),
                count: if let Some(res) = self.self_profile.as_ref() {
                    res.certificates.len()
                } else {
                    0
                },
                icon_class: classes!("fas", "fa-certificate"),
                is_active: self.profile_tab == ProfileTab::FavoriteStandards,
                is_extend: self.check_extend(ProfileTab::FavoriteStandards),
            },
            // user fav MenuItem
            MenuItem {
                title: "USERS fav".to_string(),
                action: self.cb_generator(ProfileTab::FavoriteUsers),
                count: if let Some(res) = self.self_profile.as_ref() {
                    res.certificates.len()
                } else {
                    0
                },
                icon_class: classes!("fas", "fa-certificate"),
                is_active: self.profile_tab == ProfileTab::FavoriteUsers,
                is_extend: self.check_extend(ProfileTab::FavoriteUsers),
            },
        ];

        html! {
            <div style="margin-right: 18px;z-index: 1;" >
                <SideMenu menu_arr={menu_arr} ></SideMenu>
            </div>
        }
    }

    fn view_user_info(
        &self,
        description: &str,
        position: &str,
        region: &str,
        program: &str,
    ) -> Html {
        let onclick_change_full_show = self.link
            .callback(|_| Msg::ShowFullUserInfo);

        match self.show_full_user_info {
            true => html! {<>
                <div class="columns">
                    <div class="column">
                        <div id="description" class="content">
                          { format!("{}", description) }
                        </div>
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
                <button class="button is-ghost" onclick={onclick_change_full_show}>
                    <span>{"Hide info"}</span>
                </button>
            </>},
            false => html!{
                <button class="button is-ghost" onclick={onclick_change_full_show}>
                    <span>{"Show info"}</span>
                </button>
            },
        }
    }

    fn view_certificates(&self, certificates: &[UserCertificate]) -> Html {
        match certificates.is_empty() {
            true => html! {},
            false => {
                html! {
                    // <p class="card-footer-item">
                    <footer class="card-footer is-flex is-flex-wrap-wrap">{
                        for certificates.iter().map(|cert| {
                            let view_cert: Certificate = cert.into();
                            html!{
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

    fn view_favorite_components(&self, user_uuid: Option<UUID>) -> Html {
        html! {
            <CatalogComponents
                show_create_btn = self.self_profile.is_some()
                arguments = ComponentsQueryArg::set_favorite(user_uuid)
            />
        }
    }

    fn view_components(&self, user_uuid: &UUID) -> Html {
        html! {
            <CatalogComponents
                show_create_btn = self.self_profile.is_some()
                arguments = ComponentsQueryArg::set_user_uuid(user_uuid)
            />
        }
    }

    fn view_favorite_companies(&self, user_uuid: Option<UUID>) -> Html {
        html! {
            <CatalogCompanies
                show_create_btn = self.self_profile.is_some()
                arguments = CompaniesQueryArg::set_favorite(user_uuid)
            />
        }
    }

    fn view_companies(&self, user_uuid: &UUID) -> Html {
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
