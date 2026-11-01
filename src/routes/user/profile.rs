use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::service::RouteService;
use web_sys::MouseEvent;
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::fragments::responsive::resizer;
use crate::error::Error;
use crate::fragments::{
    buttons::{ft_follow_btn, ft_see_btn},
    company::CatalogCompanies,
    component::CatalogComponents,
    list_errors::ListErrors,
    list_empty::ListEmpty,
    side_menu::{MenuBuilder, MenuItemTemplate},
    supplier_service::CatalogServices,
    standard::CatalogStandards,
    user::CatalogUsers,
    user::UserCertificatesCard,
};
use crate::services::{Counter, get_logged_user, LocaleKey, resp_parsing, title_changer};
use crate::services::content_adapter::{SHORT_TEXT_LEN, DateDisplay};
use crate::types::{
    UserDataCard, CompaniesQueryArg, ComponentsQueryArg, ServicesQueryArg, SelfUserInfo, SlimUser,
    StandardsQueryArg, UserCertificate, UserInfo, UsersQueryArg, UUID
};
use crate::gqls::make_query;
use crate::gqls::user::{
    AddUserFav, add_user_fav,
    DeleteUserFav, delete_user_fav,
    GetSelfData, get_self_data,
    GetUserData, get_user_data,
};
use crate::services::prepare_username;

impl MenuBuilder for Profile {
    type TabType = ProfileTab;

    fn menu_config() -> &'static [MenuItemTemplate<ProfileTab>] {
        use ProfileTab::*;
        &[
            MenuItemTemplate { lk_title: LocaleKey::CertificatesLabel, icon_classes: &[&["fas", "fa-certificate"]], tab: Certificates, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::AllComponents, icon_classes: &[&["fas", "fa-cogs"]], tab: Components, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::FavComponents, icon_classes: &[&["fas", "fa-cogs"], &["fas", "fa-bookmark"]], tab: FavoriteComponents, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::Services, icon_classes: &[&["fas", "fa-ticket-alt"]], tab: Services, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::AllCompanies, icon_classes: &[&["fas", "fa-building"]], tab: Companies, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::FavCompanies, icon_classes: &[&["fas", "fa-building"], &["fas", "fa-bookmark"]], tab: FavoriteCompanies, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::FavStandards, icon_classes: &[&["fas", "fa-book"], &["fas", "fa-bookmark"]], tab: FavoriteStandards, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::FavUsers, icon_classes: &[&["fas", "fa-user"], &["fas", "fa-bookmark"]], tab: FavoriteUsers, custom_class: None },
        ]
    }

    fn is_active(&self, tab: &ProfileTab) -> bool { self.profile_tab == *tab }
    fn get_count(&self, tab: &ProfileTab) -> usize { self.get_number_of_items(tab) }
    fn is_extend(&self, tab: &ProfileTab) -> bool { self.check_extend(tab) }
    fn get_action(&self, tab: &ProfileTab) -> Callback<MouseEvent> { self.cb_generator(tab.clone()) }
}

impl Counter for Profile {
    fn quantity(&self) -> usize {
        self.subscribers
    }
}

/// Profile user with relate data
pub struct Profile {
    error: Option<Error>,
    self_profile: Option<SelfUserInfo>,
    profile: Option<UserInfo>,
    current_user_uuid: UUID,
    current_username: String,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    profile_tab: ProfileTab,
    extend_tab: Option<ProfileTab>,
    show_full_user_info: bool,
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
    Services,
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
            props,
            link,
            subscribers: 0,
            is_followed: false,
            profile_tab: ProfileTab::Components,
            extend_tab: Some(ProfileTab::Components),
            show_full_user_info: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let logged_username = match get_logged_user() {
            Some(cu) => cu.username,
            None => String::new(),
        };

        // get username for request user data
        let route_service: RouteService<()> = RouteService::new();
        // get and decode target user from route
        let target_username = prepare_username(&route_service.get_fragment());

        // get flag changing current profile in route
        let not_matches_username = target_username != self.current_username;
        // debug!("self.current_username {:?}", self.current_username);

        // check get self data
        let get_self = logged_username == target_username;
        // debug!("get_self {:?}", get_self);

        title_changer::set_title(self.current_username.as_str());

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

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
            <div class="profile-page">
                <div class="container is-fluid page">
                    <div class="row">
                        <div class="card">
                            {self.view_card()}
                            {match (&self.self_profile, &self.profile) {
                                (Some(self_data), _) => {self.self_user_relate_object(self_data)},
                                (_, Some(user_data)) => {self.other_user_relate_object(user_data)},
                                _ => html!{},
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </>}
    }
}

