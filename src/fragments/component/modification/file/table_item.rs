use yew::{Component, Context, html, html::Scope, Html, Properties};

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
    modification_uuid: UUID,
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
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            modification_uuid: String::new(),
            file_uuid: String::new(),
            download_url: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestDownloadFile => {
                let modification_uuid = ctx.props().modification_uuid.clone();
                let file_uuid = ctx.props().file.uuid.clone();
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.modification_uuid == ctx.props().modification_uuid &&
            self.file_uuid == ctx.props().file.uuid {
            false
        } else {
            self.modification_uuid = ctx.props().modification_uuid.clone();
            self.file_uuid = ctx.props().file.uuid.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            {self.show_full_info_file(ctx.link(), ctx.props())}
        </>}
    }
}

impl ModificationFileListItem {
    fn show_full_info_file(
        &self,
        link: &Scope<Self>,
        props: &Properties,
    ) -> Html {
        html!{<tr>
          <td>{props.file.filename.clone()}</td>
          // <td>{props.file.content_type.clone()}</td>
          <td>{props.file.filesize.clone()}</td>
          <td>{props.file.program.name.clone()}</td>
          <td>{format!("{} {} (@{})",
            props.file.owner_user.firstname.clone(),
            props.file.owner_user.lastname.clone(),
            props.file.owner_user.username.clone(),
          )}</td>
          <td>{format!("{:.*}", 19, props.file.updated_at.to_string())}</td>
          {match &props.show_download_tag {
              true => self.show_download_tag(link),
              false => html!{},
          }}
        </tr>}
    }

    fn show_download_tag(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_download_btn = link.callback(|_| Msg::RequestDownloadFile);

        match self.download_url.is_empty() {
            true => html!{<td>
                <button class="button is-ghost" onclick={onclick_download_btn}>
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
