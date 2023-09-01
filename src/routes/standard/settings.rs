use yew::{
    agent::Bridged, html, Bridge, Component, Properties,
    ComponentLink, Html, ShouldRender, InputData, ChangeData
};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use chrono::NaiveDateTime;
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    file::UploaderFiles,
    list_errors::ListErrors,
    standard::{
        StandardFilesCard, SearchSpecsTags,
        AddKeywordsTags, UpdateStandardFaviconCard
    },
};
use crate::services::{get_logged_user, get_value_field, resp_parsing_two_level, resp_parsing, get_value_response, get_from_value};
use crate::types::{
    UUID, StandardInfo, SlimUser, Region, TypeAccessInfo, UploadFile, ShowFileInfo,
    ShowCompanyShort, StandardUpdatePreData, StandardUpdateData, StandardStatus,
};
use crate::gqls::make_query;
use crate::gqls::standard::{
    GetUpdateStandardDataOpt, get_update_standard_data_opt,
    PutStandardUpdate, put_standard_update,
    DeleteStandard, delete_standard,
    ChangeStandardAccess, change_standard_access,
    UploadStandardFiles, upload_standard_files,
    StandardFilesList, standard_files_list,
};

type FileName = String;

/// Standard with relate data
pub struct StandardSettings {
    error: Option<Error>,
    current_standard: Option<StandardInfo>,
    current_standard_uuid: UUID,
    request_standard: StandardUpdatePreData,
    request_upload_data: Vec<UploadFile>,
    request_access: i64,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    supplier_list: Vec<ShowCompanyShort>,
    standard_statuses: Vec<StandardStatus>,
    regions: Vec<Region>,
    types_access: Vec<TypeAccessInfo>,
    update_standard: bool,
    update_standard_access: bool,
    files_list: Vec<ShowFileInfo>,
    disable_delete_standard_btn: bool,
    confirm_delete_standard: String,
    hide_delete_modal: bool,
    disable_save_changes_btn: bool,
    get_result_standard_data: usize,
    get_result_access: bool,
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
    RequestStandardFilesList,
    RequestUpdateStandardData,
    RequestChangeAccess,
    RequestDeleteStandard,
    RequestUploadStandardFiles(Vec<FileName>),
    GetStandardFilesList(String),
    GetStandardData(String),
    GetListOpt(String),
    GetUpdateStandardResult(String),
    GetUpdateAccessResult(String),
    GetUploadData(String),
    UploadConfirm(usize),
    FinishUploadFiles,
    GetDeleteStandard(String),
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
    UpdateConfirmDelete(String),
    ResponseError(Error),
    ChangeHideDeleteStandard,
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
            request_access: 0,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            supplier_list: Vec::new(),
            standard_statuses: Vec::new(),
            regions: Vec::new(),
            types_access: Vec::new(),
            update_standard: false,
            update_standard_access: false,
            files_list: Vec::new(),
            disable_delete_standard_btn: true,
            confirm_delete_standard: String::new(),
            hide_delete_modal: true,
            disable_save_changes_btn: true,
            get_result_standard_data: 0,
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
        if first_render || not_matches_standard_uuid {
            let link = self.link.clone();
            // update current_standard_uuid for checking change standard in route
            self.current_standard_uuid = target_standard_uuid.clone();
            spawn_local(async move {
                let ipt_companies_arg = get_update_standard_data_opt::IptCompaniesArg{
                    companiesUuids: None,
                    userUuid: Some(logged_user_uuid),
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
                self.update_standard = false;
                self.update_standard_access = false;
                self.disable_save_changes_btn = true;
                self.get_result_standard_data = 0;
                self.get_result_access = false;
            },
            Msg::RequestStandardFilesList => {
                let standard_uuid = self.props.standard_uuid.clone();
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
            Msg::RequestUploadStandardFiles(filenames) => {
                debug!("filenames: {:?}", filenames);
                if self.current_standard_uuid.len() != 36 || filenames.is_empty() {
                    return false
                }
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
            },
            Msg::GetStandardFilesList(res) => {
                match resp_parsing_two_level(res, "standard", "standardFiles") {
                    Ok(result) => self.files_list = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("standardFilesList {:?}", self.files_list.len());
            },
            Msg::GetUploadData(res) => {
                match resp_parsing(res, "uploadStandardFiles") {
                    Ok(result) => self.request_upload_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("uploadStandardFiles {:?}", self.request_upload_data.len());
            },
            Msg::GetStandardData(res) => {
                match resp_parsing::<StandardInfo>(res, "standard") {
                    Ok(standard_data) => {
                        debug!("Standard data: {:?}", standard_data);
                        self.current_standard_uuid = standard_data.uuid.clone();
                        self.files_list = standard_data.standard_files.clone();
                        self.current_standard = Some(standard_data.clone());
                        self.request_standard = standard_data.into();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetListOpt(res) => {
                match get_value_response(res) {
                    Ok(value) => {
                        self.supplier_list = get_from_value(&value, "companies").unwrap_or_default();
                        self.standard_statuses = get_from_value(&value, "standardStatuses").unwrap_or_default();
                        self.regions = get_from_value(&value, "regions").unwrap_or_default();
                        self.types_access = get_from_value(&value, "typesAccess").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetUpdateStandardResult(res) => {
                match resp_parsing(res, "putStandardUpdate") {
                    Ok(result) => self.get_result_standard_data = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("Standard data: {:?}", self.get_result_standard_data);
            },
            Msg::GetUpdateAccessResult(res) => {
                match resp_parsing(res, "changeStandardAccess") {
                    Ok(result) => self.get_result_access = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("Standard change access: {:?}", self.get_result_access);
            },
            Msg::UploadConfirm(confirmations) => {
                debug!("Confirmation upload of files: {:?}", confirmations);
                link.send_message(Msg::FinishUploadFiles);
            },
            Msg::FinishUploadFiles => {
                self.request_upload_data.clear();
                self.files_list.clear();
                link.send_message(Msg::RequestStandardFilesList);
            },
            Msg::GetDeleteStandard(res) => {
                match resp_parsing::<UUID>(res, "deleteStandard") {
                    Ok(result) => {
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
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
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
            Msg::UpdateConfirmDelete(data) => {
                self.disable_delete_standard_btn = self.current_standard_uuid != data;
                self.confirm_delete_standard = data;
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ChangeHideDeleteStandard => self.hide_delete_modal = !self.hide_delete_modal,
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.standard_uuid == props.standard_uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

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
                                <br/>
                                <div class="columns">
                                  <div class="column">
                                    {self.update_standard_favicon()}
                                    <br/>
                                    {self.show_standard_params()}
                                  </div>
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
        let onchange_change_owner_company =
            self.link.callback(|ev: ChangeData| Msg::UpdateCompanyUuid(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
            }));
        let onchange_change_type_access =
            self.link.callback(|ev: ChangeData| Msg::UpdateTypeAccessId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let oninput_name =
            self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_description =
            self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));

        html!{<div class="card">
            <div class="column">
                <div class="control">
                    <div class="media">
                        <div class="media-content">
                            <label class="label">{get_value_field(&223)}</label> // Owner company
                            <div class="select">
                              <select
                                  id="set-owner-company"
                                  select={self.request_standard.company_uuid.clone()}
                                  onchange=onchange_change_owner_company
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
                            <label class="label">{get_value_field(&114)}</label>
                            <div class="select">
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
                </div>
                <label class="label">{get_value_field(&110)}</label>
                <input
                    id="update-name"
                    class="input"
                    type="text"
                    placeholder=get_value_field(&110)
                    value={self.request_standard.name.clone()}
                    oninput=oninput_name />
                <label class="label">{get_value_field(&61)}</label>
                <textarea
                    id="update-description"
                    class="textarea"
                    // rows="10"
                    type="text"
                    placeholder=get_value_field(&61)
                    value={self.request_standard.description.clone()}
                    oninput=oninput_description />
            </div>
        </div>}
    }

    fn update_standard_favicon(&self) -> Html {
        let callback_update_favicon = self.link.callback(|_| Msg::Ignore);

        html!{<>
            <h2 class="has-text-weight-bold">{get_value_field(&184)}</h2> // Update image for preview
            <div class="card column">
                <UpdateStandardFaviconCard
                    standard_uuid=self.current_standard_uuid.clone()
                    callback=callback_update_favicon.clone()
                />
            </div>
        </>}
    }

    fn show_standard_params(&self) -> Html {
        let oninput_classifier =
            self.link.callback(|ev: InputData| Msg::UpdateClassifier(ev.value));
        let oninput_specified_tolerance =
            self.link.callback(|ev: InputData| Msg::UpdateSpecifiedTolerance(ev.value));
        let oninput_technical_committee =
            self.link.callback(|ev: InputData| Msg::UpdateTechnicalCommittee(ev.value));
        let oninput_publication_at =
            self.link.callback(|ev: InputData| Msg::UpdatePublicationAt(ev.value));
        let onchange_standard_status_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateStandardStatusId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        let onchange_region_id =
            self.link.callback(|ev: ChangeData| Msg::UpdateRegionId(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "1".to_string(),
            }));
        html!{
            <>
              <h2 class="has-text-weight-bold">{get_value_field(&224)}</h2> // Manage standard characteristics
              <div class="card column">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{get_value_field(&146)}</td> // classifier
                        <td><input
                            id="update-classifier"
                            class="input"
                            type="text"
                            placeholder=get_value_field(&146)
                            value={self.request_standard.classifier.clone()}
                            oninput=oninput_classifier /></td>
                      </tr>
                      <tr>
                        <td>{get_value_field(&147)}</td>
                        // <td>{self.request_standard.specified_tolerance.as_ref().map(|x| x.clone()).unwrap_or_default()}</td>
                        <td><input
                            id="update-specified-tolerance"
                            class="input"
                            type="text"
                            placeholder=get_value_field(&147)
                            value={self.request_standard.specified_tolerance.clone()}
                            oninput=oninput_specified_tolerance /></td>
                      </tr>
                      <tr>
                        <td>{get_value_field(&148)}</td>
                        <td><input
                            id="update-technical-committee"
                            class="input"
                            type="text"
                            placeholder=get_value_field(&148)
                            value={self.request_standard.technical_committee.clone()}
                            oninput=oninput_technical_committee /></td>
                      </tr>
                      <tr>
                        <td>{get_value_field(&149)}</td>
                        <td><input
                            id="update-publication-at"
                            class="input"
                            type="date"
                            placeholder=get_value_field(&149)
                            value={self.request_standard.publication_at
                                .as_ref()
                                .map(|x| format!("{:.*}", 10, x.to_string()))
                                .unwrap_or_default()}
                            oninput=oninput_publication_at
                            /></td>
                      </tr>
                      <tr>
                        <td>{get_value_field(&150)}</td>
                        <td><div class="control">
                            <div class="select">
                              <select
                                  id="standard-status-id"
                                  select={self.request_standard.standard_status_id.to_string()}
                                  onchange=onchange_standard_status_id
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
                        <td>{get_value_field(&151)}</td>
                        <td><div class="select">
                              <select
                                  id="region"
                                  select={self.request_standard.region_id.to_string()}
                                  onchange=onchange_region_id
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

    fn show_standard_files(&self, standard_data: &StandardInfo) -> Html {
        let callback_upload_filenames =
            self.link.callback(move |filenames| Msg::RequestUploadStandardFiles(filenames));
        let request_upload_files = match self.request_upload_data.is_empty() {
            true => None,
            false => Some(self.request_upload_data.clone()),
        };
        let callback_upload_confirm =
            self.link.callback(|confirmations| Msg::UploadConfirm(confirmations));
        html!{
            <div class="column">
              <h2 class="has-text-weight-bold">{get_value_field(&225)}</h2> // Files stadndard
              <div class="card column">
                <UploaderFiles
                    text_choose_files={222} // Choose standard files…
                    callback_upload_filenames={callback_upload_filenames}
                    request_upload_files={request_upload_files}
                    callback_upload_confirm={callback_upload_confirm}
                    />
                <StandardFilesCard
                    show_download_btn = false
                    show_delete_btn = true
                    standard_uuid = standard_data.uuid.clone()
                    files = self.files_list.clone()
                    />
              </div>
            </div>
        }
    }

    fn show_standard_specs(&self, standard_data: &StandardInfo) -> Html {
        html!{<>
            <h2 class="has-text-weight-bold">{get_value_field(&104)}</h2>
            <div class="card">
              <SearchSpecsTags
                  standard_specs = standard_data.standard_specs.clone()
                  standard_uuid = standard_data.uuid.clone()
                />
            </div>
        </>}
    }

    fn show_standard_keywords(&self, standard_data: &StandardInfo) -> Html {
        // debug!("Keywords: {:?}", &standard_data.uuid);
        html!{<>
              <h2 class="has-text-weight-bold">{get_value_field(&105)}</h2>
              <div class="card">
                <AddKeywordsTags
                    standard_keywords = standard_data.standard_keywords.clone()
                    standard_uuid = standard_data.uuid.clone()
                  />
              </div>
        </>}
    }

    fn show_manage_btn(&self) -> Html {
        let onclick_open_standard = self.link.callback(|_| Msg::OpenStandard);
        let onclick_show_delete_modal = self.link.callback(|_| Msg::ChangeHideDeleteStandard);
        let onclick_save_changes = self.link.callback(|_| Msg::RequestManager);

        html!{
            <div class="media">
                <div class="media-left">
                    <button
                        id="open-standard"
                        class="button"
                        onclick={onclick_open_standard} >
                        {get_value_field(&226)} // Open standard
                    </button>
                </div>
                <div class="media-content">
                    {if self.get_result_standard_data > 0 || self.get_result_access {
                        html!{get_value_field(&214)} // Data updated
                    } else {
                        html!{}
                    }}
                </div>
                <div class="media-right">
                    {self.modal_delete_standard()}
                    <div class="buttons">
                        <button
                            id="delete-standard"
                            class="button is-danger"
                            onclick={onclick_show_delete_modal} >
                            {get_value_field(&135)}
                        </button>
                        <button
                            id="update-data"
                            class="button"
                            onclick={onclick_save_changes}
                            disabled={self.disable_save_changes_btn} >
                            {get_value_field(&46)}
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn modal_delete_standard(&self) -> Html {
        let onclick_hide_modal =
            self.link.callback(|_| Msg::ChangeHideDeleteStandard);
        let oninput_delete_standard =
            self.link.callback(|ev: InputData| Msg::UpdateConfirmDelete(ev.value));
        let onclick_delete_standard =
            self.link.callback(|_| Msg::RequestDeleteStandard);
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
                      <p class="modal-card-title">{get_value_field(&227)}</p> // Delete standard
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <p class="is-size-6">
                            {get_value_field(&218)}
                            <span class="has-text-danger-dark">{self.request_standard.name.clone()}</span>
                            {get_value_field(&228)}
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
                           oninput=oninput_delete_standard />
                    </section>
                    <footer class="modal-card-foot">
                        <button
                            id="delete-standard"
                            class="button is-danger"
                            disabled={self.disable_delete_standard_btn}
                            onclick={onclick_delete_standard} >{get_value_field(&220)}</button> // Yes, delete
                        <button class="button" onclick=onclick_hide_modal.clone()>{get_value_field(&221)}</button> // Cancel
                    </footer>
                </div>
              </div>
            </div>
        }
    }
}
