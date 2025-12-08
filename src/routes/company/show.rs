use yew::{agent::Bridged, classes, html, Bridge, Callback, Component, Properties, ComponentLink, Html, ShouldRender};
use yew_router::{
    service::RouteService,
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use web_sys::MouseEvent;
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    buttons::{ft_follow_btn, res_settings_btn},
    clipboard::ShareLinkBtn,
    list_errors::ListErrors,
    side_menu::{MenuItem, SideMenu},
    company::diamond_svg,
    supplier_service::ServiceRequestBtn,
    company::{view_certificates, view_components, view_content, view_represents, view_services, view_standards, view_discussion},
    responsive::resizer,
};
use crate::services::content_adapter::{ContentDisplay, DateDisplay};
use crate::services::{get_logged_user, get_value_field, resp_parsing, set_history_back, title_changer, Counter};
use crate::types::{CompanyInfo, Pathname, SlimUser, UUID};
use crate::gqls::make_query;
use crate::gqls::company::{
    GetCompanyData, get_company_data,
    AddCompanyFav, add_company_fav,
    DeleteCompanyFav, delete_company_fav,
};

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
}

impl Counter for ShowCompany {
    fn quantity(&self) -> usize {
        self.subscribers
    }
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
    OpenSettingCompany,
    ResponseError(Error),
    ClearError,
    Ignore,
}

#[derive(Clone, PartialEq)]
pub enum CompanyTab {
    Info,
    Certificates,
    Represent,
    Components,
    Services,
    Standards,
    Discussion,
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
            company_tab: CompanyTab::Components,
            extend_tab: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if let None = get_logged_user() {
            set_history_back(Some(String::new()));
            // route to login page if not found token
            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
        };

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

        if let Some(company) = &self.company {
            title_changer::set_title(company.shortname.as_str());
        }

