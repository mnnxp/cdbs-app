use std::collections::HashSet;

use yew::{classes, html, Callback, Component, ComponentLink, Html, InputData, MouseEvent, Properties, ShouldRender};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::FilesetFilesBlock;
use crate::fragments::component::modification::fileset::modal::AddModificationFilesetsModal;
use crate::services::{get_value_field, resp_parsing};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::{ft_custom_btn, ft_delete_pair_btn};
use crate::fragments::file::{UploaderFiles, commit_msg_field};
use crate::types::{UUID, UploadFile, FilesetProgramInfo};
use crate::gqls::make_query;
use crate::gqls::component::{
    ComponentModificationFilesets, component_modification_filesets,
    DeleteModificationFileset, delete_modification_fileset,
    UploadFilesToFileset, upload_files_to_fileset,
};

type FileName = String;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub select_modification_uuid: UUID,
}

pub struct ManageModificationFilesets {
    error: Option<Error>,
    request_upload_data: Vec<UploadFile>,
    props: Props,
    link: ComponentLink<Self>,
    filesets: Vec<FilesetProgramInfo>,
    current_program_ids: HashSet<usize>,
    select_fileset_uuid: UUID,
    upload_files: usize,
    open_add_fileset_card: bool,
    get_confirm: UUID,
    commit_msg: String,
}

pub enum Msg {
    RequestComponentModificationFilesetsData,
    RequestDeleteFileset(bool, UUID),
    RequestUploadFilesOfFileset(Vec<FileName>),
    GetComponentModificationFilesetResult(String),
    GetDeleteFilesetResult(String, UUID),
    GetUploadData(String),
    UploadConfirm(usize),
    UpdateCommitMsg(String),
    SelectFileset(UUID),
    NewFileset(FilesetProgramInfo),
    ShowAddFilesetCard,
    ResponseError(Error),
    ClearError,
}

