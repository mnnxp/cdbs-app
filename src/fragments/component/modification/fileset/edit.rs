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
use crate::types::{UUID, ShowFileInfo};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComModFilesOfFileset;

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
    select_fileset_program: UUID,
    files_data: Vec<ShowFileInfo>,
}

pub enum Msg {
    RequestFilesOfFileset,
    ResponseError(Error),
    GetFilesOfFilesetResult(String),
    SelectFileset(UUID),
    ClearError,
}

impl Component for ManageModificationFilesets {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let filesets_program = props.filesets_program.clone();
        let select_fileset_program = props.filesets_program
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
            select_fileset_program,
            files_data: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.select_fileset_program.len() == 36 {
            self.link.send_message(Msg::RequestFilesOfFileset);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestFilesOfFileset => {
                let ipt_file_of_fileset_arg = com_mod_files_of_fileset::IptFileOfFilesetArg{
                    filesetUuid: self.select_fileset_program.clone(),
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
            Msg::GetFilesOfFilesetResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: Vec<ShowFileInfo> = serde_json::from_value(
                            res.get("componentModificationFilesOfFileset").unwrap().clone()).unwrap();
                        // debug!("componentModificationFilesOfFileset: {:?}", result);
                        self.files_data = result;
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::SelectFileset(fileset_uuid) => self.select_fileset_program = fileset_uuid,
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
            self.select_fileset_program = props.filesets_program
                .first()
                .map(|(fileset_uuid, program_name)| {
                    debug!("mod fileset_uuid: {:?}", fileset_uuid);
                    debug!("mod program_name: {:?}", program_name);
                    fileset_uuid.clone()
                })
                .unwrap_or_default();

            if self.select_fileset_program.len() == 36 {
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

        html!{<div class="columns">
            <div class="column">
                <div class="select is-fullwidth" style="margin-right: .5rem">
                  <select
                        id="select-fileset-program"
                        onchange=onchange_select_fileset_btn >
                      {for self.filesets_program.iter().map(|(fileset_uuid, program_name)|
                          match &self.select_fileset_program == fileset_uuid {
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
                    // onclick={onclick_action_btn}
                    >
                    <span class="icon" >
                        <i class="fas fa-plus" aria-hidden="true"></i>
                    </span>
                    <span>{"Add fileset"}</span>
                </button>
            </div>
        </div>}
    }

    fn show_fileset_card(&self) -> Html {
        html!{
            <FilesOfFilesetCard
                show_manage_btn = true
                fileset_uuid = self.select_fileset_program.clone()
            />
        }
    }
}
