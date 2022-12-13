use yew::{Component, Callback, Context, html, html::Scope, Html, Properties, classes};
use yew_router::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen_futures::spawn_local;
use serde_json::Value;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::routes::AppRoute::Login;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::fragments::company::CatalogCompanies;
use crate::fragments::component::CatalogComponents;
use crate::fragments::side_menu::{MenuItem, SideMenu};
use crate::fragments::standard::CatalogStandards;
use crate::fragments::user::{CatalogUsers, UserCertificatesCard};
use crate::services::{url_decode, get_logged_user, get_value_field};
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

/// Profile user with relate data
pub struct Profile {
    error: Option<Error>,
    self_profile: Option<SelfUserInfo>,
    profile: Option<UserInfo>,
    current_user_uuid: UUID,
    current_username: String,
    subscribers: usize,
    is_followed: bool,
    profile_tab: ProfileTab,
    extend_tab: Option<ProfileTab>,
    show_full_user_info: bool,
}

#[derive(Properties, Clone, Debug, PartialEq)]
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
    GetSelfProfileResult(String),
    GetUserProfileResult(String),
    ChangeTab(ProfileTab),
    ShowFullUserInfo,
    ChangeRoute,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Profile {
            error: None,
            self_profile: None,
            profile: None,
            current_user_uuid: String::new(),
            current_username: String::new(),
            subscribers: 0,
            is_followed: false,
            profile_tab: ProfileTab::Certificates,
            extend_tab: Some(ProfileTab::Certificates),
            show_full_user_info: true,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let logged_username = match get_logged_user() {
            Some(cu) => cu.username,
            None => {
                // route to login page if not found token
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Login);
                String::new()
            },
        };
        // get and decode target user from route
        let target_username = url_decode(
            ctx.link().location().unwrap().path().trim_start_matches("/profile/")
        );
        debug!("target_username {:?}", target_username);
        // get flag changing current profile in route
        let not_matches_username = target_username != self.current_username;
        debug!("self.current_username {:?}", self.current_username);
        // check get self data
        let get_self = logged_username == target_username;
        debug!("get_self {:?}", get_self);
        if first_render || not_matches_username {
            // clear old data
            self.error = None;
            self.self_profile = None;
            self.profile = None;
            // update current_username for checking change profile in route
            self.current_username = target_username.to_string();
            ctx.link().send_message(Msg::RequestProfileData(get_self))
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestProfileData(get_self) => {
                let target_username = self.current_username.clone();

                spawn_local(async move {
                    match get_self {
                        true => {
                            let res =
                                make_query(GetSelfData::build_query(get_self_data::Variables)).await.unwrap();
                            link.send_message(Msg::GetSelfProfileResult(res));
                        }
                        false => {
                            let ipt_get_user_arg = get_user_data::IptGetUserArg {
                                user_uuid: None,
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
                let link = ctx.link().clone();
                let user_uuid = self.profile.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(AddUserFav::build_query(
                        add_user_fav::Variables { user_uuid }
                    )).await.unwrap();

                    link.send_message(Msg::AddFollow(res));
                })
            },
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
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UnFollow => {
                let link = ctx.link().clone();
                let user_uuid = self.profile.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(DeleteUserFav::build_query(
                        delete_user_fav::Variables { user_uuid }
                    )).await.unwrap();

                    link.send_message(Msg::DelFollow(res));
                })
            },
            Msg::DelFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(
                            res_value.get("deleteUserFav").unwrap().clone()
                        ).unwrap();

                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetSelfProfileResult(res) => {
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
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUserProfileResult(res) => {
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
                    },
                    true => self.error = Some(get_error(&data)),
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
            Msg::ChangeRoute => self.rendered(ctx, false),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let same_props = match (&old_props.current_user, &ctx.props().current_user) {
            (Some(old), Some(cur)) => old.uuid == cur.uuid,
            (None, None) => true,
            _ => false,
        };
        if  same_props && self.current_username ==
              url_decode(ctx.link().location().unwrap().path().trim_start_matches("/profile/")) {
            false
        } else {
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);
        let callback_change = ctx.link().callback(|_| Msg::ChangeRoute);

        match (&self.self_profile, &self.profile) {
            (Some(self_data), _) => self.self_user_card(ctx.link(), self_data, callback_change),
            (_, Some(user_data)) => self.other_user_card(ctx.link(), user_data),
            _ => html!{<ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error)} />},
        }
    }
}

