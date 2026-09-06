use yew::{agent::Bridged, services::fetch::FetchTask, html, Bridge, Component, Properties, ComponentLink, Html, ShouldRender, InputData};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::fragments::form_input::InputConfig;
use crate::fragments::markdown_edit::MarkdownEditCard;
use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_create_btn;
use crate::fragments::conditions::ConditionsBlock;
use crate::services::{Auth, get_from_value, LocaleKey, get_value_response, is_authenticated, resp_parsing, set_token, set_logged_user};
use crate::types::{LoginInfo, LoginInfoWrapper, PreServiceRequestData, RegisterInfo, ServiceCreateData, ShowCompanyShort, UserToken, UUID};
use crate::gqls::make_query;
use crate::gqls::user::{GetMySelf, get_my_self};
use crate::gqls::supplier_service::{
    GetServiceDataOpt, get_service_data_opt,
    ServiceRequest, service_request,
};


/// Service with relate data
pub struct CreateService {
    error: Option<Error>,
    request_service: ServiceCreateData,
    request_user: RegisterInfo,
    auth: Auth,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    link: ComponentLink<Self>,
    new_service_uuid: UUID,
    company_list: Vec<ShowCompanyShort>,
    disable_create_btn: bool,
    click_create_btn: bool,
    user_entry: bool,
    loading: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub pre_service_req: Option<PreServiceRequestData>,
}

