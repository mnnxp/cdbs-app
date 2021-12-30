use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html, ChangeData};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use chrono::NaiveDateTime;

use super::FilesOfFilesetCard;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, ShowFileInfo, Program};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComModFilesOfFileset;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct RegisterModificationFileset;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct GetPrograms;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub select_modification_uuid: UUID,
    pub filesets_program: Vec<(UUID, String)>,
}

pub struct ManageModificationFilesets {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    filesets_program: Vec<(UUID, String)>,
    select_fileset_uuid: UUID,
    files_data: Vec<ShowFileInfo>,
    programs: Vec<Program>,
    request_fileset_program_id: usize,
    open_add_fileset_card: bool,
}

pub enum Msg {
    RequestProgramsList,
    RequestNewFileset,
    RequestFilesOfFileset,
    ResponseError(Error),
    GetProgramsListResult(String),
    GetNewFilesetResult(String),
    GetFilesOfFilesetResult(String),
    SelectFileset(UUID),
    UpdateSelectProgramId(String),
    ShowAddFilesetCard,
    ClearError,
}

impl Component for ManageModificationFilesets {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let filesets_program = props.filesets_program.clone();
        let select_fileset_uuid = props.filesets_program
            .first()
            .map(|(fileset_uuid, program_name)| {
                debug!("mod fileset_uuid: {:?}", fileset_uuid);
                debug!("mod program_name: {:?}", program_name);
                fileset_uuid.clone()
            })
            .unwrap_or_default();

        Self {
            error: None,
            props,
            link,
            filesets_program,
            select_fileset_uuid,
            files_data: Vec::new(),
            programs: Vec::new(),
            request_fileset_program_id: 1,
            open_add_fileset_card: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.select_fileset_uuid.len() == 36 {
            self.link.send_message(Msg::RequestFilesOfFileset);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

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
            Msg::RequestFilesOfFileset => {
                let ipt_file_of_fileset_arg = com_mod_files_of_fileset::IptFileOfFilesetArg{
                    filesetUuid: self.select_fileset_uuid.clone(),
                    fileUuids: None,
                    limit: None,
                    offset: None,
                };
                spawn_local(async move {
                    let res = make_query(ComModFilesOfFileset::build_query(com_mod_files_of_fileset::Variables {
                        ipt_file_of_fileset_arg
                    })).await.unwrap();

                    link.send_message(Msg::GetFilesOfFilesetResult(res));
                })
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
                        self.programs = Vec::new();
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
            Msg::GetFilesOfFilesetResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: Vec<ShowFileInfo> = serde_json::from_value(
                            res.get("componentModificationFilesOfFileset").unwrap().clone()
                        ).unwrap();
                        // debug!("componentModificationFilesOfFileset: {:?}", result);
                        self.files_data = result;
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::SelectFileset(fileset_uuid) => self.select_fileset_uuid = fileset_uuid,
            Msg::UpdateSelectProgramId(data) =>
                self.request_fileset_program_id = data.parse::<usize>().unwrap_or_default(),
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
        if self.props.select_modification_uuid == props.select_modification_uuid &&
              self.props.filesets_program.len() == props.filesets_program.len() {
            debug!("no change filesets: {:?}", props.filesets_program.len());
            false
        } else {
            debug!("change filesets: {:?}", props.filesets_program.len());
            self.filesets_program = props.filesets_program.clone();
            self.select_fileset_uuid = props.filesets_program
                .first()
                .map(|(fileset_uuid, program_name)| {
                    debug!("mod fileset_uuid: {:?}", fileset_uuid);
                    debug!("mod program_name: {:?}", program_name);
                    fileset_uuid.clone()
                })
                .unwrap_or_default();

            if self.select_fileset_uuid.len() == 36 {
                self.link.send_message(Msg::RequestFilesOfFileset);
            }

            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {self.modal_add_fileset()}
            {self.show_manage()}
            // <h3>{"Files of select fileset"}</h3>
            {self.show_fileset_card()}
        </>}
    }
}

impl ManageModificationFilesets {
    fn show_manage(&self) -> Html {
        let onchange_select_fileset_btn = self.link
            .callback(|ev: ChangeData| Msg::SelectFileset(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
          }));

        let onclick_new_fileset_card = self.link.callback(|_| Msg::ShowAddFilesetCard);

        html!{<div class="columns">
            <div class="column">
                <div class="select is-fullwidth" style="margin-right: .5rem">
                  <select
                        id="select-fileset-program"
                        onchange=onchange_select_fileset_btn >
                      {for self.filesets_program.iter().map(|(fileset_uuid, program_name)|
                          match &self.select_fileset_uuid == fileset_uuid {
                            true => html!{<option value={fileset_uuid.clone()} selected=true>{program_name}</option>},
                            false => html!{<option value={fileset_uuid.clone()}>{program_name}</option>},
                          }
                      )}
                  </select>
                </div>
            </div>
            <div class="column">
                <button
                      id="add-modification-fileset"
                      class="button is-fullwidth"
                      onclick={onclick_new_fileset_card} >
                    <span class="icon" >
                        <i class="fas fa-plus" aria-hidden="true"></i>
                    </span>
                    <span>{"Add fileset"}</span>
                </button>
            </div>
        </div>}
    }

    fn modal_add_fileset(&self) -> Html {
        let onclick_new_fileset_card = self.link.callback(|_| Msg::ShowAddFilesetCard);

        let onclick_add_fileset_btn = self.link.callback(|_| Msg::RequestNewFileset);

        let onchange_select_program_id = self.link
            .callback(|ev: ChangeData| Msg::UpdateSelectProgramId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
          }));

        let class_modal = match &self.open_add_fileset_card {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class=class_modal>
          <div class="modal-background" onclick=onclick_new_fileset_card.clone() />
            <div class="card">
              <div class="modal-content">
                <header class="modal-card-head">
                    <p class="modal-card-title">{"Create new fileset"}</p>
                    <button class="delete" aria-label="close" onclick=onclick_new_fileset_card.clone() />
                </header>
                <div class="box itemBox">
                  <article class="media center-media">
                      <div class="media-content">
                      <label class="label">{"Program for fileset"}</label>
                      <div class="select">
                          <select
                              id="set-fileset-program"
                              select={self.request_fileset_program_id.to_string()}
                              onchange=onchange_select_program_id
                              >
                            {for self.programs.iter().map(|x|
                                match self.request_fileset_program_id == x.id {
                                    true => html!{<option value={x.id.to_string()} selected=true>{&x.name}</option>},
                                    false => html!{<option value={x.id.to_string()}>{&x.name}</option>},
                                }
                            )}
                          </select>
                      </div>
                      <br/>
                      <button
                          id="add-component-modification"
                          class="button"
                          disabled={self.props.select_modification_uuid.is_empty()}
                          onclick={onclick_add_fileset_btn} >
                          {"Add"}
                      </button>
                    </div>
                  </article>
                </div>
              </div>
          </div>
        </div>}
    }

    fn show_fileset_card(&self) -> Html {
        html!{
            <FilesOfFilesetCard
                show_manage_btn = true
                fileset_uuid = self.select_fileset_uuid.clone()
            />
        }
    }
}
