mod list_item;

pub use list_item::FilesetFileItem;

use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use crate::types::{UUID, ShowFileInfo};
use crate::services::get_value_field;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub select_fileset_uuid: UUID,
    pub files: Vec<ShowFileInfo>,
}

pub struct FilesetFilesBlock {
    link: ComponentLink<Self>,
    props: Props,
    show_full_files: bool,
    files_deleted_list: BTreeSet<UUID>,
}

#[derive(Clone)]
pub enum Msg {
    ShowFullList,
    RemoveFile(UUID),
}

impl Component for FilesetFilesBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            show_full_files: false,
            files_deleted_list: BTreeSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            Msg::RemoveFile(file_uuid) => {
                self.files_deleted_list.insert(file_uuid);
            },
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
            self.show_full_files = false;
            self.files_deleted_list.clear();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{<>
            {for self.props.files.iter().enumerate().map(|(index, file)| {
                match (index >= 3, self.show_full_files) {
                    // show full list
                    (_, true) => self.show_file_info(&file),
                    // show full list or first 3 items
                    (false, false) => self.show_file_info(&file),
                    _ => html!{},
                }
            })}
            {match self.props.files.len() {
                0 => html!{<span>{ get_value_field(&204) }</span>},
                0..=3 => html!{},
                _ => self.show_see_btn(),
            }}
        </>}
    }
}

impl FilesetFilesBlock {
    fn show_file_info(
        &self,
        file_info: &ShowFileInfo,
    ) -> Html {
        let callback_delete_file = self.link
            .callback(|value: UUID| Msg::RemoveFile(value));

        match self.files_deleted_list.get(&file_info.uuid) {
            Some(_) => html!{}, // removed file
            None => html!{
                <FilesetFileItem
                  show_download_btn = self.props.show_download_btn
                  show_delete_btn = self.props.show_delete_btn
                  select_fileset_uuid = self.props.select_fileset_uuid.clone()
                  file = file_info.clone()
                  callback_delete_file = callback_delete_file.clone()
                />
            },
        }
    }

    fn show_see_btn(&self) -> Html {
        let show_full_files_btn = self.link
            .callback(|_| Msg::ShowFullList);

        match self.show_full_files {
            true => html!{
              <button class="button is-white" onclick=show_full_files_btn>
                  { get_value_field(&99) }
              </button>
            },
            false => html!{
              <button class="button is-white" onclick=show_full_files_btn>
                  { get_value_field(&98) }
              </button>
            },
        }
    }
}
