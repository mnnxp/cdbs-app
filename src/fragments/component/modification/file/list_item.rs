use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::switch_icon::res_file_btn;
use crate::fragments::file::FileShowcase;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo, DownloadFile};
use crate::services::{resp_parsing};
use crate::gqls::make_query;
use crate::gqls::component::{
    ComponentModificationFiles, component_modification_files,
    DeleteModificationFile, delete_modification_file,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub modification_uuid: UUID,
    pub file: ShowFileInfo,
    pub callback_delete_file: Option<Callback<UUID>>,
}

pub struct ModificationFileItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    open_full_info_file: bool,
    get_result_delete: bool,
    download_url: String,
}

pub enum Msg {
    RequestDownloadFile(UUID),
    RequestDeleteFile(UUID),
    ResponseError(Error),
    GetDownloadFileResult(String, UUID),
    GetDeleteFileResult(String, UUID),
    ClickFileInfo,
    ClearError,
}

impl Component for ModificationFileItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            open_full_info_file: false,
            get_result_delete: false,
            download_url: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFile(file_uuid) => {
                self.download_url.clear();
                let modification_uuid = self.props.modification_uuid.clone();
                spawn_local(async move {
                    let ipt_modification_files_arg = component_modification_files::IptModificationFilesArg{
                        filesUuids: Some(vec![file_uuid.clone()]),
                        modificationUuid: modification_uuid,
                    };
                    let res = make_query(ComponentModificationFiles::build_query(
                        component_modification_files::Variables {
                            ipt_modification_files_arg,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetDownloadFileResult(res, file_uuid));
                })
            },
            Msg::RequestDeleteFile(file_uuid) => {
                let modification_uuid = self.props.modification_uuid.clone();
                // let file_uuid = self.props.file.uuid.clone();
                spawn_local(async move {
                    let delete_modification_file_data = delete_modification_file::DelModificationFileData{
                        fileUuid: file_uuid.clone(),
                        modificationUuid: modification_uuid,
                    };
                    let res = make_query(DeleteModificationFile::build_query(delete_modification_file::Variables {
                        delete_modification_file_data
                    })).await.unwrap();
                    link.send_message(Msg::GetDeleteFileResult(res, file_uuid));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDownloadFileResult(res, file_uuid) => {
                match resp_parsing::<Vec<DownloadFile>>(res, "componentModificationFiles") {
                    Ok(result) => {
                        // let result: Vec<DownloadFile> = result;
                        debug!("componentModificationFiles: {:?}, file_uuid: {:?}", result, file_uuid);
                        self.download_url = result.first().map(|f: &DownloadFile| f.download_url.clone()).unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetDeleteFileResult(res, file_uuid) => {
                match resp_parsing(res, "deleteModificationFile") {
                    Ok(result) => {
                        if result && &file_uuid == &self.props.file.uuid {
                            self.get_result_delete = result;
                            if let Some(rollback) = &self.props.callback_delete_file {
                                rollback.emit(self.props.file.uuid.clone());
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ClickFileInfo => self.open_full_info_file = !self.open_full_info_file,
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_file_info = self.link.callback(|_| Msg::ClickFileInfo);
        let onclick_download_btn =
            self.link.callback(|download_file_uuid| Msg::RequestDownloadFile(download_file_uuid));
        let onclick_delete_btn =
            self.link.callback(|delete_file_uuid| Msg::RequestDeleteFile(delete_file_uuid));

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {match self.get_result_delete {
                true => html!{},
                false => html!{<>
                    <FileShowcase
                        file_info={self.props.file.clone()}
                        file_info_callback={onclick_file_info}
                        file_download_callback={Some(onclick_download_btn)}
                        file_delete_callback={Some(onclick_delete_btn)}
                        open_modal_frame={self.open_full_info_file}
                        show_revisions={self.props.show_delete_btn}
                        download_url={self.download_url.clone()}
                        />
                    {self.show_file()}
                </>},
            }}
        </>}
    }
}

impl ModificationFileItem {
    fn show_file(&self) -> Html {
        let onclick_file_info = self.link.callback(|_| Msg::ClickFileInfo);
        res_file_btn(onclick_file_info, self.props.file.filename.clone())
    }
}
