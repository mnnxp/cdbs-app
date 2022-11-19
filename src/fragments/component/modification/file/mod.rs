mod edit;
mod list_item;
mod table_item;

pub use edit::ManageModificationFilesCard;
pub use list_item::ModificationFileItem;
pub use table_item::ModificationFileListItem;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
// use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo};
use crate::services::get_value_field;
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
    // ShowFullList,
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
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.files_list = serde_json::from_value(
                            res_value.get("componentModificationFilesList").unwrap().clone()
                        ).unwrap();
                        debug!("componentModificationFilesList {:?}", self.files_list.len());
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            // Msg::ShowFullList => self.show_full_files = !self.show_full_files,
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

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            <h2 class="has-text-weight-bold">{ get_value_field(&119) }</h2> // Modification files
            {self.show_files_card()}
        </>}
    }
}

impl ModificationFilesTableCard {
    fn show_files_card(&self) -> Html {
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
                  {match &self.props.show_download_btn {
                      true => html!{<th>{ get_value_field(&126) }</th>}, // Download
                      false => html!{},
                  }}
                </tr>
              </thead>
              <tfoot>
                {for self.files_list.iter().map(|file| html!{
                    <ModificationFileListItem
                        modification_uuid = {self.props.modification_uuid.clone()}
                        show_download_tag = {self.props.show_download_btn}
                        file = {file.clone()}
                    />
                })}
              </tfoot>
            </table>
        </div>}
    }
}
