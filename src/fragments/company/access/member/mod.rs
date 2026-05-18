mod add;
mod table;

pub(crate) use add::AddCompanyMemberModal;
pub(crate) use table::MembersTable;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender, Callback};

use crate::fragments::buttons::ft_add_btn;
use crate::fragments::switch_icon::res_loading_state;
use crate::services::get_value_field;
use crate::types::{UUID, CompanyMember, CompanyRole};

pub(crate) struct CompanyMembersCard {
    props: Props,
    link: ComponentLink<Self>,
    show_add_modal: bool,
}

pub(crate) enum Msg {
    ShowAddModal,
    MemberAdded,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) company_roles: Vec<CompanyRole>,
    pub(crate) members: Vec<CompanyMember>,
    pub(crate) loading: bool,
    pub(crate) on_role_change: Callback<(UUID, i64)>,
    pub(crate) on_delete: Callback<UUID>,
    pub(crate) on_member_added: Callback<()>,
}

impl Component for CompanyMembersCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            show_add_modal: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowAddModal => self.show_add_modal = !self.show_add_modal,
            Msg::MemberAdded => {
                self.show_add_modal = false;
                self.props.on_member_added.emit(());
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.company_uuid == props.company_uuid &&
        self.props.loading == props.loading &&
        self.props.company_roles == props.company_roles &&
        self.props.members.len() == props.members.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let callback_member_added = self.link.callback(|_| Msg::MemberAdded);
        let onclick_add_member = self.link.callback(|_| Msg::ShowAddModal);
        let callback_add_member = self.link.callback(|_| Msg::ShowAddModal);
        let existing_member_uuids: Vec<UUID> = self.props.members.iter()
            .map(|m| m.user.uuid.clone())
            .collect();

        html! {
            <div class="card">
                <header class="card-header">
                    <div class="card-header-title">
                        <p class="is-size-5 has-text-weight-semibold">
                            {get_value_field(&477)}
                        </p>
                        <div class="buttons right-side">
                            {ft_add_btn(
                                "add-member",
                                get_value_field(&478),
                                onclick_add_member,
                                false,
                                self.props.company_roles.is_empty(),
                            )}
                        </div>
                    </div>
                </header>
                <div class="card-content">
                    <div class="content">
                        {if self.props.loading {
                            res_loading_state()
                        } else {
                            self.render_members_table()
                        }}
                    </div>
                </div>
                <footer class="card-footer">
                    <div class="card-footer-item">
                        <span class="is-size-7 has-text-grey-light">
                            <span class="icon is-small">
                                <i class="fas fa-info-circle"></i>
                            </span>
                            {get_value_field(&479)}
                        </span>
                    </div>
                </footer>
                <AddCompanyMemberModal
                    company_uuid={self.props.company_uuid.clone()}
                    existing_member_uuids={existing_member_uuids}
                    company_roles={self.props.company_roles.clone()}
                    is_active={self.show_add_modal}
                    on_close={callback_add_member}
                    on_success={callback_member_added}
                />
            </div>
        }
    }
}

impl CompanyMembersCard {
    fn render_members_table(&self) -> Html {
        if self.props.members.is_empty() {
            html! {
                <div class="notification is-info is-light">
                    {get_value_field(&480)}
                </div>
            }
        } else {
            html! {
                <MembersTable
                    company_uuid={self.props.company_uuid.clone()}
                    members={self.props.members.clone()}
                    company_roles={self.props.company_roles.clone()}
                    on_role_change={self.props.on_role_change.clone()}
                    on_delete={self.props.on_delete.clone()}
                    on_member_added={self.props.on_member_added.clone()}
                />
            }
        }
    }
}