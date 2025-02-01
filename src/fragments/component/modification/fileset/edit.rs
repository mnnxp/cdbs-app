use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html, ChangeData, classes, InputData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::FilesetFilesBlock;
use crate::services::{get_value_field, resp_parsing};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::{ft_delete_btn, ft_cancel_btn, ft_save_btn, ft_add_btn};
use crate::fragments::file::{UploaderFiles, commit_msg_field};
use crate::types::{UUID, Program, UploadFile, FilesetProgramInfo};
use crate::gqls::make_query;
use crate::gqls::relate::{GetPrograms, get_programs};
use crate::gqls::component::{
    ComponentModificationFilesets, component_modification_filesets,
    RegisterModificationFileset, register_modification_fileset,
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
    request_fileset_program_id: usize,
    props: Props,
    link: ComponentLink<Self>,
    filesets_program: Vec<(UUID, String)>,
    select_fileset_uuid: UUID,
    upload_files: usize,
    programs: Vec<Program>,
    open_add_fileset_card: bool,
    get_confirm: UUID,
    commit_msg: String,
}

pub enum Msg {
    RequestComponentModificationFilesetsData,
    RequestProgramsList,
    RequestNewFileset,
    RequestDeleteFileset,
    RequestUploadFilesOfFileset(Vec<FileName>),
    ResponseError(Error),
    GetComponentModificationFilesetResult(String),
    GetProgramsListResult(String),
    GetNewFilesetResult(String),
    GetDeleteFilesetResult(String),
    GetUploadData(String),
    UploadConfirm(usize),
    UpdateCommitMsg(String),
    SelectFileset(UUID),
    UpdateSelectProgramId(usize),
    ShowAddFilesetCard,
    ClearError,
}

