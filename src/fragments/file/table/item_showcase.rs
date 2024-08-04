use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::fragments::buttons::ft_download_btn;
use crate::types::ShowFileInfo;
use crate::services::content_adapter::{ContentDisplay, DateDisplay};
use crate::services::Size;

pub struct FileInfoItemShow {
  props: Props,
  // link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub file_info: ShowFileInfo,
  pub download_url: String,
}

impl Component for FileInfoItemShow {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
      FileInfoItemShow {
        props,
        // link,
      }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
      false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if &self.props.file_info.uuid == &props.file_info.uuid &&
          &self.props.download_url == &props.download_url {
        false
      } else {
        self.props = props;
        true
      }
    }

    fn view(&self) -> Html {
      html!{
        <tr>
          <td>{self.props.file_info.filename.clone()}</td>
          // <td>{self.props.file_info.content_type.clone()}</td>
          <td>{self.props.file_info.revision}</td>
          <td>{self.props.file_info.show_size()}</td>
          // <td>{self.props.file_info.program.name.clone()}</td>
          <td>{self.props.file_info.owner_user.to_display()}</td>
          <td>{self.props.file_info.created_at.date_to_display()}</td>
          {match self.props.download_url.is_empty() {
            true => html!{},
            false => html!{
              <td>{ft_download_btn(self.props.download_url.clone(), false)}</td>
            },
          }}
        </tr>
      }
    }
}