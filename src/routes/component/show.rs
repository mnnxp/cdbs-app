use std::collections::HashMap;
use yew::{agent::Bridged, classes, html, Bridge, Component, Properties, ComponentLink, Html, ShouldRender};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::fragments::discussion::DiscussionCommentsBlock;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    buttons::{ft_see_btn, ft_follow_btn},
    switch_icon::res_btn,
    list_errors::ListErrors,
    user::GoToUser,
    component::{
        ComponentStandardItem, ComponentSupplierItem, ComponentLicenseTag, ComponentParamsTags,
        ModificationsTableCard, FilesOfFilesetCard, ModificationFilesetsCard,
        ComponentFilesBlock, SpecsTags, KeywordsTags,
    },
    img_showcase::ImgShowcase,
    three_showcase::ThreeShowcase,
    clipboard::ShareLinkBtn,
};
use crate::services::content_adapter::{DateDisplay, Markdownable};
use crate::services::{get_classes_table, get_logged_user, get_value_field, resp_parsing, set_history_back, title_changer, Counter};
use crate::types::{ComponentInfo, DownloadFile, ObjectType, Pathname, SlimUser, ToObject, UUID};
use crate::gqls::make_query;
use crate::gqls::component::{
    ComponentFiles, component_files,
    GetComponentData, get_component_data,
    AddComponentFav, add_component_fav,
    DeleteComponentFav, delete_component_fav,
};

pub enum ActiveTab {
    Description,
    Characteristics,
    ComponentFiles
}

/// Component with relate data
pub struct ShowComponent {
    error: Option<Error>,
    component: Option<ComponentInfo>,
    current_component_uuid: UUID,
    current_user_owner: bool,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    select_modification_uuid: UUID,
    modification_filesets: HashMap<UUID, Vec<(UUID, String)>>,
    select_fileset_uuid: UUID,
    current_filesets_program: Vec<(UUID, String)>,
    show_full_description: bool,
    open_fileset_files_card: bool,
    open_discussion_card: bool,
    show_related_standards: bool,
    file_arr: Vec<DownloadFile>,
    show_three_view: bool,
    active_tab: ActiveTab,
}

impl Counter for ShowComponent {
    fn quantity(&self) -> usize {
        self.subscribers
    }
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub component_uuid: UUID,
}

pub enum Msg {
    SelectFileset(UUID),
    SelectModification(UUID),
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    ResponseError(Error),
    GetComponentData(String),
    ShowDescription,
    ShowStandardsList,
    ShowFilesetFilesBlock(bool),
    OpenDiscussionBlock,
    OpenComponentSetting,
    GetDownloadFileResult(String),
    Show3D,
    ChangeActiveTab(ActiveTab),
    ClearError,
    Ignore,
}

impl Component for ShowComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ShowComponent {
            error: None,
            component: None,
            current_component_uuid: String::new(),
            current_user_owner: false,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            select_modification_uuid: String::new(),
            modification_filesets: HashMap::new(),
            select_fileset_uuid: String::new(),
            current_filesets_program: Vec::new(),
            show_full_description: false,
            open_fileset_files_card: false,
            open_discussion_card: false,
            show_related_standards: false,
            file_arr: Vec::new(),
            show_three_view: false,
            active_tab: ActiveTab::Description,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if get_logged_user().is_none() {
            set_history_back(Some(String::new()));
        }

        // get component uuid for request component data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_component_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/component/")
            .to_string();
        // get flag changing current component in route
        let not_matches_component_uuid = target_component_uuid != self.current_component_uuid;
        debug!("self.current_component_uuid {:#?}", self.current_component_uuid);

        let link = self.link.clone();

        if let Some(component) = &self.component {
            title_changer::set_title(component.name.as_str());
        }

