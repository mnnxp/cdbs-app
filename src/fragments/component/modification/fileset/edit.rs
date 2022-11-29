use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use graphql_client::GraphQLQuery;
use gloo::file::File;
use web_sys::{DragEvent, Event};
use wasm_bindgen_futures::spawn_local;
use web_sys::FileList;
use log::debug;
use serde_json::Value;
use super::FilesetFilesBlock;
use crate::services::storage_upload::storage_upload;
use crate::services::get_value_field;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo, Program, UploadFile};
use crate::gqls::{
    make_query,
    relate::{
        GetPrograms, get_programs,
        ConfirmUploadCompleted, confirm_upload_completed,
    },
    component::{
        ComModFilesOfFileset, com_mod_files_of_fileset,
        RegisterModificationFileset, register_modification_fileset,
        DeleteModificationFileset, delete_modification_fileset,
        UploadFilesToFileset, upload_files_to_fileset,
    },
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub select_modification_uuid: UUID,
    pub filesets_program: Vec<(UUID, String)>,
}

pub struct ManageModificationFilesets {
    error: Option<Error>,
    request_upload_data: Vec<UploadFile>,
    // request_upload_file: Callback<Result<Option<String>, Error>>,
    request_upload_confirm: Vec<UUID>,
    request_fileset_program_id: usize,
    // task_read: Vec<(FileName, ReaderTask)>,
    // task: Vec<FetchTask>,
    filesets_program: Vec<(UUID, String)>,
    select_fileset_uuid: UUID,
    files_list: Vec<ShowFileInfo>,
    programs: Vec<Program>,
    // put_upload_file: PutUploadFile,
    files: Vec<File>,
    files_index: u32,
    open_add_fileset_card: bool,
    get_result_up_file: bool,
    get_result_up_completed: usize,
    active_loading_files_btn: bool,
}

pub enum Msg {
    RequestProgramsList,
    RequestNewFileset,
    RequestDeleteFileset,
    RequestFilesOfFileset,
    RequestUploadFilesOfFileset,
    // RequestUploadFile(Vec<u8>),
    // ResponseUploadFile(Result<Option<String>, Error>),
    // RequestUploadCompleted,
    ResponseError(Error),
    GetProgramsListResult(String),
    GetNewFilesetResult(String),
    GetDeleteFilesetResult(String),
    GetFilesOfFilesetResult(String),
    GetUploadData(String),
    // GetUploadFile,
    GetUploadCompleted(Result<usize, Error>),
    UpdateFiles(FileList),
    FinishUploadFiles,
    SelectFileset(UUID),
    UpdateSelectProgramId(String),
    ShowAddFilesetCard,
    ClearFilesBoxed,
    ClearError,
    Ignore,
}

impl Component for ManageModificationFilesets {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let select_fileset_uuid = ctx.props().filesets_program
            .first()
            .map(|(fileset_uuid, program_name)| {
                debug!("mod fileset_uuid: {:?}", fileset_uuid);
                debug!("mod program_name: {:?}", program_name);
                fileset_uuid.clone()
            })
            .unwrap_or_default();

