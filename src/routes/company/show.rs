use chrono::NaiveDateTime;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{Callback, Component, classes, ComponentLink, Html, Properties, ShouldRender, html};
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
    list_errors::ListErrors,
    side_menu::{MenuItem, SideMenu},
    company::{CompanyCertificatesCard, CompanyRepresents, SpecsTags},
    component::CatalogComponents,
    standard::CatalogStandards,
};
use crate::gqls::make_query;
use crate::services::is_authenticated;
use crate::types::{
    UUID, CompanyInfo, SlimUser, ComponentsQueryArg, StandardsQueryArg
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
    company: Option<CompanyInfo>,
    current_company_uuid: UUID,
    current_user_owner: bool,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    company_tab: CompanyTab,
    extend_tab: Option<CompanyTab>,
    show_full_company_info: bool,
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
    GetCompanyResult(String),
    ChangeTab(CompanyTab),
    OpenOwnerCompany,
    OpenSettingCompany,
    ShowFullCompanyInfo,
    ClearError,
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
            company: None,
            current_company_uuid: String::new(),
            current_user_owner: false,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            company_tab: CompanyTab::Certificates,
            extend_tab: None,
            show_full_company_info: true,
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
        // get flag changing current company in route
        let not_matches_company_uuid = target_company_uuid != self.current_company_uuid;
        // debug!("self.current_company_uuid {:#?}", self.current_company_uuid);

        let link = self.link.clone();

        // debug!("get_self {:?}", get_self);

        if (first_render || not_matches_company_uuid) && is_authenticated() {
            // clear old data
            self.error = None;
            self.company = None;

            // update current_company_uuid for checking change company in route
            self.current_company_uuid = target_company_uuid.clone();

            spawn_local(async move {
                let res = make_query(GetCompanyData::build_query(get_company_data::Variables {
                    company_uuid: target_company_uuid,
                })).await.unwrap();

                link.send_message(Msg::GetCompanyResult(res.clone()));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();

        match msg {
            Msg::Follow => {
                let link = self.link.clone();
                let company_uuid = self.company.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(AddCompanyFav::build_query(add_company_fav::Variables{
                        company_uuid
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
                            serde_json::from_value(res_value.get("addCompanyFav").unwrap().clone())
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
                let link = self.link.clone();
                let company_uuid = self.company.as_ref().unwrap().uuid.clone();

                spawn_local(async move {
                    let res = make_query(DeleteCompanyFav::build_query(
                        delete_company_fav::Variables{ company_uuid }
                    )).await.unwrap();

                    link.send_message(Msg::DelFollow(res));
                })
            },
            Msg::DelFollow(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(
                            res_value.get("deleteCompanyFav").unwrap().clone()
                        ).unwrap();

                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetCompanyResult(res) => {
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
                        self.company = Some(company_data);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::ChangeTab(set_tab) => self.company_tab = set_tab,
            Msg::OpenOwnerCompany => {
                if let Some(company_data) = &self.company {
                    // Redirect to owner company page
                    self.router_agent.send(ChangeRoute(AppRoute::Profile(
                        company_data.owner_user.username.to_string()
                    ).into()));
                }
            },
            Msg::OpenSettingCompany => {
                if let Some(company_data) = &self.company {
                    // Redirect to owner company page
                    self.router_agent.send(ChangeRoute(AppRoute::CompanySettings(
                        company_data.uuid.to_string()
                    ).into()));
                }
            },
            Msg::ShowFullCompanyInfo => self.show_full_company_info = !self.show_full_company_info,
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
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        match &self.company {
            Some(company_data) => html!{
                <div class="company-page">
                    <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error) />
                    <div class="container page">
                        <div class="row">
                            <div class="card">
                              <div class="card-content">
                                {self.view_card()}
                                <div class="content">
                                    {self.view_content(company_data)}
                                </div>
                            </div>
                          </div>
                          {self.company_relate_object(company_data)}
                        </div>
                    </div>
                </div>
            },
            None => html!{<ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error) />},
        }
    }
}

impl ShowCompany {
    fn view_card(&self) -> Html {
        let onclick_owner_company_btn = self.link.callback(|_| Msg::OpenOwnerCompany);
        let onclick_setting_company_btn = self.link.callback(|_| Msg::OpenSettingCompany);

        let size_favicon = match self.show_full_company_info {
            true => "is-128x128",
            false => "is-48x48",
        };

        match &self.company {
            Some(company_data) => html!{<div class="media">
                <div class="media-left">
                  <figure class=classes!("image", size_favicon)>
                    // <div hidden={!company_data.is_supplier} class="top-tag" >{"supplier"}</div>
                    // <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                    <img
                        src={company_data.image_file.download_url.to_string()} alt="Favicon company"
                        loading="lazy"
                    />
                  </figure>
                </div>
                <div class="media-content">
                  <span>{"Company"}</span>
                  <span hidden={!company_data.is_supplier} id="company-region">
                    {" supplier"}
                    // <i class="fa fa-diamond" aria-hidden="true"></i>
                    // <svg width="25" height="25" viewBox="0 0 197.249 197.25" xmlns="http://www.w3.org/2000/svg">
                    // <g transform="translate(-11.136 -18.506)">
                    // <path style="fill:none;stroke:#000;stroke-width:.434;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1" d="m44.396 115.725 25.955-33.866h77.2l26.287 33.346-63.596 68.922z"/>
                    // <path style="fill:none;stroke:#000;stroke-width:.434204px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1" d="m43.338 116.783 129.441-.52M122.778 81.857l17.736 33.672-30.272 68.598-31.858-68.419 17.978-33.843z"/>
                    // <path d="M208.167 130.384v-26.505c-13.539-4.814-22.092-6.167-26.398-16.557v-.008c-4.321-10.423.82-17.5 6.946-30.4l-18.738-18.739c-12.801 6.085-19.952 11.276-30.4 6.946h-.008c-10.406-4.313-11.768-12.924-16.557-26.398H96.508C91.735 32.131 90.365 40.8 79.95 45.121h-.007c-10.423 4.33-17.483-.804-30.4-6.946L30.805 56.914c6.11 12.858 11.276 19.96 6.946 30.4-4.322 10.423-12.99 11.792-26.398 16.565v26.505c13.383 4.756 22.076 6.142 26.398 16.557 4.346 10.513-.935 17.762-6.946 30.4l18.738 18.746c12.81-6.093 19.96-11.276 30.4-6.946h.008c10.415 4.314 11.776 12.95 16.557 26.398h26.504c4.773-13.416 6.151-22.06 16.623-26.422h.008c10.35-4.297 17.386.828 30.326 6.979l18.739-18.747c-6.101-12.818-11.276-19.952-6.954-30.392 4.321-10.423 13.022-11.809 26.414-16.573z" style="fill:none;stroke:#000;stroke-width:.434;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"/>
                    // <ellipse style="fill:none;stroke:#000;stroke-width:.433999;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1" cx="109.449" cy="115.983" rx="69.586" ry="69.587"/></g></svg>
                  </span>
                  {match self.show_full_company_info {
                      true => html!{<>
                          <p id="title-orgname" class="title is-4">{format!("{} ", &company_data.orgname)}</p>
                          <p id="title-type" class="subtitle is-4">{company_data.company_type.name.clone()}</p>
                      </>},
                      false => html!{
                          <p id="subtitle-shortname">
                            <strong>{format!("{} ", &company_data.shortname)}</strong>
                            {company_data.company_type.shortname.clone()}
                          </p>
                      },
                  }}
                </div>
                <div class="media-right">
                    {match self.show_full_company_info {
                        true => html!{<p class="subtitle is-6 left">
                            // date formatting for show on page
                            {format!("Created at {:.*}", 19, company_data.created_at.to_string())}
                            <br/>
                            {format!("Updated at {:.*}", 19, company_data.updated_at.to_string())}
                        </p>},
                        false => html!{},
                    }}
                    <div class="buttons flexBox" >
                      {res_btn(classes!(
                          String::from("fas fa-user")),
                          onclick_owner_company_btn,
                          String::new())}
                      {match &self.current_user_owner {
                          true => {res_btn(
                              classes!("fa", "fa-tools"),
                              onclick_setting_company_btn,
                              String::new())},
                          false => html!{},
                      }}
                      {self.show_company_followers()}
                    </div>
                </div>
            </div>},
            None => html!{},
        }
    }

    fn show_company_followers(&self) -> Html {
        html!{<>
            {match &self.company {
                Some(_) => self.show_favorite_btn(),
                None => html!{<span>{self.subscribers}</span>},
            }}
        </>}
    }

    fn show_favorite_btn(&self) -> Html {
        let (class_fav, onclick_following) = match self.is_followed {
            true => ("fas fa-bookmark", self.link.callback(|_| Msg::UnFollow)),
            false => ("far fa-bookmark", self.link.callback(|_| Msg::Follow)),
        };

        html!{
            <button
                id="following-button"
                class="button"
                onclick=onclick_following >
              <span class="icon is-small">
                <i class={class_fav}></i>
              </span>
              <span>{self.subscribers}</span>
            </button>
        }
    }

    fn view_content(
        &self,
        company_data: &CompanyInfo,
    ) -> Html {
        let onclick_change_full_show = self.link.callback(|_| Msg::ShowFullCompanyInfo);

        match self.show_full_company_info {
            true => html! {<>
                <div class="columns">
                    <div class="column">
                        <div id="description" class="content">
                          {company_data.description.clone()}
                        </div>
                    </div>
                    <div class="column">
                        <div id="company-email">
                            <span class="icon is-small"><i class="fas fa-envelope" /></span>
                            <span>{" Email: "}</span>
                            <span class="overflow-title has-text-weight-bold">{company_data.email.clone()}</span>
                        </div>
                        // <br/>
                        <div id="company-phone">
                            <span class="icon is-small"><i class="fas fa-phone" /></span>
                            <span>{" Phone: "}</span>
                            <span class="overflow-title has-text-weight-bold">{company_data.phone.clone()}</span>
                        </div>
                        // <br/>
                        <div id="company-inn">
                            <span class="icon is-small"><i class="fas fa-building" /></span>
                            <span>{" Reg.№: "}</span>
                            <span class="overflow-title has-text-weight-bold">{company_data.inn.clone()}</span>
                        </div>
                        // <br/>
                        <div id="company-region">
                            <span class="icon is-small"><i class="fas fa-map-marker-alt" /></span>
                            <span>{" Location: "}</span>
                            <span class="overflow-title has-text-weight-bold">{company_data.region.region.clone()}</span>
                            <span class="overflow-title has-text-weight-bold">{", "}</span>
                            <span id="company-address" class="overflow-title has-text-weight-bold">
                                {company_data.address.clone()}
                            </span>
                        </div>
                        // <br/>
                        <div id="company-site_url">
                            <span class="icon is-small"><i class="fas fa-globe" /></span>
                            <span>{" Site: "}</span>
                            <span class="overflow-title has-text-weight-bold">{company_data.site_url.clone()}</span>
                        </div>
                    </div>
                </div>
                {match company_data.company_specs.is_empty() {
                    true => html!{},
                    false => html!{<div class="media">
                        <div class="media-left">
                            <span>{"Sphere of activity: "}</span>
                        </div>
                        <div class="media-content">
                            <SpecsTags
                                show_manage_btn = false
                                company_uuid = company_data.uuid.clone()
                                specs = company_data.company_specs.clone()
                            />
                        </div>
                    </div>}
                }}
                <button class="button is-ghost" onclick={onclick_change_full_show}>
                    <span>{"Hide info"}</span>
                </button>
            </>},
            false => html!{
                <button class="button is-ghost" onclick={onclick_change_full_show}>
                    <span>{"Show info"}</span>
                </button>
            },
        }
    }

    fn company_relate_object(
        &self,
        company_data: &CompanyInfo,
    ) -> Html {
        html!{<div class="card">
            <div class="columns is-mobile">
                <div class="column is-flex">
                    { self.show_company_action() }
                    <div class="card-relate-data" style="flex:1;" >
                        {match self.company_tab {
                            CompanyTab::Certificates =>
                                self.view_certificates(&company_data),
                            CompanyTab::Represent =>
                                self.view_represents(&company_data),
                            CompanyTab::Components =>
                                self.view_components(&company_data.uuid),
                            CompanyTab::Standards =>
                                self.view_standards(&company_data.uuid, &company_data.is_supplier),
                            // CompanyTab::Members => {},
                        }}
                    </div>
                </div>
            </div>
        </div>}
    }

    fn show_company_action(&self) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            // certificates MenuItem
            MenuItem {
                title: "CERTIFICATES".to_string(),
                action: self.cb_generator(CompanyTab::Certificates),
                count: self.get_number_of_items(&CompanyTab::Certificates),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-certificate")],
                is_active: self.company_tab == CompanyTab::Certificates,
                is_extend: self.check_extend(&CompanyTab::Certificates),
            },
            // representations MenuItem
            MenuItem {
                title: "REPRESENTATIONS".to_string(),
                action: self.cb_generator(CompanyTab::Represent),
                count: self.get_number_of_items(&CompanyTab::Represent),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-industry")],
                is_active: self.company_tab == CompanyTab::Represent,
                is_extend: self.check_extend(&CompanyTab::Represent),
            },
            // components MenuItem
            MenuItem {
                title: "COMPONENTS".to_string(),
                action: self.cb_generator(CompanyTab::Components),
                count: self.get_number_of_items(&CompanyTab::Components),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cogs")],
                is_active: self.company_tab == CompanyTab::Components,
                is_extend: self.check_extend(&CompanyTab::Components),
            },
            // standards MenuItem
            MenuItem {
                title: "STANDARDS".to_string(),
                action: self.cb_generator(CompanyTab::Standards),
                count: self.get_number_of_items(&CompanyTab::Standards),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cube")],
                is_active: self.company_tab == CompanyTab::Standards,
                is_extend: self.check_extend(&CompanyTab::Standards),
            },
            // memebers MenuItem
            // MenuItem {
            //     title: "MEMBERS".to_string(),
            //     action: self.cb_generator(CompanyTab::Members),
            //     count: self.get_number_of_items(&CompanyTab::Members),
            //     item_class: classes!("has-background-white"),
            //     icon_classes: vec![classes!("fas", "fa-user")],
            //     is_active: self.company_tab == CompanyTab::Members,
            //     is_extend: self.check_extend(&CompanyTab::Members),
            // },
        ];

        html! {
            <div style="margin-right: 18px;z-index: 1;" >
                <SideMenu menu_arr={menu_arr} />
            </div>
        }
    }

    fn view_certificates(
        &self,
        company_data: &CompanyInfo,
    ) -> Html {
        html!{<div class="profileBox" >
            <CompanyCertificatesCard
                certificates = company_data.company_certificates.clone()
                show_cert_btn = true
                download_btn = false
                manage_btn = false
             />
        </div>}
    }

    fn view_represents(
        &self,
        company_data: &CompanyInfo,
    ) -> Html {
        html!{
            <CompanyRepresents
                show_manage_btn = false
                list = company_data.company_represents.clone()
            />
        }
    }

    fn view_components(
        &self,
        company_uuid: &UUID,
    ) -> Html {
        html!{
            <CatalogComponents
                show_create_btn = false
                arguments = ComponentsQueryArg::set_company_uuid(company_uuid)
            />
        }
    }

    fn view_standards(
        &self,
        company_uuid: &UUID,
        is_supplier: &bool,
    ) -> Html {
        html!{
            <CatalogStandards
                show_create_btn = is_supplier.clone()
                arguments = StandardsQueryArg::set_company_uuid(company_uuid)
            />
        }
    }

    fn cb_generator(&self, cb: CompanyTab) -> Callback<MouseEvent> {
        self.link.callback(move |_| Msg::ChangeTab(cb.clone()))
    }

    fn check_extend(&self, tab: &CompanyTab) -> bool {
        if self.extend_tab.is_some() {
            self.extend_tab.clone().unwrap() == tab.clone()
        } else {
            false
        }
    }

    fn get_number_of_items(&self, tab: &CompanyTab) -> usize {
        match &self.company {
            Some(ref company) =>  match tab {
              CompanyTab::Certificates => company.company_certificates.len(),
              CompanyTab::Represent => company.company_represents.len(),
              CompanyTab::Components => 0,
              CompanyTab::Standards => 0,
              // CompanyTab::Members => 0,
            },
            None => 0,
        }
    }

    // fn view_members(&self) -> Html {
    //     html!{
    //         <CatalogUsers
    //             arguments = UsersQueryArg::set_favorite()
    //         />
    //     }
    // }
}
