use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::services::is_authenticated;

pub struct Banner {}

pub enum Msg {}

impl Component for Banner {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Banner {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if is_authenticated() {
            html! {}
        } else {
            html! {
                <div class="tile is-parent container">
                    <div class="tile is-parent">
                        <div class="tile is-child box">
                            <svg width="40" height="40" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <rect width="19.0909" height="19.1045" rx="6" fill="#B3BACE"/>
                                <rect y="20.8955" width="19.0909" height="19.1045" rx="6" fill="#B3BACE"/>
                                <rect x="21.9082" y="1" width="17.0909" height="17.1045" rx="5" stroke="#353E48" stroke-width="2"/>
                                <rect x="20.9082" y="20.8955" width="19.0909" height="19.1045" rx="6" fill="#B3BACE"/>
                            </svg>

                            <div class="content">
                                {r#"
                                A wide range of
                                different components
                                & component catalogs
                                "#}
                            </div>
                        </div>
                    </div>

                    <div class="tile is-parent">
                        <div class="tile is-child box">
                            <svg width="40" height="40" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <rect width="40" height="40" rx="6" fill="#B3BACE"/>
                                <path d="M31 20.5489C31 26.8617 25.8507 32 19.5245 32C13.1983 32 8 26.8617 8 20.5489L8.04904 20.0106C8.19616 17.1234 9.37313 14.4809 11.3838 12.4255C12.0213 11.7894 13.1983 11.7894 13.8358 12.4255C14.1791 12.7681 14.3262 13.2085 14.3262 13.6489C14.3262 14.0894 14.1301 14.5298 13.8358 14.8723C11.678 17.0255 10.9424 20.1574 11.9232 23.0447L12.0213 23.3383C13.1983 26.4702 16.1898 28.5255 19.5245 28.5255C23.9382 28.5255 27.5181 24.9532 27.5181 20.5489C27.5181 18.4447 26.6844 16.4383 25.2132 14.9702V16.7809C25.2132 17.7596 24.4286 18.4936 23.4968 18.4936C22.565 18.4936 21.7804 17.7106 21.7804 16.7809V10.7128C21.7804 9.73404 22.565 9 23.4968 9H29.2836C30.2644 9 31 9.78298 31 10.7128C31 11.6915 30.2154 12.4255 29.2836 12.4255H27.6652C29.774 14.5787 31 17.466 31 20.5489Z" fill="#353E48" stroke="#B2BBCC" stroke-width="1.5"/>
                            </svg>

                            <div class="content">
                                {r#"
                                    Constantly updated
                                    component base
                                "#}
                            </div>
                        </div>
                    </div>

                    <div class="tile is-parent">
                        <div class="tile is-child box">
                            <svg width="40" height="40" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <rect y="5" width="40" height="30" rx="6" fill="#B3BACE"/>
                                <rect width="40" height="2" rx="1" fill="#353E48"/>
                                <rect x="12" y="19" width="16" height="2" rx="1" fill="#353E48"/>
                                <rect x="21" y="12" width="16" height="2" rx="1" transform="rotate(90 21 12)" fill="#353E48"/>
                                <rect y="38" width="40" height="2" rx="1" fill="#353E48"/>
                            </svg>

                            <div class="content">
                                {r#"
                                    Ability to add custom
                                    components
                                "#}
                            </div>
                        </div>
                    </div>

                    <div class="tile is-parent">
                        <div class="tile is-child box">
                            <svg width="40" height="40" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <rect width="40" height="40" rx="6" fill="#B3BACE"/>
                                <path d="M24 6L30 12.5L24 19" stroke="#353E48" stroke-width="1.7" stroke-linecap="round"/>
                                <path d="M16 34L10 27.5L16 21" stroke="#353E48" stroke-width="1.7" stroke-linecap="round"/>
                                <path d="M36.5 27.5H10" stroke="#353E48" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M3.5 12.5H30" stroke="#353E48" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>

                            <div class="content">
                                {r#"
                                    Buying and selling
                                    components
                                "#}
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
