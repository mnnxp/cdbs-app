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
    standard::{StandardFilesCard, SearchSpecsTags, AddKeywordsTags},
};
use crate::gqls::make_query;
use crate::services::{
    PutUploadFile, UploadData,
    is_authenticated, get_logged_user
};
use crate::types::{
    UUID, StandardInfo, SlimUser, Region, TypeAccessInfo, UploadFile,
    ShowCompanyShort, StandardUpdatePreData, StandardUpdateData, StandardStatus,
};

type FileName = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct GetUpdateStandardDataOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct PutStandardUpdate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct DeleteStandard;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct ChangeStandardAccess;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct UploadStandardFiles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct ConfirmUploadCompleted;

/// Standard with relate data
pub struct StandardSettings {
    error: Option<Error>,
    current_standard: Option<StandardInfo>,
    current_standard_uuid: UUID,
    request_standard: StandardUpdatePreData,
    request_upload_data: Vec<UploadFile>,
    request_upload_file: Callback<Result<Option<String>, Error>>,
    request_access: i64,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    task_read: Vec<(FileName, ReaderTask)>,
    task: Vec<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
    supplier_list: Vec<ShowCompanyShort>,
    standard_statuses: Vec<StandardStatus>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessInfo>,
    update_standard: bool,
    update_standard_access: bool,
    upload_standard_files: bool,
    put_upload_file: PutUploadFile,
    files: Vec<File>,
    files_index: u32,
    disable_delete_standard_btn: bool,
    confirm_delete_standard: String,
    hide_delete_modal: bool,
    disable_save_changes_btn: bool,
    get_result_standard_data: usize,
    get_result_access: bool,
    get_result_up_file: bool,
    get_result_up_completed: usize,
    get_result_upload_files: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub standard_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    OpenStandard,
    RequestManager,
    RequestUpdateStandardData,
    RequestChangeAccess,
    RequestDeleteStandard,
    RequestUploadStandardFiles,
    RequestUploadFile(Vec<u8>),
    ResponseUploadFile(Result<Option<String>, Error>),
    RequestUploadCompleted,
    GetStandardData(String),
    GetListOpt(String),
    GetUpdateStandardResult(String),
    GetUpdateAccessResult(String),
    GetUploadData(String),
    GetUploadFile(Option<String>),
    GetUploadCompleted(String),
    GetDeleteStandard(String),
    EditFiles,
    UpdateTypeAccessId(String),
    UpdateClassifier(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdateSpecifiedTolerance(String),
    UpdateTechnicalCommittee(String),
    UpdatePublicationAt(String),
    UpdateCompanyUuid(String),
    UpdateStandardStatusId(String),
    UpdateRegionId(String),
    UpdateFiles(FileList),
    UpdateConfirmDelete(String),
    ResponseError(Error),
    ChangeHideDeleteStandard,
    ClearFilesBoxed,
    ClearError,
    Ignore,
}

impl Component for StandardSettings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        StandardSettings {
            error: None,
            current_standard: None,
            current_standard_uuid: String::new(),
            request_standard: StandardUpdatePreData::default(),
            request_upload_data: Vec::new(),
            request_upload_file: link.callback(Msg::ResponseUploadFile),
            request_access: 0,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            task_read: Vec::new(),
            task: Vec::new(),
            props,
            link,
            supplier_list: Vec::new(),
            standard_statuses: Vec::new(),
            regions: Vec::new(),
            types_access: Vec::new(),
            update_standard: false,
            update_standard_access: false,
            upload_standard_files: false,
            put_upload_file: PutUploadFile::new(),
            files: Vec::new(),
            files_index: 0,
            disable_delete_standard_btn: true,
            confirm_delete_standard: String::new(),
            hide_delete_modal: true,
            disable_save_changes_btn: true,
            get_result_standard_data: 0,
            get_result_access: false,
            get_result_up_file: false,
            get_result_up_completed: 0,
            get_result_upload_files: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // get standard uuid for request standard data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_standard_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/standard/settings/")
            .to_string();
        // get flag changing current standard in route
        let not_matches_standard_uuid = target_standard_uuid != self.current_standard_uuid;
        // debug!("self.current_standard_uuid {:#?}", self.current_standard_uuid);

