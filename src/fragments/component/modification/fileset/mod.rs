mod file;
mod edit;
mod download_block;

pub use file::FilesetFilesBlock;
pub use edit::ManageModificationFilesets;
pub use download_block::ManageFilesOfFilesetBlock;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::file::{FileHeadersShow, FileInfoItemShow};
use crate::services::{resp_parsing, get_value_field};
use crate::types::{UUID, ShowFileInfo};
use crate::gqls::make_query;
use crate::gqls::component::{ComModFilesOfFileset, com_mod_files_of_fileset};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub select_fileset_uuid: UUID,
}

pub struct FilesOfFilesetCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    select_fileset_uuid: UUID,
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
        let select_fileset_uuid = props.select_fileset_uuid.clone();
        Self {
            error: None,
            props,
            link,
            select_fileset_uuid,
            files_list: Vec::new(),
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
        if self.props.select_fileset_uuid == props.select_fileset_uuid {
            false
        } else {
            self.select_fileset_uuid = props.select_fileset_uuid.clone();
            self.props = props;

            self.files_list.clear();
            if self.select_fileset_uuid.len() == 36 {
                self.link.send_message(Msg::RequestFilesOfFileset);
            }

            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{
            <div class="card">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <header class="card-header has-background-primary-light">
                    <p class="card-header-title">{get_value_field(&106)}</p> // Files of select fileset
                </header>
                <div class="card-content">
                    <div class="content">
                        <table class="table is-fullwidth is-striped">
                            <FileHeadersShow show_long={true} />
                            <tbody>
                                {for self.files_list.iter().map(|file| html!{
                                    <FileInfoItemShow
                                        file_info={file.clone()}
                                        download_url={String::new()}
                                        />
                                })}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
}