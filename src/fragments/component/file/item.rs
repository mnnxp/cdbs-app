use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, ShowFileInfo, DownloadFile};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComponentFiles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct DeleteComponentFile;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub file: ShowFileInfo,
}

pub struct FileItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    open_full_info_file: bool,
    get_result_delete: bool,
}

pub enum Msg {
    RequestDownloadFile,
    RequestDeleteFile,
    ResponseError(Error),
    GetDownloadFileResult(String),
    GetDeleteFileResult(String),
    ClickFileInfo,
}

impl Component for FileItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            open_full_info_file: false,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFile => {
                let component_uuid = self.props.component_uuid.clone();
                let file_uuid = self.props.file.uuid.clone();
                spawn_local(async move {
                    let ipt_component_files_arg = component_files::IptComponentFilesArg{
                        filesUuids: Some(vec![file_uuid]),
                        componentUuid: component_uuid,
                        limit: None,
                        offset: None,
                    };
                    let res = make_query(ComponentFiles::build_query(
                        component_files::Variables {
                            ipt_component_files_arg,
                        }
                    )).await;
                    link.send_message(Msg::GetDownloadFileResult(res.unwrap()));
                })
            },
            Msg::RequestDeleteFile => {
                let component_uuid = self.props.component_uuid.clone();
                let file_uuid = self.props.file.uuid.clone();
                spawn_local(async move {
                    let delete_component_file_data = delete_component_file::DelComponentFileData{
                        fileUuid: file_uuid,
                        componentUuid: component_uuid,
                    };
                    let res = make_query(DeleteComponentFile::build_query(delete_component_file::Variables {
                        delete_component_file_data
                    })).await.unwrap();
                    link.send_message(Msg::GetDeleteFileResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDownloadFileResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: Vec<DownloadFile> = serde_json::from_value(res.get("componentFiles").unwrap().clone()).unwrap();
                        debug!("componentFiles: {:?}", result);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetDeleteFileResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res.get("deleteComponentFile").unwrap().clone()).unwrap();
                        debug!("deleteFile: {:?}", result);
                        self.get_result_delete = result;
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::ClickFileInfo => {
                self.open_full_info_file = !self.open_full_info_file;
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{<>
            <ListErrors error=self.error.clone()/>
            {match self.get_result_delete {
                true => html!{},
                false => html!{<>
                    {self.show_full_info_file()}
                    {self.show_file()}
                </>},
            }}
        </>}
    }
}

impl FileItem {
    fn show_file(&self) -> Html {
        let onclick_file_info = self.link
            .callback(|_| Msg::ClickFileInfo);

        html!{
            <div class="buttons">
                <div class="button is-white" onclick=onclick_file_info>
                    <span class="icon">
                      <i class="fas fa-file"></i>
                    </span>
                    <span>{self.props.file.filename.clone()}</span>
                </div>
                {self.show_download_btn()}
                {self.show_delete_btn()}
            </div>
        }
    }

    fn show_download_btn(&self) -> Html {
        let onclick_download_btn = self.link
            .callback(|_| Msg::RequestDownloadFile);

        match &self.props.show_download_btn {
            true => html!{
                <button class="button is-white" onclick=onclick_download_btn >
                  <span class="icon" >
                    <i class="fas fa-file-download" aria-hidden="true"></i>
                  </span>
                </button>
            },
            false => html!{},
        }
    }

    fn show_delete_btn(&self) -> Html {
        let onclick_delete_btn = self.link
            .callback(|_| Msg::RequestDeleteFile);

        match &self.props.show_delete_btn {
            true => html!{
                <button class="button is-white" onclick=onclick_delete_btn >
                  <span class="icon" >
                    <i class="fa fa-trash" aria-hidden="true"></i>
                  </span>
                </button>
            },
            false => html!{},
        }
    }

    fn show_full_info_file(&self) -> Html {
        let onclick_file_info = self.link
            .callback(|_| Msg::ClickFileInfo);

        let class_modal = match &self.open_full_info_file {
            true => "modal is-active",
            false => "modal",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_file_info.clone() />
              <div class="modal-content">
                  <div class="card">
                    <table class="table is-fullwidth">
                      <tbody>
                        <tr>
                          <td>{"Filename:"}</td>
                          <td>{self.props.file.filename.clone()}</td>
                        </tr>
                        <tr>
                          <td>{"Content type:"}</td>
                          <td>{self.props.file.content_type.clone()}</td>
                        </tr>
                        <tr>
                          <td>{"Filesize:"}</td>
                          <td>{self.props.file.filesize.clone()}</td>
                        </tr>
                        <tr>
                          <td>{"Program:"}</td>
                          <td>{self.props.file.program.name.clone()}</td>
                        </tr>
                        // <tr>
                        //   <td>{"parent_file_uuid"}</td>
                        //   <td>{self.props.file.parent_file_uuid.clone()}</td>
                        // </tr>
                        <tr>
                          <td>{"Upload by:"}</td>
                          <td>{format!("{} {} (@{})",
                            self.props.file.owner_user.firstname.clone(),
                            self.props.file.owner_user.lastname.clone(),
                            self.props.file.owner_user.username.clone(),
                          )}</td>
                        </tr>
                        // <tr>
                        //   <td>{"Created at:"}</td>
                        //   <td>{format!("{:.*}", 19, self.props.file.created_at.to_string())}</td>
                        // </tr>
                        <tr>
                          <td>{"Upload at:"}</td>
                          <td>{format!("{:.*}", 19, self.props.file.updated_at.to_string())}</td>
                        </tr>
                      </tbody>
                    </table>
                  </div>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick=onclick_file_info />
            </div>
        }
    }
}