        if first_render || not_matches_component_uuid {
            self.error = None;
            self.component = None;
            self.current_component_uuid = target_component_uuid.to_string();

            // update current_component_uuid for checking change component in route
            if not_matches_component_uuid {
                self.current_user_owner = false;
                self.select_modification_uuid = String::new();
                self.modification_filesets = HashMap::new();
                self.select_fileset_uuid = String::new();
                self.current_filesets_program.clear();
                self.file_arr.clear();
            }

            {
              let target_component_uuid = target_component_uuid.clone();
              let link = self.link.clone();
              spawn_local(async move {
                let res = make_query(GetComponentData::build_query(get_component_data::Variables{
                    component_uuid: target_component_uuid,
                })).await.unwrap();

                link.send_message(Msg::GetComponentData(res));
              })
            };

            spawn_local(async move {
              let ipt_component_files_arg = component_files::IptComponentFilesArg{
                  componentUuid: target_component_uuid.clone(),
                  filesUuids: None,
              };
              let res = make_query(ComponentFiles::build_query(
                  component_files::Variables {
                      ipt_component_files_arg,
                  }
              )).await;
              debug!("res {:?}", res);
              link.send_message(Msg::GetDownloadFileResult(res.unwrap()));
          });
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::SelectFileset(fileset_uuid) => self.select_fileset_uuid = fileset_uuid,
            Msg::SelectModification(modification_uuid) => self.select_modification_uuid = modification_uuid,
            Msg::Follow => {
                let component_uuid = self.component.as_ref().unwrap().uuid.clone();
                spawn_local(async move {
                    let res = make_query(AddComponentFav::build_query(add_component_fav::Variables {
                        component_uuid,
                    })).await.unwrap();
                    link.send_message(Msg::AddFollow(res));
                })
            },
            Msg::AddFollow(res) => {
                match resp_parsing(res, "addComponentFav") {
                    Ok(result) => {
                        if result {
                            self.subscribers += 1;
                            self.is_followed = true;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UnFollow => {
                let component_uuid = self.component.as_ref().unwrap().uuid.to_string();
                spawn_local(async move {
                    let res = make_query(DeleteComponentFav::build_query(delete_component_fav::Variables {
                        component_uuid,
                    })).await.unwrap();
                    link.send_message(Msg::DelFollow(res));
                })
            },
            Msg::DelFollow(res) => {
                match resp_parsing(res, "deleteComponentFav") {
                    Ok(result) => {
                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetComponentData(res) => {
                match resp_parsing::<ComponentInfo>(res, "component") {
                    Ok(component_data) => {
                        debug!("Component data: {:?}", component_data);
                        self.subscribers = component_data.subscribers;
                        self.is_followed = component_data.is_followed;
                        self.current_component_uuid = component_data.uuid.clone();
                        if let Some(user) = get_logged_user() {
                            self.current_user_owner = component_data.owner_user.uuid == user.uuid;
                            debug!("Component data: {:?}", component_data);
                        }
                        // length check for show btn more/less
                        self.show_full_description = component_data.description.len() < 250;
                        // add main image
                        self.file_arr.push(component_data.image_file.clone());
                        self.component = Some(component_data);
                    },
                    Err(err) => {
                        link.send_message(Msg::ResponseError(err));
                        if let None = get_logged_user() {
                            set_history_back(Some(String::new()));
                            // route to login page if not found token
                            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                        };
                    },
                }
            },
            Msg::GetDownloadFileResult(res) => {
                match resp_parsing::<Vec<DownloadFile>>(res, "componentFiles") {
                    Ok(mut result) => {
                        debug!("Download file: {:?}", result);
                        if result.is_empty() {
                            return true
                        }
                        // checkign have main image
                        match self.file_arr.first() {
                            Some(_) => self.file_arr.append(&mut result),
                            None => self.file_arr = result,
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ShowDescription => self.show_full_description = !self.show_full_description,
            Msg::ShowStandardsList => self.show_related_standards = !self.show_related_standards,
            Msg::ShowFilesetFilesBlock(value) => self.open_fileset_files_card = value,
            Msg::OpenDiscussionBlock => self.open_discussion_card = !self.open_discussion_card,
            Msg::OpenComponentSetting => {
                if let Some(component_data) = &self.component {
                    // Redirect to page for change and update component
                    self.router_agent.send(ChangeRoute(AppRoute::ComponentSettings(
                        component_data.uuid.clone()
                    ).into()));
                }
            },
            Msg::Show3D => self.show_three_view = !self.show_three_view,
            Msg::ChangeActiveTab(set_tab) => self.active_tab = set_tab,
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

        match &self.component {
            Some(component_data) => html!{
                <div class="component-page">
                    <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                    <div class="container page">
                        <div class="row">
                            <div class="card column">
                              {self.show_main_card(component_data)}
                            </div>
                            <FilesOfFilesetCard
                                show_card={self.open_fileset_files_card}
                                show_download_btn={true}
                                select_fileset_uuid={self.select_fileset_uuid.clone()}
                                />
                            <br/>
                            {self.show_component_discussion()}
                            {self.show_modifications_table(component_data.modifications_count)}
                            <br/>
                            <div class="columns">
                                <div class="column">
                                    {self.show_component_standards(component_data)}
                                </div>
                                <div class="column">
                                    {self.show_component_suppliers(component_data)}
                                </div>
                            </div>
                            <SpecsTags
                                show_manage_btn={false}
                                component_uuid={component_data.uuid.clone()}
                                specs={component_data.component_specs.clone()}
                            />
                            <br/>
                            <KeywordsTags
                                show_delete_btn={false}
                                component_uuid={component_data.uuid.clone()}
                                keywords={component_data.component_keywords.clone()}
                            />
                            <br/>
                        </div>
                    </div>
                </div>
            },
            None => html!{<ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>},
        }
    }
}

impl ShowComponent {
    fn show_main_card(&self, component_data: &ComponentInfo) -> Html {
        let show_description_btn = self.link.callback(|_| Msg::ShowDescription);
        let callback_select_fileset_uuid = self.link.callback(|value: UUID| Msg::SelectFileset(value));
        let callback_open_fileset = self.link.callback(|value: bool| Msg::ShowFilesetFilesBlock(value));

        html!{<>
            <div class="columns">
                {match self.show_three_view {
                    true => html!{
                        <ThreeShowcase
                            fileset_uuid={self.select_fileset_uuid.clone()}
                        />
                    },
                    false => html!{
                        <ImgShowcase
                            object_uuid={self.current_component_uuid.clone()}
                            file_arr={self.file_arr.clone()}
                        />
                    },
                }}
              <div class="column">
                <div class="has-text-weight-bold is-size-4">{
                    component_data.name.clone()
                }</div>
                {self.show_component_info(component_data)}
                <div class="buttons flexBox">
                    {self.show_three_btn()}
                    <ModificationFilesetsCard
                        modification_uuid={self.select_modification_uuid.clone()}
                        callback_select_fileset_uuid={callback_select_fileset_uuid}
                        callback_open_fileset={callback_open_fileset} />
                    {self.show_discussion_btn()}
                    {self.show_setting_btn()}
                    {self.show_followers_btn()}
                    <ShareLinkBtn />
                    {match component_data.licenses.is_empty() {
                        true => html!{},
                        false => self.show_component_licenses(component_data),
                    }}
                </div>
                {self.show_component_tabs(component_data)}
              </div>
            </div>
            {match self.show_full_description && component_data.description.len() > 249 {
                true => html!{
                    <div class="content component-description">
                        {component_data.description.to_markdown()}
                        {ft_see_btn(show_description_btn, self.show_full_description)}
                    </div>
                },
                false => html!{},
            }}
        </>}
    }

    fn show_component_licenses(&self, component_data: &ComponentInfo) -> Html {
        html!{<div class="media">
            <div class="media-right">
                <span style="" class="icon is-small">
                    <i class="fa fa-balance-scale"></i>
                </span>
            </div>
            <div class="media-content">
                <div>
                    {for component_data.licenses.iter().map(|data| html!{
                        // format!("{}; ", data.name)
                        <ComponentLicenseTag
                            show_delete_btn={false}
                            component_uuid={self.current_component_uuid.clone()}
                            license_data={data.clone()}
                            delete_license={None}
                          />
                    })}
                </div>
            </div>
        </div>}
    }

    fn show_modifications_table(&self, modifications_count: i64) -> Html {
        let onclick_select_modification = self.link.callback(|value: UUID| Msg::SelectModification(value));
        html!{
            <ModificationsTableCard
                component_uuid={self.current_component_uuid.clone()}
                modifications_count={modifications_count}
                callback_select_modification={onclick_select_modification.clone()}
                user_owner={self.current_user_owner}
              />
        }
    }

    fn show_component_info(&self, component_data: &ComponentInfo) -> Html {
        html!{
            <div class="columns is-mobile is-multiline">
                <div class="column">
                    {get_value_field(&159)}{": "}
                    {component_data.actual_status.name.clone()}
                </div>
                <div class="column">{component_data.type_access.get_with_icon()}</div>
                <div class="column is-narrow" title={get_value_field(&141)}>
                    <span class="icon is-small">
                        <i class={classes!("fa", "fa-user")}></i>
                    </span>
                    {" "}
                    <GoToUser data = {component_data.owner_user.clone()} />
                </div>
                <div class="column is-narrow" title={get_value_field(&95)}>
                    <span class="icon is-small">
                        <i class={classes!("fa", "fa-edit")}></i>
                    </span>
                    {" "}
                    <span class="id-box">
                        {component_data.updated_at.date_to_display()}
                    </span>
                </div>
            </div>
        }
    }

    fn show_component_tabs(&self, component_data: &ComponentInfo) -> Html {
        let onclick_tab_description = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::Description));
        let onclick_tab_characteristics = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::Characteristics));
        let onclick_tab_component_files = self.link.callback(|_| Msg::ChangeActiveTab(ActiveTab::ComponentFiles));
        let at = match self.active_tab {
            ActiveTab::Description => ("is-active","",""),
            ActiveTab::Characteristics => ("","is-active",""),
            ActiveTab::ComponentFiles => ("","","is-active"),
        };
        let show_description_btn = self.link.callback(|_| Msg::ShowDescription);
        html!{<>
            <div class="tabs mb-1">
                <ul>
                    <li class={at.0} onclick={onclick_tab_description}><a>{get_value_field(&61)}</a></li>
                    <li class={at.1} onclick={onclick_tab_characteristics}><a>{get_value_field(&101)}</a></li>
                    <li class={at.2} onclick={onclick_tab_component_files}><a>{get_value_field(&102)}</a></li>
                </ul>
            </div>
            <div class="card-content p-0">
                <div class="content">
                    {match self.active_tab {
                        ActiveTab::Description => html!{
                            <div class="component-description">
                                {match component_data.description.len() {
                                    250.. => match self.show_full_description {
                                        true => html!{},
                                        false => html!{<>
                                            {format!("{:.*}", 200, component_data.description).to_markdown()}
                                            {ft_see_btn(show_description_btn.clone(), self.show_full_description)}
                                        </>},
                                    },
                                    _ => component_data.description.to_markdown(),
                                }}
                            </div>
                        },
                        ActiveTab::Characteristics => html!{
                            <ComponentParamsTags
                                show_manage_btn={false}
                                component_uuid={self.current_component_uuid.clone()}
                                params_count={component_data.params_count}
                                />
                        },
                        ActiveTab::ComponentFiles => html!{
                            <ComponentFilesBlock
                                show_download_btn={true}
                                show_delete_btn={false}
                                component_uuid={component_data.uuid.clone()}
                                files_count={component_data.files_count}
                                />
                        },
                    }}
                </div>
            </div>
        </>}
    }

    fn show_component_suppliers(&self, component_data: &ComponentInfo) -> Html {
        let table_label = match component_data.is_base {
            true => get_value_field(&107).to_string(),
            false => get_value_field(&108).to_string(),
        };
        let classes_table = get_classes_table(component_data.component_suppliers.len());
        html!{
            <div class={"card"}>
                <header class={"card-header"}>
                    <p class={"card-header-title"}>{table_label}</p>
                </header>
                <div class={"card-content"}>
                    <div class={"content"}>
                        <table class={classes_table}>
                            <thead>
                            <tr>
                                <th>{get_value_field(&109)}</th> // Company
                                <th>{get_value_field(&61)}</th> // Description
                                <th>{get_value_field(&111)}</th> // Action
                            </tr>
                            </thead>
                            <tbody>
                                {match component_data.is_base {
                                    true => html!{<>
                                        {for component_data.component_suppliers.iter().map(|data| {
                                            match &data.supplier.is_supplier {
                                                true => html!{<ComponentSupplierItem
                                                    show_delete_btn={false}
                                                    component_uuid={component_data.uuid.clone()}
                                                    supplier_data={data.clone()}
                                                    delete_supplier={None}
                                                />},
                                                false => html!{},
                                            }})}
                                    </>},
                                    false => match component_data.component_suppliers.first() {
                                        Some(data) => html!{<ComponentSupplierItem
                                            show_delete_btn={false}
                                            component_uuid={component_data.uuid.clone()}
                                            supplier_data={data.clone()}
                                            delete_supplier={None}
                                        />},
                                        None => html!{},
                                    },
                                }}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn show_component_standards(&self, component_data: &ComponentInfo) -> Html {
        let classes_table = get_classes_table(component_data.component_standards.len());
        html!{
            <div class={"card"}>
                <header class={"card-header"}>
                    <p class={"card-header-title"}>{get_value_field(&103)}</p> // Standards
                </header>
                <div class={"card-content"}>
                    <div class={"content"}>
                        <table class={classes_table}>
                            <thead>
                            <tr>
                                <th>{get_value_field(&110)}</th> // Name
                                <th>{get_value_field(&111)}</th> // Action
                            </tr>
                            </thead>
                            <tbody>
                            {for component_data.component_standards.iter().map(|data| {
                                html!{<ComponentStandardItem
                                    show_delete_btn={false}
                                    component_uuid={self.current_component_uuid.clone()}
                                    standard_data={data.clone()}
                                    delete_standard={None}
                                    />}
                            })}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn show_setting_btn(&self) -> Html {
        let onclick_setting_component_btn = self.link.callback(|_| Msg::OpenComponentSetting);
        match &self.current_user_owner {
            true => {res_btn(
                classes!("fa", "fa-tools"),
                onclick_setting_component_btn,
                String::new(),
                get_value_field(&16),
                Pathname::ComponentSetting(self.current_component_uuid.clone())
            )},
            false => html!{},
        }
    }

    fn show_followers_btn(&self) -> Html {
        let onclick_following = match self.is_followed {
            true => self.link.callback(|_| Msg::UnFollow),
            false => self.link.callback(|_| Msg::Follow),
        };

        ft_follow_btn(
            onclick_following,
            self.is_followed,
            self.abbr_number(),
        )
    }

    fn show_three_btn(&self) -> Html {
        let onclick_three_viewer = self.link.callback(|_| Msg::Show3D);
        let mut class_btn = classes!("button");
        let show_btn = match self.show_three_view {
            true => {
                class_btn.push("is-focused");
                get_value_field(&301)
            },
            false => get_value_field(&300),
        };

        html!{<>
            <button
            id="three-button"
            class={class_btn}
            onclick={onclick_three_viewer}
            title={get_value_field(&325)}>
              <span class="icon is-small">
                <i class={classes!("fa", "fa-cube")} style="color: #1872f0;"></i>
              </span>
              <span>{show_btn}</span>
            </button>
        </>}
    }

    fn show_discussion_btn(&self) -> Html {
        let onclick_open_discussion_btn =
            self.link.callback(|_| Msg::OpenDiscussionBlock);
        let class_fileset_btn = match self.open_discussion_card {
            true => "button is-light is-info is-active",
            false => "button is-info",
        };
        html!{
            <button
            class={class_fileset_btn}
            onclick={onclick_open_discussion_btn}
            title={get_value_field(&106)}>
                <span class={"icon is-small"}><i class={"far fa-comments"}></i></span>
                <span>{get_value_field(&380)}</span>
            </button>
        }
    }

    fn show_component_discussion(&self) -> Html {
        match self.open_discussion_card {
            true => html!{<>
                <div class="card">
                    <header class="card-header has-background-info-light">
                        <p class="card-header-title">{get_value_field(&380)}</p>
                    </header>
                    <div class="card-content">
                        <DiscussionCommentsBlock
                            discussion_uuid={None}
                            object_type={ObjectType::new(self.current_component_uuid.clone(), ToObject::COMPONENT)}
                        />
                    </div>
                </div>
                <br/>
            </>},
            false => html!{},
        }
    }
}
