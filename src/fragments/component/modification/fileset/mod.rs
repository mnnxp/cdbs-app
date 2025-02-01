mod file;
mod edit;
mod download_block;
mod show;

pub use file::FilesetFilesBlock;
pub use edit::ManageModificationFilesets;
pub use download_block::ManageFilesOfFilesetBlock;
pub use show::ModificationFilesetsCard;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::file::{FileHeadersShow, FileInfoItemShow};
use crate::services::{get_classes_table, get_value_field, resp_parsing};
use crate::types::{UUID, ShowFileInfo};
use crate::gqls::make_query;
use crate::gqls::component::{ComModFilesOfFileset, com_mod_files_of_fileset};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_card: bool,
    pub show_download_btn: bool,
    pub select_fileset_uuid: UUID,
}

pub struct FilesOfFilesetCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    files_list: Vec<ShowFileInfo>,
}

pub enum Msg {
    RequestFilesOfFileset,
    ResponseError(Error),
    GetFilesOfFilesetResult(String),
    ClearError,
}

impl Component for FilesOfFilesetCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            files_list: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestFilesOfFileset);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestFilesOfFileset => {
                // request only if the uuid is provided and the card is showing
                if self.props.select_fileset_uuid.len() == 36 && self.props.show_card {
                    let ipt_file_of_fileset_arg = com_mod_files_of_fileset::IptFileOfFilesetArg{
                        filesetUuid: self.props.select_fileset_uuid.clone(),
                        fileUuids: None,
                    };
                    spawn_local(async move {
                        let res = make_query(ComModFilesOfFileset::build_query(com_mod_files_of_fileset::Variables {
                            ipt_file_of_fileset_arg
                        })).await.unwrap();
                        link.send_message(Msg::GetFilesOfFilesetResult(res));
                    })
                } else {
                    self.files_list.clear();
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetFilesOfFilesetResult(res) => {
                match resp_parsing(res, "componentModificationFilesOfFileset") {
                    Ok(result) => self.files_list = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_fileset_uuid == props.select_fileset_uuid &&
            self.props.show_card == props.show_card {
            debug!("No parsing files for fileset: {:?}, {}", props.select_fileset_uuid, props.show_card);
            false
        } else {
            debug!("Parsing files for fileset: {:?}, {}", props.select_fileset_uuid, props.show_card);
            self.props = props;
            self.link.send_message(Msg::RequestFilesOfFileset);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let mut classes_table = get_classes_table(self.files_list.len());
        classes_table.push("is-striped");
        match self.props.show_card {
            true => html!{<>
                <br/>
                <div class="card">
                    <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                    <header class={"card-header has-background-info-light"}>
                        <p class={"card-header-title"}>{get_value_field(&106)}</p> // Files of select fileset
                    </header>
                    <div class={"card-content"}>
                        <div class={"table-container"}>
                            <div class={"content"}>
                                <table class={classes_table}>
                                    <FileHeadersShow show_download_btn={self.props.show_download_btn} />
                                    <tbody>
                                        {for self.files_list.iter().enumerate().map(|(numer, file)| html!{
                                            <FileInfoItemShow
                                                file_info={file.clone()}
                                                show_download_btn={self.props.show_download_btn}
                                                ordinal_indicator={numer+1}
                                                />
                                        })}
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>
                </div>
            </>},
            false => html!{},
        }
    }
}