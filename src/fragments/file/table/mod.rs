pub mod item_showcase;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::services::get_value_field;

pub struct FileHeadersShow {
  props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
  #[prop_or_default]
  pub show_download_btn: bool,
}

impl Component for FileHeadersShow {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
      FileHeadersShow { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
      false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if self.props.show_download_btn == props.show_download_btn {
        false
      } else {
        self.props = props;
        true
      }
    }

    fn view(&self) -> Html {
      html!{
        <thead>
          <tr>
            <th>{"\u{2116}"}</th> // Numero sign
            <th>{get_value_field(&120)}</th> // Filename
            <th><abbr title={get_value_field(&308)}>{get_value_field(&309)}</abbr></th> // Revision
            // <th>{get_value_field(&121)}</th>
            <th>{get_value_field(&122)}</th> // Filesize
            <th><abbr title={get_value_field(&338)}>{get_value_field(&341)}</abbr></th> // Message
            // <th>{get_value_field(&26) </th> // Program
            <th>{get_value_field(&124)}</th> // Upload by
            <th>{get_value_field(&125)}</th> // Upload at
            {match &self.props.show_download_btn {
                true => html!{<th>{get_value_field(&126)}</th>}, // Download
                false => html!{},
            }}
          </tr>
        </thead>
      }
    }
}