        Self {
            error: None,
            request_upload_data: Vec::new(),
            // request_upload_file: ctx.link().callback(Msg::ResponseUploadFile),
            request_upload_confirm: Vec::new(),
            request_fileset_program_id: 1,
            // task_read: Vec::new(),
            // task: Vec::new(),
            filesets_program: ctx.props().filesets_program.clone(),
            select_fileset_uuid,
            files_list: Vec::new(),
            programs: Vec::new(),
            // put_upload_file: PutUploadFile::new(),
            files: Vec::new(),
            files_index: 0,
            open_add_fileset_card: false,
            get_result_up_file: false,
            get_result_up_completed: 0,
            active_loading_files_btn: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::RequestFilesOfFileset);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestProgramsList => {
                spawn_local(async move {
                    let res = make_query(GetPrograms::build_query(
                        get_programs::Variables { ipt_program_arg: None }
                    )).await.unwrap();

                    link.send_message(Msg::GetProgramsListResult(res));
                })
            },
            Msg::RequestNewFileset => {
                let ipt_fileset_program_data = register_modification_fileset::IptFilesetProgramData{
                    modification_uuid: ctx.props().select_modification_uuid.clone(),
                    program_id: self.request_fileset_program_id as i64,
                };
                spawn_local(async move {
                    let res = make_query(RegisterModificationFileset::build_query(
                        register_modification_fileset::Variables { ipt_fileset_program_data }
                    )).await.unwrap();

                    link.send_message(Msg::GetNewFilesetResult(res));
                })
            },
            Msg::RequestDeleteFileset => {
                let del_fileset_program_data = delete_modification_fileset::DelFilesetProgramData{
                    modification_uuid: ctx.props().select_modification_uuid.clone(),
                    fileset_uuid: self.select_fileset_uuid.clone(),
                };
                spawn_local(async move {
                    let res = make_query(DeleteModificationFileset::build_query(
                        delete_modification_fileset::Variables { del_fileset_program_data }
                    )).await.unwrap();

                    link.send_message(Msg::GetDeleteFilesetResult(res));
                })
            },
            Msg::RequestFilesOfFileset => {
                if self.select_fileset_uuid.len() == 36 {
                    let ipt_file_of_fileset_arg = com_mod_files_of_fileset::IptFileOfFilesetArg{
                        fileset_uuid: self.select_fileset_uuid.clone(),
                        file_uuids: None,
                        limit: None,
                        offset: None,
                    };
                    spawn_local(async move {
                        let res = make_query(ComModFilesOfFileset::build_query(com_mod_files_of_fileset::Variables {
                            ipt_file_of_fileset_arg
                        })).await.unwrap();

                        link.send_message(Msg::GetFilesOfFilesetResult(res));
                    })
                }
            },
            // Msg::RequestUploadFilesOfFileset => {
            //     if !self.files.is_empty() && self.select_fileset_uuid.len() == 36 {
            //         // see loading button
            //         self.active_loading_files_btn = true;
            //
            //         let mut filenames: Vec<String> = Vec::new();
            //         for file in &self.files {filenames.push(file.name().clone());}
            //         debug!("filenames: {:?}", filenames);
            //         let fileset_uuid = self.select_fileset_uuid.clone();
            //
            //         spawn_local(async move {
            //             let ipt_modification_file_from_fileset_data = upload_files_to_fileset::IptModificationFileFromFilesetData{
            //                 filesetUuid: fileset_uuid,
            //                 filenames,
            //             };
            //             let res = make_query(UploadFilesToFileset::build_query(
            //                 upload_files_to_fileset::Variables{ ipt_modification_file_from_fileset_data }
            //             )).await.unwrap();
            //             link.send_message(Msg::GetUploadData(res));
            //         })
            //     }
            // },
            // Msg::RequestUploadFile(data) => {
            //     if let Some(upload_data) = self.request_upload_data.pop() {
            //         let request = UploadData {
            //             upload_url: upload_data.upload_url.to_string(),
            //             file_data: data,
            //         };
            //         debug!("request: {:?}", request);
            //
            //         self.task.push(self.put_upload_file.put_file(request, self.request_upload_file.clone()));
            //         self.request_upload_confirm.push(upload_data.file_uuid.clone());
            //     };
            // },
            // Msg::RequestUploadCompleted => {
            //     let file_uuids = self.request_upload_confirm.clone();
            //     spawn_local(async move {
            //         let res = make_query(ConfirmUploadCompleted::build_query(
            //             confirm_upload_completed::Variables { file_uuids }
            //         )).await.unwrap();
            //         // debug!("ConfirmUploadCompleted: {:?}", res);
            //         link.send_message(Msg::GetUploadCompleted(res));
            //     });
            // },
            Msg::ResponseUploadFile(Ok(res)) => {
                debug!("ResponseUploadFile: {:?}", res);
                self.files_index -= 1;
                debug!("next: {:?}", self.files_index);
                // if self.files_index == 0 {
                //     self.get_result_up_file = true;
                //     debug!("finish: {:?}", self.request_upload_confirm.len());
                //     // link.send_message(Msg::RequestUploadCompleted);
                // }
            },
            Msg::ResponseUploadFile(Err(err)) => {
                self.error = Some(err);
                self.task.clear();
                self.task_read.clear();
                self.files_index = 0;
                self.request_upload_confirm.clear();
                self.get_result_up_completed = 0;
                self.active_loading_files_btn = false;
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetProgramsListResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: Vec<Program> = serde_json::from_value(
                            res.get("programs").unwrap().clone()
                        ).unwrap();
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
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetNewFilesetResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        self.select_fileset_uuid = serde_json::from_value(
                            res.get("registerModificationFileset").unwrap().clone()
                        ).unwrap();
                        // debug!("registerModificationFileset: {:?}", self.select_fileset_uuid);

                        // clear shown files (new fileset always empty)
                        self.files_list.clear();

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
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetDeleteFilesetResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(
                            res.get("deleteModificationFileset").unwrap().clone()
                        ).unwrap();
                        // debug!("deleteModificationFileset: {:?}", self.select_fileset_uuid);
                        if result {
                            let mut update_filesets: Vec<(UUID, String)> = Vec::new();

                            // for set next item after delete
                            let delete_fileset_uuid = self.select_fileset_uuid.clone();
                            self.select_fileset_uuid = String::new();
                            let mut flag_delete = false;

                            for x in self.filesets_program.iter() {
                                if flag_delete {
                                    self.select_fileset_uuid = x.0.clone();
                                    flag_delete = false;
                                    // debug!("self.select_fileset_uuid: {:?}", self.select_fileset_uuid);
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

                            ctx.link().send_message(Msg::RequestFilesOfFileset);
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetFilesOfFilesetResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        self.files_list = serde_json::from_value(
                            res.get("componentModificationFilesOfFileset").unwrap().clone()
                        ).unwrap();
                        debug!("componentModificationFilesOfFileset: {:?}", self.files_list.len());
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result = serde_json::from_value(res_value.get("uploadFilesToFileset").unwrap().clone()).unwrap();
                        debug!("uploadFilesToFileset {:?}", self.request_upload_data);

                        if !self.files.is_empty() {
                            let callback_confirm =
                                link.callback(|res: Result<usize, Error>| Msg::GetUploadCompleted(res));
                            storage_upload(result, self.files, callback_confirm);
                            // for file in self.files.iter().rev() {
                            //     let file_name = file.name().clone();
                            //     debug!("file name: {:?}", file_name);
                            //     let task = {
                            //         let callback = ctx.link()
                            //             .callback(move |data: FileData| Msg::RequestUploadFile(data.content));
                            //         ReaderService::read_file(file.clone(), callback).unwrap()
                            //     };
                            //     self.task_read.push((file_name, task));
                            // }
                        }
                        debug!("file: {:#?}", self.files);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            // Msg::GetUploadFile => {
            //     debug!("next: {:?}", self.files_index);
            //     self.files_index -= 1;
            //     if self.files_index == 0 {
            //         self.get_result_up_file = true;
            //         debug!("finish: {:?}", self.request_upload_confirm.len());
            //         link.send_message(Msg::RequestUploadCompleted);
            //     }
            // },
            Msg::GetUploadCompleted(res) => {
                match res {
                    Ok(value) => self.get_result_up_completed = value,
                    Err(err) => self.error = Some(err),
                }
                self.active_loading_files_btn = false;
            },
            Msg::UpdateFiles(files) => {
                while let Some(file) = files.get(self.files_index) {
                    debug!("self.files_index: {:?}", self.files_index);
                    self.files_index += 1;
                    self.active_loading_files_btn = false;
                    self.files.push(file.clone());
                }
                // self.files_index = 0;
            },
            Msg::FinishUploadFiles => {
                self.files_list.clear();
                link.send_message(Msg::RequestFilesOfFileset);
                self.active_loading_files_btn = false;
                self.task.clear();
                self.task_read.clear();
                self.request_upload_confirm.clear();
                self.files.clear();
                self.files_index = 0;
            },
            Msg::SelectFileset(fileset_uuid) => {
                debug!("SelectFileset: {:?}", fileset_uuid);
                self.select_fileset_uuid = fileset_uuid;
                self.files_list.clear();
                ctx.link().send_message(Msg::RequestFilesOfFileset);
            },
            Msg::UpdateSelectProgramId(data) =>
                self.request_fileset_program_id = data.parse::<usize>().unwrap_or_default(),
            Msg::ShowAddFilesetCard => {
                self.open_add_fileset_card = !self.open_add_fileset_card;

                if self.programs.is_empty() {
                    link.send_message(Msg::RequestProgramsList);
                }
            },
            Msg::ClearFilesBoxed => {
                self.files.clear();
                self.files_index = 0;
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().select_modification_uuid == self.select_modification_uuid &&
              ctx.props().filesets_program.len() == self.filesets_program.len() {
            debug!("no change filesets: {:?}", self.filesets_program.len());
            false
        } else {
            debug!("change filesets: {:?}", ctx.props().filesets_program.len());
            self.filesets_program = ctx.props().filesets_program.clone();
            self.select_fileset_uuid = ctx.props().filesets_program
                .first()
                .map(|(fileset_uuid, program_name)| {
                    debug!("mod fileset_uuid: {:?}", fileset_uuid);
                    debug!("mod program_name: {:?}", program_name);
                    fileset_uuid.clone()
                })
                .unwrap_or_default();

            self.files_list.clear();
            ctx.link().send_message(Msg::RequestFilesOfFileset);
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            {self.modal_add_fileset(ctx.link(), ctx.props())}
            {self.show_manage(ctx.link(), ctx.props())}
            // <br/>
            <div class="columns">
                <div class="column">
                    <h2>{ get_value_field(&198) }</h2> // Files of fileset
                    {self.show_fileset_files()}
                </div>
                <div class="column">
                    <h2>{ get_value_field(&197) }</h2> // Upload files for fileset
                    {self.show_frame_upload_files(ctx.link())}
                </div>
            </div>
        </>}
    }
}

impl ManageModificationFilesets {
    fn show_manage(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onchange_select_fileset_btn = link
            .callback(|ev: Event| Msg::SelectFileset(match ev {
              Event::Select(el) => el.value(),
              _ => String::new(),
          }));

        let onclick_new_fileset_card = link.callback(|_| Msg::ShowAddFilesetCard);

        let onclick_delete_fileset_btn = link.callback(|_| Msg::RequestDeleteFileset);

        html!{<div class="columns">
            <div class="column">
                <div class="select is-fullwidth" style="margin-right: .5rem">
                  <select
                        id="select-fileset-program"
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
                <div class="buttons">
                    <button
                      id="delete-fileset-program"
                      class="button is-danger"
                      disabled={self.select_fileset_uuid.is_empty()}
                      onclick={onclick_delete_fileset_btn} >
                        <span class="icon" >
                            <i class="fa fa-trash" aria-hidden="true"></i>
                        </span>
                        <span>{ get_value_field(&135) }</span>
                    </button>
                    <button
                      id="add-modification-fileset"
                      class="button is-success"
                      disabled={props.select_modification_uuid.is_empty()}
                      onclick={onclick_new_fileset_card} >
                        <span class="icon" >
                            <i class="fas fa-plus" aria-hidden="true"></i>
                        </span>
                        <span>{ get_value_field(&196) }</span> // Add fileset
                    </button>
                </div>
            </div>
        </div>}
    }

    fn modal_add_fileset(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_new_fileset_card = link.callback(|_| Msg::ShowAddFilesetCard);

        let onclick_add_fileset_btn = link.callback(|_| Msg::RequestNewFileset);

        let onchange_select_program_id = link
            .callback(|ev: Event| Msg::UpdateSelectProgramId(match ev {
              Event::Select(el) => el.value(),
              _ => String::new(),
          }));

        let class_modal = match &self.open_add_fileset_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_new_fileset_card.clone()} />
            <div class="card">
              <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{ get_value_field(&206) }</p> // Create new fileset
                    <button class="delete" aria-label="close" onclick={onclick_new_fileset_card.clone()} />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <div class="media-content">
                      <label class="label">{ get_value_field(&207) }</label> // Program for fileset
                      <div class="select">
                          <select
                              id="set-fileset-program"
                              select={self.request_fileset_program_id.to_string()}
                              onchange={onchange_select_program_id}
                              >
                            {for self.programs.iter().map(|x|
                                html!{
                                    <option value={x.id.to_string()}
                                          selected={x.id == self.request_fileset_program_id} >
                                        {&x.name}
                                    </option>
                                }
                            )}
                          </select>
                      </div>
                      <br/>
                      <button
                          id="add-fileset-program"
                          class="button"
                          disabled={props.select_modification_uuid.is_empty()}
                          onclick={onclick_add_fileset_btn} >
                          { get_value_field(&117) }
                      </button>
                    </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }

    fn show_fileset_files(&self) -> Html {
        html!{
            <FilesetFilesBlock
                show_download_btn = {false}
                show_delete_btn = {true}
                select_fileset_uuid = {self.select_fileset_uuid.clone()}
                files = {self.files_list.clone()}
            />
        }
    }

    fn show_frame_upload_files(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onchange_upload_files = link.callback(move |value| {
            if let Event::Files(files) = value {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });

        html!{<div class="block">
            <div class="file has-name is-boxed is-centered">
                <label class="file-label" style="width: 100%">
                  <input id="component-file-input"
                  class="file-input"
                  type="file"
                  // accept="image/*,application/vnd*,application/rtf,text/*,.pdf"
                  onchange={onchange_upload_files}
                  multiple=true />
                <span class="file-cta">
                  <span class="file-icon">
                    <i class="fas fa-upload"></i>
                  </span>
                  <span class="file-label">
                    { get_value_field(&195) } // Choose fileset files…
                  </span>
                </span>
                {match self.files.is_empty() {
                    true => html!{<span class="file-name">{ get_value_field(&194) }</span>}, // No file uploaded
                    false => html!{for self.files.iter().map(|f| html!{
                        <span class="file-name">{f.name().clone()}</span>
                    })}
                }}
              </label>
            </div>
            <div class="buttons">
                {self.show_clear_btn(link)}
                {self.show_upload_files_btn(link)}
            </div>
        </div>}
    }

    fn show_clear_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_clear_boxed = link.callback(|_| Msg::ClearFilesBoxed);

        html!{
            <button id="clear-upload-fileset-files"
              class="button"
              onclick={onclick_clear_boxed}
              disabled={self.files.is_empty()} >
                // <span class="icon" >
                //     <i class="fas fa-boom" aria-hidden="true"></i>
                // </span>
                <span>{ get_value_field(&88) }</span>
            </button>
        }
    }

    fn show_upload_files_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_upload_files = link.callback(|_| Msg::RequestUploadFilesOfFileset);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <button
              id="upload-fileset-files"
              class={class_upload_btn}
              disabled={self.files.is_empty() || self.select_fileset_uuid.len() != 36}
              onclick={onclick_upload_files} >
                // <span class="icon" >
                //     <i class="fas fa-angle-double-up" aria-hidden="true"></i>
                // </span>
                <span>{ get_value_field(&87) }</span>
            </button>
        }
    }
}
