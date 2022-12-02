mod item;

pub use item::ComponentFileItem;

use std::collections::BTreeSet;
use yew::{Component, Context, html, html::Scope, Html, Properties};
// use log::debug;
use crate::types::{UUID, ShowFileInfo};
use crate::services::get_value_field;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub files: Vec<ShowFileInfo>,
}

pub struct ComponentFilesBlock {
    component_uuid: UUID,
    first_file_uuid: UUID,
    show_full_files: bool,
    files_deleted_list: BTreeSet<UUID>,
}

#[derive(Clone)]
pub enum Msg {
    ShowFullList,
    RemoveFile(UUID),
    Ignore,
}

impl Component for ComponentFilesBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            component_uuid: ctx.props().component_uuid.clone(),
            first_file_uuid: ctx.props().files.first().map(|x| x.uuid.clone()).unwrap_or_default(),
            show_full_files: false,
            files_deleted_list: BTreeSet::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
        if ctx.props().files.first().map(|x| x.uuid == self.first_file_uuid).unwrap_or_default() &&
              self.component_uuid == ctx.props().component_uuid {
            false
        } else {
            self.files_deleted_list.clear();
            self.component_uuid = ctx.props().component_uuid.clone();
            self.first_file_uuid = ctx.props().files.first().map(|x| x.uuid.clone()).unwrap_or_default();
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

impl ComponentFilesBlock {
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
                <ComponentFileItem
                  show_download_btn = {props.show_download_btn}
                  show_delete_btn = {props.show_delete_btn}
                  component_uuid = {props.component_uuid.clone()}
                  file = {file_info.clone()}
                  callback_delete_file = {callback_delete_file.clone()}
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
