mod list_item;
mod table_item;

pub use list_item::FilesetFileItem;
pub use table_item::FileOfFilesetItem;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, ShowFileInfo};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub select_fileset_uuid: UUID,
    pub files: Vec<ShowFileInfo>,
}

pub struct FilesetFilesCard {
    link: ComponentLink<Self>,
    props: Props,
    // files_list: Vec<ShowFileInfo>,
    show_full_files: bool,
}

#[derive(Clone)]
pub enum Msg {
    ShowFullList,
    // Ignore,
}

impl Component for FilesetFilesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let files_list = props.files.clone();

        Self {
            link,
            props,
            // files_list,
            show_full_files: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            // Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_fileset_uuid == props.select_fileset_uuid &&
             self.props.files.first().map(|x| &x.uuid) == props.files.first().map(|x| &x.uuid) {
            debug!("no change fileset uuid: {:?}", props.select_fileset_uuid);
            false
        } else {
            debug!("change fileset uuid: {:?}", props.select_fileset_uuid);
            // self.files_list = props.files.clone();
            self.show_full_files = false;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        // html!{}
        self.show_files_list()
    }
}

impl FilesetFilesCard {
    fn show_files_list(&self) -> Html {
        html!{
            <div id="files" class="card">
                {for self.props.files.iter().enumerate().map(|(index, file)| {
                    match (index >= 3, self.show_full_files) {
                        // show full list
                        (_, true) => html!{<FilesetFileItem
                          show_download_btn = self.props.show_download_btn.clone()
                          show_delete_btn = self.props.show_delete_btn.clone()
                          select_fileset_uuid = self.props.select_fileset_uuid.clone()
                          file = file.clone()
                        />},
                        // show full list or first 3 items
                        (false, false) => html!{<FilesetFileItem
                          show_download_btn = self.props.show_download_btn.clone()
                          show_delete_btn = self.props.show_delete_btn.clone()
                          select_fileset_uuid = self.props.select_fileset_uuid.clone()
                          file = file.clone()
                        />},
                        _ => html!{},
                    }
                })}
                {match self.props.files.len() {
                    0 => html!{<span>{"Files not found"}</span>},
                    0..=3 => html!{},
                    _ => self.show_see_btn(),
                }}
            </div>
        }
    }

    fn show_see_btn(&self) -> Html {
        let show_full_files_btn = self.link
            .callback(|_| Msg::ShowFullList);

        match self.show_full_files {
            true => html!{<>
              <button class="button is-white"
                  onclick=show_full_files_btn
                >{"See less"}</button>
            </>},
            false => html!{<>
              <button class="button is-white"
                  onclick=show_full_files_btn
                >{"See more"}</button>
            </>},
        }
    }
}