impl Profile {
    fn self_user_card(
        &self,
        link: &Scope<Self>,
        self_data: &SelfUserInfo,
        callback_change: Callback<bool>,
    ) -> Html {
        html! {
            <div class="profile-page">
                <ListErrors error={self.error.clone()}/>
                <div class="container page">
                    <div class="row">
                        <div class="card">
                            <div class="card-content">
                                {self.view_card(link)}
                                <div class="content">
                                    { self.view_user_info(
                                        link,
                                        self_data.description.as_str(),
                                        self_data.position.as_str(),
                                        self_data.region.region.as_str(),
                                        self_data.program.name.as_str(),
                                    ) }
                                </div>
                            </div>
                            {self.self_user_relate_object(link, self_data, callback_change)}
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn self_user_relate_object(
        &self,
        link: &Scope<Self>,
        self_data: &SelfUserInfo,
        callback_change: Callback<bool>,
    ) -> Html {
        html!{<div class="card">
            <div class="columns is-mobile">
                <div class="column is-flex">
                    { self.show_profile_action(link) }
                    <div class="card-relate-data" style="flex:1;" >
                        {match self.profile_tab {
                            ProfileTab::Certificates => self.view_certificates(self_data.certificates.clone()),
                            ProfileTab::Components => self.view_components(&self_data.uuid),
                            ProfileTab::Companies => self.view_companies(&self_data.uuid),
                            ProfileTab::FavoriteComponents => self.view_favorite_components(None),
                            ProfileTab::FavoriteCompanies => self.view_favorite_companies(None),
                            ProfileTab::FavoriteStandards => self.view_favorite_standards(),
                            ProfileTab::FavoriteUsers => self.view_favorite_users(callback_change),
                        }}
                    </div>
                </div>
            </div>
        </div>}
    }

    fn other_user_card(
        &self,
        link: &Scope<Self>,
        user_data: &UserInfo,
    ) -> Html {
        html! {
            <div class="profile-page">
                <ListErrors error={self.error.clone()}/>
                <div class="container page">
                    <div class="row">
                        <div class="card">
                            <div class="card-content">
                                {self.view_card(link)}
                                <div class="content">
                                    { self.view_user_info(
                                        link,
                                        user_data.description.as_str(),
                                        user_data.position.as_str(),
                                        user_data.region.region.as_str(),
                                        user_data.program.name.as_str(),
                                    ) }
                                </div>
                            </div>
                            {self.other_user_relate_object(link, user_data)}
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn other_user_relate_object(
        &self,
        link: &Scope<Self>,
        user_data: &UserInfo,
    ) -> Html {
        html!{<div class="card">
            <div class="columns is-mobile">
                <div class="column is-flex">
                  { self.show_profile_action(link) }
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

    fn view_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
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
                { self.show_profile_followers(link) }
            </div>
        </div>}
    }

    fn show_profile_followers(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        html! {<>
            // for self user data not show button "following"
            {match &self.profile {
                Some(_) => self.show_favorite_btn(link),
                None => html!{
                    <div>
                        <span>{ get_value_field(&31) }</span>
                        <span>{self.subscribers}</span>
                    </div>
                },
            }}
        </>}
    }

    fn show_favorite_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let (class_fav, onclick_following) = match self.is_followed {
            true => ("fas fa-bookmark", link.callback(|_| Msg::UnFollow)),
            false => ("far fa-bookmark", link.callback(|_| Msg::Follow)),
        };

        html!{
            <button
                id="following-button"
                class="button"
                onclick={onclick_following} >
              <span class="icon is-small">
                <i class={class_fav}></i>
              </span>
              <span>{self.subscribers}</span>
            </button>
        }
    }

    fn cb_generator(
        &self,
        link: &Scope<Self>,
        cb: ProfileTab,
    ) -> Callback<MouseEvent> {
        link.callback(move |_| Msg::ChangeTab(cb.clone()))
    }

    fn check_extend(
        &self,
        tab: &ProfileTab,
    ) -> bool {
        if self.extend_tab.is_some() {
            self.extend_tab.clone().unwrap() == tab.clone()
        } else {
            false
        }
    }

    fn show_profile_action(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            MenuItem {
                title: get_value_field(&32).to_string(),
                action: self.cb_generator(link, ProfileTab::Certificates),
                count: self.get_number_of_items(&ProfileTab::Certificates),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-certificate")],
                is_active: self.profile_tab == ProfileTab::Certificates,
                is_extend: self.check_extend(&ProfileTab::Certificates),
            },
            MenuItem {
                title: get_value_field(&33).to_string(),
                action: self.cb_generator(link, ProfileTab::Components),
                count: self.get_number_of_items(&ProfileTab::Components),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cogs")],
                is_active: self.profile_tab == ProfileTab::Components,
                is_extend: self.check_extend(&ProfileTab::Components),
            },
            MenuItem {
                title: get_value_field(&34).to_string(),
                action: self.cb_generator(link, ProfileTab::FavoriteComponents),
                count: self.get_number_of_items(&ProfileTab::FavoriteComponents),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cogs"), classes!("fas", "fa-bookmark")],
                is_active: self.profile_tab == ProfileTab::FavoriteComponents,
                is_extend: self.check_extend(&ProfileTab::FavoriteComponents),
            },
            // company MenuItem
            MenuItem {
                title: get_value_field(&35).to_string(),
                action: self.cb_generator(link, ProfileTab::Companies),
                count: self.get_number_of_items(&ProfileTab::Companies),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-building")],
                is_active: self.profile_tab == ProfileTab::Companies,
                is_extend: self.check_extend(&ProfileTab::Companies),
            },
            // company fav MenuItem
            MenuItem {
                title: get_value_field(&36).to_string(),
                action: self.cb_generator(link, ProfileTab::FavoriteCompanies),
                count: self.get_number_of_items(&ProfileTab::FavoriteCompanies),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-building"), classes!("fas", "fa-bookmark")],
                is_active: self.profile_tab == ProfileTab::FavoriteCompanies,
                is_extend: self.check_extend(&ProfileTab::FavoriteCompanies),
            },
            // standards MenuItem
            MenuItem {
                title: get_value_field(&37).to_string(),
                action: self.cb_generator(link, ProfileTab::FavoriteStandards),
                count: self.get_number_of_items(&ProfileTab::FavoriteStandards),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cube"), classes!("fas", "fa-bookmark")],
                is_active: self.profile_tab == ProfileTab::FavoriteStandards,
                is_extend: self.check_extend(&ProfileTab::FavoriteStandards),
            },
            // user fav MenuItem
            MenuItem {
                title: get_value_field(&38).to_string(),
                action: self.cb_generator(link, ProfileTab::FavoriteUsers),
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
        link: &Scope<Self>,
        description: &str,
        position: &str,
        region: &str,
        program: &str,
    ) -> Html {
        let onclick_change_full_show = link.callback(|_| Msg::ShowFullUserInfo);

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
                  user_uuid = {self.current_user_uuid.clone()}
                  certificates = {certificates}
                  show_cert_btn = {false}
                  download_btn = {false}
                  manage_btn = {false}
             />
          </div>
        }
    }

    fn view_favorite_components(&self, user_uuid: Option<UUID>) -> Html {
        html! {
            <CatalogComponents
                show_create_btn = {self.self_profile.is_some()}
                arguments = {ComponentsQueryArg::set_favorite(user_uuid)}
            />
        }
    }

    fn view_components(&self, user_uuid: &UUID) -> Html {
        html! {
            <CatalogComponents
                show_create_btn = {self.self_profile.is_some()}
                arguments = {ComponentsQueryArg::set_user_uuid(user_uuid)}
            />
        }
    }

    fn view_favorite_companies(&self, user_uuid: Option<UUID>) -> Html {
        html! {
            <CatalogCompanies
                show_create_btn = {self.self_profile.is_some()}
                arguments = {CompaniesQueryArg::set_favorite(user_uuid)}
            />
        }
    }

    fn view_companies(&self, user_uuid: &UUID) -> Html {
        html! {
            <CatalogCompanies
                show_create_btn = {self.self_profile.is_some()}
                arguments = {CompaniesQueryArg::set_user_uuid(user_uuid)}
            />
        }
    }

    fn view_favorite_standards(&self) -> Html {
        html! {
            <CatalogStandards
                show_create_btn = {false}
                arguments = {StandardsQueryArg::set_favorite()}
            />
        }
    }

    fn view_favorite_users(
        &self,
        callback_change: Callback<bool>,
    ) -> Html {
        html!{
            <CatalogUsers
                arguments = {UsersQueryArg::set_favorite()}
                {callback_change}
            />
        }
    }

    fn get_number_of_items(&self, tab: &ProfileTab ) -> usize {
        match &self.self_profile {
            Some(ref res) =>  match tab {
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
