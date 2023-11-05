use yew::{agent::Bridged, classes, html, Bridge, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use web_sys::MouseEvent;
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    company::CatalogCompanies,
    component::CatalogComponents,
    list_errors::ListErrors,
    side_menu::{MenuItem, SideMenu},
    standard::CatalogStandards,
    user::CatalogUsers,
    user::UserCertificatesCard,
};
use crate::services::{Counter, get_logged_user, get_value_field, resp_parsing};
use crate::types::{
    UserDataCard, CompaniesQueryArg, ComponentsQueryArg, SelfUserInfo, SlimUser,
    StandardsQueryArg, UserCertificate, UserInfo, UsersQueryArg, UUID,
};
use crate::gqls::make_query;
use crate::gqls::user::{
    AddUserFav, add_user_fav,
    DeleteUserFav, delete_user_fav,
    GetSelfData, get_self_data,
    GetUserData, get_user_data,
};
use crate::services::url_decode;

/// Profile user with relate data
pub struct Profile {
    error: Option<Error>,
    self_profile: Option<SelfUserInfo>,
    profile: Option<UserInfo>,
    current_user_uuid: UUID,
    current_username: String,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    profile_tab: ProfileTab,
    extend_tab: Option<ProfileTab>,
    show_full_user_info: bool,
}

impl Counter for Profile {
    fn quantity(&self) -> usize {
        self.subscribers
    }
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
}

#[derive(Clone)]
pub enum Msg {
    RequestProfileData(bool),
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    ResponseError(Error),
    GetSelfProfileResult(String),
    GetUserProfileResult(String),
    ChangeTab(ProfileTab),
    ShowFullUserInfo,
    ClearError,
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
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            profile_tab: ProfileTab::Certificates,
            extend_tab: Some(ProfileTab::Certificates),
            show_full_user_info: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let logged_username = match get_logged_user() {
            Some(cu) => cu.username,
            None => {
                // route to login page if not found token
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                String::new()
            },
        };

        // get username for request user data
        let route_service: RouteService<()> = RouteService::new();
        // get and decode target user from route
        let target_username = url_decode(route_service.get_fragment().trim_start_matches("#/@"));

        // get flag changing current profile in route
        let not_matches_username = target_username != self.current_username;
        // debug!("self.current_username {:?}", self.current_username);

        // check get self data
        let get_self = logged_username == target_username;
        // debug!("get_self {:?}", get_self);

