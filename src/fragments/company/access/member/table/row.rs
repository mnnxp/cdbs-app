use yew::{classes, html, Callback, ChangeData, Component, ComponentLink, Html, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_delete_pair_btn;
use crate::fragments::notification::show_notification;
use crate::fragments::user::GoToUser;
use crate::services::content_adapter::DateDisplay;
use crate::services::{get_value_field, resp_parsing};
use crate::types::{UUID, CompanyMember, CompanyRole};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    ChangeRoleMember, change_role_member,
    DeleteCompanyMember, delete_company_member,
};

/// Member row component
pub(crate) struct MemberRow {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    delete_confirm: bool,
    change_role_id: i64,
    is_changed: bool,
    removing: bool,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) member: CompanyMember,
    pub(crate) company_roles: Vec<CompanyRole>,
    pub(crate) number: usize,
    pub(crate) on_role_change: Callback<(UUID, i64)>,
    pub(crate) on_delete: Callback<UUID>,
}

pub(crate) enum Msg {
    ChangeRole(i64),
    ChangeRoleResults(String),
    RequestDelete,
    DeleteResults(String),
    CancelDelete,
    ResponseError(Error),
    ClearError,
}

impl Component for MemberRow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            delete_confirm: false,
            change_role_id: 0,
            is_changed: false,
            removing: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ChangeRole(role_id) => {
                self.is_changed = false;
                self.change_role_id = role_id;
                let var_change_role_member = change_role_member::IptCompanyMemberData {
                    companyUuid: self.props.company_uuid.clone(),
                    userUuid: self.props.member.user.uuid.clone(),
                    roleId: role_id,
                };
                spawn_local(async move {
                    let res = make_query(ChangeRoleMember::build_query(
                        change_role_member::Variables { var_change_role_member }
                    )).await.unwrap();
                    link.send_message(Msg::ChangeRoleResults(res));
                });
            }
            Msg::ChangeRoleResults(res) => {
                match resp_parsing(res, "changeRoleMember") {
                    Ok(value) => {
                        debug!("Change role returned {}", value);
                        self.is_changed = value;
                        if value {
                            self.props.on_role_change.emit((
                                self.props.member.user.uuid.clone(),
                                self.change_role_id
                            ));
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::RequestDelete => {
                if self.delete_confirm {
                    self.removing = true;
                    let var_delete_company_member = delete_company_member::DelCompanyMemberData {
                        companyUuid: self.props.company_uuid.clone(),
                        userUuid: self.props.member.user.uuid.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteCompanyMember::build_query(
                            delete_company_member::Variables { var_delete_company_member }
                        )).await.unwrap();
                        link.send_message(Msg::DeleteResults(res));
                    });
                } else {
                    self.delete_confirm = true;
                }
            }
            Msg::DeleteResults(res) => {
                self.delete_confirm = false;
                match resp_parsing(res, "deleteCompanyMember") {
                    Ok(value) => {
                        debug!("Delete member returned {}", value);
                        if value {
                            self.props.on_delete.emit(self.props.member.user.uuid.clone());
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.removing = false;
            }
            Msg::CancelDelete => self.delete_confirm = false,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.member.user.uuid == props.member.user.uuid &&
        self.props.company_roles == props.company_roles {
            false
        } else {
            self.props = props;
            self.delete_confirm = false;
            self.is_changed = false;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onchange_role_member = self.link.callback(|ev: ChangeData| {
            if let ChangeData::Select(el) = ev {
                Msg::ChangeRole(el.value().parse().unwrap_or(1))
            } else {
                Msg::ChangeRole(1)
            }
        });
        let current_role_id = self.props.member.company_role.role.role_member_id;
        let onclick_del =  self.link.callback(|confirm| {
            if confirm { Msg::RequestDelete } else { Msg::CancelDelete }
        });
        html!{
            <>
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
                {show_notification(
                    get_value_field(&214),
                    "is-success",
                    self.is_changed,
                )}
                <tr>
                    <td>{self.props.number}</td>
                    // <td>{truncate_uuid(&self.props.member.user.uuid)}</td>
                    <td><GoToUser data={self.props.member.user.clone()}/></td>
                    <td>
                        <div class="select is-small">
                            <select
                                value={current_role_id.to_string()}
                                onchange={onchange_role_member}
                            >
                                {for self.props.company_roles.iter().map(|company_role| {
                                    let role_id = company_role.role.role_member_id;
                                    html!{
                                        <option value={role_id.to_string()} selected={role_id == current_role_id}>
                                            {company_role.role.name.clone()}
                                        </option>
                                    }
                                })}
                            </select>
                        </div>
                    </td>
                    <td>{self.props.member.created_at.date_to_display()}</td>
                    <td>
                        {ft_delete_pair_btn(
                            &format!("btn-delete-role-id-{}-{}", self.props.company_uuid, self.props.member.user.uuid),
                            onclick_del,
                            self.delete_confirm,
                            self.removing,
                            classes!("is-small")
                        )}
                    </td>
                </tr>
            </>
        }
    }
}
