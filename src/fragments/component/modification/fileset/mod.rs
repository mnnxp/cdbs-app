mod edit;
mod item;

pub use edit::ManageModificationFilesets;
pub use item::FileOfFilesetItem;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use chrono::NaiveDateTime;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, ShowFileInfo};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComModFilesOfFileset;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub fileset_uuid: UUID,
}

pub struct FilesOfFilesetCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    fileset_uuid: UUID,
    files_data: Vec<ShowFileInfo>,
}

pub enum Msg {
    RequestFilesOfFileset,
    ResponseError(Error),
    GetFilesOfFilesetResult(String),
}

impl Component for FilesOfFilesetCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fileset_uuid = props.fileset_uuid.clone();
        Self {
            error: None,
            props,
            link,
            fileset_uuid,
            files_data: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.fileset_uuid.len() == 36 {
            self.link.send_message(Msg::RequestFilesOfFileset);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestFilesOfFileset => {
                let ipt_file_of_fileset_arg = com_mod_files_of_fileset::IptFileOfFilesetArg{
                    filesetUuid: self.fileset_uuid.clone(),
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
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: Vec<ShowFileInfo> = serde_json::from_value(
                            res.get("componentModificationFilesOfFileset").unwrap().clone()).unwrap();
                        // debug!("componentModificationFilesOfFileset: {:?}", result);
                        self.files_data = result;
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_manage_btn == props.show_manage_btn &&
              self.props.fileset_uuid == props.fileset_uuid &&
                self.fileset_uuid == props.fileset_uuid {
            false
        } else {
            self.fileset_uuid = props.fileset_uuid.clone();
            self.props = props;
            if self.fileset_uuid.len() == 36 {
                self.link.send_message(Msg::RequestFilesOfFileset);
            }
            true
        }
    }

    fn view(&self) -> Html {
        html!{<>
            <ListErrors error=self.error.clone()/>
            <div class="card">
                <table class="table is-fullwidth is-striped">
                  <thead>
                    <tr>
                      <th>{"Filename"}</th>
                      <th>{"Content"}</th>
                      <th>{"Filesize"}</th>
                      <th>{"Program"}</th>
                      <th>{"Upload by"}</th>
                      <th>{"Upload at"}</th>
                    </tr>
                  </thead>
                  <tfoot>
                    {for self.files_data.iter().map(|file| html!{
                        <FileOfFilesetItem
                            show_download_btn = self.props.show_manage_btn
                            show_delete_btn = self.props.show_manage_btn
                            file = file.clone()
                        />
                    })}
                  </tfoot>
                </table>
            </div>
        </>}
    }
}
