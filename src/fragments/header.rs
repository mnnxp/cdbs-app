use yew::{
  agent::Bridged, html, Bridge, Callback, Component, ComponentLink,
  classes, MouseEvent, Html, Properties, ShouldRender,
};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::*
};
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::services::{set_token, get_logged_user, set_logged_user, logout, get_value_field};
use crate::routes::AppRoute;
use crate::types::SlimUser;

pub struct Header {
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    current_path: String,
    current_user: Option<SlimUser>,
    open_register_page: bool,
    open_login_page: bool,
    open_notifications_page: bool,
    open_home_page: bool,
    is_active: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub callback: Callback<()>,
}

pub enum Msg {
  Logout,
  LogoutComplete(String),
  TriggerMenu,
  CheckPath,
  SetActive(bool),
  Ignore,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Header {
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            current_path: String::new(),
            current_user: None,
            open_register_page: false,
            open_login_page: false,
            open_notifications_page: false,
            open_home_page: false,
            is_active: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // get current user data from storage
            self.current_user = get_logged_user();

            // get current path and setting navbar
            self.link.send_message(Msg::CheckPath);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
          Msg::Logout => {
              let link = self.link.clone();
              spawn_local(async move {
                  let res = logout().await;
                  link.send_message(Msg::LogoutComplete(res))
              })
          },
          Msg::LogoutComplete(res) => {
              debug!("logout: {:?}", res);
              // Clear global token and logged user after logged out
              set_token(None);
              set_logged_user(None);
              self.current_user = None;
              // Notify app to clear current user info
              self.props.callback.emit(());
              // Redirect to home page
              self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
          },
          Msg::TriggerMenu => self.is_active = !self.is_active,
          Msg::SetActive(active) => self.is_active = active,
          Msg::CheckPath => {
              // debug!("route_service: {:?}", route_service.get_fragment().as_str());
              // clear flags
              self.open_register_page = false;
              self.open_login_page = false;
              self.open_notifications_page = false;
              self.open_home_page = false;

              // get current url
              let route_service: RouteService<()> = RouteService::new();
              match route_service.get_fragment().as_str() {
                path if path.len() < 3 => self.open_home_page = true,
                "#/register" => self.open_register_page = true,
                "#/login" => self.open_login_page = true,
                "#/notifications" => self.open_notifications_page = true,
                _ => (),
              }
          }
          Msg::Ignore => {}
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        let route_service: RouteService<()> = RouteService::new();
        let current_path = route_service.get_fragment();
        if self.current_path == current_path {
            false
        } else {
            if self.is_active {
              self.link.send_message(Msg::TriggerMenu)
            }
            // update current path
            self.current_path = current_path;
            // get current user data from storage
            self.current_user = get_logged_user();
            // get current path and setting navbar
            self.link.send_message(Msg::CheckPath);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick : Callback<MouseEvent> = self.link.callback(|_| Msg::Logout);
        let triggrt_menu : Callback<MouseEvent> = self.link.callback(|_| Msg::TriggerMenu);
        let mut logo_classes = classes!("navbar-item", "is-size-3", "header-logo");
        match self.open_home_page {
            true => logo_classes.push("logo-bookmark"),
            false => logo_classes.push("logo-full"),
        }
        let active_menu = if self.is_active { "is-active" } else { "" };

        html!{
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class={logo_classes}>
                        {match &self.current_user {
                            Some(user_info) => html!{
                                <RouterAnchor<AppRoute> route={AppRoute::Profile(user_info.username.clone())}>
                                    {self.show_logo()}
                                </RouterAnchor<AppRoute>>
                            },
                            None => html!{
                                <RouterAnchor<AppRoute> route={AppRoute::Home}>
                                    {self.show_logo()}
                                </RouterAnchor<AppRoute>>
                            },
                        }}
                    </h1>
                    <div role="button" class={classes!("navbar-burger", active_menu)} onclick={triggrt_menu} aria-label="menu" aria-expanded="false">
                      <span aria-hidden="true"></span>
                      <span aria-hidden="true"></span>
                      <span aria-hidden="true"></span>
                    </div>
                </div>
                <div class={classes!("navbar-menu", active_menu)}>
                    <div class="navbar-end">
                        {match &self.current_user {
                            Some(user_info) => self.logged_in_view(&user_info, onclick),
                            None => self.logged_out_view(),
                        }}
                    </div>
                </div>
            </nav>
        }
    }
}

impl Header {
    fn show_logo(&self) -> Html {
        match self.open_home_page {
            true => html!{
                <svg width="66" height="91" viewBox="0 0 66 91" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M0 0H66V82C66 86.9706 61.9706 91 57 91H9C4.02944 91 0 86.9706 0 82V0Z" fill="#F3F6FF"/>
                    <path d="M32.6316 39.0202C32.1719 39.0202 31.7988 38.6471 31.7988 38.1875V33.8327C31.7988 33.3731 32.1719 33 32.6316 33C33.0912 33 33.4643 33.3731 33.4643 33.8327V38.1875C33.4643 38.6471 33.0912 39.0202 32.6316 39.0202Z" fill="#1872F0"/>
                    <path d="M43.4308 60.0237H21.8327C21.3731 60.0237 21 59.6507 21 59.191C21 58.7313 21.3731 58.3582 21.8327 58.3582H43.4308C43.8905 58.3582 44.2636 58.7313 44.2636 59.191C44.2636 59.6507 43.8905 60.0237 43.4308 60.0237Z" fill="#B2BBCC"/>
                    <path d="M22.1751 69.9055C22.0819 69.9055 21.9864 69.89 21.892 69.8567C21.459 69.699 21.2358 69.2227 21.3912 68.7896L29.581 46.1178C29.7376 45.6859 30.2117 45.4594 30.648 45.6171C31.0811 45.7736 31.3042 46.2511 31.1488 46.683L22.959 69.3559C22.8358 69.6946 22.516 69.9055 22.1751 69.9055Z" fill="#B2BBCC"/>
                    <path d="M43.09 69.9055C42.7491 69.9055 42.4293 69.6946 42.3061 69.3559L34.1163 46.683C33.9597 46.2511 34.184 45.7736 34.617 45.6171C35.049 45.4594 35.5264 45.6859 35.683 46.1178L43.8738 68.7896C44.0293 69.2227 43.8061 69.699 43.3731 69.8567C43.2798 69.89 43.1832 69.9055 43.09 69.9055Z" fill="#B2BBCC"/>
                    <path d="M32.6316 62.2015C32.1719 62.2015 31.7988 61.8284 31.7988 61.3688V57.0141C31.7988 56.5544 32.1719 56.1813 32.6316 56.1813C33.0912 56.1813 33.4643 56.5544 33.4643 57.0141V61.3688C33.4643 61.8284 33.0912 62.2015 32.6316 62.2015Z" fill="#1872F0"/>
                    <path d="M34.9049 50.8456C34.6006 50.8456 34.3064 50.678 34.1609 50.3871C33.9544 49.9763 34.1187 49.4755 34.5307 49.269C35.9874 48.5362 36.8934 47.0827 36.8934 45.4761C36.8934 43.1266 34.9815 41.2147 32.632 41.2147C30.2814 41.2147 28.3706 43.1266 28.3706 45.4761C28.3706 47.0827 29.2755 48.5362 30.7333 49.269C31.1442 49.4755 31.3096 49.9763 31.1031 50.3871C30.8966 50.7979 30.3958 50.9644 29.985 50.7568C27.962 49.7398 26.7051 47.7167 26.7051 45.4761C26.7051 42.2084 29.3632 39.5492 32.632 39.5492C35.8997 39.5492 38.5589 42.2084 38.5589 45.4761C38.5589 47.7167 37.302 49.7398 35.2779 50.7568C35.1591 50.8179 35.0303 50.8456 34.9049 50.8456Z" fill="#2C72F0"/>
                </svg>
            },
            false => html!{
                <svg width="145" height="35" xmlns="http://www.w3.org/2000/svg">
                        <g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                        <g id="1-copy" transform="translate(-136.000000, -89.000000)">
                            <g id="LOGO" transform="translate(136.000000, 89.000000)">
                                <path d="M0,18.9473 C0,12.2623 4.989,7.6973 11.413,7.6973 C15.815,7.6973 18.521,9.9803 20.152,12.5893 L16.891,14.2513 C15.848,12.4583 13.761,11.0893 11.413,11.0893 C7.108,11.0893 3.913,14.3823 3.913,18.9473 C3.913,23.5123 7.108,26.8053 11.413,26.8053 C13.761,26.8053 15.848,25.4683 16.891,23.6423 L20.152,25.3053 C18.489,27.9143 15.815,30.1973 11.413,30.1973 C4.989,30.1973 0,25.6323 0,18.9473" id="Fill-1" fill="#1872F0"></path>
                                <path d="M61.3062,18.9473 C61.3062,14.7403 58.6982,11.4143 53.8392,11.4143 L49.5672,11.4143 L49.5672,26.4473 L53.8392,26.4473 C58.5672,26.4473 61.3062,23.0553 61.3062,18.9473 L61.3062,18.9473 Z M45.7522,29.8053 L45.7522,8.0563 L53.8392,8.0563 C60.6212,8.0563 65.2192,12.5563 65.2192,18.9473 C65.2192,25.3383 60.6212,29.8053 53.8392,29.8053 L45.7522,29.8053 Z" id="Fill-3" fill="#1872F0"></path>
                                <path d="M82.6982,23.8057 C82.6982,21.5887 81.1662,19.5017 78.1342,19.5017 L71.1552,19.5017 L71.1552,28.1097 L78.1342,28.1097 C81.0032,28.1097 82.6982,26.4477 82.6982,23.8057 L82.6982,23.8057 Z M82.2742,13.7947 C82.2742,11.5777 80.8072,9.7517 78.0032,9.7517 L71.1552,9.7517 L71.1552,17.8057 L78.0032,17.8057 C80.8072,17.8057 82.2742,16.0447 82.2742,13.7947 L82.2742,13.7947 Z M69.2972,29.8057 L69.2972,8.0557 L78.2312,8.0557 C81.8832,8.0557 84.2312,10.1757 84.2312,13.5997 C84.2312,16.3387 82.3722,18.1977 80.4162,18.5887 C82.7312,18.9477 84.6552,21.3607 84.6552,23.9357 C84.6552,27.5557 82.3072,29.8057 78.4272,29.8057 L69.2972,29.8057 Z" id="Fill-5" fill="#353E48"></path>
                                <path d="M97.1123,10.0454 L91.9933,22.7624 L102.2643,22.7624 L97.1123,10.0454 Z M105.0363,29.8054 L102.8833,24.4584 L91.3733,24.4584 L89.2203,29.8054 L87.0693,29.8054 L95.9703,8.0564 L98.2863,8.0564 L107.1883,29.8054 L105.0363,29.8054 Z" id="Fill-7" fill="#353E48"></path>
                                <path d="M108.6348,26.7408 L109.8418,25.3388 C111.2428,26.9688 113.5268,28.5018 116.6238,28.5018 C121.0908,28.5018 122.3638,26.0228 122.3638,24.1648 C122.3638,17.7738 109.3198,21.0988 109.3198,13.5018 C109.3198,9.9478 112.5158,7.6978 116.4608,7.6978 C119.7218,7.6978 122.1348,8.8388 123.8298,10.7628 L122.5908,12.1328 C121.0268,10.2408 118.8088,9.3928 116.3638,9.3928 C113.4608,9.3928 111.2768,11.0558 111.2768,13.4038 C111.2768,18.9798 124.3198,15.9148 124.3198,24.0338 C124.3198,26.8388 122.4608,30.1968 116.5918,30.1968 C113.0048,30.1968 110.2978,28.7618 108.6348,26.7408" id="Fill-9" fill="#353E48"></path>
                                <polygon id="Fill-11" fill="#353E48" points="128.0586 29.8057 128.0586 8.0557 141.8516 8.0557 141.8516 9.7517 129.9166 9.7517 129.9166 17.8057 141.6226 17.8057 141.6226 19.5017 129.9166 19.5017 129.9166 28.1097 141.8516 28.1097 141.8516 29.8057"></polygon>
                                <path d="M32.3516,5.4224 C31.9376,5.4224 31.6016,5.0864 31.6016,4.6724 L31.6016,0.7504 C31.6016,0.3364 31.9376,0.0004 32.3516,0.0004 C32.7656,0.0004 33.1016,0.3364 33.1016,0.7504 L33.1016,4.6724 C33.1016,5.0864 32.7656,5.4224 32.3516,5.4224" id="Fill-15" fill="#1872F0"></path>
                                <path d="M42.0776,24.3389 L22.6256,24.3389 C22.2116,24.3389 21.8756,24.0029 21.8756,23.5889 C21.8756,23.1749 22.2116,22.8389 22.6256,22.8389 L42.0776,22.8389 C42.4916,22.8389 42.8276,23.1749 42.8276,23.5889 C42.8276,24.0029 42.4916,24.3389 42.0776,24.3389" id="Fill-19" fill="#B2BBCC"></path>
                                <path d="M22.9331,33.2388 C22.8491,33.2388 22.7631,33.2248 22.6781,33.1948 C22.2881,33.0528 22.0871,32.6238 22.2271,32.2338 L29.6031,11.8148 C29.7441,11.4258 30.1711,11.2218 30.5641,11.3638 C30.9541,11.5048 31.1551,11.9348 31.0151,12.3238 L23.6391,32.7438 C23.5281,33.0488 23.2401,33.2388 22.9331,33.2388" id="Fill-21" fill="#B2BBCC"></path>
                                <path d="M41.7705,33.2388 C41.4635,33.2388 41.1755,33.0488 41.0645,32.7438 L33.6885,12.3238 C33.5475,11.9348 33.7495,11.5048 34.1395,11.3638 C34.5285,11.2218 34.9585,11.4258 35.0995,11.8148 L42.4765,32.2338 C42.6165,32.6238 42.4155,33.0528 42.0255,33.1948 C41.9415,33.2248 41.8545,33.2388 41.7705,33.2388" id="Fill-23" fill="#B2BBCC"></path>
                                <path d="M32.3516,26.3003 C31.9376,26.3003 31.6016,25.9643 31.6016,25.5503 L31.6016,21.6283 C31.6016,21.2143 31.9376,20.8783 32.3516,20.8783 C32.7656,20.8783 33.1016,21.2143 33.1016,21.6283 L33.1016,25.5503 C33.1016,25.9643 32.7656,26.3003 32.3516,26.3003" id="Fill-17" fill="#1872F0"></path>
                                <path d="M34.3989,16.0728 C34.1249,16.0728 33.8599,15.9218 33.7289,15.6598 C33.5429,15.2898 33.6909,14.8388 34.0619,14.6528 C35.3739,13.9928 36.1899,12.6838 36.1899,11.2368 C36.1899,9.1208 34.4679,7.3988 32.3519,7.3988 C30.2349,7.3988 28.5139,9.1208 28.5139,11.2368 C28.5139,12.6838 29.3289,13.9928 30.6419,14.6528 C31.0119,14.8388 31.1609,15.2898 30.9749,15.6598 C30.7889,16.0298 30.3379,16.1798 29.9679,15.9928 C28.1459,15.0768 27.0139,13.2548 27.0139,11.2368 C27.0139,8.2938 29.4079,5.8988 32.3519,5.8988 C35.2949,5.8988 37.6899,8.2938 37.6899,11.2368 C37.6899,13.2548 36.5579,15.0768 34.7349,15.9928 C34.6279,16.0478 34.5119,16.0728 34.3989,16.0728" id="Fill-13" fill="#2C72F0"></path>
                            </g>
                        </g>
                    </g>
                </svg>
            },
        }
    }

