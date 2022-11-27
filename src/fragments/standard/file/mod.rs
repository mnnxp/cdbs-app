mod item;

pub use item::FileItem;

use std::collections::BTreeSet;
use yew::{Component, Context, html, html::Scope, Html, Properties};
// use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, ShowFileInfo};
use crate::services::get_value_field;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub standard_uuid: UUID,
    pub files: Vec<ShowFileInfo>,
}

pub struct StandardFilesCard {
    standard_uuid: UUID,
    show_download_btn: bool,
    show_delete_btn: bool,
    files_len: usize,
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            standard_uuid: ctx.props().standard_uuid,
            show_download_btn: ctx.props().show_download_btn,
            show_delete_btn: ctx.props().show_delete_btn,
            files_len: ctx.props().files.len(),
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
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.standard_uuid == ctx.props().standard_uuid &&
            self.show_download_btn == ctx.props().show_download_btn &&
                self.show_delete_btn == ctx.props().show_delete_btn &&
                    self.files_len == ctx.props().files.len() {
            false
        } else {
            self.files_deleted_list.clear();
            self.standard_uuid = ctx.props().standard_uuid;
            self.show_download_btn = ctx.props().show_download_btn;
            self.show_delete_btn = ctx.props().show_delete_btn;
            self.files_len = ctx.props().files.len();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div id="files" class="card">
                {for ctx.props().files.iter().enumerate().map(|(index, file)| {
                    match (index >= 3, self.show_full_files) {
                        // show full list
                        (_, true) => self.show_file_info(ctx.link(), ctx.props(), file),
                        // show full list or first 3 items
                        (false, false) => self.show_file_info(ctx.link(), ctx.props(), file),
                        _ => html!{},
                    }
                })}
                {match ctx.props().files.len() {
                    0 => html!{<span>{ get_value_field(&204) }</span>},
                    0..=3 => html!{},
                    _ => self.show_see_btn(ctx.link()),
                }}
            </div>
        }
    }
}

impl StandardFilesCard {
    fn show_file_info(
        &self,
        link: &Scope<Self>,
        props: &Props,
        file: &ShowFileInfo
    ) -> Html {
        let callback_delete_file = link.callback(|value: UUID| Msg::RemoveFile(value));

        match self.files_deleted_list.get(&file.uuid) {
            Some(_) => html!{}, // removed file
            None => html!{
                <FileItem
                  show_download_btn = {props.show_download_btn}
                  show_delete_btn = {props.show_delete_btn}
                  standard_uuid = {props.standard_uuid.clone()}
                  file = {file.clone()}
                  callback_delete_file = {Some(callback_delete_file.clone())}
                />
            },
        }
    }

    fn show_see_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let show_full_files_btn = link.callback(|_| Msg::ShowFullList);

        match self.show_full_files {
            true => html!{<>
              <button class="button is-white"
                  onclick={show_full_files_btn}
                >{ get_value_field(&99) }</button>
            </>},
            false => html!{<>
              <button class="button is-white"
                  onclick={show_full_files_btn}
                >{ get_value_field(&98) }</button>
            </>},
        }
    }
}
