use std::collections::HashMap;
use chrono::NaiveDateTime;
use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::{
    service::RouteService,
    // agent::RouteRequest::ChangeRoute,
    // prelude::*,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

// use crate::routes::AppRoute;
use crate::error::{get_error, Error};
use crate::fragments::{
    switch_icon::res_btn,
    list_errors::ListErrors,
    catalog_user::ListItemUser,
    component::{ComponentStandardItem, ComponentCompanyItem},
    component_file::FilesCard,
    component_modification::{ModificationsTable, FilesOfFilesetCard},
    component_spec::SpecsTags,
    component_keyword::KeywordsTags,
};
use crate::gqls::make_query;
use crate::services::is_authenticated;
use crate::types::{
    UUID, ComponentInfo, SlimUser,
    DownloadFile, ComponentModificationInfo,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetComponentData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct AddComponentFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct DeleteComponentFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct ComModFilesetFiles;

/// Component with relate data
pub struct ShowComponent {
    error: Option<Error>,
    component: Option<ComponentInfo>,
    current_component_uuid: UUID,
    current_user_owner: bool,
    // task: Option<FetchTask>,
    // router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    select_component_modification: UUID,
    modification_filesets: HashMap<UUID, Vec<(UUID, String)>>,
    select_fileset_program: UUID,
    current_filesets_program: Vec<(UUID, String)>,
    show_full_description: bool,
    show_full_characteristic: bool,
    open_owner_user_info: bool,
    open_modification_card: bool,
    open_fileset_files_card: bool,
    open_standard_info: bool,
    show_related_standards: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub component_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    RequestDownloadFiles,
    SelectFileset(UUID),
    SelectModification(UUID),
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    ResponseError(Error),
    GetDownloadFilesResult(String),
    GetComponentData(String),
    ShowDescription,
    ShowCharacteristic,
    ShowStandardsList,
    ShowOwnerUserCard,
    ShowStandardCard,
    ShowModificationCard,
    ShowFilesetFilesList,
    OpenComponentSetting,
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
            // task: None,
            // router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            select_component_modification: String::new(),
            modification_filesets: HashMap::new(),
            select_fileset_program: String::new(),
            current_filesets_program: Vec::new(),
            show_full_description: false,
            show_full_characteristic: false,
            open_owner_user_info: false,
            open_modification_card: false,
            open_fileset_files_card: false,
            open_standard_info: false,
            show_related_standards: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // get component uuid for request component data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_component_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/component/")
            .to_string();
        // get flag changing current component in route
        let not_matches_component_uuid = target_component_uuid != self.current_component_uuid;
        // debug!("self.current_component_uuid {:#?}", self.current_component_uuid);

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_component_uuid) && is_authenticated() {
            self.error = None;
            self.component = None;
            // update current_component_uuid for checking change component in route
            self.current_component_uuid = target_component_uuid.to_string();
            self.current_user_owner = false;
            self.select_component_modification = String::new();
            self.modification_filesets = HashMap::new();
            self.select_fileset_program = String::new();
            self.current_filesets_program = Vec::new();

            spawn_local(async move {
                let res = make_query(GetComponentData::build_query(get_component_data::Variables {
                    component_uuid: target_component_uuid,
                })).await.unwrap();

                link.send_message(Msg::GetComponentData(res.clone()));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFiles => {
                debug!("Select modification: {:?}", self.select_component_modification);
                match self.select_fileset_program.len() {
                    36 => {
                        debug!("Select fileset: {:?}", self.select_fileset_program);
                        let ipt_file_of_fileset_arg = com_mod_fileset_files::IptFileOfFilesetArg{
                            filesetUuid: self.select_fileset_program.clone(),
                            fileUuids: None,
                            limit: None,
                            offset: None,
                        };
                        spawn_local(async move {
                            let res = make_query(ComModFilesetFiles::build_query(com_mod_fileset_files::Variables {
                                ipt_file_of_fileset_arg
                            })).await.unwrap();

                            link.send_message(Msg::GetDownloadFilesResult(res.clone()));
                        })
                    },
                    _ => debug!("Bad select fileset: {:?}", self.select_fileset_program),
                }
            },
            Msg::SelectFileset(fileset_uuid) => self.select_fileset_program = fileset_uuid,
            Msg::SelectModification(modification_uuid) => {
                match self.select_component_modification == modification_uuid {
                    true => link.send_message(Msg::ShowModificationCard),
                    false => {
                        self.current_filesets_program = self.modification_filesets.get(&modification_uuid)
                            .map(|f| f.clone()).unwrap_or_default();
                        self.select_component_modification = modification_uuid;
                        self.select_fileset_program = self.current_filesets_program.first()
                            .map(|(fileset_uuid, program_name)| {
                                debug!("mod fileset_uuid: {:?}", fileset_uuid);
                                debug!("mod program_name: {:?}", program_name);
                                fileset_uuid.clone()
                            }).unwrap_or_default();
                    },
                }
            },
            Msg::Follow => {
                let component_uuid_string = self.component.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(AddComponentFav::build_query(add_component_fav::Variables {
                        component_uuid: component_uuid_string,
                    })).await.unwrap();

                    link.send_message(Msg::AddFollow(res.clone()));
                })
            },
            Msg::AddFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("addComponentFav").unwrap().clone())
                                .unwrap();

                        if result {
                            self.subscribers += 1;
                            self.is_followed = true;
                        }
                    }
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UnFollow => {
                let component_uuid = self.component.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(DeleteComponentFav::build_query(delete_component_fav::Variables {
                        component_uuid,
                    }))
                    .await
                    .unwrap();

                    link.send_message(Msg::DelFollow(res.clone()));
                })
            },
            Msg::DelFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("deleteComponentFav").unwrap().clone())
                                .unwrap();

                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    }
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDownloadFilesResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: Vec<DownloadFile> = serde_json::from_value(res.get("componentModificationFilesetFiles").unwrap().clone()).unwrap();
                        debug!("componentModificationFilesetFiles: {:?}", result);
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    },
                }
            },
            Msg::GetComponentData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let component_data: ComponentInfo =
                            serde_json::from_value(res_value.get("component").unwrap().clone()).unwrap();
                        debug!("Component data: {:?}", component_data);

                        self.subscribers = component_data.subscribers;
                        self.is_followed = component_data.is_followed;
                        self.current_component_uuid = component_data.uuid.clone();
                        if let Some(user) = &self.props.current_user {
                            self.current_user_owner = component_data.owner_user.uuid == user.uuid;
                        }
                        // length check for show btn more/less
                        self.show_full_description = component_data.description.len() < 250;
                        self.show_full_characteristic = component_data.component_params.len() < 4;
                        self.select_component_modification = component_data.component_modifications.first()
                            .map(|m| m.uuid.clone()).unwrap_or_default();
                        self.select_fileset_program = component_data.component_modifications.first().map(|m|
                            m.filesets_for_program.first().map(|f| f.uuid.clone()).unwrap_or_default()
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
                            .get(&self.select_component_modification)
                            .map(|f| f.clone()).unwrap_or_default();

                        self.component = Some(component_data);
                    }
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::ShowDescription => self.show_full_description = !self.show_full_description,
            Msg::ShowCharacteristic => self.show_full_characteristic = !self.show_full_characteristic,
            Msg::ShowStandardsList => self.show_related_standards = !self.show_related_standards,
            Msg::ShowOwnerUserCard => self.open_owner_user_info = !self.open_owner_user_info,
            Msg::ShowStandardCard => self.open_standard_info = !self.open_standard_info,
            Msg::ShowModificationCard => self.open_modification_card = !self.open_modification_card,
            Msg::ShowFilesetFilesList => self.open_fileset_files_card = !self.open_fileset_files_card,
            Msg::OpenComponentSetting => {
                // if let Some(component_data) = &self.component {
                //     // Redirect to page for change and update component
                //     self.router_agent.send(ChangeRoute(AppRoute::ComponentSettings(
                //         component_data.uuid.clone()
                //     ).into()));
                // }
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        match &self.component {
            Some(component_data) => html! {
                <div class="component-page">
                    <ListErrors error=self.error.clone()/>
                    <div class="container page">
                        <div class="row">
                            // modals cards
                            {self.show_modal_owner_user(component_data)}
                            {match self.open_modification_card {
                                true => self.show_modal_modification_card(component_data),
                                false => html!{},
                            }}

                            <div class="card">
                              {self.show_main_card(component_data)}
                            </div>
                            <br/>
                            {self.show_fileset_files_card()}
                            <br/>
                            {self.show_modifications_table(component_data)}
                            <br/>
                            {self.show_cards(component_data)}
                            {self.show_component_specs(component_data)}
                            <br/>
                            {self.show_component_keywords(component_data)}
                        </div>
                    </div>
                </div>
            },
            None => html! {<div>
                <ListErrors error=self.error.clone()/>
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
        let onclick_open_owner_company = self.link
            .callback(|_| Msg::ShowOwnerUserCard);

        let show_description_btn = self.link
            .callback(|_| Msg::ShowDescription);

        html!{
            <div class="columns">
              <div class="column is-one-quarter">
                <img class="imgBox" src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              </div>
              <div class="column">
                <div class="media">
                    <div class="media-content">
                        {"uploaded from "}
                        <a class="id-box has-text-grey-light has-text-weight-bold"
                            onclick={onclick_open_owner_company} >
                          {format!("@{}",&component_data.owner_user.username)}
                        </a>
                    </div>
                    <div class="media-right" style="margin-right: 1rem">
                        {"type access "}<span class="id-box has-text-grey-light has-text-weight-bold">{
                            component_data.type_access.name.clone()
                        }</span>
                    </div>
                </div>
                // <h1>{"Component"}</h1>
                <div class="has-text-weight-bold is-size-4">{
                    component_data.name.clone()
                }</div>
                <div class="buttons flexBox">
                    {self.show_download_block()}
                    {self.show_setting_btn()}
                    {self.show_followers_btn()}
                    {self.show_share_btn()}
                </div>
                <div class="component-description">{
                    match self.show_full_description {
                        true => html!{<>
                          {component_data.description.clone()}
                          {match component_data.description.len() {
                              250.. => html!{<>
                                <br/>
                                <button class="button is-white"
                                    onclick=show_description_btn >
                                  {"See less"}
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
                            {"See more"}
                          </button>
                        </>},
                    }
                }</div>
              </div>
            </div>
        }
    }

    fn show_modifications_table(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        let onclick_select_modification = self.link
            .callback(|value: UUID| Msg::SelectModification(value));

        html!{<>
            <h2>{"Modification"}</h2>
            <ModificationsTable
                show_manage_btn = false
                modifications = component_data.component_modifications.clone()
                select_modification = self.select_component_modification.clone()
                callback_select_modification = onclick_select_modification.clone()
              />
        </>}
    }

    fn show_component_params(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2>{"Characteristic"}</h2>
              <div class="card">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{"actual_status"}</td>
                        <td>{component_data.actual_status.name.clone()}</td>
                      </tr>
                      <tr>
                        <td>{"component_type"}</td>
                        <td>{component_data.component_type.component_type.clone()}</td>
                      </tr>
                      <tr>
                        <td>{"technical_committee"}</td>
                        <td>{for component_data.licenses.iter().map(|x| html!{format!("{}; ", x.name)})}</td>
                      </tr>
                      <tr>
                        <td>{"updated_at"}</td>
                        <td>{format!("{:.*}", 10, component_data.updated_at.to_string())}</td>
                      </tr>
                      {for component_data.component_params.iter().enumerate().map(|(index, component_param)| {
                          match (index >= 3, self.show_full_characteristic) {
                              // show full list
                              (_, true) => html!{<tr>
                                  <td>{component_param.param.paramname.clone()}</td>
                                  <td>{component_param.value.clone()}</td>
                              </tr>},
                              // show full list or first 4 items
                              (false, false) => html!{<tr>
                                  <td>{component_param.param.paramname.clone()}</td>
                                  <td>{component_param.value.clone()}</td>
                              </tr>},
                              _ => html!{},
                          }
                      })}
                      {match component_data.component_params.len() {
                          // 0 => html!{<span>{"Files not found"}</span>},
                          0..=3 => html!{},
                          _ => self.show_see_characteristic_btn(),
                      }}
                    </tbody>
                  </table>
              </div>
            </div>
        }
    }

    fn show_see_characteristic_btn(&self) -> Html {
        let show_full_characteristic_btn = self.link
            .callback(|_| Msg::ShowCharacteristic);

        match self.show_full_characteristic {
            true => html!{<>
              <button class="button is-white"
                  onclick=show_full_characteristic_btn
                >{"See less"}</button>
            </>},
            false => html!{<>
              <button class="button is-white"
                  onclick=show_full_characteristic_btn
                >{"See more"}</button>
            </>},
        }
    }

    fn show_cards(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        let count_rows =
            component_data.files.len() +
            component_data.component_standards.len() +
            component_data.component_suppliers.len();

        match count_rows {
            0..=5 => html!{
                <div class="columns">
                {self.show_component_params(component_data)}
                <div class="column">
                    <h2>{"Files"}</h2>
                    {self.show_component_files(component_data)}
                    <br/>
                    <h2>{"Standards"}</h2>
                    {self.show_component_standards(component_data)}
                    <br/>
                    {self.show_component_suppliers(component_data)}
                </div>
            </div>},
            _ => html!{<>
                <div class="columns">
                    {self.show_component_params(component_data)}
                    <div class="column">
                        <h2>{"Files"}</h2>
                        {self.show_component_files(component_data)}
                    </div>
                </div>
                <div class="columns">
                    <div class="column">
                        <h2>{"Standards"}</h2>
                        {self.show_component_standards(component_data)}
                    </div>
                    <div class="column">
                        {self.show_component_suppliers(component_data)}
                    </div>
                </div>
            </>},
        }
    }

    fn show_component_files(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{
              <FilesCard
                  show_download_btn = true
                  show_delete_btn = false
                  component_uuid = component_data.uuid.clone()
                  files = component_data.files.clone()
                />
        }
    }

    fn show_component_specs(
        &self,
        component_data: &ComponentInfo,
    ) -> Html {
        html!{<>
              <h2>{"Specs"}</h2>
              <div class="card">
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
              <h2>{"Keywords"}</h2>
              <div class="card">
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
            true => "Supplier".to_string(),
            false => "Main supplier".to_string(),
        };

        html!{<>
            <h2>{table_label}</h2>
            <div class="card">
              <table class="table is-fullwidth">
                <tbody>
                   <th>{"Company"}</th>
                   <th>{"Description"}</th>
                   <th>{"Action"}</th>
                   {match component_data.is_base {
                       true => html!{<>
                           {for component_data.component_suppliers.iter().map(|data| {
                             match &data.supplier.is_supplier {
                                true => html!{<ComponentCompanyItem supplier_data=data.clone() />},
                                false => html!{},
                            }})}
                       </>},
                       false => match component_data.component_suppliers.first() {
                           Some(data) => html!{<ComponentCompanyItem supplier_data=data.clone() />},
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
        html!{<div class="card">
          <table class="table is-fullwidth">
            <tbody>
               <th>{"Classifier"}</th>
               <th>{"Specified tolerance"}</th>
               <th>{"Action"}</th>
               {for component_data.component_standards.iter().map(|data| {
                   html!{<ComponentStandardItem
                       standard_data = data.clone()
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
            .find(|x| x.uuid == self.select_component_modification);

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
                                    <p class="overflow-title">{"Modification name"}</p>
                                    <div class="overflow-title has-text-weight-bold">
                                        {mod_data.modification_name.clone()}
                                    </div>
                                    <p class="overflow-title">{"Description"}</p>
                                    <p class="overflow-title">
                                        {mod_data.description.clone()}
                                    </p>
                                </div>
                            </div>
                            <div class="columns" style="margin-bottom:0">
                                <div class="column">
                                  {format!("Actual status: {}", &mod_data.actual_status.name)}
                                </div>
                                <div class="column">
                                  {format!("Updated at: {:.*}", 10, mod_data.updated_at.to_string())}
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
        match (self.select_fileset_program.len(), &self.open_fileset_files_card) {
            (36, true) => html!{<>
                <h2>{"Files of select fileset"}</h2>
                <FilesOfFilesetCard
                    show_manage_btn = false
                    fileset_uuid = self.select_fileset_program.clone()
                />
            </>},
            _ => html!{},
        }
    }

    fn show_download_block(&self) -> Html {
        let onchange_select_fileset_btn = self.link
            .callback(|ev: ChangeData| Msg::SelectFileset(match ev {
              ChangeData::Select(el) => el.value(),
              _ => "".to_string(),
          }));
        let onclick_open_fileset_files_list_btn = self.link
            .callback(|_| Msg::ShowFilesetFilesList);
        let onclick_download_fileset_btn = self.link
            .callback(|_| Msg::RequestDownloadFiles);

        let class_btn = match self.open_fileset_files_card {
            true => "button is-light is-active",
            false => "button",
        };

        match self.select_fileset_program.len() {
            36 => html!{<div style="margin-right: .5rem">
                <div class="select" style="margin-right: .5rem">
                  <select
                        id="region_id"
                        onchange=onchange_select_fileset_btn >
                      {for self.current_filesets_program.iter().map(|(fileset_uuid, program_name)|
                          match &self.select_fileset_program == fileset_uuid {
                            true => html!{<option value={fileset_uuid.clone()} selected=true>{program_name}</option>},
                            false => html!{<option value={fileset_uuid.clone()}>{program_name}</option>},
                          }
                      )}
                      // <option>{"CADWolf"}</option>
                      // <option>{"AutoCAD"}</option>
                  </select>
                </div>
                <button class={class_btn}
                    onclick = onclick_open_fileset_files_list_btn >
                    <span class="icon is-small"><i class="fa fa-list"></i></span>
                </button>
                <button class="button is-info"
                    onclick=onclick_download_fileset_btn >
                  <span class="has-text-weight-bold">{"Download"}</span>
                </button>
            </div>},
            _ => html!{},
        }
    }

    fn show_setting_btn(&self) -> Html {
        let onclick_setting_standard_btn = self.link
            .callback(|_| Msg::OpenComponentSetting);

        match &self.current_user_owner {
            true => {res_btn(classes!(String::from("fa fa-cog")),
                onclick_setting_standard_btn,
                String::new())},
            false => html!{},
        }
    }

    fn show_followers_btn(&self) -> Html {
        let class_fav = match self.is_followed {
            true => "fas fa-bookmark",
            false => "far fa-bookmark",
        };

        let onclick_following = match self.is_followed {
            true => self.link.callback(|_| Msg::UnFollow),
            false => self.link.callback(|_| Msg::Follow),
        };

        html! {<>
            <div class="media-right flexBox" >
              <button
                  id="following-button"
                  class="button"
                  onclick=onclick_following >
                <span class="icon is-small">
                  <i class={class_fav}></i>
                </span>
              </button>
            </div>
            { format!(" {}", &self.subscribers) }
        </>}
    }

    fn show_share_btn(&self) -> Html {
        html! {
            <div class="media-right flexBox" >
              <button
                  id="share-button"
                  class="button" >
                <span class="icon is-small">
                  <i class="far fa-share"></i>
                </span>
              </button>
            </div>
        }
    }
}
