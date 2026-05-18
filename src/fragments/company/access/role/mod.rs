mod table;
mod create;
mod delete;
mod edit;

use table::CompanyMemberRoleTable;
use create::CreateCompanyRoleModal;

use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::fragments::buttons::ft_custom_btn;
use crate::services::get_value_field;
use crate::types::{CompanyRole, PermissionLevel, UUID};

#[derive(Clone, Debug, Properties)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) company_roles: Vec<CompanyRole>,
    pub(crate) permissions: Vec<PermissionLevel>,
    pub(crate) on_need_update: Callback<()>,
    pub(crate) on_access_change: Callback<()>,
    pub(crate) on_role_delete: Callback<i64>,
}

#[derive(Clone)]
pub(crate) enum Msg {
    ShowCreateRoleModal,
}

/// Component for managing company roles
pub(crate) struct CompanyMemberRoleCard {
    props: Props,
    link: ComponentLink<Self>,
    show_create_role_modal: bool,
}

impl Component for CompanyMemberRoleCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            show_create_role_modal: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowCreateRoleModal => self.show_create_role_modal = !self.show_create_role_modal,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.company_uuid == self.props.company_uuid &&
           props.company_roles == self.props.company_roles {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_show_create_role_modal = self.link.callback(|_| Msg::ShowCreateRoleModal);
        let callback_hide_create_modal = self.link.callback(|_| Msg::ShowCreateRoleModal);

        html!{
            <div class="card">
                <header class="card-header">
                    <div class="card-header-title">
                        <div class="is-flex is-align-items-center">
                            <p class="is-size-5 has-text-weight-semibold">
                                {get_value_field(&462)}
                            </p>
                        </div>
                        {if self.props.company_roles.is_empty() {
                            html!{}
                        } else {
                            html!{
                                <div class="buttons right-side">
                                    {ft_custom_btn(
                                        &format!("create-role-{}", self.props.company_uuid),
                                        get_value_field(&463),
                                        classes!("is-success"),
                                        "fas fa-user-tag",
                                        onclick_show_create_role_modal,
                                        false,
                                    )}
                                </div>
                            }
                        }}
                    </div>
                </header>
                <div class="card-content">
                    <div class="content">
                        {self.render_roles_table()}
                    </div>
                </div>
                <footer class="card-footer">
                    <div class="card-footer-item">
                        <span class="is-size-7 has-text-grey-light">
                            <span class="icon is-small">
                                <i class="fas fa-info-circle"></i>
                            </span>
                            {get_value_field(&498)}
                        </span>
                    </div>
                </footer>
                // Create Role Modal
                <CreateCompanyRoleModal
                    company_uuid={self.props.company_uuid.clone()}
                    is_active={self.show_create_role_modal}
                    on_close={callback_hide_create_modal}
                    on_success={self.props.on_access_change.clone()}
                />
            </div>
        }
    }
}

impl CompanyMemberRoleCard {
    /// Renders the complete roles table
    fn render_roles_table(&self) -> Html {
        if self.props.company_roles.is_empty() {
            self.render_empty_state()
        } else {
            html!{
                <CompanyMemberRoleTable
                    company_uuid={self.props.company_uuid.clone()}
                    company_roles={self.props.company_roles.clone()}
                    permissions={self.props.permissions.clone()}
                    on_need_update={self.props.on_need_update.clone()}
                    on_access_change={self.props.on_access_change.clone()}
                    on_role_delete={self.props.on_role_delete.clone()}
                />
            }
        }
    }

    /// Renders the empty state when no roles exist
    fn render_empty_state(&self) -> Html {
        let onclick_show_create_role_modal = self.link.callback(|_| Msg::ShowCreateRoleModal);
        html!{
            <div class="has-text-centered p-6">
                <div class="mb-4">
                    <span class="icon is-large has-text-grey-light">
                        <i class="fas fa-user-tag fa-3x"></i>
                    </span>
                </div>
                <h4 class="title is-5 has-text-grey">{get_value_field(&464)}</h4>
                <p class="subtitle is-6 has-text-grey-light">
                    {get_value_field(&465)}
                </p>
                {ft_custom_btn(
                    &format!("create-first-role-{}", self.props.company_uuid),
                    get_value_field(&466),
                    classes!("is-success", "mt-3"),
                    "fas fa-user-tag",
                    onclick_show_create_role_modal,
                    false,
                )}
            </div>
        }
    }
}