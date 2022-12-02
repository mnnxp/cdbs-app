use yew::{Component, Context, html, Html, Properties};
// use log::debug;
use crate::types::{UUID, ShowFileInfo};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_download_btn: bool,
    pub file: ShowFileInfo,
}

pub struct FileOfFilesetItem {
    file_uuid: UUID,
}

impl Component for FileOfFilesetItem {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            file_uuid: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.file_uuid == ctx.props().file.uuid {
            false
        } else {
            self.file_uuid = ctx.props().file.uuid.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{self.show_full_info_file(ctx.props())}
    }
}

impl FileOfFilesetItem {
    fn show_full_info_file(
        &self,
        props: &Props,
    ) -> Html {
        html!{<tr>
          <td>{props.file.filename.clone()}</td>
          // <td>{props.file.content_type.clone()}</td>
          <td>{props.file.filesize.clone()}</td>
          <td>{props.file.program.name.clone()}</td>
          <td>{format!("{} {} (@{})",
            props.file.owner_user.firstname.clone(),
            props.file.owner_user.lastname.clone(),
            props.file.owner_user.username.clone(),
          )}</td>
          <td>{format!("{:.*}", 19, props.file.updated_at.to_string())}</td>
        </tr>}
    }
}
