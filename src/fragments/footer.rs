use yew::{Component, Context, html, html::Scope, Html, classes};
use crate::services::{get_value_field, set_lang, get_lang};

pub struct Footer {
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

    fn create(_ctx: &Context<Self>) -> Self {
        let current_lang = get_lang().map(|lang|
            match lang.as_str() {
                "ru" => 2,
                _ => 1,
            }
        ).unwrap_or(1);

        Footer {
            show_terms: false,
            show_about: false,
            current_lang,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_info = "© CADBase 2022";
        let onclick_lang_en = ctx.link().callback(|_| Msg::ChangeLanguage(1));
        let onclick_lang_ru = ctx.link().callback(|_| Msg::ChangeLanguage(2));
        let onclick_show_terms = ctx.link().callback(|_| Msg::ShowTerms);
        let onclick_show_about = ctx.link().callback(|_| Msg::ShowAbout);
        let (button_en, button_ru) = match self.current_lang {
            2 => (classes!("button"), classes!("button", "is-info")),
            _ => (classes!("button", "is-info"), classes!("button")),
        };

        html!{
            <footer class="footer">
                {self.modal_terms(ctx.link())}
                {self.modal_about_us(ctx.link())}
                <div class="columns">
                    // left footer
                    <div class="column">
                        <div class="tags are-large">
                            <a class={vec!("social-network")} href="mailto:info@cadbase.rs" title="Email"  style="margin-right: 0.1rem;">
                                <i class={vec!("fas", "fa-lg", "fa-envelope")}></i>
                            </a>
                            <a class={vec!("social-network")} href="https://www.youtube.com/channel/UC-dHiTHBGV88ScxFKSC3srw" title="Youtube Channel" rel="noreferrer">
                                <i class={vec!("fab", "fa-lg", "fa-youtube")}></i>
                            </a>
                        </div>
                        <div class="buttons">
                            <a class={button_en} onclick={onclick_lang_en}>
                                { get_value_field(&8) }
                            </a>
                            <a class={button_ru} onclick={onclick_lang_ru}>
                                { get_value_field(&9) }
                            </a>
                        </div>
                    </div>
                    // 1 center footer
                    <div class="column">
                    </div>
                    // 2 center footer
                    <div class="column">
                        <a onclick={onclick_show_about} >
                            { get_value_field(&11) }
                        </a>
                        <br/>
                        <a onclick={onclick_show_terms} >
                            { get_value_field(&10) }
                        </a>
                        <br/>
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
    fn modal_terms(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_show_terms = link.callback(|_| Msg::ShowTerms);
        let class_modal = match &self.show_terms {
            true => "modal is-active",
            false => "modal",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_show_terms.clone()} />
              <div class="modal-card">
                <header class="modal-card-head">
                  <p class="modal-card-title">{ get_value_field(&248) }</p> // Terms CADBase
                  <button class="delete" aria-label="close" onclick={onclick_show_terms} />
                </header>
                <section class="modal-card-body">
                    <div class="content">
                        <h1>{ get_value_field(&249) }</h1> // Thank you for using CADBase
                        <p>{ get_value_field(&250) }</p>
                        <blockquote>{ get_value_field(&251) }</blockquote>
                        <p>{ get_value_field(&252) }</p>
                        <p>{ get_value_field(&253) } <a href="mailto:support@cadbase.rs" title="Email">{"support@cadbase.rs"}</a></p>
                    </div>
                </section>
              </div>
            </div>
        }
    }

    fn modal_about_us(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_show_about = link.callback(|_| Msg::ShowAbout);

        let class_modal = match &self.show_about {
            true => "modal is-active",
            false => "modal",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_show_about.clone()} />
              <div class="modal-card">
                <header class="modal-card-head">
                  <p class="modal-card-title">{ get_value_field(&11) }</p> // About us
                  <button class="delete" aria-label="close" onclick={onclick_show_about} />
                </header>
                <section class="modal-card-body">
                    <div class="content">
                        <h1>{ get_value_field(&254) }</h1>
                        <p>{ get_value_field(&255) }</p>
                        <p>{ get_value_field(&256) }</p>
                        <p>{ get_value_field(&257) }</p>
                        <p>{ get_value_field(&258) }</p>
                        <p>{ get_value_field(&259) }</p>
                        <p>{ get_value_field(&253) } <a href="mailto:info@cadbase.rs" title="Email">{"info@cadbase.rs"}</a></p>
                        <h4>{ get_value_field(&260) }</h4>
                    </div>
                </section>
              </div>
            </div>
        }
    }
}
