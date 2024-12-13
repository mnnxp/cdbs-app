mod list_item;

pub use list_item::FilesetFileItem;

use std::collections::BTreeSet;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use crate::services::{get_value_field, resp_parsing};
use crate::error::Error;
use crate::fragments::buttons::ft_see_btn;
use crate::types::{UUID, ShowFileInfo};
use crate::gqls::make_query;
use crate::gqls::component::{ComModFilesOfFileset, com_mod_files_of_fileset};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub upload_files: usize,
    pub show_delete_btn: bool,
    pub select_fileset_uuid: UUID,
}

pub struct FilesetFilesBlock {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    show_full_files: bool,
    files: Vec<ShowFileInfo>,
    files_deleted_list: BTreeSet<UUID>,
}

#[derive(Clone)]
pub enum Msg {
    ShowFullList,
    RequestFilesOfFileset,
    ResponseError(Error),
    GetFilesOfFilesetResult(String),
    RemoveFile(UUID),
}

impl Component for FilesetFilesBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            show_full_files: false,
            files: Vec::new(),
            files_deleted_list: BTreeSet::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestFilesOfFileset);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestFilesOfFileset => {
                self.files.clear();
                if self.props.select_fileset_uuid.len() == 36 {
                    let ipt_file_of_fileset_arg = com_mod_files_of_fileset::IptFileOfFilesetArg{
                        filesetUuid: self.props.select_fileset_uuid.clone(),
                        fileUuids: None,
                    };
                    spawn_local(async move {
                        let res = make_query(ComModFilesOfFileset::build_query(com_mod_files_of_fileset::Variables {
                            ipt_file_of_fileset_arg
                        })).await.unwrap();

                        link.send_message(Msg::GetFilesOfFilesetResult(res));
                    })
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetFilesOfFilesetResult(res) => {
                match resp_parsing(res, "componentModificationFilesOfFileset") {
                    Ok(res) => {
                        self.files = res;
                        debug!("componentModificationFilesOfFileset: {:?}", self.files.len());
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            Msg::RemoveFile(file_uuid) => {self.files_deleted_list.insert(file_uuid);},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_fileset_uuid == props.select_fileset_uuid &&
        self.props.upload_files == props.upload_files &&
        self.props.show_delete_btn == props.show_delete_btn {
            debug!("no change fileset uuid: {:?}", props.select_fileset_uuid);
            false
        } else {
            debug!("change fileset uuid: {:?}", props.select_fileset_uuid);
            self.show_full_files = false;
            self.files_deleted_list.clear();
            self.props = props;
            self.link.send_message(Msg::RequestFilesOfFileset);
            true
        }
    }

    fn view(&self) -> Html {
        html!{<>
            <div class={"buttons"}>
                {for self.files.iter().enumerate().map(|(index, file)| {
                    match (index >= 3, self.show_full_files) {
                        // show full list
                        (_, true) => self.show_file_info(&file),
                        // show full list or first 3 items
                        (false, false) => self.show_file_info(&file),
                        _ => html!{},
                    }
                })}
            </div>
            <footer class="card-footer">
                {match self.files.len() {
                    0 => html!{<span>{get_value_field(&204)}</span>},
                    0..=3 => html!{},
                    _ => self.show_see_btn(),
                }}
            </footer>
        </>}
    }
}

impl FilesetFilesBlock {
    fn show_file_info(
        &self,
        file_info: &ShowFileInfo,
    ) -> Html {
        let callback_delete_file =
            self.link.callback(|value: UUID| Msg::RemoveFile(value));

        match self.files_deleted_list.get(&file_info.uuid) {
            Some(_) => html!{}, // removed file
            None => html!{
                <FilesetFileItem
                  show_delete_btn={self.props.show_delete_btn}
                  select_fileset_uuid={self.props.select_fileset_uuid.clone()}
                  file={file_info.clone()}
                  callback_delete_file={callback_delete_file.clone()}
                />
            },
        }
    }

    fn show_see_btn(&self) -> Html {
        let show_full_files_btn = self.link.callback(|_| Msg::ShowFullList);
        ft_see_btn(show_full_files_btn, self.show_full_files)
    }
}
