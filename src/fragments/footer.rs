use yew::{html, Component, ComponentLink, Html, ShouldRender, ChangeData};
use crate::services::{get_lang, get_server_location_id, LocaleKey, set_server_locations};
use crate::fragments::buttons::simple_link;

pub struct Footer {
    link: ComponentLink<Self>,
    server_location_id: usize,
}

pub enum Msg {
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
            server_location_id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
        let current_info = LocaleKey::Copyright.get_value();
        let version_number = "0.3.2";
        let (base_url, docs_url) = match get_lang().as_deref() {
            Some("zh") => ("https://cadbase.org/zh/", String::from("https://docs.cadbase.org/")),
            Some("ru") => ("https://cadbase.ru/ru/", String::from("https://docs.cadbase.ru/")),
            _ => ("https://cadbase.rs/en/", String::from("https://docs.cadbase.rs/")),
        };
        let about_url = format!("{}overviews/platform-for-creators-and-engineers/", base_url);
        let terms_url = format!("{}terms/", base_url);
        let privacy_notice_url = format!("{}privacy-notice/", base_url);
        let news_url = format!("{}#overviews", base_url);
        let glossary_url = format!("{}glossary/", base_url);

        html!{
            <footer class="footer">
                <div class="columns">
                    // left footer
                    <div class="column">
                        <div class="social-links">
                            <a class={"social-network"} href="mailto:info@cadbase.rs" title="Email" style="margin-right: 0.1rem;">
                                <i class={vec!("fas", "fa-lg", "fa-envelope")}></i>
                            </a>
                            <a class={"social-network"} href="https://www.youtube.com/channel/UC-dHiTHBGV88ScxFKSC3srw" title="Youtube" rel="noreferrer">
                                <i class={vec!("fab", "fa-lg", "fa-youtube")}></i>
                            </a>
                            <a class={"social-network"} href="https://gitlab.com/cadbase" title="CADBase Source Codes" rel="noreferrer">
                                <i class={vec!("fab", "fa-lg", "fa-gitlab")}></i>
                            </a>
                        </div>
                        {self.selector_server_location()}
                    </div>
                    // 1 center footer
                    <div class="column">
                        {simple_link(news_url, LocaleKey::Overviews.get_value())}
                        <br/>
                        {simple_link(docs_url, LocaleKey::ApiReference.get_value())}
                        <br/>
                        {simple_link(glossary_url, LocaleKey::Glossary.get_value())}
                    </div>
                    // 2 center footer
                    <div class="column">
                        {simple_link(about_url, LocaleKey::WhatIs.get_value())}
                        <br/>
                        {simple_link(terms_url, LocaleKey::Terms.get_value())}
                        <br/>
                        {simple_link(privacy_notice_url, LocaleKey::PrivacyNotice.get_value())}
                    </div>
                    // right footer
                    <div class="column">
                        <h4>{current_info}</h4>
                        <p class="help">{LocaleKey::Version.get_value()}{version_number}</p>
                    </div>
                </div>
            </footer>
        }
    }
}

impl Footer {
    fn selector_server_location(&self) -> Html {
        let oninput_server_location =
            self.link.callback(|ev: ChangeData| Msg::UpdateServerLocationId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let server_location = [
            (1, LocaleKey::Netherlands.get_value()),
            (2, LocaleKey::Russia.get_value()),
            (3, LocaleKey::China.get_value()),
            (4, LocaleKey::CustomServer.get_value()),
        ];

        html!{
            <div class="server-selector">
                <span class="is-size-7 has-text-weight-semibold mr-2">{LocaleKey::ServerLocation.get_value()}</span>
                <div class="select is-small is-narrow">
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
        }
    }
}
