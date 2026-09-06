use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use log::debug;

use crate::fragments::company::access::role::delete::DeleteCompanyRoleModal;
use crate::fragments::company::access::role::edit::EditCompanyRoleModal;
use crate::services::LocaleKey;
use crate::types::{CompanyRole, PermissionLevel, UUID};

/// Role row component properties
#[derive(Properties, Clone)]
pub(crate) struct Props {
    /// Company Uuid
    pub(crate) company_uuid: UUID,
    /// Sequential number for display
    pub(crate) number: usize,
    /// Company role data to display
    pub(crate) company_role: CompanyRole,
    /// List of permission level
    pub(crate) permissions: Vec<PermissionLevel>,
    /// Callback to refresh roles and member data
    pub(crate) on_need_update: Callback<()>,
    /// Callback when edit accesses
    pub(crate) on_change: Callback<()>,
    /// Callback to refresh roles and member data
    pub(crate) on_delete: Callback<i64>,
}

/// Component for displaying a single role row in the roles table
pub(crate) struct RoleRow {
    props: Props,
}

impl Component for RoleRow {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        debug!("old role accesses: {}, new roles accesses: {}", self.props.company_role.permissions.len(), props.company_role.permissions.len());
        if self.props.company_role.role.role_member_id == props.company_role.role.role_member_id &&
        self.props.company_role.permissions == props.company_role.permissions {
            false
        } else {
            debug!("new roles accesses: {:?}", props.company_role.permissions);
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{
                <tr>
                    <td class="has-text-weight-semibold">{self.props.number}</td>
                    <td>
                        <div class="is-flex is-align-items-center is-flex-wrap-wrap">
                            <span>{self.props.company_role.role.name.clone()}</span>
                        </div>
                        <div class="is-size-7 has-text-grey mt-1">
                            <span class="mr-3">{LocaleKey::ID.get_value()}</span>
                            <span>{self.props.company_role.role.role_member_id}</span>
                        </div>
                    </td>
                    <td>
                        <div class="tags are-small">
                            {if self.props.company_role.permissions.is_empty() {
                                html!{
                                    <span class="tag is-light">
                                        <span class="icon is-small mr-1">
                                            <i class="fas fa-ban"></i>
                                        </span>
                                        {LocaleKey::NoPermissions.get_value()}
                                    </span>
                                }
                            } else {
                                html!{
                                    {for self.props.company_role.permissions.iter().map(|access| {
                                        access.render_access_level_tag(classes!("is-small"))
                                    })}
                                }
                            }}
                        </div>
                    </td>
                    <td>
                        <div class="buttons">
                            // Edit Role Button
                            <EditCompanyRoleModal
                                company_uuid={self.props.company_uuid.clone()}
                                company_role={self.props.company_role.clone()}
                                permissions={self.props.permissions.clone()}
                                on_need_update={self.props.on_need_update.clone()}
                                on_access_change={self.props.on_change.clone()}
                            />
                            // Delete Role Button
                            <DeleteCompanyRoleModal
                                company_uuid={self.props.company_uuid.clone()}
                                role={self.props.company_role.role.clone()}
                                on_success={self.props.on_delete.clone()}
                            />
                        </div>
                    </td>
                </tr>
        }
    }
}