use yew::{
    agent::Bridged, html, Bridge, Component, Properties,
    ComponentLink, Html, ShouldRender, InputData, ChangeData
};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    file::UploaderFiles,
    list_errors::ListErrors,
    buttons::ft_save_btn,
    component::{
        ComponentStandardsCard, ComponentSuppliersCard,
        ComponentLicensesTags, ComponentParamsTags, UpdateComponentFaviconCard,
        ModificationsTableEdit, ComponentFilesBlock, SearchSpecsTags, AddKeywordsTags
    },
};
use crate::services::{get_logged_user, get_value_field, resp_parsing, get_value_response, get_from_value};
use crate::types::{
    UUID, ComponentInfo, SlimUser, TypeAccessInfo, UploadFile, ActualStatus, ComponentUpdatePreData,
    ComponentUpdateData, ShowCompanyShort, ComponentModificationInfo, ShowFileInfo,
};
use crate::gqls::make_query;
use crate::gqls::component::{
    GetUpdateComponentDataOpt, get_update_component_data_opt,
    PutComponentUpdate, put_component_update,
    DeleteComponent, delete_component,
    ChangeComponentAccess, change_component_access,
    ComponentFilesList, component_files_list,
    UploadComponentFiles, upload_component_files,
};

type FileName = String;

pub struct ComponentSettings {
    error: Option<Error>,
    current_component: Option<ComponentInfo>,
    current_component_uuid: UUID,
    current_component_is_base: bool,
    current_modifications: Vec<ComponentModificationInfo>,
    request_component: ComponentUpdatePreData,
    request_upload_data: Vec<UploadFile>,
    request_access: i64,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    supplier_list: Vec<ShowCompanyShort>,
    actual_statuses: Vec<ActualStatus>,
    types_access: Vec<TypeAccessInfo>,
    update_component: bool,
    update_component_access: bool,
    update_component_supplier: bool,
    files_list: Vec<ShowFileInfo>,
    disable_delete_component_btn: bool,
    confirm_delete_component: String,
    hide_delete_modal: bool,
    disable_save_changes_btn: bool,
    select_component_modification: UUID,
    get_result_component_data: usize,
    get_result_access: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub component_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    OpenComponent,
    RequestManager,
    RequestComponentFilesList,
    RequestUpdateComponentData,
    RequestChangeAccess,
    RequestDeleteComponent,
    RequestUploadComponentFiles(Vec<FileName>),
    GetComponentData(String),
    GetListOpt(String),
    GetUpdateComponentResult(String),
    GetUpdateAccessResult(String),
    GetComponentFilesListResult(String),
    GetUploadData(String),
    UploadConfirm(usize),
    FinishUploadFiles,
    GetDeleteComponentResult(String),
    UpdateTypeAccessId(String),
    UpdateActualStatusId(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdateConfirmDelete(String),
    ResponseError(Error),
    RegisterNewModification(UUID),
    DeleteModification(UUID),
    ChangeHideDeleteComponent,
    ClearError,
    Ignore,
}

impl Component for ComponentSettings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ComponentSettings {
            error: None,
            current_component: None,
            current_component_uuid: String::new(),
            current_component_is_base: false,
            current_modifications: Vec::new(),
            request_component: ComponentUpdatePreData::default(),
            request_upload_data: Vec::new(),
            request_access: 0,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            supplier_list: Vec::new(),
            actual_statuses: Vec::new(),
            types_access: Vec::new(),
            update_component: false,
            update_component_access: false,
            update_component_supplier: false,
            files_list: Vec::new(),
            disable_delete_component_btn: true,
            confirm_delete_component: String::new(),
            hide_delete_modal: true,
            disable_save_changes_btn: true,
            select_component_modification: String::new(),
            get_result_component_data: 0,
            get_result_access: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let logged_user_uuid = match get_logged_user() {
            Some(cu) => cu.uuid,
            None => {
                // route to login page if not found token
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                String::new()
            },
        };

        // get component uuid for request component data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_component_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/component/settings/")
            .to_string();
        // get flag changing current component in route
        let not_matches_component_uuid = target_component_uuid != self.current_component_uuid;
        // debug!("self.current_component_uuid {:#?}", self.current_component_uuid);

        if not_matches_component_uuid {
            // clear old data
            self.current_component = None;
            self.current_component_uuid = String::new();
            self.current_component_is_base = false;
            self.current_modifications.clear();
            self.request_component = ComponentUpdatePreData::default();
            self.select_component_modification = String::new();
        }

