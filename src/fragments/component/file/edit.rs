use yew::{html, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::file::{commit_msg_field, UploaderFiles};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, UploadFile};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::component::{UploadComponentFiles, upload_component_files,};
use super::ComponentFilesBlock;

type FileName = String;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub component_uuid: UUID,
    pub files_count: i64,
}

pub struct ManageComponentFilesCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    request_upload_data: Vec<UploadFile>,
    total_items: i64,
    commit_msg: String,
}

pub enum Msg {
    RequestUploadComponentFiles(Vec<FileName>),
    GetUploadData(String),
    UploadConfirm(usize),
    UpdateCommitMsg(String),
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for ManageComponentFilesCard {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let total_items = props.files_count;
        Self {
            error: None,
            props,
            link,
            request_upload_data: Vec::new(),
            total_items,
            commit_msg: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestUploadComponentFiles(filenames) => {
                debug!("filenames: {:?}", filenames);
                if self.props.component_uuid.len() != 36 || filenames.is_empty() {
                    return false
                }
                let component_uuid = self.props.component_uuid.clone();
                let commit_msg = self.commit_msg.clone();
                spawn_local(async move {
                    let ipt_component_files_data = upload_component_files::IptComponentFilesData{
                        filenames,
                        componentUuid: component_uuid,
                        commitMsg: commit_msg
                    };
                    let res = make_query(UploadComponentFiles::build_query(upload_component_files::Variables{
                        ipt_component_files_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                })
            },
            Msg::GetUploadData(res) => {
                match resp_parsing(res, "uploadComponentFiles") {
                    Ok(result) => self.request_upload_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("uploadComponentFiles {:?}", self.request_upload_data);
            },
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of files: {:?}", confirmations);
                self.total_items += confirmations as i64;
                self.request_upload_data.clear();
                self.commit_msg.clear();
            },
            Msg::UpdateCommitMsg(data) => self.commit_msg = data,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let oninput_commit_msg = self.link.callback(|ev: InputData| Msg::UpdateCommitMsg(ev.value));
        let callback_upload_filenames = self.link.callback(move |filenames| Msg::RequestUploadComponentFiles(filenames));
        let request_upload_files = match self.request_upload_data.is_empty() {
            true => None,
            false => Some(self.request_upload_data.clone()),
        };
        let callback_upload_confirm = self.link.callback(|confirmations| Msg::UploadConfirm(confirmations));
        html!{
            <div class={"columns"}>
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <div class={"column"}>
                    <p class={"title is-5"}>{get_value_field(&186)}</p> // Upload component files
                    {commit_msg_field(self.props.component_uuid.clone(), self.commit_msg.clone(), oninput_commit_msg.clone())}
                    <UploaderFiles
                        text_choose_files={200} // Choose component files…
                        callback_upload_filenames={callback_upload_filenames}
                        request_upload_files={request_upload_files}
                        callback_upload_confirm={callback_upload_confirm}
                        />
                </div>
                <div class={"column"}>
                    <p class={"title is-5"}>{get_value_field(&188)}</p> // Files for component
                    <ComponentFilesBlock
                        show_download_btn={self.props.show_download_btn}
                        show_delete_btn={true}
                        component_uuid={self.props.component_uuid.clone()}
                        files_count={self.total_items}
                        />
                </div>
            </div>
        }
    }
}