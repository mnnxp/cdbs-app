#![recursion_limit="1024"]
use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};

mod components;
mod content;
mod generator;
mod pages;
use pages::{
    author::Author, author_list::AuthorList, home::Home, page_not_found::PageNotFound, post::Post,
    post_list::PostList, about_us::AboutUs,
};
mod switch;
use switch::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    link: ComponentLink<Self>,
    navbar_active: bool,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            navbar_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_nav() }

                <main>
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                        })
                    />
                </main>
                <footer class="footer">
                    // left footer
                    <div class="content has-text-left">
                        <h4>
                            { "Let’s stay in touch!" }
                        </h4>
                        <div class="field">
                          <p class="control has-icons-left has-icons-right">
                            <input class="input" type="email" placeholder="mail@gmail.com"/>
                            <span class="icon is-small is-left">
                              <i class="fas fa-envelope"></i>
                            </span>
                            <span class="icon is-small is-right">
                              <i class="fas fa-check"></i>
                            </span>
                          </p>
                        </div>
                        <div class="buttons">
                          <a class="button">
                            <strong> { "Eng" } </strong>
                          </a>
                          <a class="button is-light">
                             { "Rus" }
                          </a>
                        </div>
                        <a class=vec!("social-network") href="mailto:info@cadbase.ru" title="Email">
                            <i class=vec!("fas", "fa-lg", "fa-envelope")></i>
                        </a>
                        <a class=vec!("social-network") href="#" title="YouTube">
                            <i class=vec!("fas", "fa-lg", "fa-youtube")></i>
                        </a>
                        <a class=vec!("social-network") href="#" title="Twitter">
                            <i class=vec!("fas", "fa-lg", "fa-twitter")></i>
                        </a>
                        <a class=vec!("social-network") href="#" title="WhatsApp">
                            <i class=vec!("fas", "fa-lg", "fa-whatsapp")></i>
                        </a>
                    </div>
                    // center footer
                    <div class="content has-text-centered">
                        <h4> { "About us" } </h4>
                        <ul>
                            <li><a href="#">{ "Licence" }</a></li>
                            <li><a href="#">{ "Corporate Info" }</a></li>
                            <li><a href="#">{ "Corporate Responsibility" }</a></li>
                            <li><a href="#">{ "Our Models" }</a></li>
                            <li><a href="#">{ "Information" }</a></li>
                        </ul>
                    </div>
                    // right footer
                    <div class="content has-text-right">
                        <h4> { "Customer services" } </h4>
                        <ul>
                            <li><a href="#">{ "FAQ" }</a></li>
                            <li><a href="#">{ "Contact" }</a></li>
                            <li><a href="#">{ "Your history" }</a></li>
                            <li><a href="#">{ "News" }</a></li>
                            <li><a href="#">{ "Payment" }</a></li>
                        </ul>
                    </div>
                </footer>
            </>
        }
    }
}
impl Model {
    fn view_nav(&self) -> Html {
        let Self {
            ref link,
            navbar_active,
            ..
        } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">
                        <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <img src="logo_min.svg" width="112" height="28"/>
                        </AppAnchor>
                    </h1>

                    <a role="button"
                        class=vec!("navbar-burger", "burger", active_class)
                        aria-label="menu" aria-expanded="false"
                        onclick=link.callback(|_| Msg::ToggleNavbar)
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>
                <div class=vec!("navbar-menu", active_class)>
                    <div class="navbar-start">
                        <AppAnchor classes="navbar-item" route=AppRoute::PostList>
                            { "Catalog" }
                        </AppAnchor>

                        <AppAnchor classes="navbar-item" route=AppRoute::AuthorList>
                            { "Tenders" }
                        </AppAnchor>

                        <AppAnchor classes="navbar-item" route=AppRoute::AboutUs>
                            { "About Us" }
                        </AppAnchor>
                    </div>
                    <div class="navbar-end">
                      <div class="navbar-item">
                        <div class="buttons">
                          <a class="button">
                            <strong> { "Sign up" } </strong>
                          </a>
                          <a class="button is-light">
                             { "Log in" }
                          </a>
                        </div>
                      </div>
                    </div>
                </div>
            </nav>
        }
    }

    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Post(id) => {
                html! { <Post seed=id /> }
            }
            AppRoute::PostListPage(page) => {
                html! { <PostList page=page.max(1) /> }
            }
            AppRoute::PostList => {
                html! { <PostList page=1 /> }
            }
            AppRoute::Author(id) => {
                html! { <Author seed=id /> }
            }
            AppRoute::AuthorList => {
                html! { <AuthorList /> }
            }
            AppRoute::AboutUs => {
                html! { <AboutUs /> }
            }
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <PageNotFound route=route /> }
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
