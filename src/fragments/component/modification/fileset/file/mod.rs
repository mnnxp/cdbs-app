mod list_item;
mod table_item;

pub use list_item::FilesetFileItem;
pub use table_item::FileOfFilesetItem;

use std::collections::BTreeSet;
use yew::{Component, Context, Html, Properties, html, html::Scope};
use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, ShowFileInfo};
use crate::services::get_value_field;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub select_fileset_uuid: UUID,
    pub files: Vec<ShowFileInfo>,
}

pub struct FilesetFilesBlock {
    select_fileset_uuid: UUID,
    first_file_uuid: UUID,
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            select_fileset_uuid: ctx.props().select_fileset_uuid,
            first_file_uuid: ctx.props().files.first().map(|x| x.uuid.clone()).unwrap_or_default(),
            show_full_files: false,
            files_deleted_list: BTreeSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // let link = ctx.link().clone();
        match msg {
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            Msg::RemoveFile(file_uuid) => {
                self.files_deleted_list.insert(file_uuid);
            },
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.select_fileset_uuid == ctx.props().select_fileset_uuid &&
             ctx.props().files.first().map(|x| x.uuid == self.first_file_uuid).unwrap_or_default() {
            debug!("no change fileset uuid: {:?}", self.select_fileset_uuid);
            false
        } else {
            debug!("change fileset uuid: {:?}", self.select_fileset_uuid);
            self.select_fileset_uuid = ctx.props().select_fileset_uuid;
            self.first_file_uuid = ctx.props().files.first().map(|x| x.uuid.clone()).unwrap_or_default();
            self.show_full_files = false;
            self.files_deleted_list.clear();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<>
            {for ctx.props().files.iter().enumerate().map(|(index, file)| {
                match (index >= 3, self.show_full_files) {
                    // show full list
                    (_, true) => self.show_file_info(ctx.link(), ctx.props(), &file),
                    // show full list or first 3 items
                    (false, false) => self.show_file_info(ctx.link(), ctx.props(), &file),
                    _ => html!{},
                }
            })}
            {match ctx.props().files.len() {
                0 => html!{<span>{ get_value_field(&204) }</span>},
                0..=3 => html!{},
                _ => self.show_see_btn(ctx.link()),
            }}
        </>}
    }
}

impl FilesetFilesBlock {
    fn show_file_info(
        &self,
        link: &Scope<Self>,
        props: &Props,
        file_info: &ShowFileInfo,
    ) -> Html {
        let callback_delete_file = link.callback(|value: UUID| Msg::RemoveFile(value));

        match self.files_deleted_list.get(&file_info.uuid) {
            Some(_) => html!{}, // removed file
            None => html!{
                <FilesetFileItem
                  show_download_btn = {props.show_download_btn}
                  show_delete_btn = {props.show_delete_btn}
                  select_fileset_uuid = {props.select_fileset_uuid.clone()}
                  file = {file_info.clone()}
                  callback_delete_file = {callback_delete_file.clone()}
                />
            },
        }
    }

    fn show_see_btn(
        &self,
        link: &Scope<Self>
    ) -> Html {
        let show_full_files_btn = link.callback(|_| Msg::ShowFullList);

        match self.show_full_files {
            true => html!{
              <button class="button is-white" onclick={show_full_files_btn}>
                  { get_value_field(&99) }
              </button>
            },
            false => html!{
              <button class="button is-white" onclick={show_full_files_btn}>
                  { get_value_field(&98) }
              </button>
            },
        }
    }
}