        if first_render || not_matches_component_uuid {
            let link = self.link.clone();

            // update current_component_uuid for checking change component in route
            self.current_component_uuid = target_component_uuid.clone();

            spawn_local(async move {
                let ipt_companies_arg = get_update_component_data_opt::IptCompaniesArg{
                    companiesUuids: None,
                    userUuid: Some(logged_user_uuid),
                    favorite: None,
                    supplier: Some(true),
                    limit: None,
                    offset: None,
                };
                let res = make_query(GetUpdateComponentDataOpt::build_query(get_update_component_data_opt::Variables {
                    component_uuid: target_component_uuid,
                    ipt_companies_arg,
                })).await.unwrap();

                link.send_message(Msg::GetComponentData(res.clone()));
                link.send_message(Msg::GetListOpt(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::OpenComponent => {
                // Redirect to component page
                self.router_agent.send(ChangeRoute(
                    AppRoute::ShowComponent(self.current_component_uuid.clone()).into()
                ));
            },
            Msg::RequestManager => {
                if self.update_component {
                    self.link.send_message(Msg::RequestUpdateComponentData)
                }
                if self.update_component_access {
                    self.link.send_message(Msg::RequestChangeAccess)
                }
                self.update_component = false;
                self.update_component_access = false;
                self.update_component_supplier = false;
                self.disable_save_changes_btn = true;
                self.get_result_component_data = 0;
                self.get_result_access = false;
            },
            Msg::RequestComponentFilesList => {
                let component_uuid = self.props.component_uuid.clone();
                spawn_local(async move {
                    let ipt_component_files_arg = component_files_list::IptComponentFilesArg{
                        filesUuids: None,
                        componentUuid: component_uuid,
                        limit: None,
                        offset: None,
                    };
                    let res = make_query(ComponentFilesList::build_query(
                        component_files_list::Variables { ipt_component_files_arg }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentFilesListResult(res));
                })
            },
            Msg::RequestUpdateComponentData => {
                let component_uuid = self.current_component_uuid.clone();
                let request_component: ComponentUpdateData = (&self.request_component).into();

                spawn_local(async move {
                    let ComponentUpdateData {
                        parent_component_uuid,
                        name,
                        description,
                        component_type_id,
                        actual_status_id,
                    } = request_component;
                    let ipt_update_component_data = put_component_update::IptUpdateComponentData {
                        parentComponentUuid: parent_component_uuid,
                        name,
                        description,
                        componentTypeId: component_type_id,
                        actualStatusId: actual_status_id,
                    };
                    let res = make_query(PutComponentUpdate::build_query(put_component_update::Variables {
                        component_uuid,
                        ipt_update_component_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateComponentResult(res));
                })
            },
            Msg::RequestChangeAccess => {
                let component_uuid = self.current_component_uuid.clone();
                let new_type_access_id = self.request_access.clone();
                spawn_local(async move {
                    let change_type_access_component = change_component_access::ChangeTypeAccessComponent{
                        componentUuid: component_uuid,
                        newTypeAccessId: new_type_access_id,
                    };
                    let res = make_query(ChangeComponentAccess::build_query(change_component_access::Variables {
                        change_type_access_component
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateAccessResult(res));
                })
            },
            Msg::RequestDeleteComponent => {
                let component_uuid = self.current_component_uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteComponent::build_query(
                        delete_component::Variables { component_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteComponentResult(res));
                })
            },
            Msg::RequestUploadComponentFiles(filenames) => {
                debug!("filenames: {:?}", filenames);
                if self.current_component_uuid.len() != 36 || filenames.is_empty() {
                    return false
                }
                let component_uuid = self.current_component_uuid.clone();
                spawn_local(async move {
                    let ipt_component_files_data = upload_component_files::IptComponentFilesData{
                        filenames,
                        componentUuid: component_uuid,
                    };
                    let res = make_query(UploadComponentFiles::build_query(upload_component_files::Variables{
                        ipt_component_files_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                })
            },
            Msg::GetComponentFilesListResult(res) => {
                match resp_parsing(res, "componentFilesList") {
                    Ok(result) => self.files_list = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("componentFilesList {:?}", self.files_list.len());
            },
            Msg::GetUploadData(res) => {
                match resp_parsing(res, "uploadComponentFiles") {
                    Ok(result) => self.request_upload_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("uploadComponentFiles {:?}", self.request_upload_data);
            },
            Msg::GetComponentData(res) => {
                match resp_parsing::<ComponentInfo>(res, "component") {
                    Ok(component_data) => {
                        // debug!("Component data: {:?}", component_data);
                        self.current_component_uuid = component_data.uuid.clone();
                        self.current_component_is_base = component_data.is_base;
                        self.current_component = Some(component_data.clone());
                        // if let Some(user) = &self.props.current_user {
                        //     self.current_user_owner = component_data.owner_user.uuid == user.uuid;
                        // }
                        self.current_modifications = component_data.component_modifications.clone();
                        self.files_list = component_data.files.clone();
                        self.request_component = component_data.into();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetListOpt(res) => {
                match get_value_response(res) {
                    Ok(value) => {
                        self.supplier_list = get_from_value(&value, "companies").unwrap_or_default();
                        self.actual_statuses = get_from_value(&value, "componentActualStatuses").unwrap_or_default();
                        self.types_access = get_from_value(&value, "typesAccess").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateComponentResult(res) => {
                match resp_parsing(res, "putComponentUpdate") {
                    Ok(result) => self.get_result_component_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateAccessResult(res) => {
                self.update_component_access = false;
                match resp_parsing(res, "changeComponentAccess") {
                    Ok(result) => self.get_result_access = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of files: {:?}", confirmations);
                link.send_message(Msg::FinishUploadFiles);
            },
            Msg::FinishUploadFiles => {
                self.request_upload_data.clear();
                self.files_list.clear();
                link.send_message(Msg::RequestComponentFilesList);
            },
            Msg::GetDeleteComponentResult(res) => {
                match resp_parsing::<UUID>(res, "deleteComponent") {
                    Ok(result) => {
                        debug!("deleteComponent: {:?}", result);
                        if self.current_component_uuid == result {
                            self.router_agent.send(ChangeRoute(AppRoute::Home.into()))
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateTypeAccessId(data) => {
                self.request_access = data.parse::<i64>().unwrap_or_default();
                self.update_component_access = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateActualStatusId(data) => {
                self.request_component.actual_status_id = data.parse::<usize>().unwrap_or_default();
                self.update_component = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateName(data) => {
                self.request_component.name = data;
                self.update_component = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateDescription(data) => {
                self.request_component.description = data;
                self.update_component = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateConfirmDelete(data) => {
                self.disable_delete_component_btn = self.current_component_uuid != data;
                self.confirm_delete_component = data;
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::RegisterNewModification(modification_uuid) => {
                // link.send_message(Msg::RequestComponentModificationsData);
                self.select_component_modification = modification_uuid.clone();
            },
            Msg::DeleteModification(_) => {
                // link.send_message(Msg::RequestComponentModificationsData);
                self.select_component_modification = String::new();
            },
            Msg::ChangeHideDeleteComponent => self.hide_delete_modal = !self.hide_delete_modal,
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

        html!{
            <div class="component-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error=self.error.clone() clear_error=onclick_clear_error.clone()/>
                        // <br/>
                        {self.show_manage_btn()}
                        <br/>
                        {self.show_main_card()}
                        {match &self.current_component {
                            Some(component_data) => html!{<>
                                <br/>
                                {self.show_modifications_table()}
                                <br/>
                                <div class="columns">
                                    {self.update_component_favicon()}
                                    {self.show_additional_params(component_data)}
                                </div>
                                // <br/>
                                {self.show_component_files()}
                                <br/>
                                <div class="columns">
                                  {self.show_component_standards(component_data)}
                                  {self.show_component_suppliers(component_data)}
                                </div>
                                // <br/>
                                {self.show_component_specs(component_data)}
                                <br/>
                                {self.show_component_keywords(component_data)}
                                <br/>
                            </>},
                            None => html!{},
                        }}
                    </div>
                </div>
            </div>
        }
    }
}

impl ComponentSettings {
    fn show_main_card(&self) -> Html {
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));

        let oninput_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        html!{<div class="card">
            <div class="column">
                <div class="column" style="margin-right: 1rem">
                    <label class="label">{ get_value_field(&110) }</label>
                    <input
                        id="update-name"
                        class="input"
                        type="text"
                        placeholder=get_value_field(&110)
                        value={self.request_component.name.clone()}
                        oninput=oninput_name />
                    <label class="label">{ get_value_field(&61) }</label>
                    <textarea
                        id="update-description"
                        class="textarea"
                        // rows="10"
                        type="text"
                        placeholder=get_value_field(&61)
                        value={self.request_component.description.clone()}
                        oninput=oninput_description />
                    <br/>
                    {match &self.current_component {
                        Some(component_data) => self.show_component_licenses(component_data),
                        None => html!{},
                    }}
                    {self.show_component_params()}
                </div>
            </div>
        </div>}
    }

    fn show_component_licenses(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<ComponentLicensesTags
            show_delete_btn = true
            component_uuid = self.current_component_uuid.clone()
            component_licenses = component_data.licenses.clone()
        />}
    }

    fn show_component_params(&self) -> Html {
        let onchange_actual_status_id = self.link.callback(|ev: ChangeData| Msg::UpdateActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));
        let onchange_change_type_access = self.link.callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        html!{
            <div class="columns">
                <div class="column">
                    <label class="label">{ get_value_field(&96) }</label>
                    <div class="select is-fullwidth">
                        <select
                            id="component-actual-status"
                            select={self.request_component.actual_status_id.to_string()}
                            onchange=onchange_actual_status_id
                            >
                          { for self.actual_statuses.iter().map(|x|
                              html!{
                                  <option value={x.actual_status_id.to_string()}
                                        selected={x.actual_status_id == self.request_component.actual_status_id} >
                                      {&x.name}
                                  </option>
                              }
                          )}
                        </select>
                    </div>
                </div>
                <div class="column">
                    <label class="label">{ get_value_field(&114) }</label>
                    <div class="select is-fullwidth">
                      <select
                          id="set-type-access"
                          select={self.request_access.to_string()}
                          onchange=onchange_change_type_access
                        >
                      { for self.types_access.iter().map(|x|
                          html!{
                              <option value={x.type_access_id.to_string()}
                                    selected={x.type_access_id as i64 == self.request_access} >
                                  {&x.name}
                              </option>
                          }
                      )}
                      </select>
                    </div>
                </div>
            </div>
        }
    }

    fn show_modifications_table(&self) -> Html {
        html!{<>
            <h2 class="has-text-weight-bold">{ get_value_field(&60) }</h2> // Manage component modifications
            <ModificationsTableEdit
                current_component_uuid = self.current_component_uuid.clone()
                component_modifications = self.current_modifications.clone()
              />
        </>}
    }

    fn show_additional_params(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<div class="column">
              <h2 class="has-text-weight-bold">{ get_value_field(&185) }</h2> // Manage component characteristics
              <ComponentParamsTags
                  show_manage_btn = true
                  component_uuid = self.current_component_uuid.clone()
                  component_params = component_data.component_params.clone()
              />
        </div>}
    }

    fn update_component_favicon(&self) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::Ignore);

        html!{<div class="column">
            <h2 class="has-text-weight-bold">{ get_value_field(&184) }</h2> // Update image for preview
            <div class="card column">
                <UpdateComponentFaviconCard
                    component_uuid=self.current_component_uuid.clone()
                    callback=callback_update_favicon.clone()
                />
            </div>
        </div>}
    }

    fn show_component_files(&self) -> Html {
        let callback_upload_filenames =
            self.link.callback(move |filenames| Msg::RequestUploadComponentFiles(filenames));
        let request_upload_files = match self.request_upload_data.is_empty() {
            true => None,
            false => Some(self.request_upload_data.clone()),
        };
        let callback_upload_confirm =
            self.link.callback(|confirmations| Msg::UploadConfirm(confirmations));
        html!{<>
            <h2 class="has-text-weight-bold">{ get_value_field(&187) }</h2> // Manage component files
            <div class="card column">
                <div class="columns">
                    <div class="column">
                      <h2 class="has-text-weight-bold">{ get_value_field(&188) }</h2> // Files for component
                      <ComponentFilesBlock
                          show_download_btn = false
                          show_delete_btn = true
                          component_uuid = self.current_component_uuid.clone()
                          files = self.files_list.clone()
                        />
                    </div>
                    <div class="column">
                      <h2 class="has-text-weight-bold">{ get_value_field(&186) }</h2> // Upload component files
                      <UploaderFiles
                        text_choose_files={200} // Choose component files…
                        callback_upload_filenames={callback_upload_filenames}
                        request_upload_files={request_upload_files}
                        callback_upload_confirm={callback_upload_confirm}
                        />
                    </div>
                </div>
            </div>
        </>}
    }

    fn show_component_standards(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<div class="column">
          <h2 class="has-text-weight-bold">{ get_value_field(&189) }</h2> // Manage component standards
          <ComponentStandardsCard
              show_delete_btn = true
              component_uuid = component_data.uuid.clone()
              component_standards = component_data.component_standards.clone()
              // delete_standard = Some(onclick_delete_standard.clone())
            />
        </div>}
    }

    fn show_component_suppliers(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<div class="column">
          <h2 class="has-text-weight-bold">{ get_value_field(&190) }</h2> // Manage component suppliers
          <ComponentSuppliersCard
              show_delete_btn = true
              component_uuid = component_data.uuid.clone()
              component_suppliers = component_data.component_suppliers.clone()
              supplier_list = self.supplier_list.clone()
              is_base = self.current_component_is_base
            />
        </div>}
    }

    fn show_component_specs(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<>
            <h2 class="has-text-weight-bold">{ get_value_field(&104) }</h2>
            <div class="card">
              <SearchSpecsTags
                  component_specs = component_data.component_specs.clone()
                  component_uuid = component_data.uuid.clone()
                />
            </div>
        </>}
    }

    fn show_component_keywords(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        // debug!("Keywords: {:?}", &component_data.uuid);
        html!{<>
              <h2 class="has-text-weight-bold">{ get_value_field(&105) }</h2>
              <div class="card">
                <AddKeywordsTags
                    component_keywords = component_data.component_keywords.clone()
                    component_uuid = component_data.uuid.clone()
                  />
              </div>
        </>}
    }

    fn show_manage_btn(&self) -> Html {
        let onclick_open_component = self.link.callback(|_| Msg::OpenComponent);
        let onclick_show_delete_modal = self.link.callback(|_| Msg::ChangeHideDeleteComponent);
        let onclick_save_changes = self.link.callback(|_| Msg::RequestManager);

        html!{
            <div class="media">
                <div class="media-left">
                    <button
                        id="open-component"
                        class="button"
                        onclick={onclick_open_component} >
                        { get_value_field(&199) } // Show component
                    </button>
                </div>
                <div class="media-content">
                    {if self.get_result_component_data > 0 || self.get_result_access {
                        html!{get_value_field(&214) } // Data updated
                    } else {
                        html!{}
                    }}
                </div>
                <div class="media-right">
                    {self.modal_delete_component()}
                    <div class="buttons">
                        <button
                            id="delete-component"
                            class="button is-danger"
                            onclick={onclick_show_delete_modal} >
                            { get_value_field(&135) }
                        </button>
                        {ft_save_btn(
                            "update-component-data",
                            onclick_save_changes,
                            false,
                            self.disable_save_changes_btn
                        )}
                    </div>
                </div>
            </div>
        }
    }

    fn modal_delete_component(&self) -> Html {
        let onclick_hide_modal = self.link.callback(|_| Msg::ChangeHideDeleteComponent);
        let oninput_delete_component = self.link.callback(|ev: InputData| Msg::UpdateConfirmDelete(ev.value));
        let onclick_delete_component = self.link.callback(|_| Msg::RequestDeleteComponent);
        let class_modal = match &self.hide_delete_modal {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_hide_modal.clone() />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{ get_value_field(&217) }</p> // Delete component
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <p class="is-size-6">
                            { get_value_field(&218) } // For confirm deleted all data this
                            <span class="has-text-danger-dark">{self.request_component.name.clone()}</span>
                            { get_value_field(&219) } // component enter this uuid
                            <br/>
                            <span class="has-text-weight-bold is-size-6">{self.current_component_uuid.clone()}</span>
                        </p>
                        <br/>
                         <input
                           id="delete-component"
                           class="input"
                           type="text"
                           placeholder="uuid"
                           value={self.confirm_delete_component.clone()}
                           oninput=oninput_delete_component />
                    </section>
                    <footer class="modal-card-foot">
                        <button
                            id="delete-component"
                            class="button is-danger"
                            disabled={self.disable_delete_component_btn}
                            onclick={onclick_delete_component} >{ get_value_field(&220) }</button> // Yes, delete
                        <button class="button" onclick=onclick_hide_modal.clone()>{ get_value_field(&221) }</button> // Cancel
                    </footer>
                </div>
              </div>
            </div>
        }
    }
}
