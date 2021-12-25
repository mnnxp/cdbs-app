use yew::services::fetch::FetchTask;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
use web_sys::FileList;
use chrono::NaiveDateTime;
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::{get_error, Error};
use crate::fragments::{
    // switch_icon::res_btn,
    list_errors::ListErrors,
    // catalog_component::CatalogComponents,
    component::{ComponentStandardsCard, ComponentSuppliersCard, ComponentLicensesTags},
    component_file::FilesCard,
    component_spec::SearchSpecsTags,
    component_keyword::AddKeywordsTags,
};
use crate::gqls::make_query;
use crate::services::{
    PutUploadFile, UploadData,
    is_authenticated, get_logged_user
};
use crate::types::{
    UUID, ComponentInfo, SlimUser, TypeAccessInfo, UploadFile, ActualStatus,
    ComponentUpdatePreData, ComponentUpdateData, ComponentType, ShowCompanyShort,
};

type FileName = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetUpdateComponentDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct PutComponentUpdate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct DeleteComponent;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ChangeComponentAccess;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct UploadComponentFiles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct ConfirmUploadCompleted;

/// Component with relate data
pub struct ComponentSettings {
    error: Option<Error>,
    current_component: Option<ComponentInfo>,
    current_component_uuid: UUID,
    current_component_is_base: bool,
    request_component: ComponentUpdatePreData,
    request_upload_data: Vec<UploadFile>,
    request_upload_file: Callback<Result<Option<String>, Error>>,
    request_access: i64,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    task_read: Vec<(FileName, ReaderTask)>,
    task: Vec<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
    supplier_list: Vec<ShowCompanyShort>,
    component_types: Vec<ComponentType>,
    actual_statuses: Vec<ActualStatus>,
    types_access: Vec<TypeAccessInfo>,
    update_component: bool,
    update_component_access: bool,
    update_component_supplier: bool,
    upload_component_files: bool,
    put_upload_file: PutUploadFile,
    files: Vec<File>,
    files_index: u32,
    disable_delete_component_btn: bool,
    confirm_delete_component: String,
    hide_delete_modal: bool,
    disable_save_changes_btn: bool,
    get_result_component_data: usize,
    get_result_access: bool,
    get_result_up_file: bool,
    get_result_up_completed: usize,
    get_result_upload_files: bool,
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
    RequestUpdateComponentData,
    RequestChangeAccess,
    RequestDeleteComponent,
    RequestUploadComponentFiles,
    RequestUploadFile(Vec<u8>),
    ResponseUploadFile(Result<Option<String>, Error>),
    RequestUploadCompleted,
    GetComponentData(String),
    GetListOpt(String),
    GetUpdateComponentResult(String),
    GetUpdateAccessResult(String),
    GetUploadData(String),
    GetUploadFile(Option<String>),
    GetUploadCompleted(String),
    GetDeleteComponent(String),
    EditFiles,
    UpdateTypeAccessId(String),
    UpdateActualStatusId(String),
    UpdateComponentTypeId(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdateFiles(FileList),
    UpdateConfirmDelete(String),
    ResponseError(Error),
    ChangeHideDeleteComponent,
    ClearFilesBoxed,
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
            request_component: ComponentUpdatePreData::default(),
            request_upload_data: Vec::new(),
            request_upload_file: link.callback(Msg::ResponseUploadFile),
            request_access: 0,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            task_read: Vec::new(),
            task: Vec::new(),
            props,
            link,
            supplier_list: Vec::new(),
            component_types: Vec::new(),
            actual_statuses: Vec::new(),
            types_access: Vec::new(),
            update_component: false,
            update_component_access: false,
            update_component_supplier: false,
            upload_component_files: false,
            put_upload_file: PutUploadFile::new(),
            files: Vec::new(),
            files_index: 0,
            disable_delete_component_btn: true,
            confirm_delete_component: String::new(),
            hide_delete_modal: true,
            disable_save_changes_btn: true,
            get_result_component_data: 0,
            get_result_access: false,
            get_result_up_file: false,
            get_result_up_completed: 0,
            get_result_upload_files: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
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
            self.request_component = ComponentUpdatePreData::default();
        }

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_component_uuid) && is_authenticated() {
            // update current_component_uuid for checking change component in route
            self.current_component_uuid = target_component_uuid.clone();
            let user_uuid = match &self.props.current_user {
                Some(user) => user.uuid.clone(),
                None => get_logged_user().unwrap().uuid.clone(),
            };

            spawn_local(async move {
                let ipt_companies_arg = get_update_component_data_opt::IptCompaniesArg{
                    companiesUuids: None,
                    userUuid: Some(user_uuid),
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

                if self.upload_component_files && !self.files.is_empty() {
                    self.link.send_message(Msg::RequestUploadComponentFiles);
                }

                self.update_component = false;
                self.update_component_access = false;
                self.update_component_supplier = false;
                self.upload_component_files = false;
                self.disable_save_changes_btn = true;
                self.get_result_component_data = 0;
                self.get_result_access = false;
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
                    link.send_message(Msg::GetDeleteComponent(res));
                })
            },
            Msg::RequestUploadComponentFiles => {
                // see loading button
                self.get_result_upload_files = true;

                if !self.files.is_empty() {
                    let mut filenames: Vec<String> = Vec::new();
                    for file in &self.files {filenames.push(file.name().clone());}
                    debug!("filenames: {:?}", filenames);
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
                }
            },
            Msg::RequestUploadFile(data) => {
                if let Some(upload_data) = self.request_upload_data.pop() {
                    let request = UploadData {
                        upload_url: upload_data.upload_url.to_string(),
                        file_data: data,
                    };
                    debug!("request: {:?}", request);

                    self.task.push(self.put_upload_file.put_file(request, self.request_upload_file.clone()));
                };
            },
            Msg::RequestUploadCompleted => {
                if !self.request_upload_data.is_empty() {
                    let mut file_uuids: Vec<UUID> = Vec::new();
                    for up_data in &self.request_upload_data {file_uuids.push(up_data.file_uuid.clone());}
                    spawn_local(async move {
                        let res = make_query(ConfirmUploadCompleted::build_query(confirm_upload_completed::Variables {
                            file_uuids,
                        })).await.unwrap();
                        debug!("ConfirmUploadCompleted: {:?}", res);
                        link.send_message(Msg::GetUploadCompleted(res));
                    });
                }
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<UploadFile> = serde_json::from_value(res_value.get("uploadComponentFiles").unwrap().clone()).unwrap();
                        debug!("uploadComponentFiles {:?}", result);
                        self.request_upload_data = result;

                        if !self.files.is_empty() {
                            for file in self.files.iter().rev() {
                                let file_name = file.name().clone();
                                debug!("file name: {:?}", file_name);
                                let task = {
                                    let callback = self.link
                                        .callback(move |data: FileData| Msg::RequestUploadFile(data.content));
                                    ReaderService::read_file(file.clone(), callback).unwrap()
                                };
                                self.task_read.push((file_name, task));
                            }
                        }
                        debug!("file: {:#?}", self.files);
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::ResponseUploadFile(Ok(res)) => {
                link.send_message(Msg::GetUploadFile(res))
            },
            Msg::ResponseUploadFile(Err(err)) => {
                self.error = Some(err);
                self.task = Vec::new();
                self.task_read = Vec::new();
            },
            Msg::GetComponentData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let component_data: ComponentInfo =
                            serde_json::from_value(res_value.get("component").unwrap().clone()).unwrap();
                        debug!("Component data: {:?}", component_data);

                        self.current_component_uuid = component_data.uuid.clone();
                        self.current_component_is_base = component_data.is_base;
                        self.current_component = Some(component_data.clone());
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
                        self.supplier_list = serde_json::from_value(
                            res_value.get("companies").unwrap().clone()
                        ).unwrap();
                        self.component_types = serde_json::from_value(
                            res_value.get("componentTypes").unwrap().clone()
                        ).unwrap();
                        self.actual_statuses = serde_json::from_value(
                            res_value.get("componentActualStatuses").unwrap().clone()
                        ).unwrap();
                        self.types_access = serde_json::from_value(
                            res_value.get("typesAccess").unwrap().clone()
                        ).unwrap();
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
                        debug!("Component data: {:?}", result);
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
            Msg::GetUploadFile(res) => {
                debug!("res: {:?}", res);
                if self.files_index == 0 {
                    self.get_result_up_file = true;
                    link.send_message(Msg::RequestUploadCompleted);
                } else {
                    self.files_index -= 1;
                }
            },
            Msg::GetUploadCompleted(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res_value.get("uploadCompleted").unwrap().clone()).unwrap();
                        debug!("uploadCompleted: {:?}", result);
                        self.get_result_up_completed = result;
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetDeleteComponent(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UUID = serde_json::from_value(res_value.get("deleteComponent").unwrap().clone()).unwrap();
                        debug!("deleteComponent: {:?}", result);
                        if self.current_component_uuid == result {
                            self.router_agent.send(ChangeRoute(AppRoute::Home.into()))
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
                    self.disable_save_changes_btn = false;
                    self.files.push(file.clone());
                }
                self.files_index = 0;
            },
            Msg::UpdateConfirmDelete(data) => {
                self.disable_delete_component_btn = self.current_component_uuid != data;
                self.confirm_delete_component = data;
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ChangeHideDeleteComponent => self.hide_delete_modal = !self.hide_delete_modal,
            Msg::ClearFilesBoxed => {
                self.files = Vec::new();
                self.files_index = 0;
                self.upload_component_files = false;
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{
            <div class="component-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
                        // <br/>
                        {self.show_manage_btn()}
                        <br/>
                        {self.show_main_card()}
                        {match &self.current_component {
                            Some(component_data) => html!{<>
                                <div class="columns">
                                  {self.show_component_params()}
                                  {self.show_component_files(component_data)}
                                </div>
                                <div class="columns">
                                  {self.show_component_standards(component_data)}
                                  {self.show_component_suppliers(component_data)}
                                </div>
                                <br/>
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
        let onchange_change_type_access = self.link
            .callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let oninput_name = self.link
            .callback(|ev: InputData| Msg::UpdateName(ev.value));

        let oninput_description = self.link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        html!{<div class="card">
            <div class="column">
                <div class="column">
                    <div class="media">
                        <div class="media-content">
                            <a class="id-box has-text-grey-light has-text-weight-bold">
                                {match self.current_component_is_base {
                                    true => {"base"},
                                    false => {"no base"},
                                }}
                            </a>
                        </div>
                        <div class="media-right" style="margin-right: 1rem">
                            <label class="label">{"Type access "}</label>
                            <div class="select">
                              <select
                                  id="set-type-access"
                                  select={self.request_access.to_string()}
                                  onchange=onchange_change_type_access
                                >
                              { for self.types_access.iter().map(|x|
                                  match self.current_component.as_ref().map(|c| c.type_access.type_access_id).unwrap_or_default() == x.type_access_id {
                                      true => html!{ <option value={x.type_access_id.to_string()} selected=true>{&x.name}</option> },
                                      false => html!{ <option value={x.type_access_id.to_string()}>{&x.name}</option> },
                                  }
                              )}
                              </select>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="column" style="margin-right: 1rem">
                    <label class="label">{"Name"}</label>
                    <input
                        id="update-name"
                        class="input"
                        type="text"
                        placeholder="component name"
                        value={self.request_component.name.clone()}
                        oninput=oninput_name />
                    <label class="label">{"Description"}</label>
                    <textarea
                        id="update-description"
                        class="textarea"
                        // rows="10"
                        type="text"
                        placeholder="component description"
                        value={self.request_component.description.clone()}
                        oninput=oninput_description />
                    {match &self.current_component {
                        Some(component_data) => self.show_component_licenses(component_data),
                        None => html!{},
                    }}
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
        let onchange_actual_status_id = self.link
            .callback(|ev: ChangeData| Msg::UpdateActualStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let onchange_change_component_type = self.link
            .callback(|ev: ChangeData| Msg::UpdateComponentTypeId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        html!{
            <div class="column">
              <h2>{"Сharacteristics"}</h2>
              <div class="card">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{"Actual status"}</td>
                        <td><div class="select">
                            <select
                                id="component-actual-status"
                                select={self.request_component.actual_status_id.to_string()}
                                onchange=onchange_actual_status_id
                                >
                              { for self.actual_statuses.iter().map(|x|
                                  match self.request_component.actual_status_id == x.actual_status_id {
                                      true => html!{<option value={x.actual_status_id.to_string()} selected=true>{&x.name}</option>},
                                      false => html!{<option value={x.actual_status_id.to_string()}>{&x.name}</option>},
                                  }
                              )}
                            </select>
                        </div></td>
                      </tr>
                      <tr>
                        <td>{"Component type"}</td>
                        <td><div class="select">
                          <select
                              id="set-component-type"
                              select={self.request_component.component_type_id.to_string()}
                              onchange=onchange_change_component_type
                            >
                          { for self.component_types.iter().map(|x|
                              match self.request_component.component_type_id == x.component_type_id {
                                  true => html!{ <option value={x.component_type_id.to_string()} selected=true>{&x.component_type}</option> },
                                  false => html!{ <option value={x.component_type_id.to_string()}>{&x.component_type}</option> },
                              }
                          )}
                          </select>
                        </div></td>
                      </tr>
                    </tbody>
                  </table>
              </div>
            </div>
        }
    }

    fn show_component_files(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2>{"Files"}</h2>
              {self.show_frame_upload_files()}
              <FilesCard
                  show_download_btn = false
                  show_delete_btn = true
                  component_uuid = component_data.uuid.clone()
                  files = component_data.files.clone()
                />
            </div>
        }
    }

    fn show_component_standards(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<div class="column">
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
            <h2>{"Specs"}</h2>
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
              <h2>{"Keywords"}</h2>
              <div class="card">
                <AddKeywordsTags
                    component_keywords = component_data.component_keywords.clone()
                    component_uuid = component_data.uuid.clone()
                  />
              </div>
        </>}
    }

    fn show_manage_btn(&self) -> Html {
        let onclick_open_component = self.link
            .callback(|_| Msg::OpenComponent);
        let onclick_show_delete_modal = self.link
            .callback(|_| Msg::ChangeHideDeleteComponent);
        let onclick_save_changes = self.link
            .callback(|_| Msg::RequestManager);

        html!{
            <div class="media">
                <div class="media-left">
                    <button
                        id="open-component"
                        class="button"
                        onclick={onclick_open_component} >
                        {"Cancel"}
                    </button>
                </div>
                <div class="media-content">
                    {if self.get_result_component_data > 0 || self.get_result_access {
                        html!{"Data updated"}
                    } else {
                        html!{}
                    }}
                </div>
                <div class="media-right">
                    {self.modal_delete_component()}
                    <button
                        id="delete-component"
                        class="button is-danger"
                        onclick={onclick_show_delete_modal} >
                        {"Delete"}
                    </button>
                    <button
                        id="update-data"
                        class="button"
                        onclick={onclick_save_changes}
                        disabled={self.disable_save_changes_btn} >
                        {"Update"}
                    </button>
                </div>
            </div>
        }
    }

    fn modal_delete_component(&self) -> Html {
        let onclick_hide_modal = self.link
            .callback(|_| Msg::ChangeHideDeleteComponent);
        let oninput_delete_component = self.link
            .callback(|ev: InputData| Msg::UpdateConfirmDelete(ev.value));
        let onclick_delete_component = self.link
            .callback(|_| Msg::RequestDeleteComponent);

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
                      <p class="modal-card-title">{"Delete component"}</p>
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <p class="is-size-6">
                            {"For confirm deleted all data this "}
                            <span class="has-text-danger-dark">{self.request_component.name.clone()}</span>
                            {" component enter this uuid:"}
                            <br/>
                            <span class="has-text-weight-bold is-size-6">{self.current_component_uuid.clone()}</span>
                        </p>
                        <br/>
                         <input
                           id="delete-component"
                           class="input"
                           type="text"
                           placeholder="component uuid"
                           value={self.confirm_delete_component.clone()}
                           oninput=oninput_delete_component />
                    </section>
                    <footer class="modal-card-foot">
                        <button
                            id="delete-component"
                            class="button is-danger"
                            disabled={self.disable_delete_component_btn}
                            onclick={onclick_delete_component} >{"Yes, delete"}</button>
                        <button class="button" onclick=onclick_hide_modal.clone()>{"Cancel"}</button>
                    </footer>
                </div>
              </div>
            </div>
        }
    }

    fn show_frame_upload_files(&self) -> Html {
        let onchange_upload_files = self.link.callback(move |value| {
            if let ChangeData::Files(files) = value {
                Msg::UpdateFiles(files)
            } else {
                Msg::Ignore
            }
        });

        html!{<>
            <div class="file has-name is-boxed">
                <label class="file-label">
                  <input id="component-file-input"
                  class="file-input"
                  type="file"
                  // accept="image/*,application/vnd*,application/rtf,text/*,.pdf"
                  onchange={onchange_upload_files}
                  multiple=true />
                <span class="file-cta">
                  <span class="file-icon">
                    <i class="fas fa-cloud-upload-alt"></i>
                  </span>
                  <span class="file-label">
                    {"Drop file here…"}
                  </span>
                </span>
                {if self.files.is_empty() {
                    html!{<span class="file-name">
                        {"No file uploaded"}
                    </span>}
                } else {
                    html!{for self.files.iter().map(|f| html!{
                        <span class="file-name">
                            {f.name().clone()}
                        </span>
                    })}
                }}
              </label>
            </div>
            {self.show_btn_clear()}
        </>}
    }

    fn show_btn_clear(&self) -> Html {
        let onclick_clear_boxed = self.link
            .callback(|_| Msg::ClearFilesBoxed);

        match self.disable_save_changes_btn {
            true => html!{},
            false => html!{
                <a id="clear-frame-upload-files"
                      class="button"
                      onclick=onclick_clear_boxed
                      disabled={self.files.is_empty()} >
                    { "Clear select files" }
                </a>
            },
        }
    }
}
