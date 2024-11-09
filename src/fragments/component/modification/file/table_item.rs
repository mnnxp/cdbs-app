use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};

use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::file::FileInfoItemShow;
use crate::fragments::list_errors::ListErrors;
use crate::services::resp_parsing;
use crate::types::{UUID, ShowFileInfo, DownloadFile};
use crate::gqls::make_query;
use crate::gqls::component::{ComponentModificationFiles, component_modification_files};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub modification_uuid: UUID,
    pub show_download_tag: bool,
    pub file: ShowFileInfo,
}

pub struct ModificationFileListItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    file_uuid: UUID,
    download_url: String,
}

pub enum Msg {
    RequestDownloadFile,
    ResponseError(Error),
    GetDownloadFileResult(String),
    ClearError,
}

impl Component for ModificationFileListItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            file_uuid: String::new(),
            download_url: String::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestDownloadFile);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFile => {
                let modification_uuid = self.props.modification_uuid.clone();
                let file_uuid = self.props.file.uuid.clone();
                spawn_local(async move {
                    let ipt_modification_files_arg = component_modification_files::IptModificationFilesArg{
                        filesUuids: Some(vec![file_uuid]),
                        modificationUuid: modification_uuid,
                    };
                    let res = make_query(ComponentModificationFiles::build_query(
                        component_modification_files::Variables {
                            ipt_modification_files_arg,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetDownloadFileResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDownloadFileResult(res) => {
                match resp_parsing::<Vec<DownloadFile>>(res, "componentModificationFiles") {
                    Ok(result) => {
                        debug!("componentModificationFiles: {:?}", result);
                        self.download_url = result.first().map(|f| f.download_url.clone()).unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.file_uuid == props.file.uuid &&
              self.props.modification_uuid == props.modification_uuid {
            false
        } else {
            self.file_uuid = props.file.uuid.clone();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <FileInfoItemShow
                file_info={self.props.file.clone()}
                download_url={self.download_url.clone()}
                />
        </>}
    }
}