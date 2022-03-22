use yew::{html, classes, Component, ComponentLink, Html, ShouldRender};
use crate::services::{set_lang, get_lang};
use crate::services::get_value_field;

pub struct Footer {
    link: ComponentLink<Self>,
    show_terms: bool,
    show_about: bool,
    current_lang: u8,
}

pub enum Msg {
    ShowTerms,
    ShowAbout,
    ChangeLanguage(u8),
}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let current_lang = get_lang().map(|lang|
            match lang.as_str() {
                "ru" => 2,
                _ => 1,
            }
        ).unwrap_or(1);

        Footer {
            link,
            show_terms: false,
            show_about: false,
            current_lang,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowTerms => self.show_terms = !self.show_terms,
            Msg::ShowAbout => self.show_about = !self.show_about,
            Msg::ChangeLanguage(lang_id) => {
                match lang_id {
                    2 => set_lang(Some(String::from("ru"))),
                    _ => set_lang(Some(String::from("en"))),
                }
                self.current_lang = lang_id;
            },
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let current_info = "© CADBase 2022";
        let onclick_lang_en = self.link.callback(|_| Msg::ChangeLanguage(1));
        let onclick_lang_ru = self.link.callback(|_| Msg::ChangeLanguage(2));
        let onclick_show_terms = self.link.callback(|_| Msg::ShowTerms);
        let onclick_show_about = self.link.callback(|_| Msg::ShowAbout);

        let (tag_en, tag_ru) = match self.current_lang {
            2 => (classes!("tag"), classes!("tag", "is-info")),
            _ => (classes!("tag", "is-info"), classes!("tag")),
        };

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
                    // left footer 2
                    <div class="column">
                        <div class="tags are-medium">
                            <a onclick=onclick_lang_en>
                                <span class=tag_en>{ get_value_field(&8) }</span>
                            </a>
                            <a onclick=onclick_lang_ru>
                                <span class=tag_ru>{ get_value_field(&9) }</span>
                            </a>
                        </div>
                    </div>
                    // 1 center footer
                    <div class="column">
                        <a onclick=onclick_show_terms >
                            { get_value_field(&10) }
                        </a>
                    </div>
                    // 2 center footer
                    <div class="column">
                        <a onclick=onclick_show_about >
                            { get_value_field(&11) }
                        </a>
                    </div>
                    // 3 center footer
                    <div class="column">
                        <a href="https://docs.cadbase.rs/" >
                            { get_value_field(&12) }
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
                        <p>{"CADBase - is home to engineers, architects and creators, when like sharing ideas, concepts and  experience other engineers, for  manufactures, that ready share drawings and documentations."}</p>
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
