use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
// use yew_agent::Bridge;
use yew_router::prelude::*;
use yew_router::hooks::use_route;
use gloo::file::File;
use graphql_client::GraphQLQuery;
use web_sys::{DragEvent, Event, FileList};
use wasm_bindgen_futures::spawn_local;
use log::debug;
use serde_json::Value;
use crate::routes::AppRoute::{self, Login, Home, ShowComponent};
use crate::error::{get_error, Error};
use crate::fragments::{
    // switch_icon::res_btn,
    list_errors::ListErrors,
    // catalog_component::CatalogComponents,
    component::{
        ComponentStandardsCard, ComponentSuppliersCard,
        ComponentLicensesTags, ComponentParamsTags, UpdateComponentFaviconCard,
        ModificationsTableEdit, ComponentFilesBlock, SearchSpecsTags, AddKeywordsTags
    },
};
use crate::services::storage_upload::storage_upload;
use crate::services::{get_value_field, get_logged_user};
use crate::types::{
    UUID, ComponentInfo, SlimUser, TypeAccessInfo, UploadFile, ActualStatus, ComponentUpdatePreData,
    ComponentUpdateData, ComponentType, ShowCompanyShort, ComponentModificationInfo, ShowFileInfo,
};
use crate::gqls::{
    make_query,
    relate::{ConfirmUploadCompleted, confirm_upload_completed},
    component::{
        GetUpdateComponentDataOpt, get_update_component_data_opt,
        PutComponentUpdate, put_component_update,
        DeleteComponent, delete_component,
        ChangeComponentAccess, change_component_access,
        ComponentFilesList, component_files_list,
        UploadComponentFiles, upload_component_files,
    },
};

pub struct ComponentSettings {
    error: Option<Error>,
    current_component: Option<ComponentInfo>,
    current_component_uuid: UUID,
    current_component_is_base: bool,
    current_modifications: Vec<ComponentModificationInfo>,
    request_component: ComponentUpdatePreData,
    request_upload_data: Vec<UploadFile>,
    // request_upload_file: Callback<Result<Option<String>, Error>>,
    request_upload_confirm: Vec<UUID>,
    request_access: i64,
    // router_agent: Box<dyn Bridge<AppRoute>>,
    // task_read: Vec<(FileName, ReaderTask)>,
    // task: Vec<FetchTask>,
    supplier_list: Vec<ShowCompanyShort>,
    component_types: Vec<ComponentType>,
    actual_statuses: Vec<ActualStatus>,
    types_access: Vec<TypeAccessInfo>,
    update_component: bool,
    update_component_access: bool,
    update_component_supplier: bool,
    upload_component_files: bool,
    // put_upload_file: PutUploadFile,
    files: Vec<File>,
    files_index: u32,
    files_list: Vec<ShowFileInfo>,
    disable_delete_component_btn: bool,
    confirm_delete_component: String,
    hide_delete_modal: bool,
    disable_save_changes_btn: bool,
    select_component_modification: UUID,
    get_result_component_data: usize,
    get_result_access: bool,
    get_result_up_file: bool,
    get_result_up_completed: usize,
    active_loading_files_btn: bool,
}

#[derive(Properties, Clone, Debug, PartialEq)]
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
    RequestUploadComponentFiles,
    // RequestUploadFile(Vec<u8>),
    // ResponseUploadFile(Result<Option<String>, Error>),
    // RequestUploadCompleted,
    GetComponentData(String),
    GetListOpt(String),
    GetUpdateComponentResult(String),
    GetUpdateAccessResult(String),
    GetComponentFilesListResult(String),
    GetUploadData(String),
    // GetUploadFile,
    GetUploadCompleted(Result<usize, Error>),
    FinishUploadFiles,
    GetDeleteComponentResult(String),
    EditFiles,
    UpdateTypeAccessId(String),
    UpdateActualStatusId(String),
    UpdateComponentTypeId(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdateFiles(FileList),
    UpdateConfirmDelete(String),
    ResponseError(Error),
    RegisterNewModification(UUID),
    DeleteModification(UUID),
    ChangeHideDeleteComponent,
    ClearFilesBoxed,
    ClearError,
    Ignore,
}

