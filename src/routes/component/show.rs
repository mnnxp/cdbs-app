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

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    switch_icon::res_btn,
    list_errors::ListErrors,
    user::ListItemUser,
    component::{
        ComponentStandardItem, ComponentSupplierItem, ComponentLicenseTag, ComponentParamTag,
        ModificationsTable, FilesOfFilesetCard, ManageFilesOfFilesetBlock,
        ComponentFilesBlock, ModificationFilesTableCard, SpecsTags, KeywordsTags,
    },
    img_showcase::ImgShowcase,
    three_showcase::ThreeShowcase,
};
use crate::services::{Counter, get_logged_user, get_value_field, resp_parsing};
use crate::types::{UUID, ComponentInfo, SlimUser, ComponentParam, ComponentModificationInfo, DownloadFile};
use crate::gqls::make_query;
use crate::gqls::component::{
    ComponentFiles, component_files,
    GetComponentData, get_component_data,
    AddComponentFav, add_component_fav,
    DeleteComponentFav, delete_component_fav,
};

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
    show_full_characteristics: bool,
    open_owner_user_info: bool,
    open_modification_card: bool,
    open_modification_files_card: bool,
    open_fileset_files_card: bool,
    show_related_standards: bool,
    file_arr: Vec<DownloadFile>,
    show_three_view: bool,
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

#[derive(Clone)]
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
    ShowFullCharacteristics,
    ShowStandardsList,
    ShowOwnerUserCard,
    ShowModificationCard,
    ShowModificationFilesList,
    ShowFilesetFilesBlock(bool),
    OpenComponentSetting,
    GetDownloadFileResult(String),
    Show3D,
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
            show_full_characteristics: false,
            open_owner_user_info: false,
            open_modification_card: false,
            open_modification_files_card: false,
            open_fileset_files_card: false,
            show_related_standards: false,
            file_arr: Vec::new(),
            show_three_view: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if let None = get_logged_user() {
            // route to login page if not found token
            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
        };

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

        // debug!("get_self {:?}", get_self);

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
                  limit: None,
                  offset: None,
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
            Msg::SelectModification(modification_uuid) => {
                match self.select_modification_uuid == modification_uuid {
                    true => link.send_message(Msg::ShowModificationCard),
                    false => {
                        self.select_modification_uuid = modification_uuid;
                        self.current_filesets_program.clear();
                        self.current_filesets_program = self.modification_filesets
                            .get(&self.select_modification_uuid)
                            .map(|f| f.clone())
                            .unwrap_or_default();
                    },
                }
            },
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
                        self.show_full_characteristics = component_data.component_params.len() < 4;
                        self.select_modification_uuid = component_data.component_modifications
                            .first()
                            .map(|m| m.uuid.clone())
                            .unwrap_or_default();
                        self.select_fileset_uuid = component_data.component_modifications
                                .first()
                                .map(|m| m.filesets_for_program.first().map(|f| f.uuid.clone())
                                .unwrap_or_default()
                            ).unwrap_or_default();
                        for component_modification in &component_data.component_modifications {
                            let mut fileset_data: Vec<(UUID, String)> = Vec::new();
                            for fileset in &component_modification.filesets_for_program {
                                fileset_data.push((fileset.uuid.clone(), fileset.program.name.clone()));
                            }
                            self.modification_filesets.insert(
                                component_modification.uuid.clone(),
                                fileset_data.clone()
                            );
                        }
                        self.current_filesets_program = self.modification_filesets
                            .get(&self.select_modification_uuid)
                            .map(|f| f.clone())
                            .unwrap_or_default();
                        self.component = Some(component_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
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
                            Some(main_img) => {
                                result.push(main_img.clone());
                                self.file_arr = result;
                            },
                            None => self.file_arr = result,
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ShowDescription => self.show_full_description = !self.show_full_description,
            Msg::ShowFullCharacteristics => self.show_full_characteristics = !self.show_full_characteristics,
            Msg::ShowStandardsList => self.show_related_standards = !self.show_related_standards,
            Msg::ShowOwnerUserCard => self.open_owner_user_info = !self.open_owner_user_info,
            Msg::ShowModificationCard => self.open_modification_card = !self.open_modification_card,
            Msg::ShowModificationFilesList => self.open_modification_files_card = !self.open_modification_files_card,
            Msg::ShowFilesetFilesBlock(value) => self.open_fileset_files_card = value,
            Msg::OpenComponentSetting => {
                if let Some(component_data) = &self.component {
                    // Redirect to page for change and update component
                    self.router_agent.send(ChangeRoute(AppRoute::ComponentSettings(
                        component_data.uuid.clone()
                    ).into()));
                }
            },
            Msg::Show3D => {
                // self.show_three_view = val;
                self.show_three_view = !self.show_three_view;
            },
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
                    <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
                    <div class="container page">
                        <div class="row">
                            // modals cards
                            {self.show_modal_owner_user(component_data)}
                            {match self.open_modification_card {
                                true => self.show_modal_modification_card(component_data),
                                false => html!{},
                            }}

                            <div class="card column">
                              {self.show_main_card(component_data)}
                            </div>
                            {self.show_fileset_files_card()}
                            <br/>
                            {self.show_modifications_table(component_data)}
                            {self.show_modification_files()}
                            <br/>
                            {self.show_cards(component_data)}
                            {self.show_component_specs(component_data)}
                            <br/>
                            {self.show_component_keywords(component_data)}
                        </div>
                    </div>
                </div>
            },
            None => html!{<div>
                <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
                // <h1>{"Not data"}</h1>
            </div>},
        }
    }
}

