pub mod item_showcase;
pub mod down_item_show;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::services::get_value_field;

pub struct FileHeadersShow {
  props: Props,
  // link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub show_long: bool,
  #[prop_or_default]
  pub show_download_btn: bool,
}

impl Component for FileHeadersShow {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
      FileHeadersShow {
        props,
        // link,
      }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
      false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if self.props.show_long == props.show_long {
        false
      } else {
        self.props = props;
        true
      }
    }

    fn view(&self) -> Html {
      match self.props.show_long {
        true => {self.head_long()},
        false => {self.head_short()},
      }
    }
}

impl FileHeadersShow {
    fn head_long(&self) -> Html {
      html!{
        <thead>
          <tr>
            <th>{ get_value_field(&120) }</th> // Filename
            <th>{ get_value_field(&308) }</th> // Revision
            // <th>{ get_value_field(&121) }</th> // Content
            <th>{ get_value_field(&122) }</th> // Filesize
            // <th>{ get_value_field(&26) }</th> // Program
            <th>{ get_value_field(&124) }</th> // Upload by
            <th>{ get_value_field(&125) }</th> // Upload at
            {match &self.props.show_download_btn {
                true => html!{<th>{ get_value_field(&126) }</th>}, // Download
                false => html!{},
            }}
          </tr>
        </thead>
      }
    }

    fn head_short(&self) -> Html {
      html!{
        <thead>
          <tr>
            <th>{ get_value_field(&120) }</th> // Filename
            <th>{ get_value_field(&122) }</th> // Filesize
            <th>{ get_value_field(&126) }</th> // Download
          </tr>
        </thead>
      }
    }
}