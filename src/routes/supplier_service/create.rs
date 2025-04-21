use yew::{agent::Bridged, services::fetch::FetchTask, html, Bridge, Component, Properties, ComponentLink, Html, ShouldRender, InputData};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_create_btn;
use crate::fragments::conditions::ConditionsBlock;
use crate::services::{Auth, get_from_value, get_value_field, get_value_response, is_authenticated, resp_parsing, set_token, set_logged_user};
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
                <div class="container page">
                    <div class="row">
                        <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                        <h1 class="title">{get_value_field(&364)}</h1>
                        <div class="card">
                        {self.show_main_card()}
                        {match self.user_entry {
                            true => html!{},
                            false => self.show_user_data(),
                        }}
                        </div>
                        <br/>
                        {self.show_manage_btn()}
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateService {
    fn show_main_card(&self) -> Html {
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let class_name = match self.request_service.name.is_empty() && self.click_create_btn {
            true => "input is-danger",
            false => "input",
        };
        let oninput_description = self.link.callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let class_description = match self.request_service.description.is_empty() && self.click_create_btn {
            true => "textarea is-danger",
            false => "textarea",
        };

        html!{
              <div class={"column"}>
                <label class={"label"}>{get_value_field(&110)}</label>
                <div class={"control has-icons-right"}>
                <input
                    id={"update-name"}
                    class={class_name}
                    type={"text"}
                    placeholder={get_value_field(&110)}
                    value={self.request_service.name.clone()}
                    oninput={oninput_name} />
                    {match self.request_service.name.is_empty() && self.click_create_btn {
                        true => html!{
                            <span class="icon is-small is-right">
                                <i class="fas fa-exclamation-triangle"></i>
                            </span>
                        },
                        false => html!{},
                    }}
                </div>
                <label class="label">{get_value_field(&61)}</label>
                <div class={"control has-icons-right"}>
                <textarea
                    id={"update-description"}
                    class={class_description}
                    // rows="10"
                    type={"text"}
                    placeholder={get_value_field(&61)}
                    value={self.request_service.description.clone()}
                    oninput={oninput_description} />
                    {match self.request_service.description.is_empty() && self.click_create_btn {
                        true => html!{
                            <span class="icon is-small is-right">
                                <i class="fas fa-exclamation-triangle"></i>
                            </span>
                        },
                        false => html!{},
                    }}
                </div>
            </div>
        }
    }

    fn show_user_data(&self) -> Html {
        let oninput_email = self.link.callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let class_email = match self.request_user.email.is_empty() && self.click_create_btn {
            true => "input is-danger",
            false => "input",
        };
        let oninput_tel = self.link.callback(|ev: InputData| Msg::UpdateTel(ev.value));
        let class_tel = match self.request_user.phone.is_empty() && self.click_create_btn {
            true => "input is-danger",
            false => "input",
        };
        let oninput_username = self.link.callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let class_username = match self.request_user.username.is_empty() && self.click_create_btn {
            true => "input is-danger",
            false => "input",
        };
        let oninput_password = self.link.callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let class_password = match self.request_user.phone.is_empty() && self.click_create_btn {
            true => "input is-danger",
            false => "input",
        };
        html!{
            <div class={"column"}>
            <div class={"columns"}>
            <div class={"column"}>
                <label class={"label"}>{get_value_field(&22)}</label>
                <div class={"control has-icons-left has-icons-right"}>
                <input
                    id={"email"}
                    class={class_email}
                    type={"email"}
                    oninput={oninput_email}
                    value={self.request_user.email.clone()}
                    autocomplete={"on"}
                    />
                <span class={"icon is-small is-left"}>
                    <i class={"fas fa-envelope"}></i>
                </span>
                {match self.request_user.email.is_empty() && self.click_create_btn {
                    true => html!{
                        <span class="icon is-small is-right">
                            <i class="fas fa-exclamation-triangle"></i>
                        </span>
                    },
                    false => html!{},
                }}
                </div>
            </div>
            <div class={"column"}>
                <label class={"label"}>{get_value_field(&56)}</label>
                <div class={"control has-icons-left has-icons-right"}>
                <input
                    id={"phone"}
                    class={class_tel}
                    type={"tel"}
                    oninput={oninput_tel}
                    value={self.request_user.phone.clone()}
                    autocomplete={"tel"}
                    />
                <span class={"icon is-small is-left"}>
                    <i class={"fas fa-phone"}></i>
                </span>
                {match self.request_user.phone.is_empty() && self.click_create_btn {
                    true => html!{
                        <span class="icon is-small is-right">
                            <i class="fas fa-exclamation-triangle"></i>
                        </span>
                    },
                    false => html!{},
                }}
                </div>
            </div>
            </div>
            <div class={"columns"}>
            <div class={"column"}>
                <label class={"label"}>{get_value_field(&50)}</label>
                <div class={"control has-icons-left has-icons-right"}>
                <input
                    id={"username"}
                    class={class_username}
                    type={"username"}
                    oninput={oninput_username}
                    value={self.request_user.username.clone()}
                    autocomplete={"username"}
                    />
                <span class={"icon is-small is-left"}>
                    <i class={"fas fa-user"}></i>
                </span>
                {match self.request_user.username.is_empty() && self.click_create_btn {
                    true => html!{
                        <span class="icon is-small is-right">
                            <i class="fas fa-exclamation-triangle"></i>
                        </span>
                    },
                    false => html!{},
                }}
                </div>
            </div>
            <div class={"column"}>
                <label class={"label"}>{get_value_field(&20)}</label>
                <div class={"control has-icons-left has-icons-right"}>
                <input
                    id={"password"}
                    class={class_password}
                    type={"password"}
                    oninput={oninput_password}
                    value={self.request_user.password.clone()}
                    autocomplete={"new-password"}
                    />
                <span class={"icon is-small is-left"}>
                    <i class={"fas fa-lock"}></i>
                </span>
                {match self.request_user.phone.is_empty() && self.click_create_btn {
                    true => html!{
                        <span class="icon is-small is-right">
                            <i class="fas fa-exclamation-triangle"></i>
                        </span>
                    },
                    false => html!{},
                }}
                </div>
            </div>
            </div>
            <div class="column is-flex is-vcentered">
                <ConditionsBlock />
            </div>
            </div>
        }
    }

    fn show_manage_btn(&self) -> Html {
        let onclick_create_changes = self.link.callback(|_| Msg::RequestManager);
        ft_create_btn(
            "create-service",
            "is-medium".into(),
            onclick_create_changes,
            self.disable_create_btn,
        )
    }
}
