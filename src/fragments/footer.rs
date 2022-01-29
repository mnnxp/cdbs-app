use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Footer {
    link: ComponentLink<Self>,
    show_terms: bool,
    show_about: bool,
}

pub enum Msg {
    ShowTerms,
    ShowAbout,
}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Footer {
            link,
            show_terms: false,
            show_about: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowTerms => self.show_terms = !self.show_terms,
            Msg::ShowAbout => self.show_about = !self.show_about,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let current_info = "© CADBase 2022";

        let onclick_show_terms = self.link.callback(|_| Msg::ShowTerms);

        let onclick_show_about = self.link.callback(|_| Msg::ShowAbout);

        html!{
            <footer class="footer">
                {self.modal_terms()}
                {self.modal_about_us()}
                <div class="columns">
                    // left footer
                    <div class="column">
                        <a class=vec!("social-network") href="mailto:info@cadbase.rs" title="Email">
                            <i class=vec!("fas", "fa-lg", "fa-envelope")></i>
                        </a>
                    </div>
                    // 1 center footer
                    <div class="column">
                        <a onclick=onclick_show_terms >
                            { "Terms" }
                        </a>
                    </div>
                    // 2 center footer
                    <div class="column">
                        <a onclick=onclick_show_about >
                            { "About us" }
                        </a>
                    </div>
                    // right footer
                    <div class="column">
                        <h4>{current_info}</h4>
                    </div>
                </div>
            </footer>
        }
    }
}

impl Footer {
    fn modal_terms(&self) -> Html {
        let onclick_show_terms = self.link.callback(|_| Msg::ShowTerms);

        let class_modal = match &self.show_terms {
            true => "modal is-active",
            false => "modal",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick=onclick_show_terms.clone() />
              <div class="modal-card">
                <header class="modal-card-head">
                  <p class="modal-card-title">{"Terms CADBase"}</p>
                  <button class="delete" aria-label="close" onclick=onclick_show_terms />
                </header>
                <section class="modal-card-body">
                    <div class="content">
                        <h1>{"Thank you for using CADBase!"}</h1>
                        <p>{"We're really happy you're here. Please read this Terms of Service agreement carefully before accessing or using CADBase. Because it is such an important contract between us and our users, we need to make it as clear as possible, but now we don't have time and invest to make full legal terms, then we just past this part:"}</p>
                        <blockquote>
                            {"THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE."}
                        </blockquote>
                        <p>{"Please let us know if there are any errors or problems, also if you need help."}</p>
                        <p>{"Contact with us: "} <a href="mailto:support@cadbase.rs" title="Email">{"support@cadbase.rs"}</a></p>
                    </div>
                </section>
              </div>
            </div>
        }
    }

    fn modal_about_us(&self) -> Html {
        let onclick_show_about = self.link.callback(|_| Msg::ShowAbout);

        let class_modal = match &self.show_about {
            true => "modal is-active",
            false => "modal",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick=onclick_show_about.clone() />
              <div class="modal-card">
                <header class="modal-card-head">
                  <p class="modal-card-title">{"About us"}</p>
                  <button class="delete" aria-label="close" onclick=onclick_show_about />
                </header>
                <section class="modal-card-body">
                    <div class="content">
                        <h1>{"Make For Into Future"}</h1>
                        <p>{"CADBase - is home to engineers, architechs and creators, when like sharing ideas, concepts and  experience other engineers, for  manufactures, that ready share drawings and documentations."}</p>
                        <p>{"The motivation behind the creation of the CADBase platform was a protest against corporations creating an artificial threshold of entry into the manufacturing sector for start-up manufacturing companies in some contries (this is one of the reasons why we don't open source project now)."}</p>
                        <p>{"We are happy any good people, no matter where you live, if you want to get and share knowledge then we try to help you with it."}</p>
                        <p>{"Project was founded start development in 2018 by Ivan Nosovsky (some guy from Russia), after some year in project accept participial Yulia Gerasimova (since 2019) and Xia Tianhao (夏添豪, since 2021)."}</p>
                        <p>{"Launched MVP took place in 2022 and we hope that you like this platform."}</p>
                        <h4>{"We hope that you like this platform."}</h4>
                    </div>
                </section>
              </div>
            </div>
        }
    }
}
