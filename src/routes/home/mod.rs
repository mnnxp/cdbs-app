mod banner;
mod main_view;
mod search_page;

pub use search_page::SearchPage;

use yew::{classes, html, Bridge, Bridged, Component, ComponentLink, Html, InputData, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use crate::routes::AppRoute;
use crate::services::{get_logged_user, get_value_field, set_history_search};

use banner::Banner;
// use main_view::MainView;

/// Home page with an article list and a tag list.
pub struct Home {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    search_text: String,
}

pub enum Msg {
    InputSearch(String),
    OpenSearchPage,
    Ignore,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            search_text: String::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(user) = get_logged_user() {
                // route to profile page if user already logged
                self.router_agent.send(ChangeRoute(AppRoute::Profile(user.username).into()));
            };
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputSearch(value) => self.search_text = value,
            Msg::OpenSearchPage => {
                // save the text to use it on SearchPage
                set_history_search(Some(self.search_text.clone()));
                self.router_agent.send(ChangeRoute(AppRoute::SearchPage.into()))
            },
            Msg::Ignore => (),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <div class={classes!("tile", "is-ancestor", "is-vertical")}>
                <div class="tile is-child hero">
                    <div class={classes!("hero-body", "container", "pb-0")}>
                        <h1 class="title is-1" title="CADBase" style="margin-bottom: 0; position: initial;">
                            <svg position="initial" viewBox="0 0 145 35" fill="none" xmlns="http://www.w3.org/2000/svg">
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
                        </h1>
                        <div class="media">
                            <div class="media-left">
                                <h2 class="subtitle"> {get_value_field(&1)} </h2>
                            </div>
                            <div class="media-content">
                                <h2 class="subtitle" style="text-align: center"> {get_value_field(&2)} </h2>
                            </div>
                            <div class="media-right">
                                <h2 class="subtitle"> {get_value_field(&3)} </h2>
                            </div>
                        </div>
                    </div>
                </div>
                {self.search_block()}
                <Banner />
            </div>
        }
    }
}

impl Home {
    fn search_block(&self) -> Html {
        let oninput_search_text = self.link.callback(|ev: InputData| Msg::InputSearch(ev.value));
        let onclick_open_search = self.link.callback(|_| Msg::OpenSearchPage);
        html!{
            <div class="tile is-parent container">
                <div class="tile is-parent">
                    <form class="field is-relative has-addons column p-0 m-0">
                        <div style="width: 100%;" class="control has-icons-left has-icons-right">
                            <input id={"input-search-block"} placeholder={get_value_field(&351)} style="width: 100%;" class="input" oninput={oninput_search_text} />
                            <span class="icon is-small is-left">
                                <i class="fas fa-search fa-xs"></i>
                            </span>
                        </div>
                        <div class="control">
                            <button class="button is-info search-button" type="submit" onclick={onclick_open_search}>{get_value_field(&349)}</button>
                        </div>
                    </form>
                </div>
            </div>
        }
    }
}