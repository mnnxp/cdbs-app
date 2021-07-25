use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::components::article_list::{ArticleList, ArticleListFilter};
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::Profiles;
use crate::types::{ProfileInfo, ProfileInfoWrapper, SlimUser};

/// Profile for an author
pub struct Profile {
    profiles: Profiles,
    profile: Option<ProfileInfo>,
    response: Callback<Result<ProfileInfoWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub uuid: String,
    pub current_user: Option<SlimUser>,
    pub tab: ProfileTab,
    pub username: String
}

#[derive(Clone)]
pub enum Msg {
    Response(Result<ProfileInfoWrapper, Error>),
    Follow,
    UnFollow,
}

#[derive(Clone, PartialEq)]
pub enum ProfileTab {
    ByAuthor,
    FavoritedBy,
}

impl Component for Profile {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Profile {
            profiles: Profiles::new(),
            profile: None,
            response: link.callback(Msg::Response),
            task: None,
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(
                self.profiles
                    .get(self.props.uuid.clone(), self.response.clone()),
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Follow => {
                self.task = Some(
                    self.profiles
                        .follow(self.props.uuid.clone(), self.response.clone()),
                );
            }
            Msg::UnFollow => {
                self.task = Some(
                    self.profiles
                        .unfollow(self.props.uuid.clone(), self.response.clone()),
                );
            }
            Msg::Response(Ok(profile_info)) => {
                self.profile = Some(profile_info.profile);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let is_current_user = if let Some(current_user) = &self.props.current_user {
            current_user.uuid == self.props.uuid
        } else {
            false
        };

        if let Some(profile) = &self.profile {
            html! {
                <div class="profile-page">
                    <div class="user-info">
                        <div class="container">
                            <div class="row">
                                <div class="col-xs-12 col-md-10 offset-md-1">
                                    <img src={ profile.image.clone() } class="user-img" alt={ profile.uuid.clone() } />
                                    <h4>{ &profile.uuid }</h4>
                                    <p>
                                        {
                                            if let Some(bio) = &profile.bio {
                                                html! { bio }
                                            } else {
                                                html! { }
                                        }}
                                    </p>
                                    {
                                        if is_current_user {
                                            self.view_edit_profile_settings()
                                        } else {
                                            self.view_follow_user_button()
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="container">
                        <div class="row">
                            <div class="col-xs-12 col-md-10 offset-md-1">
                                <div class="articles-toggle">
                                    { self.view_tabs() }
                                </div>
                                {
                                    match self.props.tab {
                                        ProfileTab::ByAuthor => {
                                            html! { <ArticleList filter=ArticleListFilter::ByAuthor(profile.uuid.clone()) /> }
                                        }
                                        ProfileTab::FavoritedBy => {
                                            html! { <ArticleList filter=ArticleListFilter::FavoritedBy(profile.uuid.clone()) /> }
                                        }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}

impl Profile {
    fn view_edit_profile_settings(&self) -> Html {
        html! {
            <RouterAnchor<AppRoute>
                route=AppRoute::Settings
                classes="btn btn-sm btn-outline-secondary action-btn">
                { "Edit Profile Settings" }
            </RouterAnchor<AppRoute>>
        }
    }

    fn view_follow_user_button(&self) -> Html {
        if let Some(profile) = &self.profile {
            let class = if profile.following {
                "btn btn-sm action-btn btn-secondary"
            } else {
                "btn btn-sm action-btn btn-outline-secondary"
            };

            let onclick = if profile.following {
                self.link.callback(|_| Msg::UnFollow)
            } else {
                self.link.callback(|_| Msg::Follow)
            };

            let text = if profile.following {
                "Unfollow"
            } else {
                "Follow"
            };

            html! {
                <button
                    class=class
                    onclick=onclick >
                    { text }
                </button>
            }
        } else {
            html! {}
        }
    }

    fn view_tabs(&self) -> Html {
        if let Some(profile) = &self.profile {
            let classes = if self.props.tab == ProfileTab::ByAuthor {
                ("nav-link active", "nav-link")
            } else {
                ("nav-link", "nav-link active")
            };

            html! {
                <ul class="nav nav-pills outline-active">
                    <li class="nav-item">
                        <RouterAnchor<AppRoute>
                            classes=classes.0
                            route=AppRoute::Profile(profile.uuid.clone())>
                            { "My Articles" }
                        </RouterAnchor<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <RouterAnchor<AppRoute>
                            classes=classes.1
                            route=AppRoute::ProfileFavorites(profile.uuid.clone())>
                            { "Favorited Articles" }
                        </RouterAnchor<AppRoute>>
                    </li>
                </ul>
            }
        } else {
            html! {}
        }
    }
}
