use std::collections::BTreeSet;
use yew::{Component, Context, html, Html, Properties};
use yew::html::{Scope, TargetCast};
use gloo::file::File;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use super::ModificationFileItem;
use crate::services::storage_upload::{storage_upload, prepare_files};
use crate::services::get_value_field;
use crate::error::{get_error, Error};
use crate::fragments::files_frame::FilesFrame;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo};
use crate::gqls::make_query;
use crate::gqls::component::{
    ComponentModificationFilesList, component_modification_files_list,
    UploadModificationFiles, upload_modification_files,
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_download_btn: bool,
    pub modification_uuid: UUID,
}

pub struct ManageModificationFilesCard {
    error: Option<Error>,
    modification_uuid: UUID,
    files_list: Vec<ShowFileInfo>,
    files_deleted_list: BTreeSet<UUID>,
    files: Vec<File>,
    files_index: u32, // count_files?
    get_result_up_completed: usize,
    active_loading_files_btn: bool,
    show_full_files: bool,
}

#[derive(Clone)]
pub enum Msg {
    RequestModificationFilesList,
    RequestUploadModificationFiles,
    ResponseError(Error),
    GetModificationFilesListResult(String),
    GetUploadData(String),
    GetUploadCompleted(Result<usize, Error>),
    UpdateFiles(Option<FileList>),
    FinishUploadFiles,
    ShowFullList,
    RemoveFile(UUID),
    ClearFilesBoxed,
    ClearError,
    Ignore,
}

