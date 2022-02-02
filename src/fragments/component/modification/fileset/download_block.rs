use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html, ChangeData};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
// use chrono::NaiveDateTime;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, DownloadFile};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComModFilesetFiles;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub select_modification_uuid: UUID,
    pub current_filesets_program: Vec<(UUID, String)>,
    pub callback_select_fileset_uuid: Callback<UUID>,
    pub callback_open_fileset_uuid: Callback<bool>,
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
        let select_fileset_uuid = props.current_filesets_program
            .first()
            .map(|(fileset_uuid, program_name)| {
                debug!("create block mod fileset_uuid: {:?}", fileset_uuid);
                debug!("create block mod program_name: {:?}", program_name);
                fileset_uuid.clone()
            })
            .unwrap_or_default();

        Self {
            error: None,
            props,
            link,
            select_fileset_uuid,
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
                        limit: None,
                        offset: None,
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
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        self.file_arr = serde_json::from_value(
                            res.get("componentModificationFilesetFiles").unwrap().clone()
                        ).unwrap();
                        debug!("componentModificationFilesetFiles: {:?}", self.file_arr);

                        self.flag_get_dowload_url = true;
                        self.open_modal_download_files = true;
                        self.active_loading_files_btn = false;
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::ParseFirstFilesetUuid => {
                self.select_fileset_uuid = self.props.current_filesets_program
                    .first()
                    .map(|(fileset_uuid, program_name)| {
                        debug!("mod fileset_uuid: {:?}", fileset_uuid);
                        debug!("mod program_name: {:?}", program_name);
                        fileset_uuid.clone()
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
                self.props.callback_open_fileset_uuid.emit(self.open_fileset_files_card);
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_modification_uuid == props.select_modification_uuid {
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
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {self.modal_download_files()}
            {self.show_download_block()}
        </>}
    }
}

impl ManageFilesOfFilesetBlock {
    fn show_download_block(&self) -> Html {
        let onchange_select_fileset_btn = self.link
            .callback(|ev: ChangeData| Msg::SelectFilesetUuid(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
          }));

        let onclick_open_fileset_files_list_btn = self.link
            .callback(|_| Msg::OpenFilesetFilesBlock);

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
                    id="select-fileset-program"
                    select={self.select_fileset_uuid.clone()}
                    onchange=onchange_select_fileset_btn >
                  {for self.props.current_filesets_program.iter().map(|(fileset_uuid, program_name)|
                      html!{
                          <option value={fileset_uuid.to_string()}
                                selected={fileset_uuid == &self.select_fileset_uuid} >
                              {program_name}
                          </option>
                      }
                  )}
              </select>
            </div>
            <button class={class_fileset_btn}
                // disabled = self.select_fileset_uuid.len() != 36
                onclick = onclick_open_fileset_files_list_btn >
                <span class="icon is-small"><i class="fa fa-list"></i></span>
            </button>
            <button class=class_download_btn
                disabled = self.select_fileset_uuid.len() != 36
                onclick=onclick_download_fileset_btn >
              <span class="has-text-weight-bold">{"Download"}</span>
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

        html!{<div class=class_modal>
          <div class="modal-background" onclick=onclick_modal_download_btn.clone() />
            <div class="card">
              <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{"Temp solution for download files"}</p>
                    <button class="delete" aria-label="close" onclick=onclick_modal_download_btn.clone() />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <div class="media-content">
                          <table class="table is-fullwidth">
                              <thead>
                                <tr>
                                  <th>{"Filename"}</th>
                                  <th>{"Filesize"}</th>
                                  <th>{"Download"}</th>
                                </tr>
                              </thead>
                            <tbody>
                              {for self.file_arr.iter().map(|file| html!{
                                  <tr>
                                    <td>{file.filename.clone()}</td>
                                    <td>{file.filesize.clone()}</td>
                                    <td>
                                        <a class="button is-ghost" href={file.download_url.clone()}  target="_blank">
                                          <span class="icon" >
                                            <i class="fas fa-file-download" aria-hidden="true"></i>
                                          </span>
                                        </a>
                                    </td>
                                  </tr>
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
