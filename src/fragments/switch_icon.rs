// use yew::prelude::*;
use yew::{
  html, Callback, MouseEvent, Html, Classes,
};
// use yew_router::prelude::*;

// #[derive(PartialEq, Properties, Clone)]
// pub struct BtnItem{
//   pub class: String,
//   pub clickEvent: yew::Callback<MouseEvent>
// }

// #[derive(PartialEq, Properties, Clone)]
// pub struct Props {
//     /// Callback when user is logged in successfully
//     pub callback: BtnItem,
// }

// pub struct SwitchIcon {
//     // `ComponentLink` is like a reference to a component.
//     // It can be used to send messages to the component
//     // link: ComponentLink<Self>
//     props: Props,
// }

// impl Component for SwitchIcon {
//     type Message = ();
//     type Properties = Props;

//     fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         Self { props }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         true
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         // Should only return "true" if new properties are different to
//         // previously received properties.
//         // This component has no properties so we will always return "false".
//         false
//     }

//     fn view(&self) -> Html {
//         html!{
//           <button class="button" onclick={self.props.callback.clickEvent.clone()} >
//             <span class="icon is-small">
//               <i class=classes!(self.props.callback.class.clone())></i>
//             </span>
//           </button>
//         }
//     }
// }

pub fn res_btn (class: Classes, onclick:Callback<MouseEvent>, span_style: String) -> Html {
  html!{
    <button class="button" onclick=onclick >
      <span class="icon is-small" style=span_style >
        <i class=class></i>
      </span>
    </button>
  }
}
