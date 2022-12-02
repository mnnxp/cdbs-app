mod file;
mod edit;
mod download_block;

pub use file::{FilesetFilesBlock, FileOfFilesetItem};
pub use edit::ManageModificationFilesets;
pub use download_block::ManageFilesOfFilesetBlock;

use yew::{Component, Context, html, Html, Properties};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
// use log::debug;
use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo};
use crate::services::get_value_field;
use crate::gqls::make_query;
use crate::gqls::component::{
    ComModFilesOfFileset, com_mod_files_of_fileset
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_download_btn: bool,
    pub select_fileset_uuid: UUID,
}

pub struct FilesOfFilesetCard {
    error: Option<Error>,
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            select_fileset_uuid: ctx.props().select_fileset_uuid.clone(),
            files_list: Vec::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render && self.select_fileset_uuid.len() == 36 {
            ctx.link().send_message(Msg::RequestFilesOfFileset);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestFilesOfFileset => {
                let ipt_file_of_fileset_arg = com_mod_files_of_fileset::IptFileOfFilesetArg{
                    fileset_uuid: self.select_fileset_uuid.clone(),
                    file_uuids: None,
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
                        self.files_list = serde_json::from_value(
                            res.get("componentModificationFilesOfFileset").unwrap().clone()
                        ).unwrap();
                        // debug!("componentModificationFilesOfFileset: {:?}", result);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.select_fileset_uuid == ctx.props().select_fileset_uuid {
            false
        } else {
            self.select_fileset_uuid = ctx.props().select_fileset_uuid.clone();
            self.files_list.clear();
            if self.select_fileset_uuid.len() == 36 {
                ctx.link().send_message(Msg::RequestFilesOfFileset);
            }
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            {self.show_files_card(ctx.props())}
        </>}
    }
}

impl FilesOfFilesetCard {
    fn show_files_card(
        &self,
        props: &Props,
    ) -> Html {
        html!{<div class="card">
            <table class="table is-fullwidth is-striped">
              <thead>
                <tr>
                  <th>{ get_value_field(&120) }</th> // Filename
                  // <th>{ get_value_field(&121) }</th> // Content
                  <th>{ get_value_field(&122) }</th> // Filesize
                  <th>{ get_value_field(&26) }</th> // Program
                  <th>{ get_value_field(&124) }</th> // Upload by
                  <th>{ get_value_field(&125) }</th> // Upload at
                </tr>
              </thead>
              <tfoot>
                {for self.files_list.iter().map(|file| html!{
                    <FileOfFilesetItem
                        show_download_btn = {props.show_download_btn}
                        file = {file.clone()}
                    />
                })}
              </tfoot>
            </table>
        </div>}
    }
}
