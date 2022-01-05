use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;

use crate::types::{UUID, ShowFileInfo};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub file: ShowFileInfo,
}

pub struct FileOfFilesetItem {
    props: Props,
    file_uuid: UUID,
}

impl Component for FileOfFilesetItem {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            file_uuid: String::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.file_uuid == props.file.uuid {
            false
        } else {
            self.file_uuid = props.file.uuid.clone();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{self.show_full_info_file()}
    }
}

impl FileOfFilesetItem {
    fn show_full_info_file(&self) -> Html {
        html!{<tr>
          <td>{self.props.file.filename.clone()}</td>
          // <td>{self.props.file.content_type.clone()}</td>
          <td>{self.props.file.filesize.clone()}</td>
          <td>{self.props.file.program.name.clone()}</td>
          <td>{format!("{} {} (@{})",
            self.props.file.owner_user.firstname.clone(),
            self.props.file.owner_user.lastname.clone(),
            self.props.file.owner_user.username.clone(),
          )}</td>
          <td>{format!("{:.*}", 19, self.props.file.updated_at.to_string())}</td>
        </tr>}
    }
}
