use crate::components::{author_card::AuthorCard, progress_delay::ProgressDelay};
use rand::{distributions, Rng};
use yew::prelude::*;

/// Amount of milliseconds to wait before showing the next set of about_us.
const CAROUSEL_DELAY_MS: u64 = 15000;

pub enum Msg {
    NextAuthors,
}

pub struct AboutUs {
    link: ComponentLink<Self>,
    seeds: Vec<u64>,
}
impl Component for AboutUs {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            seeds: random_author_seeds(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NextAuthors => {
                self.seeds = random_author_seeds();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let about_us = self.seeds.iter().map(|seed| {
            html! {
                <div class="tile is-parent">
                    <div class="tile is-child">
                        <AuthorCard seed=seed />
                    </div>
                </div>
            }
        });

        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "About Us" }</h1>
                            <h2 class="subtitle">
                                { "Meet the definitely real people behind your favourite Yew content" }
                            </h2>
                        </div>
                    </div>
                </section>
                <p class="section py-0">
                    { "It wouldn't be fair " }
                    <i>{ "(or possible :P)" }</i>
                    {" to list each and every author in alphabetical order."}
                    <br />
                    { "So instead we chose to put more focus on the individuals by introducing you to two people at a time" }
                </p>
                <div class="section">
                    <div class="tile is-ancestor">
                        { for about_us }
                    </div>
                    <ProgressDelay duration_ms=CAROUSEL_DELAY_MS on_complete=self.link.callback(|_| Msg::NextAuthors) />
                </div>
            </div>
        }
    }
}

fn random_author_seeds() -> Vec<u64> {
    rand::thread_rng()
        .sample_iter(distributions::Standard)
        .take(2)
        .collect()
}