impl ShowComponent {
    fn show_main_card(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        let onclick_open_owner_company =
            self.link.callback(|_| Msg::ShowOwnerUserCard);
        let show_description_btn =
            self.link.callback(|_| Msg::ShowDescription);

        html!{
            <div class="columns">
                {match self.show_three_view {
                    true => html!{<>
                        <ThreeShowcase
                            fileset_uuid=self.select_fileset_uuid.clone()
                        />
                    </>},
                    false => html!{<>
                        <ImgShowcase
                            object_uuid=self.current_component_uuid.clone()
                            file_arr=self.file_arr.clone()
                        />
                    </>},
                }}
                // <button onclick={onclick_three_viewer} class={class_three_btn}>{show_btn}</button>
                // {res_btn(classes!("fa", "fa-cube"), onclick_three_viewer, show_btn)}
                // <img class="imgBox" src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />

              <div class="column">
                <div class="media">
                    <div class="media-content">
                        { get_value_field(&94) }
                        <a class="id-box has-text-grey-light has-text-weight-bold"
                            onclick={onclick_open_owner_company} >
                          {format!("@{}",&component_data.owner_user.username)}
                        </a>
                    </div>
                    <div class="media-right" style="margin-right: 1rem">
                        { get_value_field(&95) }<span class="id-box has-text-grey-light has-text-weight-bold">
                            {format!("{:.*}", 10, component_data.updated_at.to_string())}
                        </span>
                    </div>
                </div>
                // <h1>{"Component"}</h1>
                <div class="has-text-weight-bold is-size-4">{
                    component_data.name.clone()
                }</div>
                <div class="buttons flexBox">
                    {self.show_three_btn()}
                    {self.show_download_block()}
                    {self.show_setting_btn()}
                    {self.show_followers_btn()}
                    // {self.show_share_btn()}
                    {match component_data.licenses.is_empty() {
                        true => html!{},
                        false => self.show_component_licenses(component_data),
                    }}
                </div>
                {self.show_component_params(component_data)}
                <div class="component-description">{
                    match self.show_full_description {
                        true => html!{<>
                          {component_data.description.clone()}
                          {match component_data.description.len() {
                              250.. => html!{<>
                                <br/>
                                <button class="button is-white"
                                    onclick=show_description_btn >
                                  { get_value_field(&99) }
                                </button>
                              </>},
                              _ => html!{},
                          }}
                        </>},
                        false => html!{<>
                          {format!("{:.*}", 200, component_data.description)}
                          <br/>
                          <button class="button is-white"
                              onclick=show_description_btn >
                            { get_value_field(&98) }
                          </button>
                        </>},
                    }
                }</div>
              </div>
            </div>
        }
    }