        if first_render || not_matches_company_uuid {
            let link = self.link.clone();

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
        let link = self.link.clone();

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
                match resp_parsing(res, "addCompanyFav") {
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
                match resp_parsing(res, "deleteCompanyFav") {
                    Ok(result) => {
                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetCompanyResult(res) => {
                match resp_parsing::<CompanyInfo>(res, "company") {
                    Ok(company_data) => {
                        debug!("Company data: {:?}", company_data);
                        self.subscribers = company_data.subscribers.to_owned();
                        self.is_followed = company_data.is_followed.to_owned();
                        self.current_company_uuid = company_data.uuid.to_owned();
                        if let Some(user) = get_logged_user() {
                            self.current_user_owner = company_data.owner_user.uuid == user.uuid;
                        }
                        self.company = Some(company_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ChangeTab(set_tab) => self.company_tab = set_tab,
            Msg::OpenSettingCompany => {
                if let Some(company_data) = &self.company {
                    // Redirect to owner company page
                    self.router_agent.send(ChangeRoute(AppRoute::CompanySettings(
                        company_data.uuid.to_string()
                    ).into()));
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.company_uuid == props.company_uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        match &self.company {
            Some(company_data) => html!{
                <div class="company-page">
                    <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
                    <div class="container page">
                        <div class="row">
                            <div class="card">
                              <div class="card-content">
                                {self.view_card()}
                            </div>
                          </div>
                          {self.company_relate_object(company_data)}
                        </div>
                    </div>
                </div>
            },
            None => html!{<ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />},
        }
    }
}

impl ShowCompany {
    fn view_card(&self) -> Html {
        let onclick_setting_company_btn = self.link.callback(|_| Msg::OpenSettingCompany);
        match &self.company {
            Some(company_data) => html!{
              <div class="columns">
                <div class="box">
                  <figure class={classes!("container", "image", "is-64x64")}>
                    <img
                        src={company_data.image_file.download_url.to_string()} alt="Favicon company"
                        loading="lazy"
                    />
                  </figure>
                </div>
                <div id="company-is-supplier" class={classes!("column", "is-three-fifths")}>
                    {company_data.to_display()}
                    <abbr title={get_value_field(&275)} hidden={!company_data.is_supplier}>
                        {diamond_svg(company_data.is_supplier, "25")}
                    </abbr>
                </div>
                <div class="column">
                    <div class="buttons flexBox mb-0" >
                      {match &self.current_user_owner {
                        true => {res_settings_btn(
                            onclick_setting_company_btn,
                            Pathname::CompanySetting(self.current_company_uuid.clone())
                        )},
                        false => html!{},
                      }}
                      {self.show_favorite_btn()}
                      <ShareLinkBtn />
                      {match company_data.is_supplier {
                        true => html!{<>
                            <ServiceRequestBtn company_uuid={company_data.uuid.clone()} />
                        </>},
                        false => html!{},
                      }}
                    </div>
                    <div id="company-on-since">
                      <span>{get_value_field(&231)}{" "}</span> // On the platform since
                      <span>{company_data.created_at.date_to_display()}</span>
                    </div>
                </div>
            </div>},
            None => html!{},
        }
    }

    fn show_favorite_btn(&self) -> Html {
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

    fn company_relate_object(
        &self,
        company_data: &CompanyInfo,
    ) -> Html {
        html!{<div id={"company-card-list"} class="card">
            <div class="columns is-mobile">
                <div class="column is-flex">
                    { self.show_company_action() }
                    <div id={"company-card-list-items"} class="card-relate-data" style={resizer("company-card-list", 5)}>
                        {match self.company_tab {
                            CompanyTab::Info => view_content(&company_data),
                            CompanyTab::Represent => view_represents(&company_data),
                            CompanyTab::Certificates => view_certificates(&company_data),
                            CompanyTab::Components => view_components(&company_data.uuid),
                            CompanyTab::Services => view_services(&company_data.uuid),
                            CompanyTab::Standards => view_standards(&company_data.uuid),
                            CompanyTab::Discussion => view_discussion(&company_data.uuid),
                            // CompanyTab::Members => {},
                        }}
                    </div>
                </div>
            </div>
        </div>}
    }

    fn show_company_action(&self) -> Html {
        let menu_arr: Vec<MenuItem> = vec![
            // company info MenuItem
            MenuItem {
                title: get_value_field(&232).to_string(), // INFO
                action: self.cb_generator(CompanyTab::Info),
                count: self.get_number_of_items(&CompanyTab::Info),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-info")],
                is_active: self.company_tab == CompanyTab::Info,
                is_extend: self.check_extend(&CompanyTab::Info),
            },
            // certificates MenuItem
            MenuItem {
                title: get_value_field(&32).to_string(), // CERTIFICATES
                action: self.cb_generator(CompanyTab::Certificates),
                count: self.get_number_of_items(&CompanyTab::Certificates),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-certificate")],
                is_active: self.company_tab == CompanyTab::Certificates,
                is_extend: self.check_extend(&CompanyTab::Certificates),
            },
            // representations MenuItem
            MenuItem {
                title: get_value_field(&266).to_string(), // REPRESENTATIONS
                action: self.cb_generator(CompanyTab::Represent),
                count: self.get_number_of_items(&CompanyTab::Represent),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-industry")],
                is_active: self.company_tab == CompanyTab::Represent,
                is_extend: self.check_extend(&CompanyTab::Represent),
            },
            // components MenuItem
            MenuItem {
                title: get_value_field(&154).to_string(), // COMPONENTS
                action: self.cb_generator(CompanyTab::Components),
                count: self.get_number_of_items(&CompanyTab::Components),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-cogs")],
                is_active: self.company_tab == CompanyTab::Components,
                is_extend: self.check_extend(&CompanyTab::Components),
            },
            // services MenuItem
            MenuItem {
                title: get_value_field(&379).to_string(), // SERVICES
                action: self.cb_generator(CompanyTab::Services),
                count: self.get_number_of_items(&CompanyTab::Services),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-ticket-alt")],
                is_active: self.company_tab == CompanyTab::Services,
                is_extend: self.check_extend(&CompanyTab::Services),
            },
            // standards MenuItem
            MenuItem {
                title: get_value_field(&103).to_string(), // STANDARDS
                action: self.cb_generator(CompanyTab::Standards),
                count: self.get_number_of_items(&CompanyTab::Standards),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("fas", "fa-book")],
                is_active: self.company_tab == CompanyTab::Standards,
                is_extend: self.check_extend(&CompanyTab::Standards),
            },
            // discussion MenuItem
            MenuItem {
                title: get_value_field(&380).to_string(), // DISCUSSION
                action: self.cb_generator(CompanyTab::Discussion),
                count: self.get_number_of_items(&CompanyTab::Discussion),
                item_class: classes!("has-background-white"),
                icon_classes: vec![classes!("far", "fa-comments")],
                is_active: self.company_tab == CompanyTab::Discussion,
                is_extend: self.check_extend(&CompanyTab::Discussion),
            },
            // memebers MenuItem
            // MenuItem {
            //     title: get_value_field(&286).to_string(), // MEMBERS
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
              CompanyTab::Info => 0,
              CompanyTab::Certificates => company.company_certificates.len(),
              CompanyTab::Represent => company.company_represents.len(),
              CompanyTab::Components => 0,
              CompanyTab::Services => 0,
              CompanyTab::Standards => 0,
              CompanyTab::Discussion => 0,
              // CompanyTab::Members => 0,
            },
            None => 0,
        }
    }
}
