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
struct ComponentModificationFilesList;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub modification_uuid: UUID,
}

pub struct ModificationFilesCard {
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

impl Component for ModificationFilesCard {
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
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            <h2>{"Modification files"}</h2>
            {self.show_files_card()}
        </>}
    }
}

impl ModificationFilesCard {
    fn show_files_card(&self) -> Html {
        html!{<div class="card">
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
                {for self.files_list.iter().map(|file| html!{
                    <ModificationFileListItem
                        show_download_btn = self.props.show_download_btn
                        file = file.clone()
                    />
                })}
              </tfoot>
            </table>
        </div>}
    }

    // fn show_files_card(&self) -> Html {
    //     html!{
    //         <div id="files" class="card">
    //             {for self.files_list.iter().enumerate().map(|(index, file)| {
    //                 match (index >= 3, self.show_full_files) {
    //                     // show full list
    //                     (_, true) => html!{<ModificationFileItem
    //                       show_download_btn = self.props.show_download_btn
    //                       show_delete_btn = false
    //                       modification_uuid = self.props.modification_uuid.clone()
    //                       file = file.clone()
    //                     />},
    //                     // show full list or first 3 items
    //                     (false, false) => html!{<ModificationFileItem
    //                       show_download_btn = self.props.show_download_btn
    //                       show_delete_btn = false
    //                       modification_uuid = self.props.modification_uuid.clone()
    //                       file = file.clone()
    //                     />},
    //                     _ => html!{},
    //                 }
    //             })}
    //             {match self.files_list.len() {
    //                 0 => html!{<span>{"Files not found"}</span>},
    //                 0..=3 => html!{},
    //                 _ => self.show_see_btn(),
    //             }}
    //         </div>
    //     }
    // }
    //
    // fn show_see_btn(&self) -> Html {
    //     let show_full_files_btn = self.link
    //         .callback(|_| Msg::ShowFullList);
    //
    //     match self.show_full_files {
    //         true => html!{<>
    //           <button class="button is-white"
    //               onclick=show_full_files_btn
    //             >{"See less"}</button>
    //         </>},
    //         false => html!{<>
    //           <button class="button is-white"
    //               onclick=show_full_files_btn
    //             >{"See more"}</button>
    //         </>},
    //     }
    // }
}
