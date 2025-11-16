use yew::{html, Component, ComponentLink, Html, ShouldRender, ChangeData};
use crate::services::content_adapter::Markdownable;
use crate::services::{get_lang, get_server_location_id, get_value_field, set_server_locations};

pub struct Footer {
    link: ComponentLink<Self>,
    show_terms: bool,
    show_about: bool,
    server_location_id: usize,
}

pub enum Msg {
    ShowTerms,
    ShowAbout,
    UpdateServerLocationId(String),
}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut server_location_id = get_server_location_id();
        if server_location_id == 0 {
            set_server_locations(None);
            server_location_id = get_server_location_id();
        }
        Footer {
            link,
            show_terms: false,
            show_about: false,
            server_location_id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowTerms => self.show_terms = !self.show_terms,
            Msg::ShowAbout => self.show_about = !self.show_about,
            Msg::UpdateServerLocationId(value) => {
                let location_id = value.parse::<usize>().unwrap_or(1);
                set_server_locations(Some(location_id));
                self.server_location_id = get_server_location_id();
            },
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let current_info = get_value_field(&258);
        let version_number = "0.3.1";
        let onclick_show_terms = self.link.callback(|_| Msg::ShowTerms);
        let onclick_show_about = self.link.callback(|_| Msg::ShowAbout);
        let (base_url, docs_url) = match get_lang().as_deref() {
            // Some("zh") => ("https://cadbase.org/zh/", "https://docs.cadbase.org/zh/"),
            Some("ru") => ("https://cadbase.ru/ru/", "https://docs.cadbase.ru/ru/"),
            _ => ("https://cadbase.rs/en/", "https://docs.cadbase.rs/en/"),
        };
        let privacy_notice_url = format!("{}privacy-notice.html", base_url, );
        let self_hosted_url = format!("{}self-hosted.html", base_url, );
        let news_url = format!("{}#overviews", base_url, );

        html!{
            <footer class="footer">
                {self.modal_terms()}
                {self.modal_about_us()}
                <div class="columns">
                    // left footer
                    <div class="column">
                        <div class="tags mb-0">
                            <div class="tag is-white is-medium">
                                <a class={vec!("social-network")} href="mailto:info@cadbase.rs" title="Email"  style="margin-right: 0.1rem;">
                                    <i class={vec!("fas", "fa-lg", "fa-envelope")}></i>
                                </a>
                            </div>
                            <div class="tag is-white is-medium">
                                <a class={vec!("social-network")} href="https://www.youtube.com/channel/UC-dHiTHBGV88ScxFKSC3srw" title="Youtube Channel" rel="noreferrer">
                                    <i class={vec!("fab", "fa-lg", "fa-youtube")}></i>
                                </a>
                            </div>
                            <div class="tag is-white is-medium">
                                <a class={vec!("social-network")} href="https://gitlab.com/cadbase" title="CADBase Source Codes" rel="noreferrer">
                                    <i class={vec!("fab", "fa-lg", "fa-brands", "fa-gitlab")}></i>
                                </a>
                            </div>
                        </div>
                        {self.selector_server_location()}
                    </div>
                    // 1 center footer
                    <div class="column">
                        <a href={news_url} >
                            {get_value_field(&256)}
                        </a>
                        <br/>
                        <a href={docs_url} >
                            {get_value_field(&12)}
                        </a>
                        <br/>
                        <a href={self_hosted_url} >
                            {get_value_field(&259)}
                        </a>
                    </div>
                    // 2 center footer
                    <div class="column">
                        <a onclick={onclick_show_about} >
                            {get_value_field(&11)}
                        </a>
                        <br/>
                        <a onclick={onclick_show_terms} >
                            {get_value_field(&10)}
                        </a>
                        <br/>
                        <a href={privacy_notice_url} >
                            {get_value_field(&269)}
                        </a>
                    </div>
                    // right footer
                    <div class="column">
                        <h4>{current_info}</h4>
                        <p class="help">{get_value_field(&257)}{version_number}</p>
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
              <div class="modal-background" onclick={onclick_show_terms.clone()} />
              <div class="modal-card">
                <header class="modal-card-head">
                  <p class="modal-card-title">{get_value_field(&248)}</p> // Terms CADBase
                  <button class="delete" aria-label="close" onclick={onclick_show_terms} />
                </header>
                <section class="modal-card-body">
                    <div class="content">
                        <h1>{get_value_field(&249)}</h1> // Thank you for using CADBase
                        <p>{get_value_field(&250)}</p>
                        <blockquote>{get_value_field(&251)}</blockquote>
                        <p>{get_value_field(&252)}</p>
                        <p>{get_value_field(&253)} <a href="mailto:support@cadbase.rs" title="Email">{"support@cadbase.rs"}</a></p>
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
              <div class="modal-background" onclick={onclick_show_about.clone()} />
              <div class="modal-card">
                <header class="modal-card-head">
                  <p class="modal-card-title">{get_value_field(&254)}</p>
                  <button class="delete" aria-label="close" onclick={onclick_show_about} />
                </header>
                <section class="modal-card-body">
                    <div class="content">
                        <h1>{get_value_field(&11)}</h1>
                        <p>{get_value_field(&255).to_markdown()}</p>
                        <p>{get_value_field(&253)} <a href="mailto:info@cadbase.rs" title="Email">{"info@cadbase.rs"}</a></p>
                        <h4>{get_value_field(&260)}</h4>
                    </div>
                </section>
              </div>
            </div>
        }
    }

    fn selector_server_location(&self) -> Html {
        let oninput_server_location =
            self.link.callback(|ev: ChangeData| Msg::UpdateServerLocationId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let server_location = [
            (1, get_value_field(&415)),
            (2, get_value_field(&416)),
            (3, get_value_field(&417)),
            (4, get_value_field(&418)),
        ];

        html!{
            <div class="server-selector ml-3">
                <div class="is-flex is-align-items-center">
                    <span class="is-size-7 has-text-weight-semibold mr-2">{get_value_field(&414)}</span>
                    <div class="select is-small">
                    <select
                        id="select_server_location"
                        select={self.server_location_id.to_string()}
                        onchange={oninput_server_location}
                        >
                        { for server_location.iter().map(|(id, name)|
                            html!{
                                <option value={id.to_string()} selected={id == &self.server_location_id} >
                                    {&name}
                                </option>
                            }
                        )}
                    </select>
                    </div>
                </div>
            </div>
        }
    }
}