impl Profile {
    fn self_user_relate_object(
        &self,
        self_data: &SelfUserInfo,
    ) -> Html {
        html!{<div id="card-list" class="card">
            <div class="columns is-mobile">
                <div class="column is-flex">
                    {self.render_menu()}
                    <div id="card-list-items" class="card-relate-data" style={resizer("card-list", 5)}>
                        {match self.profile_tab {
                            ProfileTab::Certificates => self.view_certificates(self_data.certificates.clone()),
                            ProfileTab::Components => self.view_components(&self_data.uuid),
                            ProfileTab::Companies => self.view_companies(&self_data.uuid),
                            ProfileTab::Services => self.view_self_services(&self_data.uuid),
                            ProfileTab::FavoriteComponents => self.view_favorite_components(None),
                            ProfileTab::FavoriteCompanies => self.view_favorite_companies(None),
                            ProfileTab::FavoriteStandards => self.view_favorite_standards(),
                            ProfileTab::FavoriteUsers => html!{<CatalogUsers arguments={UsersQueryArg::set_favorite()} />},
                        }}
                    </div>
                </div>
            </div>
        </div>}
    }

    fn other_user_relate_object(
        &self,
        user_data: &UserInfo,
    ) -> Html {
        html!{<div id="other-card-list" class="card">
            <div class="columns is-mobile">
                <div class="column is-flex">
                  {self.render_menu()}
                  <div id="other-card-list-items" class="card-relate-data" style={resizer("other-card-list", 5)}>
                      {match self.profile_tab {
                          ProfileTab::Certificates => self.view_certificates(user_data.certificates.clone()),
                          ProfileTab::Components => self.view_components(&user_data.uuid),
                          ProfileTab::Companies => self.view_companies(&user_data.uuid),
                          ProfileTab::Services => self.view_self_services(&user_data.uuid),
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
            description,
            ..
        } = match (&self.self_profile, &self.profile) {
            (_, Some(ref other_data)) => other_data.into(),
            (Some(ref self_data), _) => self_data.into(),
            (None, None) => UserDataCard::default(),
        };

        html!{
            <div class="card-content">
                <div class="columns">
                    <div class="box">
                        <figure class={classes!("container", "image", "is-48x48")}>
                            <img
                                src={image_file.clone()} alt="Favicon profile"
                                loading="lazy"
                            />
                        </figure>
                    </div>
                    <div class="is-flex-grow-1 ml-4">
                        <p class="title is-4">
                            {format!("{} {}", firstname, lastname)}
                        </p>
                        <p class="subtitle is-6">
                            {format!("@{}", username)}
                        </p>
                    </div>
                    {self.view_user_info()}
                </div>
                {self.view_user_description(&description)}
            </div>
        }
    }

    fn show_favorite_btn(&self) -> Html {
        let onclick_following = match self.is_followed {
            true => self.link.callback(|_| Msg::UnFollow),
            false => self.link.callback(|_| Msg::Follow),
        };

        ft_follow_btn(
            onclick_following,
            self.is_followed,
            self.abbr_number(),
        )
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

    fn view_user_info(&self) -> Html {
        let (
            created_at,
            program,
            region,
        ) = match (&self.self_profile, &self.profile) {
            (_, Some(ref p)) => (p.created_at, &p.program, &p.region),
            (Some(ref sp), _) => (sp.created_at, &sp.program, &sp.region),
            (None, None) => return html!{},
        };
        let class_item = "is-size-6 is-align-items-center has-text-grey ml-3";
        let class_icon = "icon is-small mr-1";
        let class_value = "has-text-dark";
        html!{
            <div class="is-flex is-flex-direction-column is-align-items-flex-end">
                <div class={class_item} title={LocaleKey::CreatedAtLabel.get_value()}>
                    <span class={class_icon}><i class="far fa-calendar-plus"></i></span>
                    <span class={class_value}>{created_at.date_to_display()}</span>
                </div>
                {if let Some(sp) = &self.self_profile {
                    html!{
                        <div class={class_item} title={LocaleKey::TypeAccess.get_value()}>
                            {sp.type_access.get_with_icon()}
                        </div>
                    }
                } else {
                    html!{}
                }}
                <div id="program" class={class_item} hidden={program.id == 1} title={LocaleKey::WorkingSoftware.get_value()}>
                    <span class={class_icon}><i class="fas fa-drafting-compass" /></span>
                    <span class={class_value}>{&program.name}</span>
                </div>
                <div id="region" class={class_item} hidden={region.region_id == 8} title={LocaleKey::RegionLabel.get_value()}>
                    <span class={class_icon}><i class="fas fa-map-marker-alt" /></span>
                    <span class={class_value}>{&region.region}</span>
                </div>
                {match &self.profile {
                    Some(_) => html!{
                        <div class="mt-1">{self.show_favorite_btn()}</div>
                    },
                    None => html!{
                        <div class="is-flex is-align-items-center mt-1"
                            title={LocaleKey::Followers.get_value()}>
                            <span class="icon is-small mr-2" style="color: #1872f0;">
                                <i class="fa fa-bookmark"></i>
                            </span>
                            <span class="has-text-dark">{self.abbr_number()}</span>
                        </div>
                    },
                }}
            </div>
        }
    }

    fn view_user_description(&self, description: &str) -> Html {
        if description.is_empty() {
            return html!{};
        }
        let onclick_toggle = self.link.callback(|_| Msg::ShowFullUserInfo);
        if description.chars().count() <= SHORT_TEXT_LEN {
            html!{
                <p class="is-size-6">{description}</p>
            }
        } else if self.show_full_user_info {
            html!{<>
                <p class="is-size-6">{description}</p>
                {ft_see_btn(onclick_toggle, true)}
            </>}
        } else {
            let truncated: String = description.chars().take(SHORT_TEXT_LEN).collect();
            html!{<>
                <p class="is-size-6">{format!("{}…", truncated)}</p>
                {ft_see_btn(onclick_toggle, false)}
            </>}
        }
    }

    fn view_certificates(
        &self,
        certificates: Vec<UserCertificate>
    ) -> Html {
        if certificates.is_empty() {
            html!{<ListEmpty />}
        } else {
            html!{
                <div class="profileBox" >
                    <UserCertificatesCard
                        user_uuid={self.current_user_uuid.clone()}
                        certificates={certificates}
                        show_cert_btn={false}
                        manage_btn={false}
                    />
                </div>
            }
        }
    }

    fn view_self_services(&self, user_uuid: &UUID) -> Html {
        html! {
            <CatalogServices
                arguments={ServicesQueryArg::set_user_uuid(user_uuid)}
            />
        }
    }

    fn view_favorite_components(&self, user_uuid: Option<UUID>) -> Html {
        html! {
            <CatalogComponents
                show_create_btn={self.self_profile.is_some()}
                arguments={ComponentsQueryArg::set_favorite(user_uuid)}
                show_paginate={true}
                total_items={self.get_total_items_opt(&ProfileTab::FavoriteComponents)}
            />
        }
    }

    fn view_components(&self, user_uuid: &UUID) -> Html {
        html! {
            <CatalogComponents
                show_create_btn={self.self_profile.is_some()}
                arguments={ComponentsQueryArg::set_user_uuid(user_uuid)}
                show_paginate={true}
                total_items={self.get_total_items_opt(&ProfileTab::Components)}
            />
        }
    }

    fn view_favorite_companies(&self, user_uuid: Option<UUID>) -> Html {
        html! {
            <CatalogCompanies
                show_create_btn={self.self_profile.is_some()}
                arguments={CompaniesQueryArg::set_favorite(user_uuid)}
            />
        }
    }

    fn view_companies(&self, user_uuid: &UUID) -> Html {
        html! {
            <CatalogCompanies
                show_create_btn={self.self_profile.is_some()}
                arguments={CompaniesQueryArg::set_user_uuid(user_uuid)}
            />
        }
    }

    fn view_favorite_standards(&self) -> Html {
        html! {
            <CatalogStandards
                show_create_btn={false}
                arguments={StandardsQueryArg::set_favorite()}
            />
        }
    }

    fn get_number_of_items(&self, tab: &ProfileTab) -> usize {
        match &self.self_profile {
            Some( ref res) =>  match tab {
              ProfileTab::Certificates => res.certificates.len(),
              ProfileTab::Components => res.components_count,
              ProfileTab::Services => 0,
              ProfileTab::FavoriteComponents => res.fav_components_count,
              ProfileTab::Companies => res.companies_count,
              ProfileTab::FavoriteCompanies => res.fav_companies_count,
              ProfileTab::FavoriteStandards => res.fav_standards_count,
              ProfileTab::FavoriteUsers => res.fav_users_count,
            } ,
            None => 0,
        }
    }

    fn get_total_items_opt(&self, tab: &ProfileTab) -> Option<i64> {
        match self.get_number_of_items(tab) as i64 {
            0 => None,
            x => Some(x),
        }
    }
}