impl Component for ComponentSettings {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ComponentSettings {
            error: None,
            current_component: None,
            current_component_uuid: String::new(),
            current_component_is_base: false,
            current_modifications: Vec::new(),
            request_component: ComponentUpdatePreData::default(),
            request_upload_data: Vec::new(),
            // request_upload_file: ctx.link().callback(Msg::ResponseUploadFile),
            request_upload_confirm: Vec::new(),
            request_access: 0,
            // router_agent: AppRoute::bridge(ctx.link().callback(|_| Msg::Ignore)),
            // task_read: Vec::new(),
            // task: Vec::new(),
            supplier_list: Vec::new(),
            component_types: Vec::new(),
            actual_statuses: Vec::new(),
            types_access: Vec::new(),
            update_component: false,
            update_component_access: false,
            update_component_supplier: false,
            upload_component_files: false,
            // put_upload_file: PutUploadFile::new(),
            files: Vec::new(),
            files_index: 0,
            files_list: Vec::new(),
            disable_delete_component_btn: true,
            confirm_delete_component: String::new(),
            hide_delete_modal: true,
            disable_save_changes_btn: true,
            select_component_modification: String::new(),
            get_result_component_data: 0,
            get_result_access: false,
            get_result_up_file: false,
            get_result_up_completed: 0,
            active_loading_files_btn: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let logged_user_uuid = match get_logged_user() {
            Some(cu) => cu.uuid,
            None => {
                // route to login page if not found token
                // self.router_agent.send(Login);
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Login);
                String::new()
            },
        };

        let target_component_uuid =
            use_route().unwrap_or_default().trim_start_matches("/component/settings/").to_string();
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
            let link = ctx.link().clone();
            // update current_component_uuid for checking change component in route
            self.current_component_uuid = target_component_uuid.clone();