        if not_matches_standard_uuid {
            // clear old data
            self.current_standard = None;
            self.current_standard_uuid = String::new();
            self.request_standard = StandardUpdatePreData::default();
        }

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_standard_uuid) && is_authenticated() {
            // update current_standard_uuid for checking change standard in route
            self.current_standard_uuid = target_standard_uuid.clone();
            let user_uuid = match &self.props.current_user {
                Some(user) => user.uuid.clone(),
                None => get_logged_user().unwrap().uuid.clone(),
            };

            spawn_local(async move {
                let ipt_companies_arg = get_update_standard_data_opt::IptCompaniesArg{
                    companiesUuids: None,
                    userUuid: Some(user_uuid),
                    favorite: None,
                    supplier: Some(true),
                    limit: None,
                    offset: None,
                };
                let res = make_query(GetUpdateStandardDataOpt::build_query(get_update_standard_data_opt::Variables {
                    standard_uuid: target_standard_uuid,
                    ipt_companies_arg,
                })).await.unwrap();

                link.send_message(Msg::GetStandardData(res.clone()));
                link.send_message(Msg::GetListOpt(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::OpenStandard => {
                // Redirect to standard page
                self.router_agent.send(ChangeRoute(
                    AppRoute::ShowStandard(self.current_standard_uuid.clone()).into()
                ));
            },
            Msg::RequestManager => {
                if self.update_standard {
                    self.link.send_message(Msg::RequestUpdateStandardData)
                }
                if self.update_standard_access {
                    self.link.send_message(Msg::RequestChangeAccess)
                }

                if self.upload_standard_files && !self.files.is_empty() {
                    self.link.send_message(Msg::RequestUploadStandardFiles);
                }

                self.update_standard = false;
                self.update_standard_access = false;
                self.upload_standard_files = false;
                self.disable_save_changes_btn = true;
                self.get_result_standard_data = 0;
                self.get_result_access = false;
            },
            Msg::RequestUpdateStandardData => {
                let standard_uuid = self.current_standard_uuid.clone();
                let request_standard: StandardUpdateData = (&self.request_standard).into();

                spawn_local(async move {
                    let StandardUpdateData {
                        classifier,
                        name,
                        description,
                        specified_tolerance,
                        technical_committee,
                        publication_at,
                        company_uuid,
                        standard_status_id,
                        region_id,
                    } = request_standard;
                    let ipt_update_standard_data = put_standard_update::IptUpdateStandardData {
                        classifier,
                        name,
                        description,
                        specifiedTolerance: specified_tolerance,
                        technicalCommittee: technical_committee,
                        publicationAt: publication_at,
                        companyUuid: company_uuid,
                        standardStatusId: standard_status_id,
                        regionId: region_id,
                    };
                    let res = make_query(PutStandardUpdate::build_query(put_standard_update::Variables {
                        standard_uuid,
                        ipt_update_standard_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateStandardResult(res));
                })
            },
            Msg::RequestChangeAccess => {
                let standard_uuid = self.current_standard_uuid.clone();
                let new_type_access_id = self.request_access.clone();
                spawn_local(async move {
                    let change_type_access_standard = change_standard_access::ChangeTypeAccessStandard{
                        standardUuid: standard_uuid,
                        newTypeAccessId: new_type_access_id,
                    };
                    let res = make_query(ChangeStandardAccess::build_query(change_standard_access::Variables {
                        change_type_access_standard
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateAccessResult(res));
                })
            },
            Msg::RequestDeleteStandard => {
                let standard_uuid = self.current_standard_uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteStandard::build_query(
                        delete_standard::Variables { standard_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteStandard(res));
                })
            },
            Msg::RequestUploadStandardFiles => {
                // see loading button
                self.get_result_upload_files = true;

                if !self.files.is_empty() {
                    let mut filenames: Vec<String> = Vec::new();
                    for file in &self.files {filenames.push(file.name().clone());}
                    debug!("filenames: {:?}", filenames);
                    let standard_uuid = self.current_standard_uuid.clone();

                    spawn_local(async move {
                        let ipt_standard_files_data = upload_standard_files::IptStandardFilesData{
                            filenames,
                            standardUuid: standard_uuid,
                        };
                        let res = make_query(UploadStandardFiles::build_query(upload_standard_files::Variables{
                            ipt_standard_files_data
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
                        let result: Vec<UploadFile> = serde_json::from_value(res_value.get("uploadStandardFiles").unwrap().clone()).unwrap();
                        debug!("uploadStandardFiles {:?}", result);
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
            Msg::GetStandardData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let standard_data: StandardInfo =
                            serde_json::from_value(res_value.get("standard").unwrap().clone()).unwrap();
                        debug!("Standard data: {:?}", standard_data);

                        self.current_standard_uuid = standard_data.uuid.clone();
                        self.current_standard = Some(standard_data.clone());
                        self.request_standard = standard_data.into();
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
                        self.standard_statuses = serde_json::from_value(
                            res_value.get("standardStatuses").unwrap().clone()
                        ).unwrap();
                        self.regions = serde_json::from_value(
                            res_value.get("regions").unwrap().clone()
                        ).unwrap();
                        self.types_access = serde_json::from_value(
                            res_value.get("typesAccess").unwrap().clone()
                        ).unwrap();
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetUpdateStandardResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize =
                            serde_json::from_value(res_value.get("putStandardUpdate").unwrap().clone()).unwrap();
                        debug!("Standard data: {:?}", result);
                        self.get_result_standard_data = result;
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
                            serde_json::from_value(res_value.get("changeStandardAccess").unwrap().clone()).unwrap();
                        debug!("Standard change access: {:?}", result);
                        self.update_standard_access = false;
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
            Msg::GetDeleteStandard(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UUID = serde_json::from_value(res_value.get("deleteStandard").unwrap().clone()).unwrap();
                        debug!("deleteStandard: {:?}", result);
                        if self.current_standard_uuid == result {
                            match &self.current_standard {
                                Some(company) => self.router_agent.send(ChangeRoute(
                                    AppRoute::ShowCompany(company.owner_company.uuid.clone()).into()
                                )),
                                None => self.router_agent.send(ChangeRoute(AppRoute::Home.into())),
                            }
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::EditFiles => self.upload_standard_files = !self.upload_standard_files,
            Msg::UpdateTypeAccessId(data) => {
                self.request_access = data.parse::<i64>().unwrap_or_default();
                self.update_standard_access = true;
                self.disable_save_changes_btn = false;
            },
            // items request update main standard data
            Msg::UpdateClassifier(data) => {
                self.request_standard.classifier = data;
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateName(data) => {
                self.request_standard.name = data;
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateDescription(data) => {
                self.request_standard.description = data;
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateSpecifiedTolerance(data) => {
                self.request_standard.specified_tolerance = data;
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateTechnicalCommittee(data) => {
                self.request_standard.technical_committee = data;
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdatePublicationAt(data) => {
                let date = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", data), "%Y-%m-%d %H:%M:%S");
                debug!("new date: {:?}", date);
                self.request_standard.publication_at = match date {
                    Ok(dt) => Some(dt),
                    Err(_) => match &self.current_standard {
                        Some(cs) => Some(cs.publication_at),
                        None => self.request_standard.publication_at,
                    },
                };
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateCompanyUuid(data) => {
                self.request_standard.company_uuid = data;
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateStandardStatusId(data) => {
                self.request_standard.standard_status_id = data.parse::<usize>().unwrap_or_default();
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateRegionId(data) => {
                self.request_standard.region_id = data.parse::<usize>().unwrap_or_default();
            },
            Msg::UpdateFiles(files) => {
                while let Some(file) = files.get(self.files_index) {
                    debug!("self.files_index: {:?}", self.files_index);
                    self.files_index += 1;
                    self.upload_standard_files = true;
                    self.disable_save_changes_btn = false;
                    self.files.push(file.clone());
                }
                self.files_index = 0;
            },
            Msg::UpdateConfirmDelete(data) => {
                self.disable_delete_standard_btn = self.current_standard_uuid != data;
                self.confirm_delete_standard = data;
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ChangeHideDeleteStandard => self.hide_delete_modal = !self.hide_delete_modal,
            Msg::ClearFilesBoxed => {
                self.files = Vec::new();
                self.files_index = 0;
                self.upload_standard_files = false;
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
        let onclick_clear_error = self.link
            .callback(|_| Msg::ClearError);

        html!{
            <div class="standard-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
                        // <br/>
                        {self.show_manage_btn()}
                        <br/>
                        {self.show_main_card()}
                        {match &self.current_standard {
                            Some(standard_data) => html!{<>
                                <div class="columns">
                                  {self.show_standard_params()}
                                  {self.show_standard_files(standard_data)}
                                </div>
                                {self.show_standard_specs(standard_data)}
                                <br/>
                                {self.show_standard_keywords(standard_data)}
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

impl StandardSettings {
    fn show_main_card(&self) -> Html {
        // let default_company_uuid = self.current_standard.as_ref().map(|x| x.owner_company.uuid.clone()).unwrap_or_default();
        let onchange_change_owner_company = self.link
            .callback(|ev: ChangeData| Msg::UpdateCompanyUuid(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
          }));

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
                <div class="control">
                    <div class="media">
                        <div class="media-content">
                            <label class="label">{"Owner company "}</label>
                            <div class="select">
                              <select
                                  id="set-owner-company"
                                  select={self.request_standard.company_uuid.clone()}
                                  onchange=onchange_change_owner_company
                                >
                              { for self.supplier_list.iter().map(|x|
                                  html!{<option value={x.uuid.to_string()}>{&x.shortname}</option>}
                              )}
                              </select>
                            </div>
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
                                  html!{<option value={x.type_access_id.to_string()}>{&x.name}</option>}
                              )}
                              </select>
                            </div>
                        </div>
                    </div>
                </div>
                <label class="label">{"Name"}</label>
                <input
                    id="update-name"
                    class="input"
                    type="text"
                    placeholder="standard name"
                    value={self.request_standard.name.clone()}
                    oninput=oninput_name />
                <label class="label">{"Description"}</label>
                <textarea
                    id="update-description"
                    class="textarea"
                    // rows="10"
                    type="text"
                    placeholder="standard description"
                    value={self.request_standard.description.clone()}
                    oninput=oninput_description />
            </div>
        </div>}
    }

    fn show_standard_params(&self) -> Html {
        let oninput_classifier = self.link
            .callback(|ev: InputData| Msg::UpdateClassifier(ev.value));

        let oninput_specified_tolerance = self.link
            .callback(|ev: InputData| Msg::UpdateSpecifiedTolerance(ev.value));

        let oninput_technical_committee = self.link
            .callback(|ev: InputData| Msg::UpdateTechnicalCommittee(ev.value));

        let oninput_publication_at = self.link
            .callback(|ev: InputData| Msg::UpdatePublicationAt(ev.value));

        let onchange_standard_status_id = self.link
            .callback(|ev: ChangeData| Msg::UpdateStandardStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        let onchange_region_id = self.link
            .callback(|ev: ChangeData| Msg::UpdateRegionId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
          }));

        html!{
            <div class="column">
              <h2>{"Manage standard characteristics"}</h2>
              <div class="card">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{"classifier"}</td>
                        <td><input
                            id="update-classifier"
                            class="input"
                            type="text"
                            placeholder="standard classifier"
                            value={self.request_standard.classifier.clone()}
                            oninput=oninput_classifier /></td>
                      </tr>
                      <tr>
                        <td>{"specified_tolerance"}</td>
                        // <td>{self.request_standard.specified_tolerance.as_ref().map(|x| x.clone()).unwrap_or_default()}</td>
                        <td><input
                            id="update-specified-tolerance"
                            class="input"
                            type="text"
                            placeholder="standard specified_tolerance"
                            value={self.request_standard.specified_tolerance.clone()}
                            oninput=oninput_specified_tolerance /></td>
                      </tr>
                      <tr>
                        <td>{"technical_committee"}</td>
                        <td><input
                            id="update-technical-committee"
                            class="input"
                            type="text"
                            placeholder="standard technical_committee"
                            value={self.request_standard.technical_committee.clone()}
                            oninput=oninput_technical_committee /></td>
                      </tr>
                      <tr>
                        <td>{"publication_at"}</td>
                        <td><input
                            id="update-publication-at"
                            class="input"
                            type="date"
                            placeholder="standard publication_at"
                            value={self.request_standard.publication_at
                                .as_ref()
                                .map(|x| format!("{:.*}", 10, x.to_string()))
                                .unwrap_or_default()}
                            oninput=oninput_publication_at
                            /></td>
                      </tr>
                      <tr>
                        <td>{"standard_status"}</td>
                        <td><div class="control">
                            <div class="select">
                              <select
                                  id="standard-status-id"
                                  select={self.request_standard.standard_status_id.to_string()}
                                  onchange=onchange_standard_status_id
                                  >
                                { for self.standard_statuses.iter().map(|x|
                                    html!{<option value={x.standard_status_id.to_string()}>{&x.name}</option>}
                                )}
                              </select>
                            </div>
                        </div></td>
                      </tr>
                      <tr>
                        <td>{"region"}</td>
                        <td><div class="select">
                              <select
                                  id="region"
                                  select={self.request_standard.region_id.to_string()}
                                  onchange=onchange_region_id
                                  >
                                { for self.regions.iter().map(|x|
                                    html!{<option value={x.region_id.to_string()}>{&x.region}</option>}
                                )}
                              </select>
                            </div>
                        </td>
                      </tr>
                    </tbody>
                  </table>
              </div>
            </div>
        }
    }

    fn show_standard_files(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2>{"Files"}</h2>
              {self.show_frame_upload_files()}
              <StandardFilesCard
                  show_download_btn = false
                  show_delete_btn = true
                  standard_uuid = standard_data.uuid.clone()
                  files = standard_data.standard_files.clone()
                />
            </div>
        }
    }

    fn show_standard_specs(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{<>
            <h2>{"Specs"}</h2>
            <div class="card">
              <SearchSpecsTags
                  standard_specs = standard_data.standard_specs.clone()
                  standard_uuid = standard_data.uuid.clone()
                />
            </div>
        </>}
    }

    fn show_standard_keywords(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        // debug!("Keywords: {:?}", &standard_data.uuid);
        html!{<>
              <h2>{"Keywords"}</h2>
              <div class="card">
                <AddKeywordsTags
                    standard_keywords = standard_data.standard_keywords.clone()
                    standard_uuid = standard_data.uuid.clone()
                  />
              </div>
        </>}
    }

    fn show_manage_btn(&self) -> Html {
        let onclick_open_standard = self.link
            .callback(|_| Msg::OpenStandard);
        let onclick_show_delete_modal = self.link
            .callback(|_| Msg::ChangeHideDeleteStandard);
        let onclick_save_changes = self.link
            .callback(|_| Msg::RequestManager);

        html!{
            <div class="media">
                <div class="media-left">
                    <button
                        id="open-standard"
                        class="button"
                        onclick={onclick_open_standard} >
                        {"Cancel"}
                    </button>
                </div>
                <div class="media-content">
                    {if self.get_result_standard_data > 0 || self.get_result_access {
                        html!{"Data updated"}
                    } else {
                        html!{}
                    }}
                </div>
                <div class="media-right">
                    {self.modal_delete_standard()}
                    <button
                        id="delete-standard"
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

    fn modal_delete_standard(&self) -> Html {
        let onclick_hide_modal = self.link
            .callback(|_| Msg::ChangeHideDeleteStandard);
        let oninput_delete_standard = self.link
            .callback(|ev: InputData| Msg::UpdateConfirmDelete(ev.value));
        let onclick_delete_standard = self.link
            .callback(|_| Msg::RequestDeleteStandard);

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
                      <p class="modal-card-title">{"Delete standard"}</p>
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <p class="is-size-6">
                            {"For confirm deleted all data this "}
                            <span class="has-text-danger-dark">{self.request_standard.name.clone()}</span>
                            {" standard enter this uuid:"}
                            <br/>
                            <span class="has-text-weight-bold is-size-6">{self.current_standard_uuid.clone()}</span>
                        </p>
                        <br/>
                         <input
                           id="delete-standard"
                           class="input"
                           type="text"
                           placeholder="standard uuid"
                           value={self.confirm_delete_standard.clone()}
                           oninput=oninput_delete_standard />
                    </section>
                    <footer class="modal-card-foot">
                        <button
                            id="delete-standard"
                            class="button is-danger"
                            disabled={self.disable_delete_standard_btn}
                            onclick={onclick_delete_standard} >{"Yes, delete"}</button>
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
                  <input id="standard-file-input"
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
