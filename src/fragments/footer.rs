use yew::{html, Component, ComponentLink, Html, ShouldRender};
// use yew_router::prelude::*;

// use crate::routes::AppRoute;

pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <footer class="footer columns">
                // left footer
                <div class="column">
                    <h4>
                        { "Let’s stay in touch!" }
                    </h4>
                    <div class="field">
                      <p class=vec!("control", "has-icons-left", "has-icons-right")>
                        <input class="input" type="email" placeholder="mail@gmail.com"/>
                        <span class=vec!("icon", "is-small", "is-left")>
                          <i class="fas fa-envelope"></i>
                        </span>
                        <span class=vec!("icon", "is-small", "is-right")>
                            <svg width="52" height="40" viewBox="0 0 60 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <rect width="52" height="40" rx="8" fill="white"/>
                                <path d="M27.7148 15L32.3067 20L27.7148 25" stroke="#EDF1FB" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M19 20.1057L31.8009 20.0262" stroke="#EDF1FB" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
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
                <div class="column">
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
                <div class="column">
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
        }
    }
}
