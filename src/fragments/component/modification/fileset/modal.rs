use std::collections::HashSet;

use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::services::{get_value_field, resp_parsing, unique_id};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::modal::ModalBlock;
use crate::types::{UUID, Program, FilesetProgramInfo};
use crate::gqls::make_query;
use crate::gqls::relate::{GetPrograms, get_programs};
use crate::gqls::component::{RegisterModificationFileset, register_modification_fileset};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub select_modification_uuid: UUID,
    pub existing_program_ids: HashSet<usize>,
    pub is_active: bool,
    pub on_close: Callback<()>,
    pub on_fileset_created: Callback<FilesetProgramInfo>,
}

pub struct AddModificationFilesetsModal {
    error: Option<Error>,
    selected_program_id: usize,
    search_query: String,
    props: Props,
    link: ComponentLink<Self>,
    programs: Vec<Program>,
    suitable_programs: Vec<Program>,
    filtered_programs: Vec<Program>,
    loading: bool,
    creating: bool,
}

pub enum Msg {
    RequestProgramsList,
    RequestNewFileset,
    ResponseError(Error),
    GetProgramsListResult(String),
    Reparse,
    GetNewFilesetResult(String),
    UpdateSearchQuery(String),
    UpdateFilteredPrograms,
    SelectProgram(usize),
    CloseModal,
    ClearError,
}

