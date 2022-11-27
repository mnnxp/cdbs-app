use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo, DownloadFile};
use crate::services::get_value_field;
use crate::gqls::make_query;
use crate::gqls::component::{
    ComponentFiles, component_files,
    DeleteComponentFile, delete_component_file,
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub file: ShowFileInfo,
    pub callback_delete_file: Option<Callback<UUID>>,
}

pub struct ComponentFileItem {
    error: Option<Error>,
    open_full_info_file: bool,
    get_result_delete: bool,
    download_url: String,
}

pub enum Msg {
    RequestDownloadFile,
    RequestDeleteFile,
    ResponseError(Error),
    GetDownloadFileResult(String),
    GetDeleteFileResult(String),
    ClickFileInfo,
}

impl Component for ComponentFileItem {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            open_full_info_file: false,
            get_result_delete: false,
            download_url: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestDownloadFile => {
                let component_uuid = ctx.props().component_uuid.clone();
                let file_uuid = ctx.props().file.uuid.clone();
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
                let component_uuid = ctx.props().component_uuid.clone();
                let file_uuid = ctx.props().file.uuid.clone();
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
                        self.download_url = result.first().map(|f| f.download_url.clone()).unwrap_or_default();
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetDeleteFileResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        self.get_result_delete = serde_json::from_value(
                            res.get("deleteComponentFile").unwrap().clone()
                        ).unwrap();
                        debug!("deleteFile: {:?}", self.get_result_delete);
                        if self.get_result_delete {
                            if let Some(rollback) = &ctx.props().callback_delete_file {
                                rollback.emit(ctx.props().file.uuid.clone());
                            }
                        }
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

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<>
            <ListErrors error={self.error.clone()}/>
            {match self.get_result_delete {
                true => html!{},
                false => html!{<>
                    {self.show_full_info_file(ctx.link(), ctx.props())}
                    {self.show_file(ctx.link(), ctx.props())}
                </>},
            }}
        </>}
    }
}

impl ComponentFileItem {
    fn show_file(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_file_info = link.callback(|_| Msg::ClickFileInfo);

        html!{
            <div class="buttons">
                <div class="button is-white" onclick={onclick_file_info}>
                    <span class="icon">
                      <i class="fas fa-file"></i>
                    </span>
                    <span>{props.file.filename.clone()}</span>
                </div>
                {self.show_download_btn(link, props)}
                {self.show_delete_btn(link, props)}
            </div>
        }
    }

    fn show_download_btn(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_download_btn =
            link.callback(|_| Msg::RequestDownloadFile);

        match &props.show_download_btn {
            true => match self.download_url.is_empty() {
                true => html!{
                    <button class="button is-ghost" onclick={onclick_download_btn}>
                      <span>{ get_value_field(&137) }</span>
                    </button>
                },
                false => html!{
                    <a class="button is-ghost" href={self.download_url.clone()}  target="_blank">
                      <span class="icon" >
                        <i class="fas fa-file-download" aria-hidden="true"></i>
                      </span>
                    </a>
                },
            },
            false => html!{},
        }
    }

    fn show_delete_btn(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_delete_btn = link.callback(|_| Msg::RequestDeleteFile);

        match &props.show_delete_btn {
            true => html!{
                <button class="button is-white" onclick={onclick_delete_btn} >
                  <span class="icon" >
                    <i class="fa fa-trash" aria-hidden="true"></i>
                  </span>
                </button>
            },
            false => html!{},
        }
    }

    fn show_full_info_file(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_file_info = link.callback(|_| Msg::ClickFileInfo);

        let class_modal = match &self.open_full_info_file {
            true => "modal is-active",
            false => "modal",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_file_info.clone()} />
              <div class="modal-content">
                  <div class="card column">
                    <table class="table is-fullwidth">
                      <tbody>
                        <tr>
                          <td>{ get_value_field(&236) }</td>
                          <td>{props.file.filename.clone()}</td>
                        </tr>
                        <tr>
                          <td>{ get_value_field(&237) }</td>
                          <td>{props.file.content_type.clone()}</td>
                        </tr>
                        <tr>
                          <td>{ get_value_field(&238) }</td>
                          <td>{props.file.filesize.clone()}</td>
                        </tr>
                        <tr>
                          <td>{ get_value_field(&239) }</td>
                          <td>{props.file.program.name.clone()}</td>
                        </tr>
                        // <tr>
                        //   <td>{"parent_file_uuid"}</td>
                        //   <td>{props.file.parent_file_uuid.clone()}</td>
                        // </tr>
                        <tr>
                          <td>{ get_value_field(&240) }</td>
                          <td>{format!("{} {} (@{})",
                            props.file.owner_user.firstname.clone(),
                            props.file.owner_user.lastname.clone(),
                            props.file.owner_user.username.clone(),
                          )}</td>
                        </tr>
                        // <tr>
                        //   <td>{ get_value_field(&242) }</td>
                        //   <td>{format!("{:.*}", 19, props.file.created_at.to_string())}</td>
                        // </tr>
                        <tr>
                          <td>{ get_value_field(&241) }</td>
                          <td>{format!("{:.*}", 19, props.file.updated_at.to_string())}</td>
                        </tr>
                      </tbody>
                    </table>
                  </div>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick={onclick_file_info} />
            </div>
        }
    }
}
