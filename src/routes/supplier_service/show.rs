use yew::{agent::Bridged, classes, html, Bridge, Component, ComponentLink, Html, ShouldRender, Properties};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::fragments::clipboard::ShareLinkBtn;
use crate::fragments::discussion::DiscussionCommentsBlock;
use crate::fragments::supplier_service::ServiceParamsTags;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    switch_icon::res_btn,
    list_errors::ListErrors,
    component::CatalogComponents,
    supplier_service::{ServiceFilesCard, SpecsTags, KeywordsTags},
    img_showcase::ImgShowcase,
};
use crate::services::content_adapter::Markdownable;
use crate::services::{get_logged_user, get_value_field, resp_parsing, set_history_back, title_changer};
use crate::types::{ComponentsQueryArg, DownloadFile, ObjectType, Pathname, ServiceInfo, SlimUser, UUID, ToObject};
use crate::gqls::make_query;
use crate::gqls::supplier_service::{
    GetServiceData, get_service_data,
    ServiceFiles, service_files,
};

/// Service with relate data
pub struct ShowService {
    error: Option<Error>,
    service: Option<ServiceInfo>,
    current_service_uuid: UUID,
    current_user_owner: bool,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    show_full_description: bool,
    show_related_components: bool,
    open_discussion_card: bool,
    file_arr: Vec<DownloadFile>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub service_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    RequestDownloadFiles,
    GetDownloadFilesResult(String),
    GetServiceData(String),
    // ShowDescription,
    ShowComponentsList,
    OpenDiscussionBlock,
    OpenServiceSetting,
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for ShowService {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ShowService {
            error: None,
            service: None,
            current_service_uuid: String::new(),
            current_user_owner: false,
            // task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            show_full_description: false,
            show_related_components: false,
            open_discussion_card: false,
            file_arr: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if let None = get_logged_user() {
            set_history_back(Some(String::new()));
            // route to login page if not found token
            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
        };

        // get service uuid for request service data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_service_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/service/")
            .to_string();
        // get flag changing current service in route
        let not_matches_service_uuid = target_service_uuid != self.current_service_uuid;
        // debug!("self.current_service_uuid {:#?}", self.current_service_uuid);

        if let Some(service) = &self.service {
            title_changer::set_title(service.name.as_str());
        }

        if first_render || not_matches_service_uuid {
            let link = self.link.clone();

            // update current_service_uuid for checking change service in route
            self.current_service_uuid = target_service_uuid.to_string();
            link.send_message(Msg::RequestDownloadFiles);
            spawn_local(async move {
                let res = make_query(GetServiceData::build_query(get_service_data::Variables {
                    service_uuid: target_service_uuid,
                })).await.unwrap();

                link.send_message(Msg::GetServiceData(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFiles => {
                let service_uuid = self.props.service_uuid.clone();
                spawn_local(async move {
                    let ipt_service_files_arg = service_files::IptServiceFilesArg{
                        filesUuids: None,
                        serviceUuid: service_uuid,
                    };
                    let res = make_query(ServiceFiles::build_query(service_files::Variables{
                        ipt_service_files_arg
                    })).await.unwrap();
                    link.send_message(Msg::GetDownloadFilesResult(res));
                })
            },
            Msg::GetDownloadFilesResult(res) => {
                match resp_parsing::<Vec<DownloadFile>>(res, "serviceFiles") {
                    Ok(mut result) => {
                        debug!("serviceFiles: {:?}", result);
                        if result.is_empty() {
                            return  true
                        }
                        // checkign have main image
                        if let Some(main_img) = self.file_arr.first() {
                            result.push(main_img.clone());
                        }
                        self.file_arr = result;
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetServiceData(res) => {
                match resp_parsing::<ServiceInfo>(res, "service") {
                    Ok(service_data) => {
                        debug!("Service data: {:?}", service_data);
                        self.current_service_uuid = service_data.uuid.clone();
                        if let Some(user) = get_logged_user() {
                            self.current_user_owner = service_data.owner_user.uuid == user.uuid;
                        }
                        // description length check for show
                        self.show_full_description = service_data.description.len() < 250;
                        self.service = Some(service_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            // Msg::ShowDescription => self.show_full_description = !self.show_full_description,
            Msg::ShowComponentsList => self.show_related_components = !self.show_related_components,
            Msg::OpenDiscussionBlock => self.open_discussion_card = !self.open_discussion_card,
            Msg::OpenServiceSetting => {
                if let Some(service_data) = &self.service {
                    // Redirect to page for change and update service
                    self.router_agent.send(ChangeRoute(AppRoute::ServiceSettings(
                        service_data.uuid.clone()
                    ).into()));
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {}
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

        match &self.service {
            Some(service_data) => html!{
                <div class="service-page">
                    <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
                    <div class="container page">
                        <div class="row">
                            <div class="card column">
                              {self.show_main_card(service_data)}
                            </div>
                            <br/>
                            {self.show_related_components_card(&service_data.uuid)}
                            {self.show_service_discussion()}
                            <div class="columns">
                                <div class="column">
                                    {self.show_service_params(service_data)}
                                </div>
                                <div class="column">
                                    {self.show_service_files(service_data)}
                                </div>
                            </div>
                            <SpecsTags
                                show_manage_btn={false}
                                service_uuid={service_data.uuid.clone()}
                                specs={service_data.service_specs.clone()}
                            />
                            <br/>
                            <KeywordsTags
                                show_delete_btn={false}
                                service_uuid={service_data.uuid.clone()}
                                keywords={service_data.service_keywords.clone()}
                            />
                            <br/>
                        </div>
                    </div>
                </div>
            },
            None => html!{
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
            },
        }
    }
}

impl ShowService {
    fn show_main_card(&self, service_data: &ServiceInfo) -> Html {
        // let show_description_btn = self.link.callback(|_| Msg::ShowDescription);
        let mut class_tag = classes!("tag", "is-medium");
        class_tag.push(service_data.service_status.get_class_color());

        html!{
            <div class={"columns"}>
                <div class={class_tag} >{service_data.service_status.name.to_string()}</div>
              <ImgShowcase
                object_uuid={self.current_service_uuid.clone()}
                file_arr={self.file_arr.clone()}
              />
              <div class="column">
                // <div class="media">
                //     <div class="media-content">
                //         {get_value_field(&94)}
                //         <GoToUser data = {service_data.owner_user.clone()} />
                //     </div>
                //     <div class="media-right" style="margin-right: 1rem">
                //         {get_value_field(&145)} // type access
                //         <span class="id-box has-text-weight-bold">
                //             {service_data.type_access.name.clone()}
                //         </span>
                //     </div>
                // </div>
                // <h1>{"Service"}</h1>
                <div class="has-text-weight-bold is-size-4">
                    {service_data.name.clone()}
                </div>
                // <div class="column is-narrow" title={get_value_field(&141)}>
                //     <span class="icon is-small">
                //         <i class={classes!("fa", "fa-user")}></i>
                //     </span>
                //     {" "}
                //     <GoToUser data = {service_data.owner_user.clone()} />
                // </div>
                <div class="buttons flexBox">
                    {self.show_related_components_btn()}
                    {self.show_discussion_btn()}
                    {self.show_setting_btn()}
                    <ShareLinkBtn />
                </div>
                <div class="service-description">
                    {service_data.description.to_markdown()}
                </div>
              </div>
            </div>
        }
    }

    fn show_service_params(&self, service_data: &ServiceInfo) -> Html {
        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&101)}</p> // Сharacteristics of the service
                </header>
                <div class="card-content">
                    <div class="content">
                        <ServiceParamsTags
                            show_manage_btn={false}
                            service_uuid={self.current_service_uuid.clone()}
                            params_count={service_data.files_count}
                            />
                    </div>
                </div>
            </div>
        }
    }

    fn show_service_files(&self, service_data: &ServiceInfo) -> Html {
        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&376)}</p> // Files
                </header>
                <div class="card-content">
                    <div class="content">
                        <ServiceFilesCard
                            show_delete_btn={false}
                            service_uuid={service_data.uuid.clone()}
                            files={service_data.files.clone()}
                        />
                    </div>
                </div>
            </div>
        }
    }

    fn show_related_components_btn(&self) -> Html {
        let onclick_related_components_btn = self.link.callback(|_| Msg::ShowComponentsList);
        let (text_btn, classes_btn) = match &self.show_related_components {
            true => (get_value_field(&295), "button is-info is-light is-active"),
            false => (get_value_field(&296), "button is-info"),
        };

        html!{
            <button class={classes_btn}
                onclick={onclick_related_components_btn} >
              <span class="icon is-small"><i class="fa fa-cubes"></i></span>
              <span>{text_btn}</span>
            </button>
        }
    }

    fn show_related_components_card(&self, service_uuid: &UUID) -> Html {
        match &self.show_related_components {
            true => html!{<>
                <div class="card">
                    <header class="card-header has-background-info-light">
                        <p class="card-header-title">{get_value_field(&154)}</p> // Components
                    </header>
                    <div class="card-content">
                        <div class="content">
                            <CatalogComponents
                                show_create_btn={false}
                                arguments={ComponentsQueryArg::set_service_uuid(service_uuid)}
                                />
                        </div>
                    </div>
                </div>
                <br/>
            </>},
            false => html!{},
        }
    }

    fn show_setting_btn(&self) -> Html {
        let onclick_setting_service_btn = self.link.callback(|_| Msg::OpenServiceSetting);
        match &self.current_user_owner {
            true => {res_btn(
                classes!("fa", "fa-tools"),
                onclick_setting_service_btn,
                String::new(),
                get_value_field(&16),
                Pathname::ServiceSetting(self.current_service_uuid.clone())
            )},
            false => html!{},
        }
    }

    fn show_discussion_btn(&self) -> Html {
        let onclick_open_discussion_btn =
            self.link.callback(|_| Msg::OpenDiscussionBlock);
        let class_discussion_btn = match self.open_discussion_card {
            true => "button is-light is-info is-active",
            false => "button is-info",
        };
        html!{
            <button
            class={class_discussion_btn}
            onclick={onclick_open_discussion_btn}>
                <span class={"icon is-small"}><i class={"far fa-comments"}></i></span>
                <span>{get_value_field(&380)}</span>
            </button>
        }
    }

    fn show_service_discussion(&self) -> Html {
        match self.open_discussion_card {
            true => html!{<>
                <div class="card">
                    <header class="card-header has-background-info-light">
                        <p class="card-header-title">{get_value_field(&380)}</p>
                    </header>
                    <div class="card-content">
                        <DiscussionCommentsBlock
                            discussion_uuid={None}
                            object_type={ObjectType::new(self.current_service_uuid.clone(), ToObject::SERVICE)}
                        />
                    </div>
                </div>
                <br/>
            </>},
            false => html!{},
        }
    }
}
