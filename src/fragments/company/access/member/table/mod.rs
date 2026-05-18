mod row;

use row::MemberRow;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender, Callback};

use crate::services::get_value_field;
use crate::types::{UUID, CompanyMember, CompanyRole};

/// Table component for displaying company members
pub(crate) struct MembersTable {
    props: Props,
    // link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub company_uuid: UUID,
    pub members: Vec<CompanyMember>,
    pub company_roles: Vec<CompanyRole>,
    pub on_role_change: Callback<(UUID, i64)>,
    pub on_delete: Callback<UUID>,
    pub on_member_added: Callback<()>,
}

impl Component for MembersTable {
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
           self.props.members.len() == props.members.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="table-container">
                <table class="table is-fullwidth is-striped">
                    <thead>
                        <tr>
                            <th>{"\u{2116}"}</th>
                            <th>{get_value_field(&19)}</th>
                            <th>{get_value_field(&471)}</th>
                            <th>{get_value_field(&483)}</th>
                            <th>{get_value_field(&111)}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for self.props.members.iter().enumerate().map(|(number, member)| {
                            html! {
                                <MemberRow
                                    company_uuid={self.props.company_uuid.clone()}
                                    member={member.clone()}
                                    company_roles={self.props.company_roles.clone()}
                                    number={number+1}
                                    on_role_change={self.props.on_role_change.clone()}
                                    on_delete={self.props.on_delete.clone()}
                                />
                            }
                        })}
                    </tbody>
                </table>
            </div>
        }
    }
}