    fn logged_out_view(&self) -> Html {
        let style_color = "color: #1872f0;";
        let (class_login_btn, class_register_btn) = match (self.open_register_page, self.open_login_page) {
            (true, false) => ("button", "button is-info is-light"),
            (false, true) => ("button is-info is-light", "button"),
            _ => ("button", "button"),
        };

        html!{
          <div class="navbar-item">
            <RouterAnchor<AppRoute> route={AppRoute::Login} classes={class_login_btn}>
                <span class={"icon"}>
                    <i class={"fa fa-user"} aria-hidden={"true"} style={style_color}></i>
                </span>
                <span>{get_value_field(&13)}</span>
            </RouterAnchor<AppRoute>>
            <RouterAnchor<AppRoute> route={AppRoute::Register} classes={class_register_btn}>
                <span class={"icon"}>
                    <i class={"fa fa-user-plus"} aria-hidden={"true"} style={style_color}></i>
                </span>
                <span>{get_value_field(&14)}</span>
            </RouterAnchor<AppRoute>>
          </div>
        }
    }

    fn logged_in_view(
        &self,
        user_info: &SlimUser,
        logout: Callback<MouseEvent>,
    ) -> Html {
        let active_menu = if self.is_active { "is-active" } else { "" };
        let triggrt_menu : Callback<MouseEvent> = self.link.callback(|_| Msg::SetActive(true));
        let out_menu : Callback<MouseEvent> = self.link.callback(|_| Msg::SetActive(false));

        html!{
            <div class="buttons navbar-item">
                 {match self.open_notifications_page {
                     true => html!{
                         <button id="header-notifications"
                            class="button is-active"
                            disabled={true} >
                             <span class="icon is-small" >
                               <i class="far fa-bell"></i>
                             </span>
                         </button>
                     },
                     false => html!{
                         <RouterAnchor<AppRoute> route={AppRoute::Notifications} classes="button navbar-item" >
                             <span class="icon is-small" >
                               <i class="far fa-bell"></i>
                             </span>
                         </RouterAnchor<AppRoute>>
                     },
                 }}
                 <div class={classes!("navbar-item", "has-dropdown", active_menu)} onmouseover={triggrt_menu} onmouseout={out_menu} >
                  <a id="header-menu-button"
                    class="navbar-link"
                    aria-haspopup="true"
                    aria-controls="dropdown-menu">
                      <span>{ &user_info.username }</span>
                  </a>
                  <div class="navbar-dropdown is-boxed is-right" id="dropdown-menu" role="menu">
                    <RouterAnchor<AppRoute> classes="navbar-item" route={AppRoute::Profile(user_info.username.clone())} >
                        {get_value_field(&15)}
                    </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> classes="navbar-item" route={AppRoute::Settings}>
                        {get_value_field(&16)}
                    </RouterAnchor<AppRoute>>
                    <hr class="navbar-divider" />
                    <a class="navbar-item" onclick={logout} >
                        {get_value_field(&17)}
                    </a>
                  </div>
                </div>
            </div>
        }
    }
}