#[derive(Clone)]
pub enum Msg {
    RequestManager,
    RequestCreateServiceData,
    LoginRequest,
    LoginResponse(Result<UserToken, Error>),
    GetResponseMySelf(String),
    GetListOpt(String),
    GetCreateServiceResult(String),
    UpdateName(String),
    UpdateDescription(String),
    UpdateCompanyUuid(UUID),
    UpdateEmail(String),
    UpdateTel(String),
    UpdateUsername(String),
    UpdatePassword(String),
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for CreateService {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateService {
            error: None,
            request_service: ServiceCreateData::new(),
            request_user: RegisterInfo::default(),
            auth: Auth::new(),
            task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
            new_service_uuid: String::new(),
            company_list: Vec::new(),
            disable_create_btn: false,
            click_create_btn: false,
            user_entry: is_authenticated(),
            loading: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            match &self.props.pre_service_req {
                Some(service_req) if service_req.calc_params.is_empty() => {
                    self.link.send_message(Msg::UpdateCompanyUuid(service_req.company_uuid.clone()));
                },
                Some(service_req) => {
                    self.link.send_message(Msg::UpdateName(format!("Заказ по материалу {}", service_req.calc_params[1].1)));
                    self.link.send_message(Msg::UpdateDescription(format!("Описание калькуляции: {:?}. \nСтоимость: {:?}", service_req.calc_params, service_req.cost)));
                    self.link.send_message(Msg::UpdateCompanyUuid(service_req.company_uuid.clone()));
                },
                None => {
                    if self.user_entry {
                        let link = self.link.clone();
                        spawn_local(async move {
                            let ipt_companies_arg = get_service_data_opt::IptCompaniesArg{
                                companiesUuids: None,
                                userUuid: None,
                                favorite: None,
                                supplier: Some(true),
                                search: None,
                                excludeUuids: None,
                            };
                            let res = make_query(GetServiceDataOpt::build_query(get_service_data_opt::Variables {
                                ipt_companies_arg
                            })).await.unwrap();
                            link.send_message(Msg::GetListOpt(res));
                        })
                    }
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestManager => {
                self.click_create_btn = true;
                let mut flag = true;
                // checking have data
                if self.request_service.company_uuid.is_empty() {
                    debug!("company_uuid is none: {:?}", self.request_service.company_uuid);
                    flag = false;
                }
                if self.request_service.name.is_empty() {
                    debug!("name is empty: {:?}", self.request_service.name);
                    flag = false;
                }
                if self.request_service.description.is_empty() {
                    debug!("description is empty: {:?}", self.request_service.description);
                    flag = false;
                }
                if !self.user_entry {
                    if self.request_user.username.is_empty() {
                        debug!("username is empty: {:?}", self.request_user.username);
                        flag = false;
                    }
                    if self.request_user.password.is_empty() {
                        debug!("password is empty: {:?}", self.request_user.password);
                        flag = false;
                    }
                    if self.request_user.email.is_empty() {
                        debug!("email is empty: {:?}", self.request_user.email);
                        flag = false;
                    }
                    if self.request_user.phone.is_empty() {
                        debug!("phone is empty: {:?}", self.request_user.phone);
                        flag = false;
                    }
                }
                self.disable_create_btn = flag;
                if flag {
                    link.send_message(Msg::RequestCreateServiceData);
                }
            },
            Msg::RequestCreateServiceData => {
                self.loading = true;
                let ipt_service_data = service_request::IptServiceData {
                    name: self.request_service.name.clone(),
                    description: self.request_service.description.clone(),
                    companyUuid: self.request_service.company_uuid.clone(),
                    regionId: self.request_service.region_id as i64,
                };
                let ipt_user_data = match self.user_entry {
                    true => None,
                    false => {
                        Some(service_request::IptUserData {
                            email: self.request_user.email.clone(),
                            username: self.request_user.username.clone(),
                            password: self.request_user.password.clone(),
                            firstname: Some(self.request_user.firstname.clone()),
                            lastname: Some(self.request_user.lastname.clone()),
                            secondname: Some(self.request_user.secondname.clone()),
                            phone: Some(self.request_user.phone.clone()),
                            description: Some(self.request_user.description.clone()),
                            address: Some(self.request_user.address.clone()),
                            timeZone: Some(self.request_user.time_zone.clone()),
                            position: Some(self.request_user.position.clone()),
                            regionId: Some(8_i64), // set region "Other"
                            programId: Some(self.request_user.program_id as i64),
                            typeAccessId: Some(self.request_user.type_access_id as i64),
                        })
                    },
                };
                spawn_local(async move {
                    let res = make_query(ServiceRequest::build_query(service_request::Variables {
                        ipt_service_data, ipt_user_data
                    })).await.unwrap();
                    link.send_message(Msg::GetCreateServiceResult(res));
                })
            },
            Msg::GetListOpt(res) => {
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.company_list = get_from_value(value, "companies").unwrap_or_default();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::LoginRequest => {
                self.task = Some(self.auth.login(
                    LoginInfoWrapper { user: LoginInfo {
                        username: self.request_user.username.clone(),
                        password: self.request_user.password.clone(),
                    }},
                    link.callback(Msg::LoginResponse)
                ));
            },
            Msg::LoginResponse(res) => {
                match res {
                    Ok(user_info) => {
                        set_token(Some(user_info.to_string()));
                        spawn_local(async move {
                            let res = make_query(GetMySelf::build_query(get_my_self::Variables)).await.unwrap();
                            link.send_message(Msg::GetResponseMySelf(res));
                        });
                    },
                    Err(err) => {
                        link.send_message(Msg::ResponseError(err));
                        self.task = None;
                    }
                }
            }
            Msg::GetResponseMySelf(res) => {
                debug!("res: {}", res);
                let data: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();
                let user_json = res.get("myself").unwrap().clone();
                set_logged_user(Some(user_json.to_string()));
                self.router_agent.send(
                    ChangeRoute(AppRoute::ShowService(self.new_service_uuid.clone()).into())
                );
            },
            Msg::GetCreateServiceResult(res) => {
                self.loading = false;
                match resp_parsing::<UUID>(res, "serviceRequest") {
                    Ok(result) => {
                        debug!("serviceRequest: {:?}", result);
                        if result.is_empty() {
                            return true;
                        }
                        self.new_service_uuid = result;
                        if self.user_entry {
                            self.router_agent.send(
                                ChangeRoute(AppRoute::ShowService(self.new_service_uuid.clone()).into())
                            );
                        } else {
                            link.send_message(Msg::LoginRequest);
                        }
                        return false
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            // items request create main service data
            Msg::UpdateName(data) => {
                self.request_service.name = data;
                self.disable_create_btn = self.request_service.name.is_empty();
            },
            Msg::UpdateDescription(data) => {
                self.request_service.description = data;
                self.disable_create_btn = self.request_service.description.is_empty();
            },
            Msg::UpdateCompanyUuid(data) => {
                self.request_service.company_uuid = data;
                self.disable_create_btn = self.request_service.company_uuid.is_empty();
            },
            Msg::UpdateEmail(data) => {
                self.request_user.email = data;
                self.disable_create_btn = self.request_user.email.is_empty();
            },
            Msg::UpdateTel(data) => {
                self.request_user.phone = data;
                self.disable_create_btn = self.request_user.phone.is_empty();
            },
            Msg::UpdateUsername(data) => {
                self.request_user.username = data;
                self.disable_create_btn = self.request_user.username.is_empty();
            },
            Msg::UpdatePassword(data) => {
                self.request_user.password = data;
                self.disable_create_btn = self.request_user.password.is_empty();
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            self.request_service = ServiceCreateData::new();
            self.request_user = RegisterInfo::default();
            self.new_service_uuid.clear();
            self.company_list.clear();
            self.disable_create_btn = false;
            self.click_create_btn = false;
            self.user_entry = false;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{
            <div class="service-page">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <div class="container is-fluid page">
                    <div class="block mb-5">
                        <h2 class="title is-4 mb-4">
                            // <span class="icon mr-3"><i class=""></i></span>
                            {LocaleKey::CreateOrder.get_value()}
                        </h2>
                        {self.show_main_card()}
                        {match self.user_entry {
                            true => html! {},
                            false => html! {
                                <div class="mt-5 pt-4" style="border-top: 1px solid #f0f0f0;">
                                    { self.show_user_data() }
                                </div>
                            },
                        }}
                        <div class="field mt-5">
                            <div class="control">
                                { self.show_manage_btn() }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateService {
    fn show_main_card(&self) -> Html {
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let name_has_error = self.request_service.name.is_empty() && self.click_create_btn;
        let oninput_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        html!{
            <div class="box mb-5">
                <div class="field mb-4">
                    {InputConfig::profile_input(
                        "create-service-name",
                        LocaleKey::Name,
                        Some(self.request_service.name.clone()),
                        oninput_name,
                        None,
                        self.loading,
                        name_has_error
                    )}
                </div>
                <MarkdownEditCard
                    id_tag={"create-service-description"}
                    title={LocaleKey::Description.get_value()}
                    placeholder={String::new()}
                    raw_text={self.request_service.description.clone()}
                    oninput_text={oninput_description}
                    />
            </div>
        }
    }

    fn show_user_data(&self) -> Html {
        let oninput_email = self.link.callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let email_error = self.request_user.email.is_empty() && self.click_create_btn;
        let oninput_tel = self.link.callback(|ev: InputData| Msg::UpdateTel(ev.value));
        let phone_error = self.request_user.phone.is_empty() && self.click_create_btn;
        let oninput_username = self.link.callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let username_error = self.request_user.username.is_empty() && self.click_create_btn;
        let oninput_password = self.link.callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let password_error = self.request_user.password.is_empty() && self.click_create_btn;
        html!{
            <div class="box mb-5">
                <div class="columns is-desktop mb-0">
                    <div class="column">
                        {InputConfig::profile_input(
                            "email",
                            LocaleKey::Email,
                            Some(self.request_user.email.clone()),
                            oninput_email,
                            Some("fas fa-envelope"),
                            self.loading,
                            email_error
                        )}
                    </div>
                    <div class="column">
                        {InputConfig::profile_input(
                            "tel",
                            LocaleKey::Phone,
                            Some(self.request_user.phone.clone()),
                            oninput_tel,
                            Some("fas fa-phone"),
                            self.loading,
                            phone_error
                        )}
                    </div>
                </div>
                <div class="columns is-desktop mb-4">
                    <div class="column">
                        {InputConfig::profile_input(
                            "username",
                            LocaleKey::Username,
                            Some(self.request_user.username.clone()),
                            oninput_username,
                            Some("fas fa-user"),
                            self.loading,
                            username_error
                        )}
                    </div>
                    <div class="column">
                        {InputConfig::profile_input(
                            "password",
                            LocaleKey::Password,
                            Some(self.request_user.password.clone()),
                            oninput_password,
                            Some("fas fa-lock"),
                            self.loading,
                            password_error
                        )}
                    </div>
                </div>
                <div class="block pt-2">
                    <ConditionsBlock />
                </div>
            </div>
        }
    }

    fn show_manage_btn(&self) -> Html {
        let onclick_create_changes = self.link.callback(|_| Msg::RequestManager);
        html!{
            <div class="field">
                <div class="control">
                    {ft_create_btn(
                        "create-service",
                        "is-medium".into(),
                        onclick_create_changes,
                        self.disable_create_btn,
                    )}
                </div>
            </div>
        }
    }
}
