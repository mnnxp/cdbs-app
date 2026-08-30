mod member;
mod role;

pub(crate) use member::CompanyMembersCard;
pub(crate) use role::CompanyMemberRoleCard;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::services::{LocaleKey, resp_parsing};
use crate::types::{CompanyMember, CompanyRole, PermissionLevel, UUID};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    GetPermissions, get_permissions,
    GetCompanyMembers, get_company_members,
    GetCompanyRoles, get_company_roles,
};

/// Main company members management component
pub(crate) struct CompanyAccessBlock {
    error: Option<Error>,
    company_uuid: UUID,
    members: Vec<CompanyMember>,
    company_roles: Vec<CompanyRole>,
    permissions: Vec<PermissionLevel>,
    link: ComponentLink<Self>,
    loading: bool,
}

pub(crate) enum Msg {
    LoadData,
    GetPermissions,
    PermissionsResult(String),
    LoadMembersData,
    MembersLoaded(String),
    LoadRolesData,
    RolesLoaded(String),
    RoleChanged((UUID, i64)),
    MemberDeleted(UUID),
    ResponseError(Error),
    ClearError,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
}

impl Component for CompanyAccessBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            company_uuid: props.company_uuid.clone(),
            members: Vec::new(),
            company_roles: Vec::new(),
            permissions: Vec::new(),
            link,
            loading: true,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::LoadData);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::LoadData => {
                if self.permissions.is_empty() {
                    self.link.send_message(Msg::GetPermissions);
                }
                self.link.send_message(Msg::LoadMembersData);
                self.link.send_message(Msg::LoadRolesData);
            },
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
            Msg::LoadMembersData => {
                self.loading = true;
                let company_uuid = self.company_uuid.clone();
                debug!("Fetching members for company: {}", company_uuid);
                spawn_local(async move {
                    let res = make_query(GetCompanyMembers::build_query(
                        get_company_members::Variables { company_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::MembersLoaded(res));
                });
            }
            Msg::MembersLoaded(res) => {
                self.loading = false;
                match resp_parsing(res, "companyMembers") {
                    Ok(members) => self.members = members,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::LoadRolesData => {
                self.loading = true;
                let company_uuid = self.company_uuid.clone();
                debug!("Fetching roles for company: {}", company_uuid);
                spawn_local(async move {
                    let res = make_query(GetCompanyRoles::build_query(
                        get_company_roles::Variables { company_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::RolesLoaded(res));
                });
            }
            Msg::RolesLoaded(res) => {
                self.loading = false;
                match resp_parsing(res, "companyRoles") {
                    Ok(company_roles) => self.company_roles = company_roles,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::RoleChanged((user_uuid, role_id)) => {
                if let Some(member) = self.members.iter_mut().find(|m| m.user.uuid == user_uuid) {
                    if let Some(company_role) = self.company_roles.iter().find(|r| r.role.role_member_id == role_id) {
                        member.company_role = company_role.clone();
                    }
                }
            }
            Msg::MemberDeleted(user_uuid) => {
                self.members.retain(|m| m.user.uuid != user_uuid);
            }
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.company_uuid == props.company_uuid {
            false
        } else {
            self.company_uuid = props.company_uuid;
            self.link.send_message(Msg::LoadData);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclickon_need_update = self.link.callback(|_| Msg::LoadData);
        let onclick_role_delete = self.link.callback(|value| {
            debug!("Removed role ID: {}", value);
            Msg::LoadData
        });
        let onclick_access_change = self.link.callback(|_| Msg::LoadRolesData);
        let onclick_role_change = self.link.callback(|value| Msg::RoleChanged(value));
        let onclick_delete = self.link.callback(|value| Msg::MemberDeleted(value));
        let onclick_member_added = self.link.callback(|_| Msg::LoadMembersData);
        html! {<>
            <h4 id="settings-members" class="title is-4">{LocaleKey::Members.get_value()}</h4>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            <div class="columns">
                <div class="column">
                    <CompanyMemberRoleCard
                        company_uuid={self.company_uuid.clone()}
                        company_roles={self.company_roles.clone()}
                        permissions={self.permissions.clone()}
                        on_need_update={onclickon_need_update}
                        on_access_change={onclick_access_change}
                        on_role_delete={onclick_role_delete}
                    />
                </div>
                <div class="column">
                    <CompanyMembersCard
                        company_uuid={self.company_uuid.clone()}
                        company_roles={self.company_roles.clone()}
                        members={self.members.clone()}
                        loading={self.loading}
                        on_role_change={onclick_role_change}
                        on_delete={onclick_delete}
                        on_member_added={onclick_member_added}
                    />
                </div>
            </div>
        </>}
    }
}