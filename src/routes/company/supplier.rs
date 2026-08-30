use yew::{classes, html, Callback, Component, Properties, ComponentLink, Html, ShouldRender};
use yew_router::service::RouteService;
use web_sys::MouseEvent;
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::{
    buttons::ft_follow_btn,
    clipboard::ShareLinkBtn,
    list_errors::ListErrors,
    side_menu::{MenuBuilder, MenuItemTemplate},
    company::diamond_svg,
    supplier_service::ServiceRequestBtn,
    company::{view_certificates, view_components, view_content, view_represents, view_standards},
    responsive::resizer,
};
use crate::services::content_adapter::ContentDisplay;
use crate::services::{Counter, LocaleKey, resp_parsing, title_changer};
use crate::types::{UUID, CompanyInfo};
use crate::gqls::make_query;
use crate::gqls::company::{
    GetSupplierCompanyData, get_supplier_company_data,
    AddCompanyFav, add_company_fav,
    DeleteCompanyFav, delete_company_fav,
};

impl MenuBuilder for ShowSupplierCompany {
    type TabType = CompanyTab;

    fn menu_config() -> &'static [MenuItemTemplate<CompanyTab>] {
        use CompanyTab::*;
        &[
            MenuItemTemplate { lk_title: LocaleKey::Information, icon_classes: &[&["fas", "fa-info"]], tab: Info, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::CertificatesLabel, icon_classes: &[&["fas", "fa-certificate"]], tab: Certificates, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::Representations, icon_classes: &[&["fas", "fa-industry"]], tab: Represent, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::ComponentsLabel, icon_classes: &[&["fas", "fa-cogs"]], tab: Components, custom_class: None },
            MenuItemTemplate { lk_title: LocaleKey::Standards, icon_classes: &[&["fas", "fa-book"]], tab: Standards, custom_class: None },
        ]
    }

    fn is_active(&self, tab: &CompanyTab) -> bool { self.company_tab == *tab}
    fn get_count(&self, tab: &CompanyTab) -> usize { self.get_number_of_items(tab)}
    fn is_extend(&self, tab: &CompanyTab) -> bool { self.check_extend(tab)}
    fn get_action(&self, tab: &CompanyTab) -> Callback<MouseEvent> { self.cb_generator(tab.clone())}
}


/// Company with relate data
pub struct ShowSupplierCompany {
    error: Option<Error>,
    company: Option<CompanyInfo>,
    current_company_uuid: UUID,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    company_tab: CompanyTab,
    extend_tab: Option<CompanyTab>,
}

impl Counter for ShowSupplierCompany {
    fn quantity(&self) -> usize {
        self.subscribers
    }
}

#[derive(Properties, Clone)]
pub struct Props {
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
    Standards,
}

impl Component for ShowSupplierCompany {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ShowSupplierCompany {
            error: None,
            company: None,
            current_company_uuid: String::new(),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            company_tab: CompanyTab::Info,
            extend_tab: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // get company uuid for request company data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_company_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/supplier/")
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
                let res = make_query(GetSupplierCompanyData::build_query(get_supplier_company_data::Variables {
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
                match resp_parsing::<CompanyInfo>(res, "supplierCompany") {
                    Ok(company_data) => {
                        debug!("Supplier company data: {:?}", company_data);
                        self.subscribers = company_data.subscribers.to_owned();
                        self.is_followed = company_data.is_followed.to_owned();
                        self.current_company_uuid = company_data.uuid.to_owned();
                        self.company = Some(company_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ChangeTab(set_tab) => self.company_tab = set_tab,
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
                    <div class="container is-fluid page">
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

impl ShowSupplierCompany {
    fn view_card(&self) -> Html {
        match &self.company {
            Some(company_data) => html!{
              <div class="columns">
                <div class="box">
                  <figure class={classes!("container", "image", "is-128x128")}>
                    <img
                        src={company_data.image_file.download_url.to_string()} alt="Favicon company"
                        loading="lazy"
                    />
                  </figure>
                </div>
                <div id="company-region" class={classes!("column", "is-three-fifths")}>
                <abbr title={LocaleKey::SupplierLabel2.get_value()} style="position: absolute;margin-left: 10rem;">
                    {diamond_svg(company_data.is_supplier, "175")}
                </abbr>
                  {company_data.to_display()}
                </div>
                <div class="column m-0 p-0">
                  <div class="buttons flexBox" >
                    {self.show_favorite_btn()}
                    <ShareLinkBtn />
                    <ServiceRequestBtn company_uuid={company_data.uuid.clone()} />
                  </div>
                </div>
              </div>
            },
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
        html!{<div id="supplier-card-list" class="card">
            <div class="columns is-mobile">
                <div class="column is-flex">
                    {self.render_menu()}
                    <div id="supplier-card-list-items" class="card-relate-data" style={resizer("supplier-card-list", 5)}>
                        {match self.company_tab {
                            CompanyTab::Info => view_content(&company_data),
                            CompanyTab::Certificates => view_certificates(&company_data),
                            CompanyTab::Represent => view_represents(&company_data),
                            CompanyTab::Components => view_components(&company_data.uuid),
                            CompanyTab::Standards => view_standards(&company_data.uuid),
                        }}
                    </div>
                </div>
            </div>
        </div>}
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
              CompanyTab::Standards => 0,
            },
            None => 0,
        }
    }
}