        if first_render || not_matches_username {
            // clear old data
            self.error = None;
            self.self_profile = None;
            self.profile = None;

            // update current_username for checking change profile in route
            self.current_username = target_username.to_string();

            self.link.send_message(Msg::RequestProfileData(get_self))
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestProfileData(get_self) => {
                let target_username = self.current_username.clone();

                spawn_local(async move {
                    match get_self {
                        true => {
                            let res = make_query(GetSelfData::build_query(
                                get_self_data::Variables
                            )).await.unwrap();

                            link.send_message(Msg::GetSelfProfileResult(res));
                        }
                        false => {
                            let ipt_get_user_arg = get_user_data::IptGetUserArg {
                                userUuid: None,
                                username: Some(target_username),
                            };
                            let res = make_query(GetUserData::build_query(
                                get_user_data::Variables { ipt_get_user_arg }
                            )).await.unwrap();

                            link.send_message(Msg::GetUserProfileResult(res));
                        }
                    }
                })
            },
            Msg::Follow => {
                let link = self.link.clone();
                let user_uuid = self.profile.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(AddUserFav::build_query(
                        add_user_fav::Variables { user_uuid }
                    )).await.unwrap();

                    link.send_message(Msg::AddFollow(res));
                })
            },
            Msg::AddFollow(res) => {
                match resp_parsing(res, "addUserFav") {
                    Ok(result) => {
                        if result {
                            self.subscribers += 1;
                            self.is_followed = true;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UnFollow => {
                let link = self.link.clone();
                let user_uuid = self.profile.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(DeleteUserFav::build_query(
                        delete_user_fav::Variables { user_uuid }
                    )).await.unwrap();

                    link.send_message(Msg::DelFollow(res));
                })
            },
            Msg::DelFollow(res) => {
                match resp_parsing(res, "deleteUserFav") {
                    Ok(result) => {
                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetSelfProfileResult(res) => {
                self.profile = None; // clean profile data if get self user data
                match resp_parsing::<SelfUserInfo>(res, "selfData") {
                    Ok(self_data) => {
                        debug!("User self data: {:?}", self_data);
                        self.subscribers = self_data.subscribers.to_owned();
                        self.current_user_uuid = self_data.uuid.to_owned();
                        self.self_profile = Some(self_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUserProfileResult(res) => {
                self.self_profile = None; // clean sef data if get data other user
                match resp_parsing::<UserInfo>(res, "user") {
                    Ok(user_data) => {
                        debug!("User data: {:?}", user_data);
                        self.subscribers = user_data.subscribers.to_owned();
                        self.is_followed = user_data.is_followed.to_owned();
                        self.current_user_uuid = user_data.uuid.to_owned();
                        self.profile = Some(user_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
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
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        match (&self.self_profile, &self.profile) {
            (Some(self_data), _) => self.self_user_card(self_data),
            (_, Some(user_data)) => self.other_user_card(user_data),
            _ => html!{<ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error) />},
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
                                {self.view_card()}
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
                <div class="column is-flex">
                    { self.show_profile_action() }
                    <div class="card-relate-data" style="flex:1;" >
                        {match self.profile_tab {
                            ProfileTab::Certificates => self.view_certificates(self_data.certificates.clone()),
                            ProfileTab::Components => self.view_components(&self_data.uuid),
                            ProfileTab::Companies => self.view_companies(&self_data.uuid),
                            ProfileTab::FavoriteComponents => self.view_favorite_components(None),
                            ProfileTab::FavoriteCompanies => self.view_favorite_companies(None),
                            ProfileTab::FavoriteStandards => self.view_favorite_standards(),
                            ProfileTab::FavoriteUsers => html!{<CatalogUsers arguments = UsersQueryArg::set_favorite() />},
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
                        <div class="card">
                            <div class="card-content">
                                {self.view_card()}
                                <div class="content">
                                    { self.view_user_info(
                                        user_data.description.as_str(),
                                        user_data.position.as_str(),
                                        user_data.region.region.as_str(),
                                        user_data.program.name.as_str(),
                                    ) }
                                </div>
                            </div>
                            {self.other_user_relate_object(user_data)}
                        </div>
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
            <div class="columns is-mobile">
                <div class="column is-flex">
                  { self.show_profile_action() }
                  <div class="card-relate-data" style="flex:1;">
                      {match self.profile_tab {
                          ProfileTab::Certificates => self.view_certificates(user_data.certificates.clone()),
                          ProfileTab::Components => self.view_components(&user_data.uuid),
                          ProfileTab::Companies => self.view_companies(&user_data.uuid),
                          ProfileTab::FavoriteComponents => self.view_favorite_components(Some(user_data.uuid.clone())),
                          ProfileTab::FavoriteCompanies => self.view_favorite_companies(Some(user_data.uuid.clone())),
                          _ => html!{},
                      }}
                  </div>
              </div>
          </div>
        </div>}
    }

    fn view_card(&self) -> Html {
        let UserDataCard {
            image_file,
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

        html!{<div class="media">
            <div class="media-left">
              <figure class="image is-48x48">
                // <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                <img
                    src={image_file.clone()} alt="Favicon profile"
                    loading="lazy"
                />
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
                {match self.show_full_user_info {
                    true => html!{
                        <div>
                            <span>{ get_value_field(&30) }</span>
                            <span>{updated_at}</span>
                        </div>
                    },
                    false => html!{},
                }}
                { self.show_profile_followers() }
            </div>
        </div>}
    }

    fn show_profile_followers(&self) -> Html {
        html! {<>
            // for self user data not show button "following"
            {match &self.profile {
                Some(_) => self.show_favorite_btn(),
                None => html!{
                    <div>
                        <span>{ get_value_field(&31) }</span>
                        <span>{self.abbr_number()}</span>
                    </div>
                },
            }}
        </>}
    }

    fn show_favorite_btn(&self) -> Html {
        let (class_fav, onclick_following) = match self.is_followed {
            true => ("fas fa-bookmark", self.link.callback(|_| Msg::UnFollow)),
            false => ("far fa-bookmark", self.link.callback(|_| Msg::Follow)),
        };

        html!{
            <button
                id="following-button"
                class="button"
                onclick=onclick_following >
              <span class="icon is-small">
                <i class={class_fav}></i>
              </span>
              <span>{self.abbr_number()}</span>
            </button>
        }
    }

    fn cb_generator(&self, cb: ProfileTab) -> Callback<MouseEvent> {
        self.link.callback(move |_| Msg::ChangeTab(cb.clone()))
    }

    fn check_extend(&self, tab: &ProfileTab) -> bool {
        if self.extend_tab.is_some() {
            self.extend_tab.clone().unwrap() == tab.clone()
        } else {
            false
        }
    }

    fn show_profile_action(&self) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            MenuItem {
                title: get_value_field(&32).to_string(),
                action: self.cb_generator(ProfileTab::Certificates),
                count: self.get_number_of_items(&ProfileTab::Certificates),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-certificate")],
                is_active: self.profile_tab == ProfileTab::Certificates,
                is_extend: self.check_extend(&ProfileTab::Certificates),
            },
            MenuItem {
                title: get_value_field(&33).to_string(),
                action: self.cb_generator(ProfileTab::Components),
                count: self.get_number_of_items(&ProfileTab::Components),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cogs")],
                is_active: self.profile_tab == ProfileTab::Components,
                is_extend: self.check_extend(&ProfileTab::Components),
            },
            MenuItem {
                title: get_value_field(&34).to_string(),
                action: self.cb_generator(ProfileTab::FavoriteComponents),
                count: self.get_number_of_items(&ProfileTab::FavoriteComponents),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cogs"), classes!("fas", "fa-bookmark")],
                is_active: self.profile_tab == ProfileTab::FavoriteComponents,
                is_extend: self.check_extend(&ProfileTab::FavoriteComponents),
            },
            // company MenuItem
            MenuItem {
                title: get_value_field(&35).to_string(),
                action: self.cb_generator(ProfileTab::Companies),
                count: self.get_number_of_items(&ProfileTab::Companies),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-building")],
                is_active: self.profile_tab == ProfileTab::Companies,
                is_extend: self.check_extend(&ProfileTab::Companies),
            },
            // company fav MenuItem
            MenuItem {
                title: get_value_field(&36).to_string(),
                action: self.cb_generator(ProfileTab::FavoriteCompanies),
                count: self.get_number_of_items(&ProfileTab::FavoriteCompanies),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-building"), classes!("fas", "fa-bookmark")],
                is_active: self.profile_tab == ProfileTab::FavoriteCompanies,
                is_extend: self.check_extend(&ProfileTab::FavoriteCompanies),
            },
            // standards MenuItem
            MenuItem {
                title: get_value_field(&37).to_string(),
                action: self.cb_generator(ProfileTab::FavoriteStandards),
                count: self.get_number_of_items(&ProfileTab::FavoriteStandards),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cube"), classes!("fas", "fa-bookmark")],
                is_active: self.profile_tab == ProfileTab::FavoriteStandards,
                is_extend: self.check_extend(&ProfileTab::FavoriteStandards),
            },
            // user fav MenuItem
            MenuItem {
                title: get_value_field(&38).to_string(),
                action: self.cb_generator(ProfileTab::FavoriteUsers),
                count: self.get_number_of_items(&ProfileTab::FavoriteUsers),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-user"), classes!("fas", "fa-bookmark")],
                is_active: self.profile_tab == ProfileTab::FavoriteUsers,
                is_extend: self.check_extend(&ProfileTab::FavoriteUsers),
            },
        ];

        html! {
            <div style="margin-right: 18px;z-index: 1;" >
                <SideMenu menu_arr={menu_arr} />
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
        let onclick_change_full_show =
            self.link.callback(|_| Msg::ShowFullUserInfo);

        match self.show_full_user_info {
            true => html! {<>
                <div class="columns">
                    <div class="column">
                        <div id="description" class="content">
                          {description}
                        </div>
                    </div>
                    <div class="column">
                        <div id="position">
                            <span class="icon is-small"><i class="fas fa-briefcase" /></span>
                            <span>{ get_value_field(&39) }</span>
                            <span class="overflow-title has-text-weight-bold">{position}</span>
                        </div>
                        // <br/>
                        <div id="region">
                            <span class="icon is-small"><i class="fas fa-map-marker-alt" /></span>
                            <span>{ get_value_field(&40) }</span>
                            <span class="overflow-title has-text-weight-bold">{region}</span>
                        </div>
                        // <br/>
                        <div id="program">
                            <span class="icon is-small"><i class="fas fa-drafting-compass" /></span>
                            <span>{ get_value_field(&41) }</span>
                            <span class="overflow-title has-text-weight-bold">{program}</span>
                        </div>
                    </div>
                </div>
                <button class="button is-ghost" onclick={onclick_change_full_show}>
                    <span>{ get_value_field(&42) }</span>
                </button>
            </>},
            false => html!{
                <button class="button is-ghost" onclick={onclick_change_full_show}>
                    <span>{ get_value_field(&43) }</span>
                </button>
            },
        }
    }

    fn view_certificates(
        &self,
        certificates: Vec<UserCertificate>
    ) -> Html {
        html!{
          <div class="profileBox" >
            <UserCertificatesCard
                  user_uuid = self.current_user_uuid.clone()
                  certificates = certificates
                  show_cert_btn = false
                  download_btn = false
                  manage_btn = false
             />
          </div>
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

    fn get_number_of_items(&self, tab: &ProfileTab ) -> usize {
        match &self.self_profile {
            Some( ref res) =>  match tab {
              ProfileTab::Certificates => res.certificates.len(),
              ProfileTab::Components => res.components_count,
              ProfileTab::FavoriteComponents => res.fav_components_count,
              ProfileTab::Companies => res.companies_count,
              ProfileTab::FavoriteCompanies => res.fav_companies_count,
              ProfileTab::FavoriteStandards => res.fav_standards_count,
              ProfileTab::FavoriteUsers => res.fav_users_count,
            } ,
            None => 0,
        }
    }
}
