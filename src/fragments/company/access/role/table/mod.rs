mod row;

use row::RoleRow;

use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::services::get_value_field;
use crate::types::{CompanyRole, PermissionLevel, UUID};

/// Properties for the CompanyMemberRoleTable component
#[derive(Clone, Debug, Properties)]
pub(crate) struct Props {
    /// UUID of the company
    pub(crate) company_uuid: UUID,
    /// List of roles for the company
    pub(crate) company_roles: Vec<CompanyRole>,
    /// List of permission level
    pub(crate) permissions: Vec<PermissionLevel>,
    /// Callback to refresh roles and member data
    pub(crate) on_need_update: Callback<()>,
    /// Callback to refresh roles data
    pub(crate) on_access_change: Callback<()>,
    /// Callback to refresh roles and member data
    pub(crate) on_role_delete: Callback<i64>,
}

/// Component for managing company roles
pub(crate) struct CompanyMemberRoleTable {
    props: Props,
    // link: ComponentLink<Self>,
}

impl Component for CompanyMemberRoleTable {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            // link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.company_uuid == props.company_uuid &&
           self.props.company_roles == props.company_roles {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{
            <div class="table-container">
                <table class="table is-fullwidth is-striped is-hoverable">
                    <thead>
                        <tr>
                            <th>{"\u{2116}"}</th>
                            <th>{get_value_field(&467)}</th>
                            <th>{get_value_field(&468)}</th>
                            <th>{get_value_field(&461)}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for self.props.company_roles.iter().enumerate().map(|(number, role)| {
                            html!{
                                <RoleRow
                                    company_uuid={self.props.company_uuid.clone()}
                                    permissions={self.props.permissions.clone()}
                                    number={number + 1}
                                    company_role={role.clone()}
                                    on_need_update={self.props.on_need_update.clone()}
                                    on_change={self.props.on_access_change.clone()}
                                    on_delete={self.props.on_role_delete.clone()}
                                />
                            }
                        })}
                    </tbody>
                </table>
            </div>
        }
    }
}