impl Component for ManageModificationFilesets {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: Vec::new(),
            props,
            link,
            filesets: Vec::new(),
            current_program_ids: HashSet::new(),
            select_fileset_uuid: String::new(),
            upload_files: 0,
            open_add_fileset_card: false,
            get_confirm: String::new(),
            commit_msg: String::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestComponentModificationFilesetsData);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestComponentModificationFilesetsData => {
                debug!("Request filesets for modification uuid: {:?}", self.props.select_modification_uuid);
                self.filesets.clear(); // clear old filesets
                if self.props.select_modification_uuid.len() == 36 {
                    let ipt_fileset_program_arg = component_modification_filesets::IptFilesetProgramArg{
                        modificationUuid: self.props.select_modification_uuid.clone(),
                        programIds: None,
                    };
                    spawn_local(async move {
                        let res = make_query(ComponentModificationFilesets::build_query(
                            component_modification_filesets::Variables { ipt_fileset_program_arg }
                        )).await.unwrap();
                        link.send_message(Msg::GetComponentModificationFilesetResult(res));
                    })
                }
            },
            Msg::RequestDeleteFileset(is_confirmed, fileset_uuid) => {
                if !is_confirmed {
                    self.get_confirm.clear();
                    return true;
                }
                if self.get_confirm == fileset_uuid {
                    let del_fileset_program_data = delete_modification_fileset::DelFilesetProgramData{
                        modificationUuid: self.props.select_modification_uuid.clone(),
                        filesetUuid: fileset_uuid.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteModificationFileset::build_query(
                            delete_modification_fileset::Variables { del_fileset_program_data }
                        )).await.unwrap();
                        link.send_message(Msg::GetDeleteFilesetResult(res, fileset_uuid));
                    });
                    self.get_confirm.clear();
                } else {
                    self.get_confirm = fileset_uuid;
                }
            },
            Msg::RequestUploadFilesOfFileset(filenames) => {
                debug!("filenames: {:?}", filenames);
                if self.select_fileset_uuid.len() != 36 || filenames.is_empty() {
                    return false
                }
                let fileset_uuid = self.select_fileset_uuid.clone();
                let commit_msg = self.commit_msg.clone();
                spawn_local(async move {
                    let ipt_modification_file_from_fileset_data = upload_files_to_fileset::IptModificationFileFromFilesetData{
                        filesetUuid: fileset_uuid,
                        filenames,
                        commitMsg: commit_msg,
                    };
                    let res = make_query(UploadFilesToFileset::build_query(
                        upload_files_to_fileset::Variables{ ipt_modification_file_from_fileset_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                })
            },
            Msg::GetComponentModificationFilesetResult(res) => {
                match resp_parsing::<Vec<FilesetProgramInfo>>(res, "componentModificationFilesets") {
                    Ok(result) => {
                        debug!("Update modification filesets list");
                        self.filesets = result;
                        self.select_fileset_uuid = match self.filesets.first() {
                            Some(ft) => ft.uuid.clone(),
                            None => String::new(),
                        };
                        self.update_program_ids();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetDeleteFilesetResult(res, deleted_uuid) => {
                match resp_parsing::<bool>(res, "deleteModificationFileset") {
                    Ok(result) => {
                        debug!("deleteModificationFileset: {:?}", result);
                        if !result {
                            return false;
                        }
                        if self.select_fileset_uuid == deleted_uuid {
                            self.select_fileset_uuid = self.filesets.iter()
                                .find(|f| f.uuid != deleted_uuid)
                                .map(|f| f.uuid.clone())
                                .unwrap_or_default();
                        }
                        self.filesets.retain(|fileset| fileset.uuid != deleted_uuid);
                        self.update_program_ids();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUploadData(res) => {
                self.upload_files = 0;
                match resp_parsing(res, "uploadFilesToFileset") {
                    Ok(result) => {
                        self.request_upload_data = result;
                        debug!("uploadFilesToFileset: {:?}", self.request_upload_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of files: {:?}", confirmations);
                self.request_upload_data.clear();
                self.commit_msg.clear();
                self.upload_files = confirmations;
            },
            Msg::UpdateCommitMsg(data) => self.commit_msg = data,
            Msg::SelectFileset(fileset_uuid) => {
                debug!("SelectFileset: {:?}", fileset_uuid);
                self.select_fileset_uuid = fileset_uuid;
                self.upload_files = 0;
                self.get_confirm.clear(); // clear the check flag
            },
            Msg::NewFileset(new_fileset) => {
                self.select_fileset_uuid = new_fileset.uuid.clone();
                let already_exists = self.filesets.iter().any(|f| f.uuid == new_fileset.uuid);
                if !already_exists {
                    self.filesets.push(new_fileset);
                    self.update_program_ids();
                }
            },
            Msg::ShowAddFilesetCard => self.open_add_fileset_card = !self.open_add_fileset_card,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_modification_uuid == props.select_modification_uuid {
            false
        } else {
            self.props = props;
            self.upload_files = 0;
            self.select_fileset_uuid.clear();
            self.link.send_message(Msg::RequestComponentModificationFilesetsData);
            true
        }
    }
    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let callback_new_fileset = self.link.callback(|new_fileset| Msg::NewFileset(new_fileset));
        let callback_open_modal = self.link.callback(|_| Msg::ShowAddFilesetCard);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {if self.filesets.is_empty() {
                html!{}
            } else {
                html!{<>
                    <div class="columns">
                        {self.exist_filesets()}
                        {self.fileset_files()}
                    </div>
                </>}
            }}
            {match self.select_fileset_uuid.is_empty() {
                true => self.render_empty_state(),
                false => self.fileset_block(),
            }}
            <AddModificationFilesetsModal
                select_modification_uuid={self.props.select_modification_uuid.clone()}
                existing_program_ids={self.current_program_ids.clone()}
                is_active={self.open_add_fileset_card}
                on_close={callback_open_modal}
                on_fileset_created={callback_new_fileset}
            />
        </>}
    }
}

impl ManageModificationFilesets {
    /// Renders the empty state when no filesets exist
    fn render_empty_state(&self) -> Html {
        html!{
            <div class="has-text-centered p-6 card-content box">
                <div class="mb-4">
                    <span class="icon is-large has-text-grey-light">
                        <i class="fas fa-folder-plus fa-3x"></i>
                    </span>
                </div>
                <h4 class="title is-5 has-text-grey mb-2">{get_value_field(&403)}</h4>
                <p class="subtitle is-6 has-text-grey-light mb-5" style="max-width: 500px; margin: 0 auto;">
                    {get_value_field(&402)}
                </p>
                {self.show_add_btn()}
            </div>
        }
    }

    fn show_add_btn(&self) -> Html {
        let onclick_new_fileset_card = self.link.callback(|_| Msg::ShowAddFilesetCard);
        html!{
            {ft_custom_btn(
                &format!("create-first-fileset-{}", self.props.select_modification_uuid),
                get_value_field(&196),
                classes!("is-success"),
                "fas fa-folder-plus",
                onclick_new_fileset_card,
                self.props.select_modification_uuid.is_empty(),
            )}
        }
    }

    fn exist_filesets(&self) -> Html {
        html! {
            <div class="column">
                <div class="is-flex is-justify-content-space-between is-align-items-center mb-3">
                    <p class="subtitle is-5 has-text-weight-bold mb-1">{get_value_field(&404)}</p>
                    {self.show_add_btn()}
                </div>
                <div class="panel p-2 mb-4" style="max-height: 300px; overflow-y: auto;">
                    {for self.filesets.iter().map(|fileset| {
                        self.fileset_item(fileset)
                    })}
                </div>
            </div>
        }
    }

    fn fileset_files(&self) -> Html {
        html!{
            <div class="column">
                <p class="subtitle is-5 has-text-weight-bold mb-3">{get_value_field(&198)}</p>
                <div class="box">
                    {match self.filesets.iter().find(|sf| sf.uuid == self.select_fileset_uuid) {
                        Some(f) => html!{
                            <FilesetFilesBlock
                                upload_files={self.upload_files}
                                show_delete_btn={true}
                                select_fileset={f.clone()}
                            />
                        },
                        None => html!{
                            <div class="has-text-centered py-5">
                                <p class="has-text-grey-light">{get_value_field(&204)}</p>
                            </div>
                        },
                    }}
                </div>
            </div>
        }
    }

    fn fileset_item(&self, fileset: &FilesetProgramInfo) -> Html {
        let is_selected = fileset.uuid == self.select_fileset_uuid;
        let item_classes = if is_selected {
            classes!("panel-block", "is-clickable", "is-active", "has-background-link-light", "has-text-link")
        } else {
            classes!("panel-block", "is-clickable")
        };
        let uuid_clone = fileset.uuid.to_string();
        let on_click_item = self.link.callback(move |_| Msg::SelectFileset(uuid_clone.clone()));
        let tag_classes = if fileset.files_count > 0 {
            "tag is-rounded mr-2 is-light is-info"
        } else {
            "tag is-rounded mr-2 is-light"
        };

        // Callback stub to prevent click event bubbling from the trash icon
        let on_click_delete_container = Callback::from(|e: MouseEvent| {
            e.stop_propagation(); // Prevents triggering the parent's on_click_item handler
        });

        html! {
            <div
                class={item_classes}
                onclick={on_click_item}
                style="display: flex; justify-content: space-between; align-items: center; padding: 0.5rem 0.75rem;"
            >
                <div class="is-flex is-align-items-center">
                    <span class="icon mr-2">
                        <i class="fas fa-folder-open"></i>
                    </span>
                    <span class="has-text-weight-medium">{ &fileset.program.name }</span>
                </div>
                <div class="is-flex is-align-items-center">
                    <span class={tag_classes}>
                        { fileset.files_count }
                    </span>
                    <span onclick={on_click_delete_container}>
                        {self.show_delete_btn(&fileset.uuid)}
                    </span>
                </div>
            </div>
        }
    }

    fn fileset_block(&self) -> Html {
        let oninput_commit_msg = self.link.callback(|ev: InputData| Msg::UpdateCommitMsg(ev.value));
        let callback_upload_filenames =
            self.link.callback(move |filenames| Msg::RequestUploadFilesOfFileset(filenames));
        let request_upload_files = match self.request_upload_data.is_empty() {
            true => None,
            false => Some(self.request_upload_data.clone()),
        };
        let callback_upload_confirm = self.link.callback(|confirmations| Msg::UploadConfirm(confirmations));

        html!{
            <div class="column">
                <p class="subtitle is-5 has-text-weight-bold mb-3">{get_value_field(&197)}</p>
                <div class="mt-4 pt-3" style="border-top: 1px solid #f5f5f5;">
                    {commit_msg_field(self.select_fileset_uuid.clone(), self.commit_msg.clone(), oninput_commit_msg.clone())}
                </div>
                <UploaderFiles
                    text_choose_files={195}
                    callback_upload_filenames={callback_upload_filenames}
                    request_upload_files={request_upload_files}
                    callback_upload_confirm={callback_upload_confirm}
                />
            </div>
        }
    }

    fn show_delete_btn(&self, fileset_uuid: &UUID) -> Html {
        let fileset_uuid_clone = fileset_uuid.clone();
        let onclick_delete_fileset_btn = self.link.callback(move |is_confirmed|
            Msg::RequestDeleteFileset(is_confirmed, fileset_uuid_clone.clone())
        );
        ft_delete_pair_btn(
            &format!("fileset-program-{}", fileset_uuid),
            onclick_delete_fileset_btn,
            &self.get_confirm == fileset_uuid,
            fileset_uuid.is_empty(),
            classes!("is-small")
        )
    }

    fn update_program_ids(&mut self) {
        self.current_program_ids = self.filesets
            .iter()
            .map(|f| f.program.id)
            .collect();
    }
}