impl Component for ManageModificationFilesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            modification_uuid: ctx.props().modification_uuid.clone(),
            files_list: Vec::new(),
            files_deleted_list: BTreeSet::new(),
            files: Vec::new(),
            files_index: 0,
            get_result_up_completed: 0,
            active_loading_files_btn: false,
            show_full_files: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render && ctx.props().modification_uuid.len() == 36 {
            debug!("First render modification files list");
            ctx.link().send_message(Msg::RequestModificationFilesList);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestModificationFilesList => {
                let modification_uuid = ctx.props().modification_uuid.clone();
                spawn_local(async move {
                    let ipt_modification_files_arg = component_modification_files_list::IptModificationFilesArg{
                        files_uuids: None,
                        modification_uuid,
                        limit: None,
                        offset: None,
                    };
                    let res = make_query(ComponentModificationFilesList::build_query(
                        component_modification_files_list::Variables { ipt_modification_files_arg }
                    )).await.unwrap();
                    link.send_message(Msg::GetModificationFilesListResult(res));
                })
            },
            Msg::RequestUploadModificationFiles => {
                if !self.files.is_empty() && ctx.props().modification_uuid.len() == 36 {
                    // see loading button
                    self.active_loading_files_btn = true;
                    let mut filenames: Vec<String> = Vec::new();
                    for file in &self.files {
                        filenames.push(file.name().clone());
                    }
                    debug!("filenames: {:?}", filenames);
                    let modification_uuid = ctx.props().modification_uuid.clone();

                    spawn_local(async move {
                        let ipt_modification_files_data = upload_modification_files::IptModificationFilesData{
                            modification_uuid,
                            filenames,
                        };
                        let res = make_query(UploadModificationFiles::build_query(
                            upload_modification_files::Variables{ ipt_modification_files_data }
                        )).await.unwrap();
                        link.send_message(Msg::GetUploadData(res));
                    })
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetModificationFilesListResult(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.files_list = serde_json::from_value(
                            res_value.get("componentModificationFilesList").unwrap().clone()
                        ).unwrap();
                        debug!("componentModificationFilesList {:?}", self.files_list.len());
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result = serde_json::from_value(
                            res_value.get("uploadModificationFiles").unwrap().clone()
                        ).unwrap();
                        // debug!("uploadModificationFiles {:?}", self.request_upload_data);

                        if !self.files.is_empty() {
                            let callback_confirm =
                                link.callback(|res: Result<usize, Error>| Msg::GetUploadCompleted(res));
                            storage_upload(result, self.files.clone(), callback_confirm);
                        }
                        debug!("file: {:#?}", self.files);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetUploadCompleted(res) => {
                match res {
                    Ok(value) => self.get_result_up_completed = value,
                    Err(err) => self.error = Some(err),
                }
                self.active_loading_files_btn = false;
            },
            Msg::UpdateFiles(file_list) => {
                prepare_files(&file_list, &mut self.files);
            },
            Msg::FinishUploadFiles => {
                self.files_list.clear();
                link.send_message(Msg::RequestModificationFilesList);
                self.active_loading_files_btn = false;
                self.files.clear();
                self.files_index = 0;
            },
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            Msg::RemoveFile(file_uuid) => {
                self.files_deleted_list.insert(file_uuid);
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
        if self.modification_uuid == ctx.props().modification_uuid {
            debug!("not update modification files {:?}", self.modification_uuid);
            false
        } else {
            debug!("update modification files {:?}", ctx.props().modification_uuid);
            self.files_deleted_list.clear();
            self.files_list.clear();
            if ctx.props().modification_uuid.len() == 36 {
                ctx.link().send_message(Msg::RequestModificationFilesList);
            }
            self.modification_uuid = ctx.props().modification_uuid.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            <div class="columns">
                <div class="column">
                  <h2>{ get_value_field(&203) }</h2> // Files for modification
                  {self.show_files_list(ctx.link(), ctx.props())}
                </div>
                <div class="column">
                  <h2>{ get_value_field(&202) }</h2> // Upload modification files
                  {self.show_frame_upload_files(ctx.link(), ctx.props())}
                </div>
            </div>
        </>}
    }
}

impl ManageModificationFilesCard {
    fn show_files_list(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        html!{<>
            {for self.files_list.iter().enumerate().map(|(index, file)| {
                match (index >= 3, self.show_full_files) {
                    // show full list
                    (_, true) => self.show_file_info(link, props, &file),
                    // show full list or first 3 items
                    (false, false) => self.show_file_info(link, props, &file),
                    _ => html!{},
                }
            })}
            {match self.files_list.len() {
                0 => html!{<span>{ get_value_field(&204) }</span>},
                0..=3 => html!{},
                _ => self.show_see_btn(link),
            }}
        </>}
    }

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
                <ModificationFileItem
                  show_download_btn = {props.show_download_btn}
                  show_delete_btn = {true}
                  modification_uuid = {props.modification_uuid.clone()}
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

    fn show_frame_upload_files(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onchange = link.callback(move |ev: Event| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateFiles(input.files())
        });
        let ondrop = link.callback(move |ev: DragEvent| {
            ev.prevent_default();
            Msg::UpdateFiles(ev.data_transfer().unwrap().files())
        });
        let ondragover = link.callback(move |ev: DragEvent| {
            ev.prevent_default();
            Msg::Ignore
        });
        let ondragenter = ondragover.clone();

        html!{<>
            <div class="file has-name is-boxed is-centered">
                <FilesFrame
                    {onchange}
                    {ondrop}
                    {ondragover}
                    {ondragenter}
                    input_id={"component-file-input".to_string()}
                    multiple={true}
                    file_label={201} // Choose fileset files…
                />
                // </label> todo!(Исправить стиль: сделать обёртку для рамки и выбранных файлов)
                {match self.files.is_empty() {
                    true => html!{<span class="file-name">{ get_value_field(&194) }</span>}, // No file uploaded
                    false => html!{for self.files.iter().map(|f| html!{
                        <span class="file-name">{f.name().clone()}</span>
                    })}
                }}
            </div>
            <div class="buttons">
                {self.show_clear_btn(link)}
                {self.show_upload_files_btn(link, props)}
            </div>
        </>}
    }

    fn show_clear_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_clear_boxed = link.callback(|_| Msg::ClearFilesBoxed);

        html!{
            <button id="clear-upload-modification-files"
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
        props: &Props,
    ) -> Html {
        let onclick_upload_files = link.callback(|_| Msg::RequestUploadModificationFiles);
        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <button
              id="upload-modification-files"
              class={class_upload_btn}
              disabled={self.files.is_empty() || props.modification_uuid.len() != 36}
              onclick={onclick_upload_files} >
                // <span class="icon" >
                //     <i class="fas fa-angle-double-up" aria-hidden="true"></i>
                // </span>
                <span>{ get_value_field(&87) }</span>
            </button>
        }
    }
}
