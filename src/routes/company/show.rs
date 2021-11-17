use chrono::NaiveDateTime;
use web_sys::MouseEvent;
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::service::RouteService;
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::{
    certificate::CertificateCard,
    list_errors::ListErrors,
    catalog_component::CatalogComponents,
    catalog_standard::CatalogStandards,
};
use crate::gqls::make_query;
// use crate::routes::AppRoute;
use crate::services::{
    is_authenticated,
    // get_logged_user
};
use crate::types::{
    UUID, CompanyInfo, Certificate, SlimUser,
    ComponentsQueryArg, StandardsQueryArg,
    // CompanyCertificate, Program, Region, ShowUserShort,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct GetCompanyData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct AddCompanyFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct DeleteCompanyFav;

/// Company with relate data
pub struct ShowCompany {
    error: Option<Error>,
    profile: Option<CompanyInfo>,
    current_company_uuid: UUID,
    // task: Option<FetchTask>,
    // router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    profile_tab: CompanyTab,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub company_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    GetCompanyData(String),
    ChangeTab(CompanyTab),
    Ignore,
}

#[derive(Clone, PartialEq)]
pub enum CompanyTab {
    Certificates,
    Components,
    Standards,
    Represent,
    // Members,
}

impl Component for ShowCompany {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ShowCompany {
            error: None,
            profile: None,
            current_company_uuid: String::new(),
            // task: None,
            // router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            profile_tab: CompanyTab::Certificates,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // get company uuid for request company data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_company_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/company/")
            .to_string();
        // get flag changing current profile in route
        let not_matches_company_uuid = target_company_uuid != self.current_company_uuid;
        // debug!("self.current_company_uuid {:#?}", self.current_company_uuid);

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_company_uuid) && is_authenticated() {
            // update current_company_uuid for checking change profile in route
            self.current_company_uuid = target_company_uuid.to_string();

            spawn_local(async move {
                let res = make_query(GetCompanyData::build_query(get_company_data::Variables {
                    company_uuid: Some(target_company_uuid),
                })).await.unwrap();

                link.send_message(Msg::GetCompanyData(res.clone()));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();

        match msg {
            Msg::Follow => {
                let link = self.link.clone();
                let company_uuid_string = self.profile.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(AddCompanyFav::build_query(add_company_fav::Variables {
                        company_uuid: company_uuid_string,
                    })).await.unwrap();

                    link.send_message(Msg::AddFollow(res.clone()));
                })
            }
            Msg::AddFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("addCompanyFav").unwrap().clone())
                                .unwrap();

                        if result {
                            self.subscribers += 1;
                            self.is_followed = true;
                        }
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::UnFollow => {
                let link = self.link.clone();
                let company_uuid_string = self.profile.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(DeleteCompanyFav::build_query(delete_company_fav::Variables {
                        company_uuid: company_uuid_string,
                    }))
                    .await
                    .unwrap();

                    link.send_message(Msg::DelFollow(res.clone()));
                })
            }
            Msg::DelFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("deleteCompanyFav").unwrap().clone())
                                .unwrap();

                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::GetCompanyData(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let company_data: CompanyInfo =
                            serde_json::from_value(res_value.get("company").unwrap().clone()).unwrap();
                        debug!("Company data: {:?}", company_data);

                        self.subscribers = company_data.subscribers.to_owned();
                        self.is_followed = company_data.is_followed.to_owned();
                        self.current_company_uuid = company_data.uuid.to_owned();
                        self.profile = Some(company_data);
                    }
                    true => {
                        self.error = Some(get_error(&data));
                    }
                }
            }
            Msg::ChangeTab(set_tab) => {
                self.profile_tab = set_tab;
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // let onsubmit = self.link.callback(|ev: FocusEvent| {
        //     ev.prevent_default();
        //     Msg::Request
        // });
        // let title = match &self.profile {
        //     Some(data) => format!("Company {}", data.username),
        //     None => "Not data".to_string(),
        // };

        match &self.profile {
            Some(company_data) => html! {
                <div class="profile-page">
                    <ListErrors error=self.error.clone()/>
                    <div class="container page">
                        <div class="row">
                            <div class="card">
                              <div class="card-content">
                                <div class="media">
                                  { self.view_card() }
                                </div>

                                <div class="content">
                                    { self.view_content(
                                        company_data.description.as_str(),
                                        company_data.region.region.as_str(),
                                    ) }
                                </div>
                                <div style="display: flex;padding: 10px;padding-top: 20px;border-top: 5px dashed;">
                                { self.show_profile_action() }
                                // <hr/>
                                <div class="card-relate-data" style="flex:1;" >
                                    {match self.profile_tab {
                                        CompanyTab::Certificates => {
                                            self.view_certificates()
                                        },
                                        CompanyTab::Represent => {
                                            // self.view_represent_users(&company_data.uuid)
                                            unimplemented!()
                                        },
                                        CompanyTab::Components => {
                                            self.view_components(&company_data.uuid)
                                        },
                                        CompanyTab::Standards => {
                                            self.view_standards(&company_data.uuid)
                                            // unimplemented!()
                                        },
                                        // CompanyTab::Members => {
                                        //     self.view_members_users(&company_data.uuid)
                                        //     unimplemented!()
                                        // },
                                    }}
                                </div>
                                </div>
                            </div>
                          </div>
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

impl ShowCompany {
    fn view_card(&self) -> Html {

        match &self.profile {
            Some(company_data) => html! {<>
                <div class="media-left">
                  <figure class="image is-48x48">
                    // <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                    <img src={company_data.image_file.download_url.to_string()} alt="Favicon profile"/>
                  </figure>
                </div>
                <div class="media-content">
                  <p id="title-fl" class="title is-4">{
                      format!("{} {}",
                        company_data.company_type.name.to_string(),
                        company_data.orgname.to_string()
                  )}</p>
                  <p id="subtitle-username" class="subtitle is-6">{
                      format!("{} {}",
                        company_data.company_type.shortname.to_string(),
                        company_data.shortname.to_string(),
                  )}</p>
                </div>
                <div class="media-right">
                  <p class="subtitle is-6 left">
                      // date formatting for show on page
                      {format!("Created at {:.*}", 19, company_data.created_at.to_string())}
                      <br/>
                      {format!("Updated at {:.*}", 19, company_data.updated_at.to_string())}
                      <br/>
                      // for self user data not show button "following"
                      <div class="media-right flexBox " >
                        { self.show_profile_followers() }
                      </div>
                  </p>
                </div>
            </>},
            None => html! {},
        }
    }

    fn show_profile_followers(&self) -> Html {
        html! {<>
            {match &self.profile {
                Some(_) => {
                    let class_fav = match self.is_followed {
                        true => "fas fa-bookmark",
                        false => "far fa-bookmark",
                    };

                    let onclick_following = match self.is_followed {
                        true => self.link.callback(|_| Msg::UnFollow),
                        false => self.link.callback(|_| Msg::Follow),
                    };

                    html! {
                        // for self user data not show button "following"
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
                    }
                },
                None => html!{},
            }}
            { format!(" {}", &self.subscribers) }
        </>}
    }

    fn show_profile_action(&self) -> Html {
        let onclick_certificates = self
            .link
            .callback(|_| Msg::ChangeTab(CompanyTab::Certificates));

        let onclick_represent = self
            .link
            .callback(|_| Msg::ChangeTab(CompanyTab::Represent));

        let onclick_components = self
            .link
            .callback(|_| Msg::ChangeTab(CompanyTab::Components));

        let onclick_standards = self
            .link
            .callback(|_| Msg::ChangeTab(CompanyTab::Standards));

        // let onclick_members = self
        //     .link
        //     .callback(|_| Msg::ChangeTab(CompanyTab::Members));

        let mut active_certificates = "";
        let mut active_represent = "";
        let mut active_components = "";
        let mut active_standards = "";
        // let mut active_members = "";

        match &self.profile_tab {
            CompanyTab::Certificates => active_certificates = "is-active",
            CompanyTab::Represent => active_represent = "is-active",
            CompanyTab::Components => active_components = "is-active",
            CompanyTab::Standards => active_standards = "is-active",
            // CompanyTab::Members => active_members = "is-active",
        }

        fn li_generator(class:&'static str, onclick: Callback<MouseEvent>, info:String, number: usize) -> Html {
          let show_tag = number==0;

          html!(
            <li>
              <a class={class} onclick=onclick style="display: flex;justify-content: space-between;" >
                <span>{info}</span>
                <span hidden=show_tag>
                  <span class="tag is-info is-small" >{number}</span>
                </span>
              </a>
            </li>
          )
        }

        html! {
            <div class="card" style="padding: 10px;margin-right: 18px;" >
                // <ul>
                <aside class="menu">
                    {match &self.profile {
                        Some(company_data) => html! {<>
                            <p class="menu-label">
                              {"General"}
                            </p>
                            <ul class="menu-list">
                                {li_generator(active_certificates, onclick_certificates, "certificates".to_string(), company_data.company_certificates.len())}
                                {li_generator(active_represent, onclick_represent, "represent".to_string(), 0)}
                                {li_generator(active_components, onclick_components, "components".to_string(), 0)}
                                {li_generator(active_standards, onclick_standards, "standards".to_string(), 0)}
                                // {li_generator(active_members, onclick_members, "members".to_string(), self_data.fav_standards_count)}
                            </ul>
                        </>},
                        None => html!{},
                    }
                }
                // </ul>
                </aside>
            </div>
        }
    }

    fn view_content(
        &self,
        description: &str,
        region: &str,
    ) -> Html {
        html! {
            <div class="columns">
                <div class="column">
                    <div id="description" class="content">
                      { format!("{}", description) }
                    </div>
                    <br/>
                </div>
                <div class="column">
                    <span id="region">
                      <i class="fas fa-map-marker-alt"></i>
                      { format!("Region: {}", region) }
                    </span>
                </div>
            </div>
        }
    }

    fn view_certificates(&self) -> Html {
        match &self.profile {
            None => html! {},
            Some(company_profile) => {
                html! {
                    // <p class="card-footer-item">
                    <footer class="card-footer">{
                        for company_profile.company_certificates.iter().map(|cert| {
                            let view_cert: Certificate = cert.into();
                            html! {
                                <CertificateCard
                                    certificate = view_cert
                                    show_cert_btn = true
                                    download_btn = false
                                    change_btn = false
                                    company_uuid = None
                                 />
                            }
                        })
                    }</footer>
                    // </p>
                }
            }
        }
    }

    fn view_components(
        &self,
        company_uuid: &UUID,
    ) -> Html {
        html! {
            <CatalogComponents
                show_create_btn = false
                arguments = ComponentsQueryArg::set_company_uuid(company_uuid)
            />
        }
    }

    fn view_standards(
        &self,
        company_uuid: &UUID,
    ) -> Html {
        html! {
            <CatalogStandards
                show_create_btn = false
                arguments = StandardsQueryArg::set_company_uuid(company_uuid)
            />
        }
    }

    // fn view_members(&self) -> Html {
    //     html! {
    //         <CatalogUsers
    //             arguments = UsersQueryArg::set_favorite()
    //         />
    //     }
    // }
}