    fn show_component_licenses(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
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
                            show_delete_btn = false
                            component_uuid = self.current_component_uuid.clone()
                            license_data = data.clone()
                            delete_license = None
                          />
                    })}
                </div>
            </div>
        </div>}
    }

    fn show_modifications_table(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        let onclick_select_modification = self.link
            .callback(|value: UUID| Msg::SelectModification(value));

        let callback_open_modification_uuid = self.link
            .callback(|_| Msg::ShowModificationFilesList);

        html!{<>
            <h2 class="has-text-weight-bold">{ get_value_field(&100) }</h2> // Modifications
            <ModificationsTable
                modifications = component_data.component_modifications.clone()
                select_modification = self.select_modification_uuid.clone()
                open_modification_files = self.open_modification_files_card
                callback_select_modification = onclick_select_modification.clone()
                callback_open_modification_files = callback_open_modification_uuid.clone()
              />
        </>}
    }

    fn show_modification_files(&self) -> Html {
        match self.open_modification_files_card {
            true => html!{<>
                <br/>
                <ModificationFilesTableCard
                    show_download_btn = true
                    modification_uuid = self.select_modification_uuid.clone()
                  />
            </>},
            false => html!{},
        }
    }

    fn show_component_params(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{
            <div class="columns">
                <div class="column">
                    <label class="label">{ get_value_field(&96) }</label>
                    {component_data.actual_status.name.clone()}
                </div>
                <div class="column">
                    <label class="label">{ get_value_field(&97) }</label>
                    {component_data.component_type.component_type.clone()}
                </div>
                <div class="column">
                    <label class="label">{ get_value_field(&114) }</label>
                    {component_data.type_access.name.clone()}
                </div>
            </div>
        }
    }

    fn show_additional_params(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2 class="has-text-weight-bold">{ get_value_field(&101) }</h2> // Сharacteristics of the component
              <div class="card column">
                <table class="table is-fullwidth">
                    <tbody>
                      {for component_data.component_params.iter().enumerate().map(|(index, data)| {
                          match (index >= 3, self.show_full_characteristics) {
                              // show full list
                              (_, true) => self.show_param_item(data),
                              // show full list or first 3 items
                              (false, false) => self.show_param_item(data),
                              _ => html!{},
                          }
                      })}
                      {match component_data.component_params.len() {
                          0 => html!{<span>{ get_value_field(&136) }</span>},
                          0..=3 => html!{},
                          _ => self.show_see_characteristic_btn(),
                      }}
                    </tbody>
                  </table>
              </div>
            </div>
        }
    }

    fn show_param_item(
        &self,
        data: &ComponentParam,
    ) -> Html {
        html!{<ComponentParamTag
            show_manage_btn = false
            component_uuid = self.props.component_uuid.clone()
            param_data = data.clone()
            delete_param = None
          />}
    }

    fn show_see_characteristic_btn(&self) -> Html {
        let show_full_characteristics_btn = self.link
            .callback(|_| Msg::ShowFullCharacteristics);

        match self.show_full_characteristics {
            true => html!{<>
              <button class="button is-white"
                  onclick=show_full_characteristics_btn
                >{ get_value_field(&99) }</button>
            </>},
            false => html!{<>
              <button class="button is-white"
                  onclick=show_full_characteristics_btn
                >{ get_value_field(&98) }</button>
            </>},
        }
    }

    fn show_cards(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<>
            <div class="columns">
                {self.show_additional_params(component_data)}
                <div class="column">
                    <h2 class="has-text-weight-bold">{ get_value_field(&102) }</h2> // Component files
                    {self.show_component_files(component_data)}
                </div>
            </div>
            <div class="columns">
                <div class="column">
                    <h2 class="has-text-weight-bold">{ get_value_field(&103) }</h2> // Standards
                    {self.show_component_standards(component_data)}
                </div>
                <div class="column">
                    {self.show_component_suppliers(component_data)}
                </div>
            </div>
        </>}
    }

    fn show_component_files(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<div id="files" class="card">
            <ComponentFilesBlock
                  show_download_btn = true
                  show_delete_btn = false
                  component_uuid = component_data.uuid.clone()
                  files = component_data.files.clone()
                />
        </div>}
    }

    fn show_component_specs(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<>
              <h2 class="has-text-weight-bold">{ get_value_field(&104) }</h2> // Specs
              <div class="card column">
                <SpecsTags
                    show_manage_btn = false
                    component_uuid = component_data.uuid.clone()
                    specs = component_data.component_specs.clone()
                  />
              </div>
        </>}
    }

    fn show_component_keywords(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<>
              <h2 class="has-text-weight-bold">{ get_value_field(&105) }</h2> // Keywords
              <div class="card column">
                <KeywordsTags
                    show_delete_btn = false
                    component_uuid = component_data.uuid.clone()
                    keywords = component_data.component_keywords.clone()
                  />
              </div>
        </>}
    }

    fn show_component_suppliers(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        let table_label = match component_data.is_base {
            true => get_value_field(&107).to_string(),
            false => get_value_field(&108).to_string(),
        };

        html!{<>
            <h2 class="has-text-weight-bold">{table_label}</h2>
            <div class="card column">
              <table class="table is-fullwidth">
                <tbody>
                   <th>{ get_value_field(&109) }</th> // Company
                   <th>{ get_value_field(&61) }</th> // Description
                   <th>{ get_value_field(&111) }</th> // Action
                   {match component_data.is_base {
                       true => html!{<>
                           {for component_data.component_suppliers.iter().map(|data| {
                             match &data.supplier.is_supplier {
                                true => html!{<ComponentSupplierItem
                                    show_delete_btn = false
                                    component_uuid = component_data.uuid.clone()
                                    supplier_data = data.clone()
                                    delete_supplier = None
                                />},
                                false => html!{},
                            }})}
                       </>},
                       false => match component_data.component_suppliers.first() {
                           Some(data) => html!{<ComponentSupplierItem
                               show_delete_btn = false
                               component_uuid = component_data.uuid.clone()
                               supplier_data = data.clone()
                               delete_supplier = None
                           />},
                           None => html!{},
                       },
                   }}
                </tbody>
              </table>
            </div>
        </>}
    }

    fn show_component_standards(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<div class="card column">
          <table class="table is-fullwidth">
            <tbody>
               <th>{ get_value_field(&112) }</th> // Classifier
               <th>{ get_value_field(&113) }</th> // Specified tolerance
               <th>{ get_value_field(&111) }</th> // Action
               {for component_data.component_standards.iter().map(|data| {
                   html!{<ComponentStandardItem
                       show_delete_btn = false
                       component_uuid = self.current_component_uuid.clone()
                       standard_data = data.clone()
                       delete_standard = None
                     />}
               })}
            </tbody>
          </table>
        </div>}
    }

    fn show_modal_owner_user(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        let onclick_owner_user_info = self.link
            .callback(|_| Msg::ShowOwnerUserCard);

        let class_modal = match &self.open_owner_user_info {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class=class_modal>
          <div class="modal-background" onclick=onclick_owner_user_info.clone() />
          <div class="modal-content">
              <div class="card">
                <ListItemUser
                    data = component_data.owner_user.clone()
                    show_list = true
                  />
              </div>
          </div>
          <button class="modal-close is-large" aria-label="close" onclick=onclick_owner_user_info />
        </div>}
    }

    fn show_modal_modification_card(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        let onclick_modification_card = self.link
            .callback(|_| Msg::ShowModificationCard);

        let class_modal = match &self.open_modification_card {
            true => "modal is-active",
            false => "modal",
        };

        let modification_data: Option<&ComponentModificationInfo> = component_data.component_modifications.iter()
            .find(|x| x.uuid == self.select_modification_uuid);

        match modification_data {
            Some(mod_data) => html!{<div class=class_modal>
              <div class="modal-background" onclick=onclick_modification_card.clone() />
              <div class="modal-content">
                  <div class="card">
                    <div class="box itemBox">
                      <article class="media center-media">
                          <div class="media-content">
                            <div class="columns" style="margin-bottom:0">
                                <div class="column">
                                    <p class="overflow-title">{ get_value_field(&116) }</p> // Modification
                                    <div class="has-text-weight-bold">
                                        {mod_data.modification_name.clone()}
                                    </div>
                                    <p class="overflow-title">{ get_value_field(&61) }</p> // Description
                                    <p>{mod_data.description.clone()}</p>
                                </div>
                            </div>
                            <div class="columns" style="margin-bottom:0">
                                <div class="column">
                                  {format!("{}: {}", get_value_field(&96), &mod_data.actual_status.name)}
                                </div>
                                <div class="column">
                                  {format!("{} {:.*}", get_value_field(&30), 10, mod_data.updated_at.to_string())}
                                </div>
                            </div>
                          </div>
                      </article>
                    </div>
                  </div>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick=onclick_modification_card />
            </div>},
            None => html!{},
        }
    }

    fn show_fileset_files_card(&self) -> Html {
        match &self.open_fileset_files_card {
            true => html!{<>
                <br/>
                <h2 class="has-text-weight-bold">{ get_value_field(&106) }</h2> // Files of select fileset
                <FilesOfFilesetCard
                    show_download_btn = false
                    select_fileset_uuid = self.select_fileset_uuid.clone()
                />
            </>},
            false => html!{},
        }
    }

    fn show_download_block(&self) -> Html {
        let callback_select_fileset_uuid =
            self.link.callback(|value: UUID| Msg::SelectFileset(value));

        let callback_open_fileset_uuid =
            self.link.callback(|value: bool| Msg::ShowFilesetFilesBlock(value));

        html!{
            <ManageFilesOfFilesetBlock
                select_modification_uuid = self.select_modification_uuid.clone()
                current_filesets_program = self.current_filesets_program.clone()
                callback_select_fileset_uuid = callback_select_fileset_uuid.clone()
                callback_open_fileset_uuid = callback_open_fileset_uuid.clone()
            />
        }
    }

    fn show_setting_btn(&self) -> Html {
        let onclick_setting_standard_btn =
            self.link.callback(|_| Msg::OpenComponentSetting);

        match &self.current_user_owner {
            true => {res_btn(
                classes!("fa", "fa-tools"),
                onclick_setting_standard_btn,
                String::new())},
            false => html!{},
        }
    }

    fn show_followers_btn(&self) -> Html {
        let (class_fav, onclick_following) = match self.is_followed {
            true => ("fas fa-bookmark", self.link.callback(|_| Msg::UnFollow)),
            false => ("far fa-bookmark", self.link.callback(|_| Msg::Follow)),
        };

        html!{<>
            <button
                id="following-button"
                class="button"
                onclick=onclick_following >
              <span class="icon is-small">
                <i class={class_fav}></i>
              </span>
              <span>{self.abbr_number()}</span>
            </button>
        </>}
    }

    fn show_three_btn(&self) -> Html {
        let onclick_three_viewer =
            self.link.callback(|_| Msg::Show3D);

        let mut class_btn = classes!("button");
        let show_btn = match self.show_three_view {
            true => {
                class_btn.push("is-focused");
                get_value_field(&301)
            },
            false => get_value_field(&300),
        };
        // <button onclick={onclick_three_viewer} class={class_three_btn}>{show_btn}</button>

        html!{<>
            <button
                id="three-button"
                class={class_btn}
                onclick={onclick_three_viewer} >
              <span class="icon is-small">
                <i class={classes!("fa", "fa-cube")} style="color: #1872f0;"></i>
              </span>
              <span>{show_btn}</span>
            </button>
        </>}
    }

    // fn show_share_btn(&self) -> Html {
    //     html!{
    //         <div class="media-right flexBox" >
    //           <button
    //               id="share-button"
    //               class="button" >
    //             <span class="icon is-small">
    //               <i class="fas fa-share-alt"></i>
    //             </span>
    //           </button>
    //         </div>
    //     }
    // }
}
