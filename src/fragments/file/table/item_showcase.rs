use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::fragments::buttons::ft_download_btn;
use crate::types::ShowFileInfo;
use crate::services::content_adapter::{ContentDisplay, DateDisplay};
use crate::services::Size;

pub struct FileInfoItemShow {
  props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub file_info: ShowFileInfo,
  pub show_download_btn: bool,
  pub ordinal_indicator: usize,
}

impl Component for FileInfoItemShow {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
      FileInfoItemShow { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
      false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if &self.props.file_info.uuid == &props.file_info.uuid {
        false
      } else {
        self.props = props;
        true
      }
    }

    fn view(&self) -> Html {
      html!{
        <tr>
          <th>{self.props.ordinal_indicator}</th>
          <td>{self.props.file_info.filename.clone()}</td>
          // <td>{self.props.file_info.content_type.clone()}</td>
          <td>{self.props.file_info.revision}</td>
          <td>{self.props.file_info.show_size()}</td>
          // <td>{self.props.file_info.program.name.clone()}</td>
          <td>{self.props.file_info.owner_user.to_display()}</td>
          <td>{self.props.file_info.created_at.date_to_display()}</td>
          {match self.props.show_download_btn {
            true => html!{
              <td>{ft_download_btn(self.props.file_info.download_url.clone(), false)}</td>
            },
            false => html!{},
          }}
        </tr>
      }
    }
}