            spawn_local(async move {
                let ipt_companies_arg = get_update_component_data_opt::IptCompaniesArg{
                    companies_uuids: None,
                    user_uuid: Some(logged_user_uuid),
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let navigator: Navigator = ctx.link().navigator().unwrap();

        match msg {
            Msg::OpenComponent => {
                // Redirect to component page
                // self.router_agent.send(ShowComponent { uuid: self.current_component_uuid.clone() });
                navigator.clone().replace(&ShowComponent { uuid: self.current_component_uuid.clone() });
            },
            Msg::RequestManager => {
                if self.update_component {
                    ctx.link().send_message(Msg::RequestUpdateComponentData)
                }
                if self.update_component_access {
                    ctx.link().send_message(Msg::RequestChangeAccess)
                }
                self.update_component = false;
                self.update_component_access = false;
                self.update_component_supplier = false;
                self.upload_component_files = false;
                self.disable_save_changes_btn = true;
                self.get_result_component_data = 0;
                self.get_result_access = false;
            },
            Msg::RequestComponentFilesList => {
                let component_uuid = ctx.props().component_uuid.clone();
                spawn_local(async move {
                    let ipt_component_files_arg = component_files_list::IptComponentFilesArg{
                        files_uuids: None,
                        component_uuid,
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
                        parent_component_uuid,
                        name,
                        description,
                        component_type_id,
                        actual_status_id,
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
                        component_uuid,
                        new_type_access_id,
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
            Msg::RequestUploadComponentFiles => {
                if !self.files.is_empty() && self.current_component_uuid.len() == 36 {
                    // see loading button
                    self.active_loading_files_btn = true;

                    let mut filenames: Vec<String> = Vec::new();
                    for file in &self.files {filenames.push(file.name().clone());}
                    debug!("filenames: {:?}", filenames);
                    let component_uuid = self.current_component_uuid.clone();

                    spawn_local(async move {
                        let ipt_component_files_data = upload_component_files::IptComponentFilesData{
                            filenames,
                            component_uuid,
                        };
                        let res = make_query(UploadComponentFiles::build_query(upload_component_files::Variables{
                            ipt_component_files_data
                        })).await.unwrap();
                        link.send_message(Msg::GetUploadData(res));
                    })
                }
            },
            // Msg::RequestUploadFile(data) => {
            //     if let Some(upload_data) = self.request_upload_data.pop() {
            //         let request = UploadData {
            //             upload_url: upload_data.upload_url.to_string(),
            //             file_data: data,
            //         };
            //         debug!("request: {:?}", request);
            //
            //         self.task.push(self.put_upload_file.put_file(request, self.request_upload_file.clone()));
            //         self.request_upload_confirm.push(upload_data.file_uuid.clone());
            //     };
            // },
            // Msg::RequestUploadCompleted => {
            //     let file_uuids = self.request_upload_confirm.clone();
            //     spawn_local(async move {
            //         let res = make_query(ConfirmUploadCompleted::build_query(
            //             confirm_upload_completed::Variables { file_uuids }
            //         )).await.unwrap();
            //         // debug!("ConfirmUploadCompleted: {:?}", res);
            //         link.send_message(Msg::GetUploadCompleted(res));
            //     });
            // },
            Msg::GetComponentFilesListResult(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.files_list = serde_json::from_value(
                            res_value.get("componentFilesList").unwrap().clone()
                        ).unwrap();
                        debug!("componentFilesList {:?}", self.files_list.len());
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result = serde_json::from_value(
                            res_value.get("uploadComponentFiles").unwrap().clone()
                        ).unwrap();
                        debug!("uploadComponentFiles {:?}", self.request_upload_data);

                        if !self.files.is_empty() {
                            let callback_confirm =
                                link.callback(|res: Result<usize, Error>| Msg::GetUploadCompleted(res));
                            storage_upload(result, self.files, callback_confirm);
                            // for file in self.files.iter().rev() {
                            //     let file_name = file.name().clone();
                            //     debug!("file name: {:?}", file_name);
                            //     let task = {
                            //         let callback = ctx.link()
                            //             .callback(move |data: FileData| Msg::RequestUploadFile(data.content));
                            //         ReaderService::read_file(file.clone(), callback).unwrap()
                            //     };
                            //     self.task_read.push((file_name, task));
                            // }
                        }
                        debug!("file: {:#?}", self.files);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            // Msg::ResponseUploadFile(Ok(res)) => {
            //     debug!("ResponseUploadFile: {:?}", res);
            //     // link.send_message(Msg::GetUploadFile)
            // },
            // Msg::ResponseUploadFile(Err(err)) => {
            //     self.error = Some(err);
            //     self.task.clear();
            //     self.task_read.clear();
            //     self.files_index = 0;
            //     self.request_upload_confirm.clear();
            //     self.get_result_up_completed = 0;
            //     self.active_loading_files_btn = false;
            // },
            Msg::GetComponentData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let component_data: ComponentInfo =
                            serde_json::from_value(res_value.get("component").unwrap().clone()).unwrap();
                        // debug!("Component data: {:?}", component_data);

                        self.current_component_uuid = component_data.uuid.clone();
                        self.current_component_is_base = component_data.is_base;
                        self.current_component = Some(component_data.clone());
                        // if let Some(user) = &ctx.props().current_user {
                        //     self.current_user_owner = component_data.owner_user.uuid == user.uuid;
                        // }
                        self.current_modifications = component_data.component_modifications.clone();
                        self.files_list = component_data.files.clone();
                        self.request_component = component_data.into();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetListOpt(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.supplier_list =
                            serde_json::from_value(res_value.get("companies").unwrap().clone()).unwrap();
                        self.component_types =
                            serde_json::from_value(res_value.get("componentTypes").unwrap().clone()).unwrap();
                        self.actual_statuses =
                            serde_json::from_value(res_value.get("componentActualStatuses").unwrap().clone()).unwrap();
                        self.types_access =
                            serde_json::from_value(res_value.get("typesAccess").unwrap().clone()).unwrap();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUpdateComponentResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize =
                            serde_json::from_value(res_value.get("putComponentUpdate").unwrap().clone()).unwrap();
                        // debug!("Component data: {:?}", result);
                        self.get_result_component_data = result;
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUpdateAccessResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("changeComponentAccess").unwrap().clone()).unwrap();
                        debug!("Component change access: {:?}", result);
                        self.update_component_access = false;
                        self.get_result_access = result;
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            // Msg::GetUploadFile => {
            //     debug!("next: {:?}", self.files_index);
            //     self.files_index -= 1;
            //     if self.files_index == 0 {
            //         self.get_result_up_file = true;
            //         debug!("finish: {:?}", self.request_upload_confirm.len());
            //         link.send_message(Msg::RequestUploadCompleted);
            //     }
            // },
            Msg::GetUploadCompleted(res) => {
                match res {
                    Ok(value) => self.get_result_up_completed = value,
                    Err(err) => self.error = Some(err),
                }
                self.active_loading_files_btn = false;
            },
            Msg::FinishUploadFiles => {
                self.files_list.clear();
                link.send_message(Msg::RequestComponentFilesList);
                self.active_loading_files_btn = false;
                // self.task.clear();
                // self.task_read.clear();
                self.request_upload_confirm.clear();
                self.files.clear();
                self.files_index = 0;
            },
            Msg::GetDeleteComponentResult(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UUID = serde_json::from_value(res_value.get("deleteComponent").unwrap().clone()).unwrap();
                        debug!("deleteComponent: {:?}", result);
                        if self.current_component_uuid == result {
                            // self.router_agent.send(Home)
                            navigator.replace(&Home);
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::EditFiles => self.upload_component_files = !self.upload_component_files,
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
            Msg::UpdateComponentTypeId(data) => {
                self.request_component.component_type_id = data.parse::<usize>().unwrap_or_default();
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
            Msg::UpdateFiles(files) => {
                while let Some(file) = files.get(self.files_index) {
                    debug!("self.files_index: {:?}", self.files_index);
                    self.files_index += 1;
                    self.upload_component_files = true;
                    // self.disable_save_changes_btn = false;
                    self.files.push(file.clone());
                }
                // self.files_index = 0;
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
            Msg::ClearFilesBoxed => {
                self.files.clear();
                self.files_index = 0;
                self.upload_component_files = false;
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.current_component_uuid == ctx.props().component_uuid {
            false
        } else {
            self.current_component_uuid = ctx.props().component_uuid;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{
            <div class="component-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
                        // <br/>
                        {self.show_manage_btn(ctx.link())}
                        <br/>
                        {self.show_main_card(ctx.link())}
                        {match &self.current_component {
                            Some(component_data) => html!{<>
                                <br/>
                                {self.show_modifications_table()}
                                <br/>
                                <div class="columns">
                                    {self.update_component_favicon(ctx.link())}
                                    {self.show_additional_params(component_data)}
                                </div>
                                // <br/>
                                {self.show_component_files(ctx.link())}
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
    fn show_main_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_name = link.callback(|ev: Event| Msg::UpdateName(ev.value));
        let oninput_description = link.callback(|ev: Event| Msg::UpdateDescription(ev.value));

        html!{<div class="card">
            <div class="column">
                <span class="id-box has-text-grey-light has-text-weight-bold">
                    {match self.current_component_is_base {
                        true => { get_value_field(&208) }, // base
                        false => { get_value_field(&209) }, // no base
                    }}
                </span>
                <div class="column" style="margin-right: 1rem">
                    <label class="label">{ get_value_field(&110) }</label>
                    <input
                        id="update-name"
                        class="input"
                        type="text"
                        placeholder={get_value_field(&110)}
                        value={self.request_component.name.clone()}
                        oninput={oninput_name} />
                    <label class="label">{ get_value_field(&61) }</label>
                    <textarea
                        id="update-description"
                        class="textarea"
                        // rows="10"
                        type="text"
                        placeholder={get_value_field(&61)}
                        value={self.request_component.description.clone()}
                        oninput={oninput_description} />
                    <br/>
                    {match &self.current_component {
                        Some(component_data) => self.show_component_licenses(component_data),
                        None => html!{},
                    }}
                    {self.show_component_params(link)}
                </div>
            </div>
        </div>}
    }

    fn show_component_licenses(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<ComponentLicensesTags
            show_delete_btn = {true}
            component_uuid = {self.current_component_uuid.clone()}
            component_licenses = {component_data.licenses.clone()}
        />}
    }

    fn show_component_params(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onchange_actual_status_id =
            link.callback(|ev: Event| Msg::UpdateActualStatusId(match ev {
              Event::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let onchange_change_component_type =
            link.callback(|ev: Event| Msg::UpdateComponentTypeId(match ev {
              Event::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let onchange_change_type_access =
            link.callback(|ev: Event| Msg::UpdateTypeAccessId(match ev {
              Event::Select(el) => el.value(),
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
                            onchange={onchange_actual_status_id}
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
                    <label class="label">{ get_value_field(&97) }</label>
                    <div class="select is-fullwidth">
                      <select
                          id="set-component-type"
                          select={self.request_component.component_type_id.to_string()}
                          onchange={onchange_change_component_type}
                        >
                      { for self.component_types.iter().map(|x|
                          html!{
                              <option value={x.component_type_id.to_string()}
                                    selected={x.component_type_id == self.request_component.component_type_id} >
                                  {&x.component_type}
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
                          onchange={onchange_change_type_access}
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
                current_component_uuid = {self.current_component_uuid.clone()}
                component_modifications = {self.current_modifications.clone()}
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
                  show_manage_btn = {true}
                  component_uuid = {self.current_component_uuid.clone()}
                  component_params = {component_data.component_params.clone()}
              />
        </div>}
    }

    fn update_component_favicon(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let callback_update_favicon = link.callback(|_| Msg::Ignore);

        html!{<div class="column">
            <h2 class="has-text-weight-bold">{ get_value_field(&184) }</h2> // Update image for preview
            <div class="card column">
                <UpdateComponentFaviconCard
                    component_uuid={self.current_component_uuid.clone()}
                    callback={callback_update_favicon.clone()}
                />
            </div>
        </div>}
    }

    fn show_component_files(
        self,
        link: &Scope<Self>,
    ) -> Html {
        html!{<>
            <h2 class="has-text-weight-bold">{ get_value_field(&187) }</h2> // Manage component files
            <div class="card column">
                <div class="columns">
                    <div class="column">
                      <h2 class="has-text-weight-bold">{ get_value_field(&188) }</h2> // Files for component
                      <ComponentFilesBlock
                          show_download_btn = {false}
                          show_delete_btn = {true}
                          component_uuid = {self.current_component_uuid.clone()}
                          files = {self.files_list.clone()}
                        />
                    </div>
                    <div class="column">
                      <h2 class="has-text-weight-bold">{ get_value_field(&186) }</h2> // Upload component files
                      {self.show_frame_upload_files(link)}
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
              show_delete_btn = {true}
              component_uuid = {component_data.uuid.clone()}
              component_standards = {component_data.component_standards.clone()}
              // delete_standard = {Some(onclick_delete_standard.clone())}
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
              show_delete_btn = {true}
              component_uuid = {component_data.uuid.clone()}
              component_suppliers = {component_data.component_suppliers.clone()}
              supplier_list = {self.supplier_list.clone()}
              is_base = {self.current_component_is_base}
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
                  component_specs = {component_data.component_specs.clone()}
                  component_uuid = {component_data.uuid.clone()}
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
                    component_keywords = {component_data.component_keywords.clone()}
                    component_uuid = {component_data.uuid.clone()}
                  />
              </div>
        </>}
    }

    fn show_manage_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_open_component = link.callback(|_| Msg::OpenComponent);
        let onclick_show_delete_modal = link.callback(|_| Msg::ChangeHideDeleteComponent);
        let onclick_save_changes = link.callback(|_| Msg::RequestManager);

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
                    {self.modal_delete_component(link)}
                    <div class="buttons">
                        <button
                            id="delete-component"
                            class="button is-danger"
                            onclick={onclick_show_delete_modal} >
                            { get_value_field(&135) }
                        </button>
                        <button
                            id="update-data"
                            class="button"
                            onclick={onclick_save_changes}
                            disabled={self.disable_save_changes_btn} >
                            {  get_value_field(&46) }
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn modal_delete_component(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_hide_modal = link.callback(|_| Msg::ChangeHideDeleteComponent);
        let oninput_delete_component = link.callback(|ev: Event| Msg::UpdateConfirmDelete(ev.value));
        let onclick_delete_component = link.callback(|_| Msg::RequestDeleteComponent);

        let class_modal = match &self.hide_delete_modal {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_hide_modal.clone()} />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{ get_value_field(&217) }</p> // Delete component
                      <button class="delete" aria-label="close" onclick={onclick_hide_modal.clone()} />
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
                           oninput={oninput_delete_component} />
                    </section>
                    <footer class="modal-card-foot">
                        <button
                            id="delete-component"
                            class="button is-danger"
                            disabled={self.disable_delete_component_btn}
                            onclick={onclick_delete_component} >{ get_value_field(&220) }</button> // Yes, delete
                        <button class="button" onclick={onclick_hide_modal.clone()}>{ get_value_field(&221) }</button> // Cancel
                    </footer>
                </div>
              </div>
            </div>
        }
    }

    fn show_frame_upload_files(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onchange_upload_files = link.callback(move |value| {
            if let Event::Files(files) = value {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });

        html!{<>
            <div class="file has-name is-boxed is-centered">
                <label class="file-label" style="width: 100%">
                  <input id="component-file-input"
                  class="file-input"
                  type="file"
                  // accept="image/*,application/vnd*,application/rtf,text/*,.pdf"
                  onchange={onchange_upload_files}
                  multiple=true />
                <span class="file-cta">
                  <span class="file-icon">
                    <i class="fas fa-upload"></i>
                  </span>
                  <span class="file-label">
                    { get_value_field(&200) } // Choose component files…
                  </span>
                </span>
                {match self.files.is_empty() {
                    true => html!{<span class="file-name">{ get_value_field(&194) }</span>}, // No file uploaded
                    false => html!{for self.files.iter().map(|f| html!{
                        <span class="file-name">{f.name().clone()}</span>
                    })}
                }}
              </label>
            </div>
            <div class="buttons">
                {self.show_clear_btn(link)}
                {self.show_upload_files_btn(link)}
            </div>
        </>}
    }

    fn show_clear_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_clear_boxed = link.callback(|_| Msg::ClearFilesBoxed);

        html!{
            <button id="clear-upload-component-files"
              class="button"
              onclick={onclick_clear_boxed}
              disabled={self.files.is_empty()} >
                // <span class="icon" >
                //     <i class="fas fa-boom" aria-hidden="true"></i>
                // </span>
                <span>{ get_value_field(&88) }</span>
            </button>
        }
    }

    fn show_upload_files_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_upload_files = link.callback(|_| Msg::RequestUploadComponentFiles);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <button
              id="upload-component-files"
              class={class_upload_btn}
              disabled={self.files.is_empty() || self.current_component_uuid.len() != 36}
              onclick={onclick_upload_files} >
                // <span class="icon" >
                //     <i class="fas fa-angle-double-up" aria-hidden="true"></i>
                // </span>
                <span>{ get_value_field(&87) }</span>
            </button>
        }
    }
}
