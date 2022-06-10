use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};

use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo, DownloadFile};
use crate::services::get_value_field;
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
                        limit: None,
                        offset: None,
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
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: Vec<DownloadFile> = serde_json::from_value(res.get("componentModificationFiles").unwrap().clone()).unwrap();
                        debug!("componentModificationFiles: {:?}", result);
                        self.download_url = result.first().map(|f| f.download_url.clone()).unwrap_or_default();
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
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
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {self.show_full_info_file()}
        </>}
    }
}

impl ModificationFileListItem {
    fn show_full_info_file(&self) -> Html {
        html!{<tr>
          <td>{self.props.file.filename.clone()}</td>
          // <td>{self.props.file.content_type.clone()}</td>
          <td>{self.props.file.filesize.clone()}</td>
          <td>{self.props.file.program.name.clone()}</td>
          <td>{format!("{} {} (@{})",
            self.props.file.owner_user.firstname.clone(),
            self.props.file.owner_user.lastname.clone(),
            self.props.file.owner_user.username.clone(),
          )}</td>
          <td>{format!("{:.*}", 19, self.props.file.updated_at.to_string())}</td>
          {match &self.props.show_download_tag {
              true => self.show_download_tag(),
              false => html!{},
          }}
        </tr>}
    }

    fn show_download_tag(&self) -> Html {
        let onclick_download_btn =
            self.link.callback(|_| Msg::RequestDownloadFile);

        match self.download_url.is_empty() {
            true => html!{<td>
                <button class="button is-ghost" onclick=onclick_download_btn>
                  <span>{ get_value_field(&137) }</span>
                </button>
            </td>},
            false => html!{<td>
                <a class="button is-ghost" href={self.download_url.clone()}  target="_blank">
                  <span class="icon" >
                    <i class="fas fa-file-download" aria-hidden="true"></i>
                  </span>
                </a>
            </td>},
        }
    }
}
