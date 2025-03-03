use yew::{Component, ComponentLink, Callback, Html, Properties, ShouldRender, html, InputData, ChangeData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use super::file::ManageModificationFilesCard;
use super::fileset::ManageModificationFilesets;
use crate::error::Error;
use crate::fragments::buttons::{ft_delete_btn, ft_save_btn};
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_value_field, resp_parsing};
use crate::services::content_adapter::Markdownable;
use crate::types::{UUID, ComponentModificationInfo, ActualStatus, ModificationUpdatePreData};
use crate::gqls::make_query;
use crate::gqls::component::{
    PutComponentModificationUpdate, put_component_modification_update,
    DeleteComponentModification, delete_component_modification,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub modification: ComponentModificationInfo,
    pub actual_statuses: Vec<ActualStatus>,
    pub callback_delete_modification: Callback<UUID>,
}

pub enum ActiveTab {
    Data,
    ModificationFiles,
    Fileset
}

pub struct ModificationEdit {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    request_edit_modification: ModificationUpdatePreData,
    modification_changed: bool,
    get_confirm: UUID,
    active_tab: ActiveTab,
    preview_description: bool,
}

pub enum Msg {
    RequestUpdateModificationData,
    RequestDeleteModificationData,
    GetUpdateModificationResult(String),
    GetDeleteModificationResult(String),
    ResponseError(Error),
    UpdateEditName(String),
    UpdateEditDescription(String),
    UpdateEditActualStatusId(String),
    ResetSelectModification,
    ChangeActiveTab(ActiveTab),
    PreviewDescription,
    ClearError,
}

