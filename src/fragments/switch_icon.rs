use yew::{html, Callback, MouseEvent, Html, Classes};

pub fn res_btn (class: Classes, onclick:Callback<MouseEvent>, span_style: String) -> Html {
  html!{
    <button class="button" onclick={onclick} >
      <span class="icon is-small" style={span_style} >
        <i class={class}></i>
      </span>
    </button>
  }
}