impl Component for AddModificationFilesetsModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            selected_program_id: 0,
            search_query: String::new(),
            props,
            link,
            programs: Vec::new(),
            suitable_programs: Vec::new(),
            filtered_programs: Vec::new(),
            loading: false,
            creating: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestProgramsList => {
                self.loading = true;
                spawn_local(async move {
                    let res = make_query(GetPrograms::build_query(
                        get_programs::Variables { program_ids: None }
                    )).await.unwrap();
                    link.send_message(Msg::GetProgramsListResult(res));
                });
            }
            Msg::RequestNewFileset => {
                if self.selected_program_id == 0 {
                    return true;
                }
                self.creating = true;
                let ipt_fileset_program_data = register_modification_fileset::IptFilesetProgramData{
                    modificationUuid: self.props.select_modification_uuid.clone(),
                    programId: self.selected_program_id as i64,
                };
                spawn_local(async move {
                    let res = make_query(RegisterModificationFileset::build_query(
                        register_modification_fileset::Variables { ipt_fileset_program_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetNewFilesetResult(res));
                });
            }
            Msg::GetProgramsListResult(res) => {
                self.loading = false;
                match resp_parsing::<Vec<Program>>(res, "programs") {
                    Ok(result) => {
                        self.programs = result;
                        self.link.send_message(Msg::Reparse);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::Reparse => {
                self.suitable_programs = self.programs
                    .iter()
                    .filter(|p| !self.props.existing_program_ids.contains(&p.id))
                    .cloned()
                    .collect();
                self.link.send_message(Msg::UpdateFilteredPrograms);
            }
            Msg::GetNewFilesetResult(res) => {
                self.creating = false;
                match resp_parsing::<UUID>(res, "registerModificationFileset") {
                    Ok(new_uuid) => {
                        debug!("registerModificationFileset: {:?}", new_uuid);
                        if let Some(program) = self.programs.iter().find(|p| p.id == self.selected_program_id) {
                            let new_fileset = FilesetProgramInfo {
                                uuid: new_uuid,
                                modification_uuid: self.props.select_modification_uuid.clone(),
                                program: program.clone(),
                                files_count: 0,
                            };
                            self.props.on_fileset_created.emit(new_fileset);
                        }
                        self.link.send_message(Msg::CloseModal);
                    }
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::UpdateSearchQuery(query) => {
                self.search_query = query;
                self.link.send_message(Msg::UpdateFilteredPrograms);
            }
            Msg::UpdateFilteredPrograms => {
                let query = self.search_query.to_lowercase();
                self.filtered_programs = self.suitable_programs
                    .iter()
                    .filter(|p| query.is_empty() || p.name.to_lowercase().contains(&query))
                    .cloned()
                    .collect();
                if self.filtered_programs.len() == 1 {
                    if let Some(p) = self.filtered_programs.first() {
                        if p.name.to_lowercase() == query {
                            let program_id = p.id;
                            self.link.send_message(Msg::SelectProgram(program_id))
                        }
                    }
                }
                // Reset selection if current selected program no longer matches filter
                if !self.filtered_programs.iter().any(|p| p.id == self.selected_program_id) {
                    self.selected_program_id = 0;
                }
            }
            Msg::SelectProgram(program_id) => {
                self.selected_program_id = program_id;
                if let Some(program) = self.suitable_programs.iter().find(|p| p.id == program_id) {
                    self.search_query = program.name.clone();
                }
            }
            Msg::ResponseError(err) => {
                self.error = Some(err);
                self.loading = false;
                self.creating = false;
            }
            Msg::CloseModal => {
                self.props.on_close.emit(());
                self.search_query.clear();
                self.selected_program_id = 0;
            }
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_modification_uuid == props.select_modification_uuid &&
            self.props.is_active == props.is_active {
            false
        } else {
            self.props = props;
            if self.props.is_active {
                if self.programs.is_empty() && !self.loading {
                    self.link.send_message(Msg::RequestProgramsList);
                } else {
                    self.link.send_message(Msg::Reparse);
                }
            }
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html! {<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
            <ModalBlock
                modal_id="add-fileset-modal"
                title={get_value_field(&206)}
                is_active={self.props.is_active}
                on_close={self.link.callback(|_| Msg::CloseModal)}
                on_save={Some(self.link.callback(|_| Msg::RequestNewFileset))}
                save_disabled={self.selected_program_id == 0 || self.creating}
            >
                {self.modal_content()}
            </ModalBlock>
        </>}
    }
}

impl AddModificationFilesetsModal {
    fn modal_content(&self) -> Html {
        let on_search = self.link.callback(|ev: InputData| Msg::UpdateSearchQuery(ev.value));
        let search_id = unique_id("program-search");

        html! {<>
            <div class="block">
                <div class="field">
                    <div class="control has-icons-left">
                        <input
                            id={search_id}
                            class="input"
                            type="text"
                            placeholder=get_value_field(&500)
                            value={self.search_query.clone()}
                            oninput={on_search}
                            disabled={self.creating}
                            autocomplete="off"
                        />
                        <span class="icon is-left">
                            <i class="fas fa-search"></i>
                        </span>
                    </div>
                </div>
            </div>
            <div class="field">
                <label class="label">{get_value_field(&499)}</label>
                <div class="panel p-2">
                    {if self.loading && self.programs.is_empty() {
                        html!{
                            <div class="panel-block is-justify-content-center has-text-grey py-4">
                                <span class="icon mr-2"><i class="fas fa-spinner fa-pulse"></i></span>
                                {get_value_field(&452)}
                            </div>
                        }
                    } else if self.filtered_programs.is_empty() {
                        html!{
                            <div class="panel-block is-justify-content-center has-text-grey-light py-4">
                                <span class="icon mr-2"><i class="fas fa-exclamation-triangle"></i></span>
                                {get_value_field(&324)}
                            </div>
                        }
                    } else {
                        html! {
                            {for self.filtered_programs.iter().map(|program|
                                self.program_item(program)
                            )}
                        }
                    }}
                </div>
            </div>
            {if self.creating {
                html!{
                    <div class="notification is-small is-info is-light mt-3 py-2 has-text-centered">
                        <span class="icon mr-1"><i class="fas fa-spinner fa-pulse"></i></span>
                        {get_value_field(&501)}
                    </div>
                }
            } else {
                html!{}
            }}
        </>}
    }

    fn program_item(&self, program: &Program) -> Html {
        let program_id = program.id;
        let is_selected = self.selected_program_id == program_id;
        let block_class = if is_selected {
            "panel-block is-active has-background-link-light has-text-link"
        } else {
            "panel-block"
        };
        let selected_icon = match is_selected {
            true => "fas fa-check-circle",
            false => "far fa-circle",
        };

        html! {
            <a
                class={block_class}
                onclick={self.link.callback(move |_| Msg::SelectProgram(program_id))}
            >
                <span class="panel-icon">
                    <i class={selected_icon} aria-hidden="true"></i>
                </span>
                <span class="is-size-6">{&program.name}</span>
            </a>
        }
    }
}