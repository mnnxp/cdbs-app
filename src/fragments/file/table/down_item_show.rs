use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::fragments::buttons::ft_download_btn;
use crate::types::DownloadFile;
use crate::services::Size;

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
        <td>{self.props.file_down.show_size()}</td>
        <td>{ft_download_btn(self.props.file_down.download_url.clone(), false)}</td>
      </tr>
    }
  }
}
