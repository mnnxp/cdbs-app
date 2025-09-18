use yew::{Component, ComponentLink, Callback, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::ManageFilesOfFilesetBlock;
use crate::services::resp_parsing;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, FilesetProgramInfo};
use crate::gqls::make_query;
use crate::gqls::component::{ComponentModificationFilesets, component_modification_filesets};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub modification_uuid: UUID,
    pub callback_select_fileset: Callback<FilesetProgramInfo>,
    pub callback_open_fileset: Callback<bool>,
}

pub struct ModificationFilesetsCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    filesets_program: Vec<FilesetProgramInfo>,
    select_fileset_uuid: UUID,
}

pub enum Msg {
    RequestComponentModificationFilesetsData,
    ResponseError(Error),
    GetComponentModificationFilesetResult(String),
    SelectFileset(UUID),
    ClearError,
}

impl Component for ModificationFilesetsCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            filesets_program: Vec::new(),
            select_fileset_uuid: String::new(),
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
                self.filesets_program.clear(); // fix for bug with displaying old files
                if self.props.modification_uuid.len() == 36 {
                    let ipt_fileset_program_arg = component_modification_filesets::IptFilesetProgramArg{
                        modificationUuid: self.props.modification_uuid.clone(),
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
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetComponentModificationFilesetResult(res) => {
                match resp_parsing::<Vec<FilesetProgramInfo>>(res, "componentModificationFilesets") {
                    Ok(res) => {
                        self.filesets_program = res;
                        self.select_fileset_uuid = self.filesets_program
                            .first()
                            .map(|m| m.uuid.clone())
                            .unwrap_or_default();
                        debug!("Update modification filesets list");
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::SelectFileset(fileset_uuid) => {
                debug!("SelectFileset: {:?}", fileset_uuid);
                self.select_fileset_uuid = fileset_uuid;
                if let Some(fp) = self.filesets_program.iter().find(|sf| sf.uuid == self.select_fileset_uuid) {
                    self.props.callback_select_fileset.emit(fp.clone());
                }
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.modification_uuid == props.modification_uuid {
            debug!("No parsing filesets for modification: {:?}", props.modification_uuid);
            false
        } else {
            debug!("Parsing filesets for modification: {:?}", props.modification_uuid);
            self.props = props;
            self.link.send_message(Msg::RequestComponentModificationFilesetsData);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let callback_select_fileset_uuid = self.link.callback(|value: UUID| Msg::SelectFileset(value));
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <ManageFilesOfFilesetBlock
                select_modification_uuid={self.props.modification_uuid.clone()}
                current_filesets_program={self.filesets_program.clone()}
                callback_select_fileset_uuid={callback_select_fileset_uuid}
                callback_open_fileset={self.props.callback_open_fileset.clone()}
            />
        </>}
    }
}