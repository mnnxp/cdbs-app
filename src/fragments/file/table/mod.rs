pub mod item_showcase;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::services::LocaleKey;

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
            <th>{LocaleKey::Filename.get_value()}</th>
            <th><abbr title={LocaleKey::Revision.get_value()}>{LocaleKey::Rev.get_value()}</abbr></th>
            // <th>{LocaleKey::Content.get_value()}</th>
            <th>{LocaleKey::Filesize.get_value()}</th>
            <th><abbr title={LocaleKey::MessageToChange.get_value()}>{LocaleKey::Message.get_value()}</abbr></th>
            // <th>{LocaleKey::Program.get_value() </th> // Program
            <th>{LocaleKey::UploadBy.get_value()}</th>
            <th>{LocaleKey::UploadAt.get_value()}</th>
            {match &self.props.show_download_btn {
                true => html!{<th>{LocaleKey::Download.get_value()}</th>},
                false => html!{},
            }}
          </tr>
        </thead>
      }
    }
}