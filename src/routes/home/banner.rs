use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::services::get_value_field;

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
        html!{
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
                            { get_value_field(&4) }
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 40 40">
                            <rect fill="#b3bace" height="40" rx="5.991" width="40"/>
                            <text space="preserve" x="11.88" y="30.044" font-size="10.885" font-family="URW Gothic" fill="#353e48">
                            <tspan x="11.88" y="30.044" font-weight="normal">{"API"}</tspan>
                            </text>
                            <path d="M26.855 15.576l-.572.309a13.24 13.24 0 0 0-1.703 1.21l-1.225-1.402c.425-.393.883-.75 1.369-1.066.386-.252.789-.477 1.207-.671l.506-.235c-.751-1.345-1.9-2.424-3.29-3.088a8 8 0 0 0-4.227-.736c-1.437.131-2.808.666-3.953 1.544a7.41 7.41 0 0 0-2.708 4.591l-.121.635-.633.111c-.567.094-1.123.244-1.66.448a5.48 5.48 0 0 0-1.265.673 4.19 4.19 0 0 0-.759.709c-.643.792-.984 1.787-.962 2.807a5.09 5.09 0 0 0 .959 2.918c.231.319.5.608.802.861.314.261.665.472 1.043.628.414.167.848.281 1.291.339h.44a9.05 9.05 0 0 0-.066 1.119l.03.759h-.44-.116c-.633-.076-1.254-.235-1.845-.473-.554-.226-1.071-.536-1.531-.919a6.55 6.55 0 0 1-1.114-1.197c-.847-1.175-1.305-2.586-1.311-4.034a6.15 6.15 0 0 1 1.397-4.004c.323-.381.692-.722 1.098-1.012a7.44 7.44 0 0 1 1.678-.898c.454-.177.92-.322 1.395-.433.48-2.022 1.633-3.82 3.27-5.1 1.424-1.091 3.128-1.757 4.915-1.921a9.87 9.87 0 0 1 5.198.911c1.881.899 3.406 2.402 4.333 4.27h0c.401-.063.806-.093 1.212-.091 1.762-.013 3.446.723 4.634 2.025a7.38 7.38 0 0 1 .759.987 8.35 8.35 0 0 1 1.207 4.535c-.018 1.594-.45 3.186-1.329 4.356-.589.768-1.332 1.403-2.182 1.865-.889.483-1.84.843-2.827 1.068l.025-.693a9.16 9.16 0 0 0-.081-1.215 8.73 8.73 0 0 0 1.972-.782c.629-.34 1.18-.806 1.62-1.369.633-.843.944-2.025.957-3.257.038-1.232-.276-2.449-.906-3.508a5.06 5.06 0 0 0-.569-.731 4.39 4.39 0 0 0-3.29-1.422c-.909.005-1.806.204-2.632.582z" fill="#353e48"/>
                        </svg>

                        <div class="content">
                            { get_value_field(&5) }
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
                            { get_value_field(&6) }
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
                            { get_value_field(&7) }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
