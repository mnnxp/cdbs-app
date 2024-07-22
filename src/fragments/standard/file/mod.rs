mod item;

pub use item::FileItem;

use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
use crate::fragments::buttons::ft_see_btn;
use crate::types::{UUID, ShowFileInfo};
use crate::services::get_value_field;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub standard_uuid: UUID,
    pub files: Vec<ShowFileInfo>,
}

pub struct StandardFilesCard {
    link: ComponentLink<Self>,
    props: Props,
    show_full_files: bool,
    files_deleted_list: BTreeSet<UUID>,
}

#[derive(Clone)]
pub enum Msg {
    ShowFullList,
    RemoveFile(UUID),
    Ignore,
}

impl Component for StandardFilesCard {
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
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.standard_uuid == props.standard_uuid &&
                self.props.show_delete_btn == props.show_delete_btn &&
                    self.props.files.len() == props.files.len() {
            false
        } else {
            self.files_deleted_list.clear();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let class_card = match self.props.show_delete_btn {
            true => "",
            false => "card",
        };
        html!{
            <div id="files" class={class_card}>
                {for self.props.files.iter().enumerate().map(|(index, file)| {
                    match (index >= 3, self.show_full_files) {
                        // show full list
                        (_, true) => self.show_file_info(file),
                        // show full list or first 3 items
                        (false, false) => self.show_file_info(file),
                        _ => html!{},
                    }
                })}
                {match self.props.files.len() {
                    0 => html!{<span>{ get_value_field(&204) }</span>},
                    0..=3 => html!{},
                    _ => self.show_see_btn(),
                }}
            </div>
        }
    }
}

impl StandardFilesCard {
    fn show_file_info(&self, file: &ShowFileInfo) -> Html {
        let callback_delete_file =
            self.link.callback(|value: UUID| Msg::RemoveFile(value));

        match self.files_deleted_list.get(&file.uuid) {
            Some(_) => html!{}, // removed file
            None => html!{
                <FileItem
                  show_delete_btn={self.props.show_delete_btn}
                  standard_uuid={self.props.standard_uuid.clone()}
                  file={file.clone()}
                  callback_delete_file={Some(callback_delete_file.clone())}
                />
            },
        }
    }

    fn show_see_btn(&self) -> Html {
        let show_full_files_btn = self.link.callback(|_| Msg::ShowFullList);
        ft_see_btn(show_full_files_btn, self.show_full_files)
    }
}
