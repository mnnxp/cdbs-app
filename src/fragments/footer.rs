use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Footer {
    link: ComponentLink<Self>,
    show_about: bool,
}

pub enum Msg {
    ShowAbout,
}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Footer {
            link,
            show_about: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowAbout => self.show_about = !self.show_about,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let current_info = "© CADBase 2022";

        // let onclick_show_about = self.link.callback(|_| Msg::ShowAbout);

        html!{
            <footer class="footer">
                {self.modal_about_us()}
                <div class="columns">
                    // left footer
                    <div class="column">
                        <a class=vec!("social-network") href="mailto:info@cadbase.rs" title="Email">
                            <i class=vec!("fas", "fa-lg", "fa-envelope")></i>
                        </a>
                        // <h4>
                        //     { "Let’s stay in touch!" }
                        // </h4>
                        // <div class="field">
                        //   <p class=vec!("control", "has-icons-left", "has-icons-right")>
                        //     <input class="input" type="email" placeholder="mail@gmail.com"/>
                        //     <span class=vec!("icon", "is-small", "is-left")>
                        //       <i class="fas fa-envelope"></i>
                        //     </span>
                        //     <span class=vec!("icon", "is-small", "is-right")>
                        //         <svg width="52" height="40" viewBox="0 0 60 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                        //             <rect width="52" height="40" rx="8" fill="white"/>
                        //             <path d="M27.7148 15L32.3067 20L27.7148 25" stroke="#EDF1FB" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        //             <path d="M19 20.1057L31.8009 20.0262" stroke="#EDF1FB" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        //         </svg>
                        //     </span>
                        //   </p>
                        // </div>
                        // <div class="buttons">
                        //   <a class="button">
                        //     <strong> { "Eng" } </strong>
                        //   </a>
                        //   <a class="button is-light">
                        //      { "Rus" }
                        //   </a>
                        // </div>
                        // <a class=vec!("social-network") href="#" title="YouTube">
                        //     <i class=vec!("fas", "fa-lg", "fa-youtube")></i>
                        // </a>
                        // <a class=vec!("social-network") href="#" title="Twitter">
                        //     <i class=vec!("fas", "fa-lg", "fa-twitter")></i>
                        // </a>
                        // <a class=vec!("social-network") href="#" title="WhatsApp">
                        //     <i class=vec!("fas", "fa-lg", "fa-whatsapp")></i>
                        // </a>
                    </div>
                    // center footer
                    <div class="column">
                        // <h4> { "About us" } </h4>
                        // <ul>
                        //     <li><a href="#">{ "Licence" }</a></li>
                        //     <li><a href="#">{ "Corporate Info" }</a></li>
                        //     <li><a href="#">{ "Corporate Responsibility" }</a></li>
                        //     <li><a href="#">{ "Our Models" }</a></li>
                        //     <li><a href="#">{ "Information" }</a></li>
                        // </ul>
                    </div>
                    // right footer
                    <div class="column">
                        <h4>{current_info}</h4>
                        // <span style="margin-left: 1rem;">
                        //     <a class="button is-ghost" onclick=onclick_show_about >{ "About us" }</a>
                        // </span>
                        // <h4> { "Customer services" } </h4>
                        // <ul>
                        //     <li><a href="#">{ "FAQ" }</a></li>
                        //     <li><a href="#">{ "Contact" }</a></li>
                        //     <li><a href="#">{ "Your history" }</a></li>
                        //     <li><a href="#">{ "News" }</a></li>
                        //     <li><a href="#">{ "Payment" }</a></li>
                        // </ul>
                    </div>
                </div>
            </footer>
        }
    }
}

impl Footer {
    fn modal_about_us(&self) -> Html {
        let onclick_show_about = self.link.callback(|_| Msg::ShowAbout);

        let class_modal = match &self.show_about {
            true => "modal is-active",
            false => "modal",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_show_about.clone() />
              <div class="modal-content">
                {"Привет! Как дела?"}
              </div>
              <button class="modal-close is-large" aria-label="close"></button>
            </div>
        }
    }
}
