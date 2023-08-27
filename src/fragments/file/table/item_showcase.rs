use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::types::ShowFileInfo;

pub struct FileInfoItemShow {
  props: Props,
  // link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub file_info: ShowFileInfo,
  // pub file_download_callback: Option<Callback<UUID>>,
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
      self.show_item_data()
    }
}

impl FileInfoItemShow {
  fn show_item_data(&self) -> Html {
    html!{<tr>
      <td>{self.props.file_info.filename.clone()}</td>
      // <td>{self.props.file_info.content_type.clone()}</td>
      <td>{self.props.file_info.revision}</td>
      <td>{self.props.file_info.filesize.clone()}</td>
      // <td>{self.props.file_info.program.name.clone()}</td>
      <td>{format!("{} {} (@{})",
        self.props.file_info.owner_user.firstname.clone(),
        self.props.file_info.owner_user.lastname.clone(),
        self.props.file_info.owner_user.username.clone(),
      )}</td>
      <td>{format!("{:.*}", 19, self.props.file_info.created_at.to_string())}</td>
      // {match self.props.file_download_callback {
      {match self.props.download_url.is_empty() {
        true => html!{},
        false => html!{self.show_download_btn()},
      }}
    </tr>}
  }

  fn show_download_btn(&self) -> Html {
    html!{<td>
      <a class="button is-white"
          href={self.props.download_url.clone()}
          disabled={self.props.download_url.is_empty()}
          target="_blank"
          >
        <span class="icon" >
          <i class="fas fa-file-download" style="color: #1872f0;" aria-hidden="true"></i>
        </span>
      </a>
    </td>}
  }
}