impl Component for ModificationEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request_edit_modification = ModificationUpdatePreData{
            modification_name: props.modification.modification_name.clone(),
            description: props.modification.description.clone(),
            actual_status_id: props.modification.actual_status.actual_status_id,
        };
        Self {
            error: None,
            props,
            link,
            request_edit_modification,
            modification_changed: false,
            get_confirm: String::new(),
            active_tab: ActiveTab::Data,
            preview_description: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestUpdateModificationData => {
                self.modification_changed = false;
                let modification_uuid = self.props.modification.uuid.clone();
                let ipt_update_component_modification_data = put_component_modification_update::IptUpdateComponentModificationData{
                    modificationName: match self.request_edit_modification.modification_name.is_empty() {
                        true => None,
                        false => Some(self.request_edit_modification.modification_name.clone())
                    },
                    description: match self.request_edit_modification.description.is_empty() {
                        true => None,
                        false => Some(self.request_edit_modification.description.clone())
                    },
                    actualStatusId: match self.request_edit_modification.actual_status_id == 0 {
                        true => None,
                        false => Some(self.request_edit_modification.actual_status_id as i64)
                    },
                };
                spawn_local(async move {
                    let res = make_query(PutComponentModificationUpdate::build_query(
                        put_component_modification_update::Variables {
                            modification_uuid,
                            ipt_update_component_modification_data
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetUpdateModificationResult(res));
                })
            },
            Msg::RequestDeleteModificationData => {
                if self.get_confirm == self.props.modification.uuid {
                    let del_component_modification_data = delete_component_modification::DelComponentModificationData{
                        componentUuid: self.props.modification.component_uuid.clone(),
                        modificationUuid: self.props.modification.uuid.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteComponentModification::build_query(
                            delete_component_modification::Variables {
                                del_component_modification_data
                            }
                        )).await.unwrap();
                        link.send_message(Msg::GetDeleteModificationResult(res));
                    })
                } else {
                    self.get_confirm = self.props.modification.uuid.clone();
                }
            },
            Msg::GetUpdateModificationResult(res) => {
                match resp_parsing::<usize>(res, "putComponentModificationUpdate") {
                    Ok(result) => debug!("putComponentModificationUpdate: {:?}", result),
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                // clear the check flags
                self.get_confirm.clear();
                self.modification_changed = false;
            },
            Msg::GetDeleteModificationResult(res) => {
                match resp_parsing::<UUID>(res, "deleteComponentModification") {
                    Ok(result) => {
                        debug!("deleteComponentModification: {:?}", result);
                        self.props.callback_delete_modification.emit(result);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::UpdateEditName(data) => {
                self.request_edit_modification.modification_name = data;
                self.modification_changed = true;
            },
            Msg::UpdateEditDescription(data) => {
                self.request_edit_modification.description = data;
                self.modification_changed = true;
            },
            Msg::UpdateEditActualStatusId(data) => {
                self.request_edit_modification.actual_status_id = data.parse::<usize>().unwrap_or_default();
                self.modification_changed = true;
            },
            Msg::ResetSelectModification => {
                // clear the check flags
                self.get_confirm.clear();
                self.modification_changed = false;
                // set initial values
                self.request_edit_modification.modification_name = self.props.modification.modification_name.clone();
                self.request_edit_modification.description = self.props.modification.description.clone();
                self.request_edit_modification.actual_status_id = self.props.modification.actual_status.actual_status_id;
            },
            Msg::ChangeActiveTab(set_tab) => self.active_tab = set_tab,
            Msg::PreviewDescription => self.preview_description = !self.preview_description,
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.modification.uuid == props.modification.uuid {
            false
        } else {
            self.props = props;
            self.link.send_message(Msg::ResetSelectModification);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {self.show_modification_tabs()}
        </>}
    }
}

impl ModificationEdit {
    fn show_modification_tabs(&self) -> Html {
        let onclick_tab_data = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::Data));
        let onclick_tab_files = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::ModificationFiles));
        let onclick_tab_fileset = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::Fileset));
        let at = match self.active_tab {
            ActiveTab::Data => ("is-active","",""),
            ActiveTab::ModificationFiles => ("","is-active",""),
            ActiveTab::Fileset => ("","","is-active"),
        };
        html!{<>
            <div class="tabs is-centered is-medium">
                <ul>
                    <li class={at.0} onclick={onclick_tab_data}><a>{get_value_field(&177)}</a></li>
                    <li class={at.1} onclick={onclick_tab_files}><a>{get_value_field(&172)}</a></li>
                    <li class={at.2} onclick={onclick_tab_fileset}><a>{get_value_field(&173)}</a></li>
                </ul>
            </div>
            <div class="card-content">
                {match self.active_tab {
                    ActiveTab::Data => self.show_modification_card(),
                    ActiveTab::ModificationFiles => html!{
                        <div class="content">
                            <ManageModificationFilesCard
                                show_download_btn={false}
                                modification_uuid={self.props.modification.uuid.clone()}
                                files_count={self.props.modification.files_count}
                                />
                        </div>
                    },
                    ActiveTab::Fileset => html!{
                        <div class="content">
                            <ManageModificationFilesets select_modification_uuid={self.props.modification.uuid.clone()} />
                        </div>
                    },
                }}
            </div>
        </>}
    }

    fn show_modification_card(&self) -> Html {
        let oninput_modification_name = self.link.callback(|ev: InputData| Msg::UpdateEditName(ev.value));
        let oninput_modification_description = self.link.callback(|ev: InputData| Msg::UpdateEditDescription(ev.value));
        let onchange_modification_actual_status_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateEditActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let onclick_delete_component_modification = self.link.callback(|_| Msg::RequestDeleteModificationData);
        let onclick_component_modification_update = self.link.callback(|_| Msg::RequestUpdateModificationData);
        let onclick_show_preview_description = self.link.callback(|_| Msg::PreviewDescription);
        html!{<>
                <div class={"content"}>
                    <div class={"column"}>
                    <div class={"columns"}>
                        <div class={"column is-narrow"}>
                            <p class={"title is-5 select-title"}>{get_value_field(&96)}</p>
                        </div>
                        <div class={"column"}>
                            <div class={"select"}>
                            <select
                                id={"update-modification-actual-status"}
                                select={self.props.modification.actual_status.actual_status_id.to_string()}
                                onchange={onchange_modification_actual_status_id}
                                >
                                {for self.props.actual_statuses.iter().map(|x|
                                    html!{
                                        <option value={x.actual_status_id.to_string()}
                                            selected={x.actual_status_id == self.request_edit_modification.actual_status_id} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                            </select>
                            </div>
                        </div>
                    </div>
                    </div>
                    <div class={"column"}>
                        <p class={"title is-5"}>{get_value_field(&176)}</p>
                        <input
                            id={"add-modification-name"}
                            class={"input is-fullwidth"}
                            type={"text"}
                            placeholder={self.props.modification.modification_name.clone()}
                            value={self.request_edit_modification.modification_name.clone()}
                            oninput={oninput_modification_name} />
                    </div>
                    <div class={"column"}> // Description
                        <p class={"title is-5"}>{get_value_field(&61)}</p>
                        <button class={"button is-small is-fullwidth"} onclick={onclick_show_preview_description}>
                            <a>{match self.preview_description {
                                true => {get_value_field(&334)},
                                false => {get_value_field(&335)},
                            }}</a>
                        </button>
                        {match self.preview_description {
                            true => html!{
                                <div id={"update-modification-preview-description"} class={"box"}>
                                    {self.request_edit_modification.description.to_markdown()}
                                </div>
                            },
                            false => html!{
                                <textarea
                                    id={"update-modification-description"}
                                    class={"textarea is-fullwidth"}
                                    type={"text"}
                                    placeholder={self.props.modification.description.clone()}
                                    value={self.request_edit_modification.description.clone()}
                                    oninput={oninput_modification_description} />
                            },
                        }}
                        <p class="help">{get_value_field(&336)}</p>
                    </div>
                </div>
                <div class="columns">
                    <div class="column">
                        {ft_delete_btn(
                            "delete-component-modification",
                            onclick_delete_component_modification,
                            self.get_confirm == self.props.modification.uuid,
                            false
                        )}
                    </div>
                    <div class="column">
                        {ft_save_btn(
                            "update-component-modification",
                            onclick_component_modification_update,
                            true,
                            !self.modification_changed
                        )}
                    </div>
                </div>
            </>
        }
    }
}
