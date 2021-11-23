use chrono::NaiveDateTime;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::{get_error, Error};
use crate::fragments::{
    switch_icon::res_btn,
    company_certificate::CompanyCertificateCard,
    company_represent::CompanyRepresents,
    list_errors::ListErrors,
    catalog_component::CatalogComponents,
    catalog_standard::CatalogStandards,
    spec::SpecsTags,
};
use crate::gqls::make_query;
use crate::services::{
    is_authenticated,
    // get_logged_user
};
use crate::types::{
    UUID, CompanyInfo, Certificate, SlimUser,
    ComponentsQueryArg, StandardsQueryArg, CompanyRepresentInfo
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
    current_user_owner: bool,
    // task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
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
    OpenOwnerCompany,
    OpenSettingCompany,
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
            current_user_owner: false,
            // task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
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
                        if let Some(user) = &self.props.current_user {
                            self.current_user_owner = company_data.owner_user.uuid == user.uuid;
                        }
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
            Msg::OpenOwnerCompany => {
                if let Some(company_data) = &self.profile {
                    // Redirect to owner profile page
                    self.router_agent.send(ChangeRoute(AppRoute::Profile(
                        company_data.owner_user.username.to_string()
                    ).into()));
                }
            }
            Msg::OpenSettingCompany => {
                if let Some(company_data) = &self.profile {
                    // Redirect to owner profile page
                    self.router_agent.send(ChangeRoute(AppRoute::CompanySettings(
                        company_data.uuid.to_string()
                    ).into()));
                }
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
                                    { self.view_content(company_data) }
                                </div>
                                <div style="display: flex;padding: 10px;padding-top: 20px;border-top: 5px dashed;">
                                { self.show_profile_action() }
                                // <hr/>
                                <div class="card-relate-data" style="flex:1;" >
                                    {match self.profile_tab {
                                        CompanyTab::Certificates => {
                                            self.view_certificates(company_data)
                                        },
                                        CompanyTab::Represent => {
                                            self.view_represents(&company_data.company_represents)
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
        let onclick_owner_profile_btn = self.link.callback(|_| Msg::OpenOwnerCompany);
        let onclick_setting_company_btn = self.link.callback(|_| Msg::OpenSettingCompany);

        match &self.profile {
            Some(company_data) => html! {<>
                <div class="media-left">
                  <figure class="image is-48x48">
                    // <div hidden={!company_data.is_supplier} class="top-tag" >{"supplier"}</div>
                    <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                    // <img src={company_data.image_file.download_url.to_string()} alt="Favicon profile"/>
                  </figure>
                </div>
                <div class="media-content">
                  <span>{"Company"}</span>
                  <span hidden={!company_data.is_supplier} id="company-region">
                    {" supplier"}
                    // <i class="fa fa-diamond" aria-hidden="true"></i>
                    <svg width="25" height="25" viewBox="0 0 197.249 197.25" xmlns="http://www.w3.org/2000/svg">
                    <g transform="translate(-11.136 -18.506)">
                    <path style="fill:none;stroke:#000;stroke-width:.434;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1" d="m44.396 115.725 25.955-33.866h77.2l26.287 33.346-63.596 68.922z"/>
                    <path style="fill:none;stroke:#000;stroke-width:.434204px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1" d="m43.338 116.783 129.441-.52M122.778 81.857l17.736 33.672-30.272 68.598-31.858-68.419 17.978-33.843z"/>
                    <path d="M208.167 130.384v-26.505c-13.539-4.814-22.092-6.167-26.398-16.557v-.008c-4.321-10.423.82-17.5 6.946-30.4l-18.738-18.739c-12.801 6.085-19.952 11.276-30.4 6.946h-.008c-10.406-4.313-11.768-12.924-16.557-26.398H96.508C91.735 32.131 90.365 40.8 79.95 45.121h-.007c-10.423 4.33-17.483-.804-30.4-6.946L30.805 56.914c6.11 12.858 11.276 19.96 6.946 30.4-4.322 10.423-12.99 11.792-26.398 16.565v26.505c13.383 4.756 22.076 6.142 26.398 16.557 4.346 10.513-.935 17.762-6.946 30.4l18.738 18.746c12.81-6.093 19.96-11.276 30.4-6.946h.008c10.415 4.314 11.776 12.95 16.557 26.398h26.504c4.773-13.416 6.151-22.06 16.623-26.422h.008c10.35-4.297 17.386.828 30.326 6.979l18.739-18.747c-6.101-12.818-11.276-19.952-6.954-30.392 4.321-10.423 13.022-11.809 26.414-16.573z" style="fill:none;stroke:#000;stroke-width:.434;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"/>
                    <ellipse style="fill:none;stroke:#000;stroke-width:.433999;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1" cx="109.449" cy="115.983" rx="69.586" ry="69.587"/></g></svg>
                  </span>
                  <p id="title-orgname" class="title is-4">{format!("{} ", &company_data.orgname)}</p>
                  <p id="title-type" class="subtitle is-4">{company_data.company_type.name.clone()}</p>
                  <p id="subtitle-shortname">
                    <strong>{format!("{} ", &company_data.shortname)}</strong>
                    {company_data.company_type.shortname.clone()}
                  </p>
                </div>
                <div class="media-right">
                  <p class="subtitle is-6 left">
                      // date formatting for show on page
                      {format!("Created at {:.*}", 19, company_data.created_at.to_string())}
                      <br/>
                      {format!("Updated at {:.*}", 19, company_data.updated_at.to_string())}
                      <br/>
                      <div class="media-right buttons flexBox" >
                        {res_btn(classes!(
                            String::from("fas fa-user")),
                            onclick_owner_profile_btn,
                            String::new())}
                        {match &self.current_user_owner {
                            true => {res_btn(classes!(
                                String::from("fa fa-cog")),
                                onclick_setting_company_btn,
                                String::new())},
                            false => html!{},
                        }}
                        // for self user data not show button "following"
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
            <div class="media-right flexBox" >
              <button
                  id="share-button"
                  class="button" >
                <span class="icon is-small">
                  <i class="far fa-share"></i>
                </span>
              </button>
            </div>
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
                                {li_generator(active_represent, onclick_represent, "represent".to_string(), company_data.company_represents.len())}
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
        company_data: &CompanyInfo,
    ) -> Html {
        html! {
            <div class="columns">
                <div class="column">
                    <div id="description" class="content">
                      { format!("{}", &company_data.description) }
                    </div>
                    <br/>
                    <span>{"company specs: "}</span>
                    <SpecsTags
                          show_delete_btn = self.current_user_owner.clone()
                          company_uuid = company_data.uuid.clone()
                          specs = company_data.company_specs.clone()
                        />
                    // <div id="specs" class="tags">
                    //     {for company_data.company_specs.iter().map(|spec| {
                    //         html! {<div class="tag is-light">{&spec.spec.spec}</div>}
                    //     })}
                    // </div>
                    <br/>
                </div>
                <div class="column">
                    <span id="company-email">
                      <i class="fas fa-envelope"></i>
                      { format!("Email: {}", &company_data.email) }
                    </span>
                    <br/>
                    <span id="company-phone">
                      <i class="fas fa-phone"></i>
                      { format!("Phone: {}", &company_data.phone) }
                    </span>
                    <br/>
                    <span id="company-inn">
                      <i class="fas fa-building"></i>
                      { format!("Reg.№: {}", &company_data.inn) }
                    </span>
                    <br/>
                    <span id="company-region">
                      <i class="fas fa-map-marker-alt"></i>
                      { format!("Location: {}, ", &company_data.region.region) }
                    </span>
                    <span id="company-address">
                      { company_data.address.to_string() }
                    </span>
                    <br/>
                    <span id="company-site">
                      <i class="fas fa-building"></i>
                      { format!("Site: {}", &company_data.site_url) }
                    </span>
                </div>
            </div>
        }
    }

    fn view_certificates(
        &self,
        company_data: &CompanyInfo,
    ) -> Html {
        html! {
            // <p class="card-footer-item">
            <footer class="card-footer">{
                for company_data.company_certificates.iter().map(|cert| {
                    let view_cert: Certificate = cert.into();
                    html! {
                        <CompanyCertificateCard
                            company_uuid = company_data.uuid.to_string()
                            certificate = view_cert
                            show_cert_btn = true
                            download_btn = false
                            change_btn = false
                         />
                    }
                })
            }</footer>
            // </p>
        }
    }

    fn view_represents(
        &self,
        company_represents: &[CompanyRepresentInfo],
    ) -> Html {
        html! {
            <CompanyRepresents
                show_manage_btn = false
                list = company_represents.to_vec()
            />
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