impl Component for ManageModificationFilesets {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            request_upload_data: Vec::new(),
            request_fileset_program_id: 1,
            props,
            link,
            filesets_program: Vec::new(),
            select_fileset_uuid: String::new(),
            upload_files: 0,
            programs: Vec::new(),
            open_add_fileset_card: false,
            get_confirm: String::new(),
            commit_msg: String::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestComponentModificationFilesetsData);
            self.link.send_message(Msg::RequestProgramsList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestComponentModificationFilesetsData => {
                debug!("Request filesets for modification uuid: {:?}", self.props.select_modification_uuid);
                self.filesets_program.clear(); // fix for bug with displaying old files
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
            Msg::RequestProgramsList => {
                spawn_local(async move {
                    let res = make_query(GetPrograms::build_query(
                        get_programs::Variables { program_ids: None }
                    )).await.unwrap();

                    link.send_message(Msg::GetProgramsListResult(res));
                })
            },
            Msg::RequestNewFileset => {
                let ipt_fileset_program_data = register_modification_fileset::IptFilesetProgramData{
                    modificationUuid: self.props.select_modification_uuid.clone(),
                    programId: self.request_fileset_program_id as i64,
                };
                spawn_local(async move {
                    let res = make_query(RegisterModificationFileset::build_query(
                        register_modification_fileset::Variables { ipt_fileset_program_data }
                    )).await.unwrap();

                    link.send_message(Msg::GetNewFilesetResult(res));
                })
            },
            Msg::RequestDeleteFileset => {
                if self.get_confirm == self.select_fileset_uuid {
                    let del_fileset_program_data = delete_modification_fileset::DelFilesetProgramData{
                        modificationUuid: self.props.select_modification_uuid.clone(),
                        filesetUuid: self.select_fileset_uuid.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteModificationFileset::build_query(
                            delete_modification_fileset::Variables { del_fileset_program_data }
                        )).await.unwrap();
                        link.send_message(Msg::GetDeleteFilesetResult(res));
                    })
                } else {
                    self.get_confirm = self.select_fileset_uuid.clone();
                }
            },
            Msg::RequestUploadFilesOfFileset(filenames) => {
                debug!("filenames: {:?}", filenames);
                if self.select_fileset_uuid.len() != 36 || filenames.is_empty() {
                    return false
                }
                self.upload_files = 0;
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
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetComponentModificationFilesetResult(res) => {
                match resp_parsing::<Vec<FilesetProgramInfo>>(res, "componentModificationFilesets") {
                    Ok(filesets) => {
                        debug!("Update modification filesets list");
                        for fileset in &filesets {
                            self.filesets_program.push((fileset.uuid.clone(), fileset.program.name.clone()));
                        }
                        if let Some((ft_uuid, _)) = self.filesets_program.first() {
                            self.select_fileset_uuid = match ft_uuid.len() == 36 {
                                true => ft_uuid.clone(),
                                false => String::new(),
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetProgramsListResult(res) => {
                match resp_parsing::<Vec<Program>>(res, "programs") {
                    Ok(result) => {
                        // debug!("programs: {:?}", result);
                        self.programs.clear();
                        for x in result.iter() {
                            if let None = self.filesets_program.iter().find(|(_, program_name)| program_name == &x.name) {
                                self.programs.push(x.clone());
                                continue;
                            }
                        }
                        if let Some(program) = self.programs.first() {
                            self.request_fileset_program_id = program.id;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetNewFilesetResult(res) => {
                match resp_parsing(res, "registerModificationFileset") {
                    Ok(result) => {
                        self.select_fileset_uuid = result;
                        debug!("registerModificationFileset: {:?}", self.select_fileset_uuid);
                        self.upload_files = 0;
                        if let Some(program) = self.programs.iter().find(|x| x.id == self.request_fileset_program_id) {
                            if let None = self.filesets_program.iter().find(|(_, p_name)| p_name == &program.name) {
                                self.filesets_program.push((
                                    self.select_fileset_uuid.clone(),
                                    program.name.clone(),
                                ));
                            }
                        }
                        self.open_add_fileset_card = false;
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetDeleteFilesetResult(res) => {
                match resp_parsing::<bool>(res, "deleteModificationFileset") {
                    Ok(result) => {
                        debug!("deleteModificationFileset: {:?}", result);
                        if !result {
                            return false
                        }
                        let mut update_filesets: Vec<(UUID, String)> = Vec::new();
                        // for set next item after delete
                        let delete_fileset_uuid = self.select_fileset_uuid.clone();
                        self.select_fileset_uuid = String::new();
                        let mut flag_delete = false;
                        for x in self.filesets_program.iter() {
                            if flag_delete {
                                self.select_fileset_uuid = x.0.clone();
                                flag_delete = false;
                            }
                            if x.0 != delete_fileset_uuid {
                                update_filesets.push(x.clone());
                            } else {
                                flag_delete = true;
                            }
                        }
                        if self.select_fileset_uuid.is_empty() {
                            self.select_fileset_uuid = update_filesets
                                .first()
                                .map(|(fileset_uuid, _)| fileset_uuid.clone())
                                .unwrap_or_default();
                        }
                        self.filesets_program = update_filesets;
                        // self.link.send_message(Msg::RequestFilesOfFileset);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUploadData(res) => {
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
            Msg::UpdateSelectProgramId(program_id) => self.request_fileset_program_id = program_id,
            Msg::ShowAddFilesetCard => {
                self.open_add_fileset_card = !self.open_add_fileset_card;
                if self.programs.is_empty() {
                    link.send_message(Msg::RequestProgramsList);
                }
            },
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
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {match &self.open_add_fileset_card {
                true => self.add_fileset_block(),
                false => html!{<>
                    {self.show_manage()}
                    {self.fileset_block()}
                </>},
            }}
        </>}
    }
}

impl ManageModificationFilesets {
    fn show_manage(&self) -> Html {
        let onchange_select_fileset_btn =
            self.link.callback(|ev: ChangeData| Msg::SelectFileset(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
            }));

        html!{<div class="columns">
            <div class="column">
                <div class="select is-fullwidth" style="margin-right: .5rem">
                  <select
                        id="select-fileset-program-edit"
                        select={self.select_fileset_uuid.clone()}
                        onchange={onchange_select_fileset_btn} >
                      {for self.filesets_program.iter().map(|(fileset_uuid, program_name)|
                          html!{
                              <option value={fileset_uuid.to_string()}
                                    selected={fileset_uuid == &self.select_fileset_uuid} >
                                  {program_name}
                              </option>
                          }
                      )}
                  </select>
                </div>
            </div>
            <div class="column">
                {self.show_delete_btn()}
            </div>
        </div>}
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
        let onclick_new_fileset_card = self.link.callback(|_| Msg::ShowAddFilesetCard);
        html!{<>
            <div class="column">
                <p class={"title is-4"}>{get_value_field(&197)}</p> // Upload files for fileset
            </div>
            {commit_msg_field(self.commit_msg.clone(), oninput_commit_msg.clone())}
            <div class="column">
                <UploaderFiles
                    text_choose_files={195} // Choose fileset files…
                    callback_upload_filenames={callback_upload_filenames}
                    request_upload_files={request_upload_files}
                    callback_upload_confirm={callback_upload_confirm}
                    />
            </div>
            <div class="column">
                <p class={"title is-4"}>{get_value_field(&198)}</p> // Files of fileset
                <FilesetFilesBlock
                    upload_files={self.upload_files}
                    show_delete_btn={true}
                    select_fileset_uuid={self.select_fileset_uuid.clone()}
                />
            </div>
            {ft_add_btn(
                "create-new-fileset",
                get_value_field(&196),
                onclick_new_fileset_card,
                true,
                self.props.select_modification_uuid.is_empty()
            )}
        </>}
    }

    fn add_fileset_block(&self) -> Html {
        let close_add_fileset_block = self.link.callback(|_| Msg::ShowAddFilesetCard);
        let onclick_add_fileset_btn = self.link.callback(|_| Msg::RequestNewFileset);
        html!{
            <div class="column">
                <span class="has-text-weight-bold is-size-4">{get_value_field(&206)}</span>
                <div class="block">
                    <label class="label">{get_value_field(&207)}</label> // Program for fileset
                    <div class="buttons">
                        {for self.programs.iter().map(|x|
                            self.fileset_items(x.id, &x.name)
                        )}
                    </div>
                </div>
                <hr/>
                <div class="buttons">
                    {ft_cancel_btn(
                        "close-add-fileset-program",
                        close_add_fileset_block
                    )}
                    {ft_save_btn(
                        "add-modification-fileset",
                        onclick_add_fileset_btn,
                        true,
                        self.props.select_modification_uuid.is_empty()
                    )}
                </div>
            </div>
        }
    }

    fn show_delete_btn(&self) -> Html {
        let onclick_delete_fileset_btn = self.link.callback(|_| Msg::RequestDeleteFileset);
        ft_delete_btn(
            "delete-fileset-program",
            onclick_delete_fileset_btn,
            self.get_confirm == self.select_fileset_uuid,
            self.select_fileset_uuid.is_empty()
        )
    }

    fn fileset_items(&self, program_id: usize, program_name: &str) -> Html {
        let onchange_select_program_id = self.link.callback(move |_| Msg::UpdateSelectProgramId(program_id));
        let class_item = match program_id == self.request_fileset_program_id {
            true => classes!("button", "is-info", "is-focused"),
            false => classes!("button", "is-info", "is-outlined"),
        };

        html!{
            <button
                id={"set-fileset-program"}
                class={class_item}
                disabled={program_id == self.request_fileset_program_id}
                onclick={onchange_select_program_id} >
                <span>{program_name}</span>
            </button>
        }
    }
}
