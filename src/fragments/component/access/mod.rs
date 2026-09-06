mod company;
mod user;

pub(crate) use company::CompanyAccessComponentTable;
pub(crate) use user::UserAccessComponentTable;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::switch_icon::res_loading_state;
use crate::types::{CompanyAccess, OwnerInfo, PermissionLevel, UserAccess, UUID};
use crate::services::{get_from_value, get_value_response, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    GetPermissions, get_permissions,
    GetComponentAccessList, get_component_access_list
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub component_uuid: UUID,
    pub owner_info: Option<OwnerInfo>,
}

pub struct ComponentAccessBlock {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    access_users: Vec<UserAccess>,
    access_companies: Vec<CompanyAccess>,
    permissions: Vec<PermissionLevel>,
    loading: bool,
}

#[derive(Clone)]
pub enum Msg {
    GetPermissions,
    PermissionsResult(String),
    RefreshAccessData,
    AccessDataLoaded(String),
    ResponseError(Error),
    ClearError,
}

impl Component for ComponentAccessBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            access_users: Vec::new(),
            access_companies: Vec::new(),
            permissions: Vec::new(),
            loading: true,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::GetPermissions);
            self.link.send_message(Msg::RefreshAccessData);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::GetPermissions => {
                spawn_local(async move {
                    let res = make_query(GetPermissions::build_query(
                        get_permissions::Variables
                    )).await.unwrap();
                    link.send_message(Msg::PermissionsResult(res));
                });
            }
            Msg::PermissionsResult(res) => {
                match resp_parsing(res, "permissions") {
                    Ok(value) => {
                        debug!("Permissions: {:?}", value);
                        self.permissions = value;
                    }
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::RefreshAccessData => {
                let component_uuid = self.props.component_uuid.clone();
                spawn_local(async move {
                    let res = make_query(GetComponentAccessList::build_query(
                        get_component_access_list::Variables { component_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::AccessDataLoaded(res));
                })
            },
            Msg::AccessDataLoaded(res) => {
                self.loading = false;
                match get_value_response(res) {
                    Ok(value) => {
                        self.access_users = get_from_value(&value, "getUsersListAccessComponent").unwrap_or_default();
                        self.access_companies = get_from_value(&value, "getCompaniesListAccessComponent").unwrap_or_default();
                        debug!("AccessDataLoaded... Users: {}, Companies: {}.", self.access_users.len(), self.access_companies.len())
                    }
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.component_uuid == self.props.component_uuid {
            false
        } else {
            self.props = props;
            self.link.send_message(Msg::RefreshAccessData);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let callback_refresh_data = self.link.callback(|_| Msg::RefreshAccessData);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {if self.loading || self.permissions.is_empty() {
                res_loading_state()
            } else {
                html!{
                    <div class="columns">
                        <div class="column">
                            <UserAccessComponentTable
                                component_uuid={self.props.component_uuid.clone()}
                                users={self.access_users.clone()}
                                permissions={self.permissions.clone()}
                                need_update={callback_refresh_data.clone()}
                            />
                        </div>
                        <div class="column">
                            <CompanyAccessComponentTable
                                component_uuid={self.props.component_uuid.clone()}
                                companies={self.access_companies.clone()}
                                permissions={self.permissions.clone()}
                                need_update={callback_refresh_data}
                            />
                        </div>
                    </div>
                }
            }}
        </>}
    }
}