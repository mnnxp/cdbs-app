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
use crate::fragments::company::ListItemCompany;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::{
    buttons::{ft_see_btn, ft_follow_btn, res_settings_btn},
    list_errors::ListErrors,
    component::CatalogComponents,
    standard::{StandardFilesCard, SpecsTags, KeywordsTags},
    img_showcase::ImgShowcase,
};
use crate::services::content_adapter::{DateDisplay, Markdownable};
use crate::services::{get_logged_user, get_value_field, resp_parsing, set_history_back, title_changer, Counter};
use crate::types::{ComponentsQueryArg, DownloadFile, Pathname, SlimUser, StandardInfo, UUID};
use crate::gqls::make_query;
use crate::gqls::standard::{
    GetStandardData, get_standard_data,
    StandardFiles, standard_files,
    AddStandardFav, add_standard_fav,
    DeleteStandardFav, delete_standard_fav,
};

/// Standard with relate data
pub struct ShowStandard {
    error: Option<Error>,
    standard: Option<StandardInfo>,
    current_standard_uuid: UUID,
    current_user_owner: bool,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    subscribers: usize,
    is_followed: bool,
    show_full_description: bool,
    show_related_components: bool,
    show_owner_company: bool,
    file_arr: Vec<DownloadFile>,
}

impl Counter for ShowStandard {
    fn quantity(&self) -> usize {
        self.subscribers
    }
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub standard_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    RequestDownloadFiles,
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    GetDownloadFilesResult(String),
    GetStandardData(String),
    ShowDescription,
    ShowComponentsList,
    ShowCompanyCard,
    OpenStandardSetting,
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for ShowStandard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ShowStandard {
            error: None,
            standard: None,
            current_standard_uuid: String::new(),
            current_user_owner: false,
            // task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            subscribers: 0,
            is_followed: false,
            show_full_description: false,
            show_related_components: false,
            show_owner_company: false,
            file_arr: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if let None = get_logged_user() {
            set_history_back(Some(String::new()));
            // route to login page if not found token
            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
        };

        // get standard uuid for request standard data
        let route_service: RouteService<()> = RouteService::new();
        // get target user from route
        let target_standard_uuid = route_service
            .get_fragment()
            .trim_start_matches("#/standard/")
            .to_string();
        // get flag changing current standard in route
        let not_matches_standard_uuid = target_standard_uuid != self.current_standard_uuid;
        // debug!("self.current_standard_uuid {:#?}", self.current_standard_uuid);

        if let Some(standard) = &self.standard {
            title_changer::set_title(standard.name.as_str());
        }

        if first_render || not_matches_standard_uuid {
            let link = self.link.clone();

            // update current_standard_uuid for checking change standard in route
            self.current_standard_uuid = target_standard_uuid.to_string();
            link.send_message(Msg::RequestDownloadFiles);
            spawn_local(async move {
                let res = make_query(GetStandardData::build_query(get_standard_data::Variables {
                    standard_uuid: target_standard_uuid,
                })).await.unwrap();

                link.send_message(Msg::GetStandardData(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFiles => {
                let standard_uuid = self.props.standard_uuid.clone();
                spawn_local(async move {
                    let ipt_standard_files_arg = standard_files::IptStandardFilesArg{
                        filesUuids: None,
                        standardUuid: standard_uuid,
                    };
                    let res = make_query(StandardFiles::build_query(standard_files::Variables{
                        ipt_standard_files_arg
                    })).await.unwrap();
                    link.send_message(Msg::GetDownloadFilesResult(res));
                })
            },
            Msg::Follow => {
                let standard_uuid_string = self.standard.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(AddStandardFav::build_query(add_standard_fav::Variables {
                        standard_uuid: standard_uuid_string,
                    })).await.unwrap();

                    link.send_message(Msg::AddFollow(res));
                })
            },
            Msg::AddFollow(res) => {
                match resp_parsing::<bool>(res, "addStandardFav") {
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
                let standard_uuid_string = self.standard.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(DeleteStandardFav::build_query(delete_standard_fav::Variables{
                        standard_uuid: standard_uuid_string,
                    })).await.unwrap();

                    link.send_message(Msg::DelFollow(res));
                })
            },
            Msg::DelFollow(res) => {
                match resp_parsing::<bool>(res, "deleteStandardFav") {
                    Ok(result) => {
                        if result {
                            self.subscribers -= 1;
                            self.is_followed = false;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetDownloadFilesResult(res) => {
                match resp_parsing::<Vec<DownloadFile>>(res, "standardFiles") {
                    Ok(mut result) => {
                        debug!("standardFiles: {:?}", result);
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
            Msg::GetStandardData(res) => {
                match resp_parsing::<StandardInfo>(res, "standard") {
                    Ok(standard_data) => {
                        debug!("Standard data: {:?}", standard_data);
                        self.subscribers = standard_data.subscribers;
                        self.is_followed = standard_data.is_followed;
                        self.current_standard_uuid = standard_data.uuid.clone();
                        if let Some(user) = get_logged_user() {
                            self.current_user_owner = standard_data.owner_user.uuid == user.uuid;
                        }
                        // description length check for show
                        self.show_full_description = standard_data.description.len() < 250;
                        // add main image
                        self.file_arr.push(standard_data.image_file.clone());
                        self.standard = Some(standard_data);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ShowDescription => self.show_full_description = !self.show_full_description,
            Msg::ShowComponentsList => self.show_related_components = !self.show_related_components,
            Msg::ShowCompanyCard => self.show_owner_company = !self.show_owner_company,
            Msg::OpenStandardSetting => {
                if let Some(standard_data) = &self.standard {
                    // Redirect to page for change and update standard
                    self.router_agent.send(ChangeRoute(AppRoute::StandardSettings(
                        standard_data.uuid.clone()
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
        if self.props.standard_uuid == props.standard_uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        match &self.standard {
            Some(standard_data) => html!{
                <div class="standard-page">
                    <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
                    <div class="container page">
                        <div class="row">
                            <div class="card column">
                              {self.show_main_card(standard_data)}
                            </div>
                            <br/>
                            {match &self.show_related_components {
                                true => html!{<>
                                    {self.show_related_components_card(&standard_data.uuid)}
                                    <br/>
                                </>},
                                false => html!{},
                            }}
                            {self.show_standard_files(standard_data)}
                            <br/>
                            <SpecsTags
                                show_manage_btn={false}
                                standard_uuid={standard_data.uuid.clone()}
                                specs={standard_data.standard_specs.clone()}
                            />
                            <br/>
                            <KeywordsTags
                                show_delete_btn={false}
                                standard_uuid={standard_data.uuid.clone()}
                                keywords={standard_data.standard_keywords.clone()}
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

impl ShowStandard {
    fn show_main_card(&self, standard_data: &StandardInfo) -> Html {
        let show_description_btn = self.link.callback(|_| Msg::ShowDescription);

        html!{
            <div class="columns">
              <ImgShowcase
                object_uuid={self.current_standard_uuid.clone()}
                file_arr={self.file_arr.clone()}
              />
              <div class="column">
                <div class="columns pb-0 mb-0">
                    <div class="column">
                        {get_value_field(&159)}{": "}
                        {standard_data.standard_status.name.clone()}
                    </div>
                    <div class="column">
                        {standard_data.type_access.get_with_icon()}
                    </div>
                    <div class="column is-narrow" title={get_value_field(&159)}>
                        <span class="icon is-small">
                            <i class={classes!("fa", "fa-edit")}></i>
                        </span>
                        {" "}
                        <span class="id-box">
                            {standard_data.publication_at.date_to_display()}
                        </span>
                    </div>
                </div>
                {self.show_modal_company_info(standard_data)}
                <div class="has-text-weight-bold is-size-4">
                    {standard_data.name.clone()}
                </div>
                <div class="buttons flexBox">
                    {self.show_related_components_btn()}
                    {self.show_setting_btn()}
                    {self.show_followers_btn()}
                    <ShareLinkBtn />
                </div>
                <div id={"standard-description"} class="pl-5 pr-5">
                    {match standard_data.description.len() {
                        250.. => html!{<>
                            {match self.show_full_description {
                                true => standard_data.description.to_markdown(),
                                false => format!("{:.*}", 200, standard_data.description).to_markdown(),
                            }}
                            {ft_see_btn(show_description_btn, self.show_full_description)}
                        </>},
                        _ => standard_data.description.to_markdown(),
                    }}
                </div>
              </div>
            </div>
        }
    }

    fn show_standard_files(&self, standard_data: &StandardInfo) -> Html {
        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&153)}</p> // Files
                </header>
                <div class="card-content">
                    <div class="content">
                        <StandardFilesCard
                            show_delete_btn={false}
                            standard_uuid={standard_data.uuid.clone()}
                            files={standard_data.standard_files.clone()}
                        />
                    </div>
                </div>
            </div>
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

    fn show_related_components_card(&self, standard_uuid: &UUID) -> Html {
        html!{
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{get_value_field(&154)}</p> // Components
                </header>
                <div class="card-content">
                    <div class="content">
                        <CatalogComponents
                            show_create_btn={false}
                            arguments={ComponentsQueryArg::set_standard_uuid(standard_uuid)}
                            />
                    </div>
                </div>
            </div>
        }
    }

    fn show_setting_btn(&self) -> Html {
        let onclick_setting_standard_btn = self.link.callback(|_| Msg::OpenStandardSetting);
        match &self.current_user_owner {
            true => {res_settings_btn(
                onclick_setting_standard_btn,
                Pathname::StandardSetting(self.current_standard_uuid.clone())
            )},
            false => html!{},
        }
    }

    fn show_modal_company_info(&self, standard_data: &StandardInfo) -> Html {
        let onclick_company_data_info = self.link.callback(|_| Msg::ShowCompanyCard);
        let class_modal = match &self.show_owner_company {
            true => "modal is-active",
            false => "modal",
        };

        html!{<>
            {get_value_field(&109)}{" "}
            <a class={"has-text-grey-light has-text-weight-bold"} onclick={onclick_company_data_info.clone()} >
                {standard_data.owner_company.shortname.clone()}
            </a>
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_company_data_info.clone()} />
                <div class="card">
                    <ListItemCompany
                        data={standard_data.owner_company.clone()}
                        show_list={true}
                    />
                </div>
              <button class="modal-close is-large" aria-label="close" onclick={onclick_company_data_info} />
            </div>
        </>}
    }
}
