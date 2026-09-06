use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender, classes};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::buttons::ft_custom_btn;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::modal::ModalBlock;
use crate::services::{LocaleKey, resp_parsing};
use crate::types::{CompanyRole, PermissionLevel, UUID};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    AddAccessRole, add_access_role,
    DeleteAccessRole, delete_access_role,
    // ChangeNameRoleCompany, change_name_role_company,
};

/// Properties for the EditCompanyRoleModal component
#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) company_role: CompanyRole,
    pub(crate) permissions: Vec<PermissionLevel>,
    pub(crate) on_need_update: Callback<()>,
    pub(crate) on_access_change: Callback<()>,
}

/// Messages for the EditCompanyRoleModal component
#[derive(Clone)]
pub(crate) enum Msg {
    ShowEditAccessRoleModal,
    ToggleAccess(i64),
    Submit,
    AddResult(String),
    RemoveResult(String),
    AwaitResults,
    ResponseError(Error),
    ClearError,
}

/// Modal component for editing company role access permissions
pub(crate) struct EditCompanyRoleModal {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    selected_access_ids: Vec<i64>,
    add_type_access_id: Vec<i64>,
    remove_type_access_id: Vec<i64>,
    wait_count: u8,
    is_active: bool,
    submitting: bool,
}

impl Component for EditCompanyRoleModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut selected_access_ids: Vec<i64> = Vec::new();
        props.company_role.permissions
            .iter()
            .for_each(|a| selected_access_ids.push(a.type_access_id as i64));

        Self {
            error: None,
            props,
            link,
            selected_access_ids,
            add_type_access_id: Vec::new(),
            remove_type_access_id: Vec::new(),
            wait_count: 0,
            is_active: false,
            submitting: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::ShowEditAccessRoleModal => {
                self.is_active = !self.is_active;
                // Reset to original state when opening
                if self.is_active {
                    self.selected_access_ids.clear();
                    for ca in &self.props.company_role.permissions {
                        self.selected_access_ids.push(ca.type_access_id as i64);
                    }
                    self.add_type_access_id.clear();
                    self.remove_type_access_id.clear();
                    self.error = None;
                }
            }
            // todo!(ui: change name)
            // Msg::EditNameRoleModal => {
            //     // skip updating user list since role name hasn't changed
            //     if self.name_changed {
            //         self.props.on_need_update.emit(())
            //     } else {
            //         self.props.on_access_change.emit(())
            //     }
            // }
            Msg::ToggleAccess(access_id) => {
                if self.selected_access_ids.contains(&access_id) {
                    self.selected_access_ids.retain(|&id| id != access_id);
                } else {
                    self.selected_access_ids.push(access_id);
                }
                self.selected_access_ids.sort();
            }
            Msg::Submit => {
                if self.selected_access_ids.is_empty() {
                    debug!("Role must have at least one permission");
                    return true;
                }

                self.submitting = true;
                let mut current_access_ids = Vec::new();
                for ca in &self.props.company_role.permissions {
                    current_access_ids.push(ca.type_access_id as i64);
                }

                let to_add: Vec<i64> = self.selected_access_ids
                    .iter()
                    .filter(|&id| !current_access_ids.contains(id))
                    .cloned()
                    .collect();

                let to_remove: Vec<i64> = current_access_ids
                    .iter()
                    .filter(|&id| !self.selected_access_ids.contains(id))
                    .cloned()
                    .collect();

                match (to_add.is_empty(), to_remove.is_empty()) {
                    (true, true) => return true,
                    (false, false) => self.wait_count = 2,
                    _ => self.wait_count = 1,
                }

                let link = self.link.clone();
                let role_id = self.props.company_role.role.role_member_id;

                spawn_local(async move {
                    // Add new permissions
                    if !to_add.is_empty() {
                        let var_add_access_role = add_access_role::IptRoleAccessData {
                            roleId: role_id,
                            typesAccessIds: to_add,
                        };
                        let res = make_query(AddAccessRole::build_query(
                            add_access_role::Variables { var_add_access_role }
                        )).await.unwrap();
                        link.send_message(Msg::AddResult(res));
                    }
                    // Remove permissions
                    if !to_remove.is_empty() {
                        let var_delete_access_role = delete_access_role::DelRoleAccessData {
                            roleId: role_id,
                            typesAccessIds: to_remove,
                        };
                        let res = make_query(DeleteAccessRole::build_query(
                            delete_access_role::Variables { var_delete_access_role }
                        )).await.unwrap();
                        link.send_message(Msg::RemoveResult(res));
                    }
                });
            }
            Msg::AddResult(res) => {
                match resp_parsing::<usize>(res, "addAccessRole") {
                    Ok(value) => {
                        debug!("Access added successfully: {}", value);
                        self.add_type_access_id.clear();
                        link.send_message(Msg::AwaitResults);
                    }
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.submitting = false;
            }
            Msg::RemoveResult(res) => {
                match resp_parsing::<i32>(res, "deleteAccessRole") {
                    Ok(value) => {
                        debug!("Access removed successfully: {}", value);
                        self.remove_type_access_id.clear();
                        link.send_message(Msg::AwaitResults);
                    }
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.submitting = false;
            }
            Msg::AwaitResults => {
                debug!("Waiting results: {}", self.wait_count);
                // Don't close modal yet - wait for both operations
                if self.wait_count > 1 {
                    self.wait_count -= 1;
                } else {
                    self.submitting = false;
                    self.is_active = false;
                    self.props.on_access_change.emit(());
                }
            }
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.company_uuid == props.company_uuid &&
        self.props.company_role == props.company_role &&
        self.props.permissions.len() == props.permissions.len() {
            false
        } else {
            self.selected_access_ids.clear();
            props.company_role.permissions
                .iter()
                .for_each(|a| self.selected_access_ids.push(a.type_access_id as i64));
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let callback_event_edit_role = self.link.callback(|_| Msg::ShowEditAccessRoleModal);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
            {ft_custom_btn(
                &format!("btn-edit-role-{}", self.props.company_role.role.role_member_id),
                LocaleKey::Edit.get_value(),
                classes!("button", "is-info", "is-light", "is-fullwidth", "is-small"),
                "fas fa-pencil-alt",
                callback_event_edit_role,
                false
            )}
            <ModalBlock
                modal_id="edit-role"
                title={LocaleKey::EditRole.get_value()}
                is_active={self.is_active}
                on_close={self.link.callback(|_| Msg::ShowEditAccessRoleModal)}
                on_save={Some(self.link.callback(|_| Msg::Submit))}
                save_disabled={self.submitting}
            >
                { self.modal_content() }
            </ModalBlock>
        </>}
    }
}

impl EditCompanyRoleModal {
    fn modal_content(&self) -> Html {
        html!{<>
            <div class="field">
                <label class="label">{LocaleKey::RoleName.get_value()}</label>
                <div class="control">
                    <input
                        class="input"
                        type="text"
                        value={self.props.company_role.role.name.clone()}
                        disabled={true}
                    />
                </div>
                <p class="help">
                    <span class="mr-3">{LocaleKey::RoleID.get_value()}</span>
                    <span>{self.props.company_role.role.role_member_id}</span>
                </p>
            </div>
            <div class="field mt-5">
                <label class="label">{LocaleKey::AccessPermissions.get_value()}</label>
                <p class="help mb-3">{LocaleKey::SelectPermissions.get_value()}</p>
                <div class="box has-background-light">
                    {for self.props.permissions.iter().map(|permission| {
                        self.show_access_level_item(permission)
                    })}
                </div>
            </div>
            {if self.selected_access_ids.is_empty() {
                html!{
                    <div class="notification is-warning is-light">
                        <span class="icon">
                            <i class="fas fa-exclamation-triangle"></i>
                        </span>
                        {LocaleKey::RoleNoPermissions.get_value()}
                    </div>
                }
            } else {
                html!{}
            }}
        </>}
    }

    fn show_access_level_item(
        &self,
        permission: &PermissionLevel,
    ) -> Html {
        let id_clone = permission.type_access_id as i64;
        html!{
            <div class="field">
                <div class="control">
                    <label class="checkbox is-flex is-align-items-center">
                        <input
                            type="checkbox"
                            checked={self.selected_access_ids.contains(&id_clone)}
                            onchange={self.link.callback(move |_| Msg::ToggleAccess(id_clone))}
                            class="mr-3"
                        />
                        {permission.render_access_level_tag(classes!("is-medium"))}
                        // <span class="ml-3 has-text-grey is-size-7">
                        //     {description}
                        // </span>
                    </label>
                </div>
            </div>
        }
    }
}