use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::services::get_value_field;

pub struct ListEmpty {}

impl Component for ListEmpty {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ListEmpty {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
          <div class="column">
            {self.no_result_image()}
            <p class="is-size-3 has-text-centered">{get_value_field(&324)}</p>
          </div>
        }
    }
}

impl ListEmpty {
    fn no_result_image(&self) -> Html {
      html!{
        <svg width="63" height="80" viewBox="0 0 63 80" style="display: block;margin-left: auto;margin-right: auto;" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <path id="0i2sqtp3ia" d="M.27.595h17.303v36.457H.269z"/>
            </defs>
            <g fill="none" fill-rule="evenodd">
                <path d="M48.305 5.01h-20.31C12.645 5.01.2 17.452.2 32.803V63.73c0 .374.304.678.678.678h26.44a.678.678 0 0 0 .677-.678V36.313c0-1.938 1.571-3.509 3.51-3.509h16.8V5.01z" fill="#EDF1FB"/>
                <path d="M59.871 18.907c0 9.505-3.064 17.211-6.844 17.211-3.78 0-6.843-7.706-6.843-17.211 0-9.506 3.063-17.212 6.843-17.212S59.87 9.4 59.87 18.907" fill="#8E96A0"/>
                <path d="M50.897 3.846c-2.925 1.772-5.08 7.842-5.08 15.06 0 7.218 2.155 13.288 5.08 15.062 3.537-3.452 5.876-9.464 5.876-15.061 0-5.597-1.822-11.435-5.876-15.061" fill="#CED5E8"/>
                <g transform="translate(35.454 .083)">
                    <path d="M17.573 2.583V.594H6.735c-1.04 0-2.017.518-2.58 1.392C1.523 6.075.27 12.648.27 18.823c0 7.909 1.656 13.517 3.993 16.903a3.076 3.076 0 0 0 2.533 1.324l10.778.002v-1.968h-3.055l-1.643-2.778c.172 0 .296.046.33.064-1.014-.552-3.444-5.234-3.444-13.547s2.43-12.995 3.443-13.547a.786.786 0 0 1-.329.064l1.869-2.757h2.829z" fill="#B3BACE" mask="url(#yqc8s76fub)"/>
                </g>
                <path d="M54.322 14.106c.27 2.021-.419 3.782-1.54 3.931-1.12.15-2.248-1.368-2.518-3.389-.27-2.021.42-3.781 1.54-3.931 1.12-.15 2.248 1.368 2.518 3.389" fill="#8E96A0"/>
                <path d="M52.067 16.253a1.2 1.2 0 1 1-2.4-.001 1.2 1.2 0 0 1 2.4.001" fill="#FFF"/>
                <path d="M.878 79.856h26.44a.677.677 0 0 0 .677-.678v-2.575a.677.677 0 0 0-.678-.678H.878a.677.677 0 0 0-.678.678v2.575c0 .375.303.678.678.678M.878 71.995h26.44a.677.677 0 0 0 .677-.678v-2.575a.677.677 0 0 0-.678-.678H.878a.677.677 0 0 0-.678.678v2.575c0 .375.303.678.678.678" fill="#EDF1FB"/>
                <path d="M.878 56.271a.69.69 0 0 0-.678.702v6.732a.69.69 0 0 0 .678.702h26.44a.69.69 0 0 0 .677-.702v-6.732a.69.69 0 0 0-.678-.702H.878z" fill-opacity=".5" fill="#B3BACE"/>
                <path d="M53.027 2.712c-2.756 0-5.827 6.651-5.827 16.195S50.271 35.1 53.027 35.1s5.827-6.65 5.827-16.194c0-9.544-3.07-16.195-5.827-16.195m0 34.423c-4.481 0-7.86-7.836-7.86-18.228 0-10.392 3.379-18.23 7.86-18.23s7.861 7.838 7.861 18.23-3.38 18.228-7.86 18.228" fill="#EDF1FB"/>
            </g>
        </svg>
      }
    }
}
