// use yew_router::hooks::use_route;
use yew::{Component, Context, html, Html, Properties};
use yew::html::{Scope, TargetCast};
use yew_router::prelude::*;
use gloo::file::File;
use web_sys::{InputEvent, DragEvent, Event, FileList, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use serde_json::Value;
use chrono::NaiveDateTime;
use crate::routes::AppRoute::{Login, Home, ShowCompany, ShowStandard};
use crate::error::{get_error, Error};
use crate::fragments::files_frame::FilesFrame;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::standard::{
    StandardFilesCard, SearchSpecsTags,
    AddKeywordsTags, UpdateStandardFaviconCard,
};
use crate::services::storage_upload::{storage_upload, prepare_files};
use crate::services::{get_logged_user, get_value_field};
use crate::types::{
    UUID, StandardInfo, SlimUser, Region, TypeAccessInfo, UploadFile, ShowFileInfo,
    ShowCompanyShort, StandardUpdatePreData, StandardUpdateData, StandardStatus,
};
use crate::gqls::{
    make_query,
    // relate::{ConfirmUploadCompleted, confirm_upload_completed},
    standard::{
        GetUpdateStandardDataOpt, get_update_standard_data_opt,
        PutStandardUpdate, put_standard_update,
        DeleteStandard, delete_standard,
        ChangeStandardAccess, change_standard_access,
        UploadStandardFiles, upload_standard_files,
        StandardFilesList, standard_files_list,
    },
};

// type FileName = String;

/// Standard with relate data
pub struct StandardSettings {
    error: Option<Error>,
    current_standard: Option<StandardInfo>,
    current_standard_uuid: UUID,
    request_standard: StandardUpdatePreData,
    // request_upload_data: Vec<UploadFile>,
    // request_upload_file: Callback<Result<Option<String>, Error>>,
    // request_upload_confirm: Vec<UUID>,
    request_access: i64,
    // router_agent: Box<dyn Bridge<AppRoute>>,
    // task_read: Vec<(FileName, ReaderTask)>,
    // task: Vec<FetchTask>,
    supplier_list: Vec<ShowCompanyShort>,
    standard_statuses: Vec<StandardStatus>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessInfo>,
    update_standard: bool,
    update_standard_access: bool,
    upload_standard_files: bool,
    // put_upload_file: PutUploadFile,
    files: Vec<File>,
    // files_index: u32,
    files_list: Vec<ShowFileInfo>,
    disable_delete_standard_btn: bool,
    confirm_delete_standard: String,
    hide_delete_modal: bool,
    disable_save_changes_btn: bool,
    get_result_standard_data: usize,
    get_result_access: bool,
    // get_result_up_file: bool,
    get_result_up_completed: usize,
    active_loading_files_btn: bool,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub standard_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    OpenStandard,
    RequestManager,
    RequestStandardFilesList,
    RequestUpdateStandardData,
    RequestChangeAccess,
    RequestDeleteStandard,
    RequestUploadStandardFiles,
    // RequestUploadFile(Vec<u8>),
    // ResponseUploadFile(Result<Option<String>, Error>),
    // RequestUploadCompleted,
    GetStandardFilesList(String),
    GetStandardData(String),
    GetListOpt(String),
    GetUpdateStandardResult(String),
    GetUpdateAccessResult(String),
    GetUploadData(String),
    // GetUploadFile,
    GetUploadCompleted(Result<usize, Error>),
    // FinishUploadFiles,
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
    UpdateFiles(Option<FileList>),
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

    fn create(ctx: &Context<Self>) -> Self {
        StandardSettings {
            error: None,
            current_standard: None,
            current_standard_uuid: String::new(),
            request_standard: StandardUpdatePreData::default(),
            // request_upload_data: Vec::new(),
            // request_upload_file: ctx.link().callback(Msg::ResponseUploadFile),
            // request_upload_confirm: Vec::new(),
            request_access: 0,
            // router_agent: AppRoute::bridge(ctx.link().callback(|_| Msg::Ignore)),
            // task_read: Vec::new(),
            // task: Vec::new(),
            supplier_list: Vec::new(),
            standard_statuses: Vec::new(),
            regions: Vec::new(),
            types_access: Vec::new(),
            update_standard: false,
            update_standard_access: false,
            upload_standard_files: false,
            // put_upload_file: PutUploadFile::new(),
            files: Vec::new(),
            // files_index: 0,
            files_list: Vec::new(),
            disable_delete_standard_btn: true,
            confirm_delete_standard: String::new(),
            hide_delete_modal: true,
            disable_save_changes_btn: true,
            get_result_standard_data: 0,
            get_result_access: false,
            // get_result_up_file: false,
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

        let target_standard_uuid =
            ctx.link().location().unwrap().path().trim_start_matches("/standard/settings/").to_string();
            // ctx.link().location().unwrap().path().trim_start_matches("/standard/settings/").to_string();
        // get flag changing current standard in route
        let not_matches_standard_uuid = target_standard_uuid != self.current_standard_uuid;
        // debug!("self.current_standard_uuid {:#?}", self.current_standard_uuid);
        if not_matches_standard_uuid {
            // clear old data
            self.current_standard = None;
            self.current_standard_uuid = String::new();
            self.request_standard = StandardUpdatePreData::default();
        }
        if first_render || not_matches_standard_uuid {
            let link = ctx.link().clone();
            // update current_standard_uuid for checking change standard in route
            self.current_standard_uuid = target_standard_uuid.clone();
            spawn_local(async move {
                let ipt_companies_arg = get_update_standard_data_opt::IptCompaniesArg{
                    companies_uuids: None,
                    user_uuid: Some(logged_user_uuid),
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let navigator: Navigator = ctx.link().navigator().unwrap();

        match msg {
            Msg::OpenStandard => {
                // Redirect to standard page
                // self.router_agent.send(ShowStandard { uuid: self.current_standard_uuid.clone() });
                navigator.clone().replace(&ShowStandard { uuid: self.current_standard_uuid.clone() });
            },
            Msg::RequestManager => {
                if self.update_standard {
                    ctx.link().send_message(Msg::RequestUpdateStandardData)
                }
                if self.update_standard_access {
                    ctx.link().send_message(Msg::RequestChangeAccess)
                }
                // if self.upload_standard_files && !self.files.is_empty() {
                //     ctx.link().send_message(Msg::RequestUploadStandardFiles);
                // }
                self.update_standard = false;
                self.update_standard_access = false;
                // self.upload_standard_files = false;
                self.disable_save_changes_btn = true;
                self.get_result_standard_data = 0;
                self.get_result_access = false;
            },
            Msg::RequestStandardFilesList => {
                let standard_uuid = ctx.props().standard_uuid.clone();
                spawn_local(async move {
                    let res = make_query(StandardFilesList::build_query(
                        standard_files_list::Variables { standard_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetStandardFilesList(res));
                })
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
                        specified_tolerance,
                        technical_committee,
                        publication_at,
                        company_uuid,
                        standard_status_id,
                        region_id,
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
                        standard_uuid,
                        new_type_access_id,
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
                self.active_loading_files_btn = true;

                if !self.files.is_empty() {
                    let mut filenames: Vec<String> = Vec::new();
                    for file in &self.files {
                        filenames.push(file.name().clone());
                    }
                    debug!("filenames: {:?}", filenames);
                    let standard_uuid = self.current_standard_uuid.clone();

                    spawn_local(async move {
                        let ipt_standard_files_data = upload_standard_files::IptStandardFilesData{
                            filenames,
                            standard_uuid,
                        };
                        let res = make_query(UploadStandardFiles::build_query(upload_standard_files::Variables{
                            ipt_standard_files_data
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
            Msg::GetStandardFilesList(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        self.files_list = serde_json::from_value(
                            res_value.get("standard").unwrap()
                                .get("standardFiles").unwrap().clone()
                        ).unwrap();
                        debug!("standardFilesList {:?}", self.files_list.len());
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetUploadData(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<UploadFile> = serde_json::from_value(res_value.get("uploadStandardFiles").unwrap().clone()).unwrap();
                        debug!("uploadStandardFiles {:?}", result);
                        // self.request_upload_data = result;
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
            //     link.send_message(Msg::GetUploadFile)
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
            Msg::GetStandardData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let standard_data: StandardInfo =
                            serde_json::from_value(res_value.get("standard").unwrap().clone()).unwrap();
                        debug!("Standard data: {:?}", standard_data);

                        self.current_standard_uuid = standard_data.uuid.clone();
                        self.files_list = standard_data.standard_files.clone();
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
                        self.supplier_list =
                            serde_json::from_value(res_value.get("companies").unwrap().clone()).unwrap();
                        self.standard_statuses =
                            serde_json::from_value(res_value.get("standardStatuses").unwrap().clone()).unwrap();
                        self.regions =
                            serde_json::from_value(res_value.get("regions").unwrap().clone()).unwrap();
                        self.types_access =
                            serde_json::from_value(res_value.get("typesAccess").unwrap().clone()).unwrap();
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
            // Msg::GetUploadFile => {
            //     debug!("next: {:?}", self.files_index);
            //     self.files_index -= 1;
            //     if self.files_index == 0 {
            //         self.get_result_up_file = true;
            //         debug!("finish: {:?}", self.request_upload_confirm.len());
            //         // link.send_message(Msg::RequestUploadCompleted);
            //     }
            // },
            Msg::GetUploadCompleted(res) => {
                match res {
                    Ok(value) => self.get_result_up_completed = value,
                    Err(err) => self.error = Some(err),
                }
                self.active_loading_files_btn = false;
            },
            // Msg::FinishUploadFiles => {
            //     self.files_list.clear();
            //     link.send_message(Msg::RequestStandardFilesList);
            //     self.active_loading_files_btn = false;
            //     self.task.clear();
            //     self.task_read.clear();
            //     self.request_upload_confirm.clear();
            //     self.files.clear();
            //     self.files_index = 0;
            // },
            Msg::GetDeleteStandard(res) => {
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: UUID = serde_json::from_value(res_value.get("deleteStandard").unwrap().clone()).unwrap();
                        debug!("deleteStandard: {:?}", result);
                        if self.current_standard_uuid == result {
                            match &self.current_standard {
                                Some(company) =>
                                    navigator.clone().replace(&ShowCompany { uuid: company.owner_company.uuid.clone() }),
                                    // self.router_agent.send(ShowCompany { uuid: company.owner_company.uuid.clone() }),
                                None => navigator.clone().replace(&Home),
                                // None => self.router_agent.send(Home),
                            }
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::EditFiles => self.upload_standard_files = !self.upload_standard_files,
            Msg::UpdateTypeAccessId(data) => {
                self.request_access = data.parse::<i64>().unwrap_or(1);
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
                self.request_standard.standard_status_id = data.parse::<usize>().unwrap_or(1);
                self.update_standard = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateRegionId(data) => {
                self.request_standard.region_id = data.parse::<usize>().unwrap_or(1);
            },
            Msg::UpdateFiles(file_list) => {
                prepare_files(&file_list, &mut self.files);
                // while let Some(file) = files.get(self.files_index) {
                //     debug!("self.files_index: {:?}", self.files_index);
                //     self.files_index += 1;
                //     self.upload_standard_files = true;
                //     self.files.push(file.clone());
                // }
                // self.files_index = 0;
            },
            Msg::UpdateConfirmDelete(data) => {
                self.disable_delete_standard_btn = self.current_standard_uuid != data;
                self.confirm_delete_standard = data;
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ChangeHideDeleteStandard => self.hide_delete_modal = !self.hide_delete_modal,
            Msg::ClearFilesBoxed => {
                self.files = Vec::new();
                // self.files_index = 0;
                self.upload_standard_files = false;
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.current_standard_uuid == ctx.props().standard_uuid {
            false
        } else {
            self.current_standard_uuid = ctx.props().standard_uuid;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{
            <div class="standard-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
                        // <br/>
                        {self.show_manage_btn(ctx.link())}
                        <br/>
                        {self.show_main_card(ctx.link())}
                        {match &self.current_standard {
                            Some(standard_data) => html!{<>
                                <br/>
                                <div class="columns">
                                  <div class="column">
                                    {self.update_standard_favicon(ctx.link())}
                                    <br/>
                                    {self.show_standard_params(ctx.link())}
                                  </div>
                                  {self.show_standard_files(ctx.link(), standard_data)}
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
    fn show_main_card(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        // let default_company_uuid = self.current_standard.as_ref().map(|x| x.owner_company.uuid.clone()).unwrap_or_default();
        let onchange_change_owner_company = link.callback(|ev: Event| {
            Msg::UpdateCompanyUuid(ev.current_target().map(|ev| ev.as_string().unwrap_or_default()).unwrap_or_default())
          });
        let onchange_change_type_access = link.callback(|ev: Event| {
            Msg::UpdateTypeAccessId(ev.current_target().map(|ev| ev.as_string().unwrap_or_default()).unwrap_or_default())
          });
        let oninput_name = link.callback(|ev: InputEvent| Msg::UpdateName(ev.input_type()));
        let oninput_description = link.callback(|ev: InputEvent| Msg::UpdateDescription(ev.input_type()));

        html!{<div class="card">
            <div class="column">
                <div class="control">
                    <div class="media">
                        <div class="media-content">
                            <label class="label">{ get_value_field(&223) }</label> // Owner company
                            <div class="select">
                              <select
                                  id="set-owner-company"
                                  select={self.request_standard.company_uuid.clone()}
                                  onchange={onchange_change_owner_company}
                                >
                              { for self.supplier_list.iter().map(|x|
                                  html!{
                                      <option value={x.uuid.to_string()}
                                            selected={x.uuid == self.request_standard.company_uuid} >
                                          {&x.shortname}
                                      </option>
                                  }
                              )}
                              </select>
                            </div>
                        </div>
                        <div class="media-right" style="margin-right: 1rem">
                            <label class="label">{ get_value_field(&114) }</label>
                            <div class="select">
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
                </div>
                <label class="label">{ get_value_field(&110) }</label>
                <input
                    id="update-name"
                    class="input"
                    type="text"
                    placeholder={get_value_field(&110)}
                    value={self.request_standard.name.clone()}
                    oninput={oninput_name} />
                <label class="label">{ get_value_field(&61) }</label>
                <textarea
                    id="update-description"
                    class="textarea"
                    // rows="10"
                    type="text"
                    placeholder={get_value_field(&61)}
                    value={self.request_standard.description.clone()}
                    oninput={oninput_description} />
            </div>
        </div>}
    }

    fn update_standard_favicon(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let callback_update_favicon = link.callback(|_| Msg::Ignore);

        html!{<>
            <h2 class="has-text-weight-bold">{ get_value_field(&184) }</h2> // Update image for preview
            <div class="card column">
                <UpdateStandardFaviconCard
                    standard_uuid={self.current_standard_uuid.clone()}
                    callback={callback_update_favicon.clone()}
                />
            </div>
        </>}
    }

    fn show_standard_params(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let oninput_classifier = link.callback(|ev: InputEvent| Msg::UpdateClassifier(ev.input_type()));
        let oninput_specified_tolerance =
            link.callback(|ev: InputEvent| Msg::UpdateSpecifiedTolerance(ev.input_type()));
        let oninput_technical_committee =
            link.callback(|ev: InputEvent| Msg::UpdateTechnicalCommittee(ev.input_type()));
        let oninput_publication_at =
            link.callback(|ev: InputEvent| Msg::UpdatePublicationAt(ev.input_type()));
        let onchange_standard_status_id = link.callback(|ev: Event| {
            Msg::UpdateStandardStatusId(ev.current_target().map(|ev| ev.as_string().unwrap_or_default()).unwrap_or_default())
          });
        let onchange_region_id = link.callback(|ev: Event| {
            Msg::UpdateRegionId(ev.current_target().map(|ev| ev.as_string().unwrap_or_default()).unwrap_or_default())
          });

        html!{
            <>
              <h2 class="has-text-weight-bold">{ get_value_field(&224) }</h2> // Manage standard characteristics
              <div class="card column">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{ get_value_field(&146) }</td> // classifier
                        <td><input
                            id="update-classifier"
                            class="input"
                            type="text"
                            placeholder={get_value_field(&146)}
                            value={self.request_standard.classifier.clone()}
                            oninput={oninput_classifier} /></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&147) }</td>
                        // <td>{self.request_standard.specified_tolerance.as_ref().map(|x| x.clone()).unwrap_or_default()}</td>
                        <td><input
                            id="update-specified-tolerance"
                            class="input"
                            type="text"
                            placeholder={get_value_field(&147)}
                            value={self.request_standard.specified_tolerance.clone()}
                            oninput={oninput_specified_tolerance} /></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&148) }</td>
                        <td><input
                            id="update-technical-committee"
                            class="input"
                            type="text"
                            placeholder={get_value_field(&148)}
                            value={self.request_standard.technical_committee.clone()}
                            oninput={oninput_technical_committee} /></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&149) }</td>
                        <td><input
                            id="update-publication-at"
                            class="input"
                            type="date"
                            placeholder={get_value_field(&149)}
                            value={self.request_standard.publication_at
                                .as_ref()
                                .map(|x| format!("{:.*}", 10, x.to_string()))
                                .unwrap_or_default()}
                            oninput={oninput_publication_at}
                            /></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&150) }</td>
                        <td><div class="control">
                            <div class="select">
                              <select
                                  id="standard-status-id"
                                  select={self.request_standard.standard_status_id.to_string()}
                                  onchange={onchange_standard_status_id}
                                  >
                                { for self.standard_statuses.iter().map(|x|
                                    html!{
                                        <option value={x.standard_status_id.to_string()}
                                              selected={x.standard_status_id == self.request_standard.standard_status_id} >
                                            {&x.name}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div></td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&151) }</td>
                        <td><div class="select">
                              <select
                                  id="region"
                                  select={self.request_standard.region_id.to_string()}
                                  onchange={onchange_region_id}
                                  >
                                { for self.regions.iter().map(|x|
                                    html!{
                                        <option value={x.region_id.to_string()}
                                              selected={x.region_id == self.request_standard.region_id} >
                                            {&x.region}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </td>
                      </tr>
                    </tbody>
                  </table>
              </div>
            </>
        }
    }

    fn show_standard_files(
        &self,
        link: &Scope<Self>,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2 class="has-text-weight-bold">{ get_value_field(&225) }</h2> // Files stadndard
              <div class="card column">
                  {self.show_frame_upload_files(link)}
                  <StandardFilesCard
                      show_download_btn = {false}
                      show_delete_btn = {true}
                      standard_uuid = {standard_data.uuid.clone()}
                      files = {self.files_list.clone()}
                    />
                </div>
            </div>
        }
    }

    fn show_standard_specs(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{<>
            <h2 class="has-text-weight-bold">{ get_value_field(&104) }</h2>
            <div class="card">
              <SearchSpecsTags
                  standard_specs = {standard_data.standard_specs.clone()}
                  standard_uuid = {standard_data.uuid.clone()}
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
              <h2 class="has-text-weight-bold">{ get_value_field(&105) }</h2>
              <div class="card">
                <AddKeywordsTags
                    standard_keywords = {standard_data.standard_keywords.clone()}
                    standard_uuid = {standard_data.uuid.clone()}
                  />
              </div>
        </>}
    }

    fn show_manage_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_open_standard = link.callback(|_| Msg::OpenStandard);
        let onclick_show_delete_modal = link.callback(|_| Msg::ChangeHideDeleteStandard);
        let onclick_save_changes = link.callback(|_| Msg::RequestManager);

        html!{
            <div class="media">
                <div class="media-left">
                    <button
                        id="open-standard"
                        class="button"
                        onclick={onclick_open_standard} >
                        { get_value_field(&226) } // Open standard
                    </button>
                </div>
                <div class="media-content">
                    {if self.get_result_standard_data > 0 || self.get_result_access {
                        html!{get_value_field(&214) } // Data updated
                    } else {
                        html!{}
                    }}
                </div>
                <div class="media-right">
                    {self.modal_delete_standard(link)}
                    <div class="buttons">
                        <button
                            id="delete-standard"
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

    fn modal_delete_standard(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_hide_modal = link.callback(|_| Msg::ChangeHideDeleteStandard);
        let oninput_delete_standard =
            link.callback(|ev: InputEvent| Msg::UpdateConfirmDelete(ev.input_type()));
        let onclick_delete_standard = link.callback(|_| Msg::RequestDeleteStandard);

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
                      <p class="modal-card-title">{ get_value_field(&227) }</p> // Delete standard
                      <button class="delete" aria-label="close" onclick={onclick_hide_modal.clone()} />
                    </header>
                    <section class="modal-card-body">
                        <p class="is-size-6">
                            { get_value_field(&218) }
                            <span class="has-text-danger-dark">{self.request_standard.name.clone()}</span>
                            { get_value_field(&228) }
                            <br/>
                            <span class="has-text-weight-bold is-size-6">{self.current_standard_uuid.clone()}</span>
                        </p>
                        <br/>
                         <input
                           id="delete-standard"
                           class="input"
                           type="text"
                           placeholder="uuid"
                           value={self.confirm_delete_standard.clone()}
                           oninput={oninput_delete_standard} />
                    </section>
                    <footer class="modal-card-foot">
                        <button
                            id="delete-standard"
                            class="button is-danger"
                            disabled={self.disable_delete_standard_btn}
                            onclick={onclick_delete_standard} >{ get_value_field(&220) }</button> // Yes, delete
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
        let onchange = link.callback(move |ev: Event| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateFiles(input.files())
        });
        let ondrop = link.callback(move |ev: DragEvent| {
            ev.prevent_default();
            Msg::UpdateFiles(ev.data_transfer().unwrap().files())
        });
        let ondragover = link.callback(move |ev: DragEvent| {
            ev.prevent_default();
            Msg::Ignore
        });
        let ondragenter = ondragover.clone();

        html!{<>
            <div class="file has-name is-boxed is-centered">
                <FilesFrame
                    {onchange}
                    {ondrop}
                    {ondragover}
                    {ondragenter}
                    input_id={"standard-file-input".to_string()}
                    multiple={true}
                    file_label={222}
                />
                // <label class="file-label" style="width: 100%">
                //   <input id="standard-file-input"
                //   class="file-input"
                //   type="file"
                //   // accept="image/*,application/vnd*,application/rtf,text/*,.pdf"
                //   onchange={onchange_upload_files}
                //   multiple=true />
                // <span class="file-cta">
                //   <span class="file-icon">
                //     <i class="fas fa-upload"></i>
                //   </span>
                //   <span class="file-label">
                //     { get_value_field(&222) } // Choose standard files…
                //   </span>
                // </span>
                {match self.files.is_empty() {
                    true => html!{<span class="file-name">{ get_value_field(&194) }</span>}, // No file uploaded
                    false => html!{for self.files.iter().map(|f| html!{
                        <span class="file-name">{f.name().clone()}</span>
                    })}
                }}
              // </label>
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
            <button id="clear-upload-standard-files"
              class="button"
              onclick={onclick_clear_boxed}
              disabled={self.files.is_empty()} >
                <span>{ get_value_field(&88) }</span>
            </button>
        }
    }

    fn show_upload_files_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_upload_files = link.callback(|_| Msg::RequestUploadStandardFiles);

        let class_upload_btn = match self.active_loading_files_btn {
            true => "button is-loading",
            false => "button",
        };

        html!{
            <button
              id="upload-component-files"
              class={class_upload_btn}
              disabled={self.files.is_empty() || self.current_standard_uuid.len() != 36}
              onclick={onclick_upload_files} >
                // <span class="icon" >
                //     <i class="fas fa-angle-double-up" aria-hidden="true"></i>
                // </span>
                <span>{ get_value_field(&87) }</span>
            </button>
        }
    }
}
