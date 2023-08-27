use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::types::DownloadFile;

pub struct FileDownItemShow {
  props: Props,
  // link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub file_down: DownloadFile,
}

impl Component for FileDownItemShow {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
      FileDownItemShow {
        props,
        // link,
      }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
      false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if &self.props.file_down.uuid == &props.file_down.uuid {
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

impl FileDownItemShow {
  fn show_item_data(&self) -> Html {
    html!{
      <tr>
        <td>{self.props.file_down.filename.clone()}</td>
        <td>{self.props.file_down.filesize.clone()}</td>
        {self.show_download_btn()}
      </tr>
    }
  }

  fn show_download_btn(&self) -> Html {
    html!{<td>
      <a class="button is-white"
          href={self.props.file_down.download_url.clone()}
          disabled={self.props.file_down.download_url.is_empty()}
          target="_blank"
          >
        <span class="icon" >
          <i class="fas fa-file-download" style="color: #1872f0;" aria-hidden="true"></i>
        </span>
      </a>
    </td>}
  }
}
