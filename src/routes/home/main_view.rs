use yew::{html, Component, ComponentLink, Html, ShouldRender};

// use crate::services::is_authenticated;

/// Main content and welcom page
pub struct MainView {
    // props: Props,
    // link: ComponentLink<Self>,
}

impl Component for MainView {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MainView { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{}
    }
}
