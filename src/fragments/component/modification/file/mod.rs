mod edit;
mod list_item;
mod table_item;

pub use edit::ManageModificationFilesCard;
pub use list_item::ModificationFileItem;
pub use table_item::ModificationFileListItem;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::file::FileHeadersShow;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::component::{ComponentModificationFilesList, component_modification_files_list};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub modification_uuid: UUID,
}

pub struct ModificationFilesTableCard {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    files_list: Vec<ShowFileInfo>,
    // show_full_files: bool,
}

#[derive(Clone)]
pub enum Msg {
    RequestModificationFilesList,
    ResponseError(Error),
    GetModificationFilesListResult(String),
    ClearError,
}

impl Component for ModificationFilesTableCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            link,
            props,
            files_list: Vec::new(),
            // show_full_files: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.props.modification_uuid.len() == 36 {
            debug!("First render modification files list");
            // self.clear_current_data();
            self.link.send_message(Msg::RequestModificationFilesList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestModificationFilesList => {
                let modification_uuid = self.props.modification_uuid.clone();
                spawn_local(async move {
                    let ipt_modification_files_arg = component_modification_files_list::IptModificationFilesArg{
                        filesUuids: None,
                        modificationUuid: modification_uuid,
                        limit: None,
                        offset: None,
                    };
                    let res = make_query(ComponentModificationFilesList::build_query(
                        component_modification_files_list::Variables { ipt_modification_files_arg }
                    )).await.unwrap();
                    link.send_message(Msg::GetModificationFilesListResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetModificationFilesListResult(res) => {
                match resp_parsing(res, "componentModificationFilesList") {
                    Ok(result) => self.files_list = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("componentModificationFilesList {:?}", self.files_list.len());
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.modification_uuid == props.modification_uuid {
            debug!("not update modification files {:?}", props.modification_uuid);
            false
        } else {
            debug!("update modification files {:?}", props.modification_uuid);
            self.props = props;

            self.files_list.clear();
            if self.props.modification_uuid.len() == 36 {
                self.link.send_message(Msg::RequestModificationFilesList);
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
                    <p class="card-header-title">{get_value_field(&119)}</p> // Modification files
                </header>
                <div class="card-content">
                    <div class="content">
                        <table class="table is-fullwidth is-striped">
                            <FileHeadersShow
                                show_long={true}
                                show_download_btn={self.props.show_download_btn}
                                />
                            <tbody>
                                {for self.files_list.iter().map(|file| html!{
                                    <ModificationFileListItem
                                        modification_uuid={self.props.modification_uuid.clone()}
                                        show_download_tag={self.props.show_download_btn}
                                        file={file.clone()}
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