use yew::{
    agent::Bridged, html, Bridge, Component, Properties,
    ComponentLink, Html, ShouldRender, InputData
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
    buttons::{ft_save_btn, ft_back_btn},
    file::UploaderFiles,
    list_errors::ListErrors,
    notification::show_notification,
    supplier_service::{ServiceFilesCard, SearchSpecsTags, AddKeywordsTags, ServiceParamsTags},
};
use crate::services::{get_from_value, get_logged_user, get_value_field, get_value_response, resp_parsing, resp_parsing_two_level, set_history_back};
use crate::types::{
    UUID, ServiceInfo, SlimUser, Region, UploadFile, ShowFileInfo,
    ShowCompanyShort, ServiceUpdatePreData, ServiceUpdateData,
};
use crate::gqls::make_query;
use crate::gqls::supplier_service::{
    GetUpdateServiceDataOpt, get_update_service_data_opt,
    PutServiceUpdate, put_service_update,
    UploadServiceFiles, upload_service_files,
    ServiceFilesList, service_files_list,
};

type FileName = String;

/// Service with relate data
pub struct ServiceSettings {
    error: Option<Error>,
    current_service: Option<ServiceInfo>,
    current_service_uuid: UUID,
    request_service: ServiceUpdatePreData,
    request_upload_data: Vec<UploadFile>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    supplier_list: Vec<ShowCompanyShort>,
    regions: Vec<Region>,
    update_service: bool,
    files_list: Vec<ShowFileInfo>,
    disable_save_changes_btn: bool,
    get_result_service_data: usize,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub service_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    OpenService,
    RequestManager,
    RequestServiceFilesList,
    RequestUpdateServiceData,
    RequestUploadServiceFiles(Vec<FileName>),
    GetServiceFilesList(String),
    GetServiceData(String),
    GetListOpt(String),
    GetUpdateServiceResult(String),
    GetUploadData(String),
    UploadConfirm(usize),
    FinishUploadFiles,
    UpdateName(String),
    UpdateDescription(String),
    // UpdateRegionId(String),
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for ServiceSettings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ServiceSettings {
            error: None,
            current_service: None,
            current_service_uuid: String::new(),
            request_service: ServiceUpdatePreData::default(),
            request_upload_data: Vec::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            supplier_list: Vec::new(),
            regions: Vec::new(),
            update_service: false,
            files_list: Vec::new(),
            disable_save_changes_btn: true,
            get_result_service_data: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let logged_user_uuid = match get_logged_user() {
            Some(cu) => cu.uuid,
            None => {
                set_history_back(Some(String::new()));
                // route to login page if not found token
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                String::new()
            },
        };
        // get service uuid for request service data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_service_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/service/settings/")
            .to_string();
        // get flag changing current service in route
        let not_matches_service_uuid = target_service_uuid != self.current_service_uuid;
        // debug!("self.current_service_uuid {:#?}", self.current_service_uuid);
        if not_matches_service_uuid {
            // clear old data
            self.current_service = None;
            self.current_service_uuid = String::new();
            self.request_service = ServiceUpdatePreData::default();
        }
        if first_render || not_matches_service_uuid {
            let link = self.link.clone();
            // update current_service_uuid for checking change service in route
            self.current_service_uuid = target_service_uuid.clone();
            spawn_local(async move {
                let ipt_companies_arg = get_update_service_data_opt::IptCompaniesArg{
                    companiesUuids: None,
                    userUuid: Some(logged_user_uuid),
                    favorite: None,
                    supplier: Some(true),
                };
                let res = make_query(GetUpdateServiceDataOpt::build_query(get_update_service_data_opt::Variables {
                    service_uuid: target_service_uuid,
                    ipt_companies_arg,
                })).await.unwrap();
                link.send_message(Msg::GetServiceData(res.clone()));
                link.send_message(Msg::GetListOpt(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::OpenService => {
                // Redirect to service page
                self.router_agent.send(ChangeRoute(
                    AppRoute::ShowService(self.current_service_uuid.clone()).into()
                ));
            },
            Msg::RequestManager => {
                if self.update_service {
                    self.link.send_message(Msg::RequestUpdateServiceData)
                }
                self.update_service = false;
                self.disable_save_changes_btn = true;
                self.get_result_service_data = 0;
            },
            Msg::RequestServiceFilesList => {
                let service_uuid = self.props.service_uuid.clone();
                spawn_local(async move {
                    let res = make_query(ServiceFilesList::build_query(
                        service_files_list::Variables { service_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetServiceFilesList(res));
                })
            },
            Msg::RequestUpdateServiceData => {
                let service_uuid = self.current_service_uuid.clone();
                let request_service: ServiceUpdateData = (&self.request_service).into();

                spawn_local(async move {
                    let ServiceUpdateData {
                        name,
                        description,
                        region_id,
                    } = request_service;
                    let ipt_update_service_data = put_service_update::IptUpdateServiceData {
                        name,
                        description,
                        regionId: region_id,
                    };
                    let res = make_query(PutServiceUpdate::build_query(put_service_update::Variables {
                        service_uuid,
                        ipt_update_service_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUpdateServiceResult(res));
                })
            },
            Msg::RequestUploadServiceFiles(filenames) => {
                debug!("filenames: {:?}", filenames);
                if self.current_service_uuid.len() != 36 || filenames.is_empty() {
                    return false
                }
                debug!("filenames: {:?}", filenames);
                let service_uuid = self.current_service_uuid.clone();
                spawn_local(async move {
                    let ipt_service_files_data = upload_service_files::IptServiceFilesData{
                        filenames,
                        serviceUuid: service_uuid,
                        commitMsg: String::new(),
                    };
                    let res = make_query(UploadServiceFiles::build_query(upload_service_files::Variables{
                        ipt_service_files_data
                    })).await.unwrap();
                    link.send_message(Msg::GetUploadData(res));
                })
            },
            Msg::GetServiceFilesList(res) => {
                match resp_parsing_two_level(res, "service", "serviceFiles") {
                    Ok(result) => self.files_list = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("serviceFilesList {:?}", self.files_list.len());
            },
            Msg::GetUploadData(res) => {
                match resp_parsing(res, "uploadServiceFiles") {
                    Ok(result) => self.request_upload_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("uploadServiceFiles {:?}", self.request_upload_data.len());
            },
            Msg::GetServiceData(res) => {
                match resp_parsing::<ServiceInfo>(res, "service") {
                    Ok(service_data) => {
                        debug!("Service data: {:?}", service_data);
                        self.current_service_uuid = service_data.uuid.clone();
                        self.files_list = service_data.files.clone();
                        self.current_service = Some(service_data.clone());
                        self.request_service = service_data.into();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetListOpt(res) => {
                match get_value_response(res) {
                    Ok(value) => {
                        self.supplier_list = get_from_value(&value, "companies").unwrap_or_default();
                        self.regions = get_from_value(&value, "regions").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateServiceResult(res) => {
                match resp_parsing(res, "putServiceUpdate") {
                    Ok(result) => self.get_result_service_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("Service data: {:?}", self.get_result_service_data);
            },
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of files: {:?}", confirmations);
                link.send_message(Msg::FinishUploadFiles);
            },
            Msg::FinishUploadFiles => {
                self.request_upload_data.clear();
                self.files_list.clear();
                link.send_message(Msg::RequestServiceFilesList);
            },
            Msg::UpdateName(data) => {
                self.request_service.name = data;
                self.update_service = true;
                self.disable_save_changes_btn = false;
            },
            Msg::UpdateDescription(data) => {
                self.request_service.description = data;
                self.update_service = true;
                self.disable_save_changes_btn = false;
            },
            // Msg::UpdateRegionId(data) => {
            //     self.request_service.region_id = data.parse::<usize>().unwrap_or_default();
            // },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.service_uuid == props.service_uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{
            <div class="service-page">
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                        {show_notification(
                            get_value_field(&214),
                            "is-success",
                            self.get_result_service_data > 0
                        )}
                        {self.show_top_btn()}
                        <br/>
                        {self.show_main_card()}
                        <br/>
                        {match &self.current_service {
                            Some(service_data) => html!{<>
                                {self.show_service_params(service_data)}
                                <br/>
                                {self.show_service_files(service_data)}
                                <br/>
                                <SearchSpecsTags
                                    service_specs={service_data.service_specs.clone()}
                                    service_uuid={service_data.uuid.clone()}
                                />
                                <br/>
                                <AddKeywordsTags
                                    service_keywords={service_data.service_keywords.clone()}
                                    service_uuid={service_data.uuid.clone()}
                                />
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

impl ServiceSettings {
    fn show_main_card(&self) -> Html {
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let onclick_save_changes = self.link.callback(|_| Msg::RequestManager);

        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&373)}</p>
                </header>
                <div class="card-content">
                    <div class="content">
                        <div class="field">
                            <label class="label">{get_value_field(&110)}</label>
                            <input
                                id="update-name"
                                class="input"
                                type="text"
                                placeholder={get_value_field(&110)}
                                value={self.request_service.name.clone()}
                                oninput={oninput_name} />
                        </div>
                        <div class="field">
                            <label class="label">{get_value_field(&61)}</label>
                            <textarea
                                id="update-description"
                                class="textarea"
                                // rows="10"
                                type="text"
                                placeholder={get_value_field(&61)}
                                value={self.request_service.description.clone()}
                                oninput={oninput_description} />
                        </div>
                    </div>
                    <footer class="card-footer">
                        {ft_save_btn(
                            "update-service-data",
                            onclick_save_changes,
                            true,
                            self.disable_save_changes_btn
                        )}
                    </footer>
                </div>
            </div>
        }
    }

    fn show_service_params(&self, service_data: &ServiceInfo) -> Html {
        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&374)}</p> // Manage service characteristics
                </header>
                <div class="card-content">
                    <div class="content">
                        <ServiceParamsTags
                            show_manage_btn={true}
                            service_uuid={self.current_service_uuid.clone()}
                            params_count={service_data.files_count}
                            />
                    </div>
                </div>
            </div>
        }
    }

    fn show_service_files(&self, service_data: &ServiceInfo) -> Html {
        let callback_upload_filenames =
            self.link.callback(move |filenames| Msg::RequestUploadServiceFiles(filenames));
        let request_upload_files = match self.request_upload_data.is_empty() {
            true => None,
            false => Some(self.request_upload_data.clone()),
        };
        let callback_upload_confirm =
            self.link.callback(|confirmations| Msg::UploadConfirm(confirmations));

        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&375)}</p>
                </header>
                <div class="card-content">
                    <div class="content">
                        <div class="columns">
                            <div class="column">
                                <h3 class="has-text-weight-bold">{get_value_field(&376)}</h3> // Files stadndard
                                <ServiceFilesCard
                                    show_delete_btn={true}
                                    service_uuid={service_data.uuid.clone()}
                                    files={self.files_list.clone()}
                                />
                            </div>
                            <div class="column">
                                <h3 class="has-text-weight-bold">{get_value_field(&377)}</h3>
                                <UploaderFiles
                                    text_choose_files={222} // Choose service files…
                                    callback_upload_filenames={callback_upload_filenames}
                                    request_upload_files={request_upload_files}
                                    callback_upload_confirm={callback_upload_confirm}
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn show_top_btn(&self) -> Html {
        let onclick_open_service = self.link.callback(|_| Msg::OpenService);

        html!{
            <div class="media">
                <div class="media-left">
                    {ft_back_btn(
                        "open-service",
                        onclick_open_service,
                        get_value_field(&378), // Open service
                    )}
                </div>
                <div class="media-content">
                </div>
                <div class="media-right">
                </div>
            </div>
        }
    }
}
