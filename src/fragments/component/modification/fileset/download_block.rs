use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html, ChangeData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::file::{FileHeadersShow, FileDownItemShow};
use crate::types::{UUID, DownloadFile, FilesetProgramInfo};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::component::{ComModFilesetFiles, com_mod_fileset_files};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub select_modification_uuid: UUID,
    pub current_filesets_program: Vec<FilesetProgramInfo>,
    pub callback_select_fileset_uuid: Callback<UUID>,
    pub callback_open_fileset: Callback<bool>,
}

pub struct ManageFilesOfFilesetBlock {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    select_fileset_uuid: UUID,
    open_fileset_files_card: bool,
    open_modal_download_files: bool,
    file_arr: Vec<DownloadFile>,
    flag_get_dowload_url: bool,
    active_loading_files_btn: bool,
}

pub enum Msg {
    RequestDownloadFilesetFiles,
    ResponseError(Error),
    GetDownloadFilesetFilesResult(String),
    ParseFirstFilesetUuid,
    SelectFilesetUuid(UUID),
    ShowModalDownloadFiles,
    OpenFilesetFilesBlock,
    ClearError,
}

impl Component for ManageFilesOfFilesetBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            select_fileset_uuid: String::new(),
            open_fileset_files_card: false,
            open_modal_download_files: false,
            file_arr: Vec::new(),
            flag_get_dowload_url: false,
            active_loading_files_btn: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFilesetFiles => {
                debug!("Select fileset: {:?}", self.select_fileset_uuid);
                if self.select_fileset_uuid.len() == 36 {
                    // set active loading button
                    self.active_loading_files_btn = true;
                    let ipt_file_of_fileset_arg = com_mod_fileset_files::IptFileOfFilesetArg{
                        filesetUuid: self.select_fileset_uuid.clone(),
                        fileUuids: None,
                    };
                    spawn_local(async move {
                        let res = make_query(ComModFilesetFiles::build_query(com_mod_fileset_files::Variables {
                            ipt_file_of_fileset_arg
                        })).await.unwrap();

                        link.send_message(Msg::GetDownloadFilesetFilesResult(res));
                    })
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDownloadFilesetFilesResult(res) => {
                match resp_parsing(res, "componentModificationFilesetFiles") {
                    Ok(result) => {
                        self.file_arr = result;
                        debug!("componentModificationFilesetFiles: {:?}", self.file_arr);
                        self.flag_get_dowload_url = true;
                        self.open_modal_download_files = true;
                        self.active_loading_files_btn = false;
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ParseFirstFilesetUuid => {
                self.select_fileset_uuid = self.props.current_filesets_program
                    .first()
                    .map(|fd| {
                        debug!("mod fileset_uuid: {:?}", fd.uuid);
                        debug!("mod program_name: {:?}", fd.program.name);
                        fd.uuid.clone()
                    })
                    .unwrap_or_default();
                self.props.callback_select_fileset_uuid.emit(self.select_fileset_uuid.clone());
            },
            Msg::SelectFilesetUuid(fileset_uuid) => {
                self.props.callback_select_fileset_uuid.emit(fileset_uuid.clone());
                self.select_fileset_uuid = fileset_uuid;
                // for get new download urls
                self.flag_get_dowload_url = false;
            },
            Msg::ShowModalDownloadFiles => self.open_modal_download_files = !self.open_modal_download_files,
            Msg::OpenFilesetFilesBlock => {
                self.open_fileset_files_card = !self.open_fileset_files_card;
                self.props.callback_open_fileset.emit(self.open_fileset_files_card);
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_modification_uuid == props.select_modification_uuid &&
            self.props.current_filesets_program.len() == props.current_filesets_program.len() {
            debug!("no change download block: {:?}", props.select_modification_uuid);
            false
        } else {
            debug!("change download block: {:?}", props.select_modification_uuid);
            self.props = props;
            self.link.send_message(Msg::ParseFirstFilesetUuid);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {self.modal_download_files()}
            {self.show_download_block()}
        </>}
    }
}

impl ManageFilesOfFilesetBlock {
    fn show_download_block(&self) -> Html {
        let onchange_select_fileset_btn =
            self.link.callback(|ev: ChangeData| Msg::SelectFilesetUuid(match ev {
                ChangeData::Select(el) => el.value(),
                _ => String::new(),
            }));
        let onclick_open_fileset_files_list_btn =
            self.link.callback(|_| Msg::OpenFilesetFilesBlock);
        let onclick_download_fileset_btn = match self.flag_get_dowload_url {
            true => self.link.callback(|_| Msg::ShowModalDownloadFiles),
            false => self.link.callback(|_| Msg::RequestDownloadFilesetFiles),
        };
        let class_fileset_btn = match self.open_fileset_files_card {
            true => "button is-light is-active",
            false => "button",
        };
        let class_download_btn = match self.active_loading_files_btn {
            true => "button is-info is-active is-loading",
            false => "button is-info",
        };

        html!{<div style="margin-right: .5rem">
            <div class="select" style="margin-right: .5rem">
                <select
                id={"select-fileset-program-download"}
                select={self.select_fileset_uuid.clone()}
                onchange={onchange_select_fileset_btn}
                title={get_value_field(&207)}>
                  {for self.props.current_filesets_program.iter().map(|fd|
                      html!{
                          <option value={fd.uuid.to_string()}
                                selected={fd.uuid == self.select_fileset_uuid} >
                              {fd.program.name.clone()}
                          </option>
                      }
                  )}
                </select>
            </div>
            <button
            class={class_fileset_btn}
            // disabled = self.select_fileset_uuid.len() != 36
            onclick={onclick_open_fileset_files_list_btn}
            title={get_value_field(&106)}>
                <span class="icon is-small"><i class="fa fa-list"></i></span>
            </button>
            <button
            class={class_download_btn}
            disabled={self.select_fileset_uuid.len() != 36}
            onclick={onclick_download_fileset_btn}
            title={get_value_field(&126)}>
              <span class="has-text-weight-bold">{get_value_field(&126)}</span>
            </button>
        </div>}
    }

    fn modal_download_files(&self) -> Html {
        let onclick_modal_download_btn =
            self.link.callback(|_| Msg::ShowModalDownloadFiles);
        let class_modal = match &self.open_modal_download_files {
            true => "modal is-active",
            false => "modal",
        };
        let mut fileset_name = "";
        for fd in self.props.current_filesets_program.iter() {
            if fd.uuid == self.select_fileset_uuid {
                fileset_name = &fd.program.name;
                break;
            }
        }

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_modal_download_btn.clone()} />
            <div class="card">
              <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">
                        {get_value_field(&138)}
                        {fileset_name}
                    </p>
                    <button class="delete" aria-label="close" onclick={onclick_modal_download_btn.clone()} />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <div class="media-content">
                        <table class="table is-fullwidth is-striped">
                          <FileHeadersShow show_long={false} />
                          <tbody>
                            {for self.file_arr.iter().map(|file| html!{
                              <FileDownItemShow file_down={file.clone()} />
                            })}
                          </tbody>
                        </table>
                      </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }
}
