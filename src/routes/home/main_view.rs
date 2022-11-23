use yew::{Component, Context, html, Html};

// use crate::services::is_authenticated;

/// Main content and welcom page
pub struct MainView {}

impl Component for MainView {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainView {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{}
    }
}
