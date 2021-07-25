use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::types::SlimUser;

pub struct Header {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">
                        <RouterAnchor<AppRoute> route=AppRoute::Home>
                            <svg width="66" height="91" viewBox="0 0 66 91" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M0 0H66V82C66 86.9706 61.9706 91 57 91H9C4.02944 91 0 86.9706 0 82V0Z" fill="#F3F6FF"/>
                                <path d="M32.6316 39.0202C32.1719 39.0202 31.7988 38.6471 31.7988 38.1875V33.8327C31.7988 33.3731 32.1719 33 32.6316 33C33.0912 33 33.4643 33.3731 33.4643 33.8327V38.1875C33.4643 38.6471 33.0912 39.0202 32.6316 39.0202Z" fill="#1872F0"/>
                                <path d="M43.4308 60.0237H21.8327C21.3731 60.0237 21 59.6507 21 59.191C21 58.7313 21.3731 58.3582 21.8327 58.3582H43.4308C43.8905 58.3582 44.2636 58.7313 44.2636 59.191C44.2636 59.6507 43.8905 60.0237 43.4308 60.0237Z" fill="#B2BBCC"/>
                                <path d="M22.1751 69.9055C22.0819 69.9055 21.9864 69.89 21.892 69.8567C21.459 69.699 21.2358 69.2227 21.3912 68.7896L29.581 46.1178C29.7376 45.6859 30.2117 45.4594 30.648 45.6171C31.0811 45.7736 31.3042 46.2511 31.1488 46.683L22.959 69.3559C22.8358 69.6946 22.516 69.9055 22.1751 69.9055Z" fill="#B2BBCC"/>
                                <path d="M43.09 69.9055C42.7491 69.9055 42.4293 69.6946 42.3061 69.3559L34.1163 46.683C33.9597 46.2511 34.184 45.7736 34.617 45.6171C35.049 45.4594 35.5264 45.6859 35.683 46.1178L43.8738 68.7896C44.0293 69.2227 43.8061 69.699 43.3731 69.8567C43.2798 69.89 43.1832 69.9055 43.09 69.9055Z" fill="#B2BBCC"/>
                                <path d="M32.6316 62.2015C32.1719 62.2015 31.7988 61.8284 31.7988 61.3688V57.0141C31.7988 56.5544 32.1719 56.1813 32.6316 56.1813C33.0912 56.1813 33.4643 56.5544 33.4643 57.0141V61.3688C33.4643 61.8284 33.0912 62.2015 32.6316 62.2015Z" fill="#1872F0"/>
                                <path d="M34.9049 50.8456C34.6006 50.8456 34.3064 50.678 34.1609 50.3871C33.9544 49.9763 34.1187 49.4755 34.5307 49.269C35.9874 48.5362 36.8934 47.0827 36.8934 45.4761C36.8934 43.1266 34.9815 41.2147 32.632 41.2147C30.2814 41.2147 28.3706 43.1266 28.3706 45.4761C28.3706 47.0827 29.2755 48.5362 30.7333 49.269C31.1442 49.4755 31.3096 49.9763 31.1031 50.3871C30.8966 50.7979 30.3958 50.9644 29.985 50.7568C27.962 49.7398 26.7051 47.7167 26.7051 45.4761C26.7051 42.2084 29.3632 39.5492 32.632 39.5492C35.8997 39.5492 38.5589 42.2084 38.5589 45.4761C38.5589 47.7167 37.302 49.7398 35.2779 50.7568C35.1591 50.8179 35.0303 50.8456 34.9049 50.8456Z" fill="#2C72F0"/>
                            </svg>
                        </RouterAnchor<AppRoute>>
                    </h1>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-start">
                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="navbar-item">
                            { "Catalog" }
                        </RouterAnchor<AppRoute>>

                        <RouterAnchor<AppRoute> route=AppRoute::Tenders classes="navbar-item">
                            { "Tenders" }
                        </RouterAnchor<AppRoute>>

                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="navbar-item">
                            { "About Us" }
                        </RouterAnchor<AppRoute>>
                    </div>
                    <div class="navbar-end">
                    {
                        if let Some(user_info) = &self.props.current_user {
                            self.logged_in_view(&user_info)
                        } else {
                            self.logged_out_view()
                        }
                    }
                    </div>
                </div>
            </nav>
        }
    }
}

impl Header {
    fn logged_out_view(&self) -> Html {
        html! {
            <div class="buttons">
                 <RouterAnchor<AppRoute> route=AppRoute::Login classes="button">
                  { "Sign in" }
                 </RouterAnchor<AppRoute>>
                 <RouterAnchor<AppRoute> route=AppRoute::Register classes="button">
                  { "Sign up" }
                 </RouterAnchor<AppRoute>>
            </div>
        }
    }

    fn logged_in_view(&self, user_info: &SlimUser) -> Html {
        html! {
            <div class="buttons">
                 <RouterAnchor<AppRoute> route=AppRoute::Settings classes="button">
                  { "Settings" }
                 </RouterAnchor<AppRoute>>
                 <RouterAnchor<AppRoute> route=AppRoute::Profile(user_info.username.clone())  classes="button">
                  { &user_info.username }
                 </RouterAnchor<AppRoute>>
            </div>
        }
    }
}
