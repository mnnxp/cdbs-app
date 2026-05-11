use yew::{classes, html, Callback, ChangeData, Component, ComponentLink, Html, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_delete_pair_btn;
use crate::fragments::notification::show_notification;
use crate::fragments::user::GoToUser;
use crate::services::{get_value_field, resp_parsing};
use crate::services::content_adapter::DateDisplay;
use crate::types::{PermissionLevel, UserAccess, UUID};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    SetUserAccessComponent, set_user_access_component,
    DeleteUserAccessComponent, delete_user_access_component,
};

/// User access row component
pub(crate) struct UserAccessRow {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    delete_confirm: UUID,
    is_changed: bool,
    loading: bool,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) component_uuid: UUID,
    pub(crate) access: UserAccess,
    pub(crate) permissions: Vec<PermissionLevel>,
    pub(crate) on_delete: Callback<UUID>,
    pub(crate) number: usize,
}

pub(crate) enum Msg {
    ChangePermission(i64),
    ChangePermissionResults(String),
    RequestDelete(bool),
    ConfirmDelete(String),
    ResponseError(Error),
    ClearError,
}

impl Component for UserAccessRow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            delete_confirm: String::new(),
            is_changed: false,
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ChangePermission(permission_id) => {
                if permission_id < 1 || self.loading {
                    return true
                }
                self.loading = true;
                let var_set_user_access_component = set_user_access_component::IptUserAccessComponentData {
                    componentUuid: self.props.component_uuid.clone(),
                    userUuid: self.props.access.user.uuid.clone(),
                    typeAccessId: permission_id,
                };
                spawn_local(async move {
                    let res = make_query(SetUserAccessComponent::build_query(
                        set_user_access_component::Variables { var_set_user_access_component }
                    )).await.unwrap();
                    link.send_message(Msg::ChangePermissionResults(res));
                });
            }
            Msg::ChangePermissionResults(res) => {
                self.loading = false;
                match resp_parsing(res, "setUserAccessComponent") {
                    Ok(value) => {
                        debug!("Change user access returned {}", value);
                        self.is_changed = value;
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::RequestDelete(confirm) => {
                if self.delete_confirm == self.props.access.user.uuid {
                    if !confirm {
                        self.delete_confirm.clear();
                        return true
                    }
                    debug!("Deleting user access: user={} from component={}", self.props.access.user.uuid, self.props.component_uuid);
                    let var_delete_user_access_component = delete_user_access_component::DelUserAccessComponentData {
                        componentUuid: self.props.component_uuid.clone(),
                        userUuid: self.props.access.user.uuid.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteUserAccessComponent::build_query(
                            delete_user_access_component::Variables { var_delete_user_access_component }
                        )).await.unwrap();
                        link.send_message(Msg::ConfirmDelete(res));
                    });
                } else {
                    self.delete_confirm = self.props.access.user.uuid.clone();
                }
            }
            Msg::ConfirmDelete(res) => {
                self.delete_confirm.clear();
                match resp_parsing(res, "deleteUserAccessComponent") {
                    Ok(value) => {
                        debug!("Delete user access returned {}", value);
                        if value {
                            self.props.on_delete.emit(self.props.access.user.uuid.clone());
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
        self.props.access.user.uuid == props.access.user.uuid {
            false
        } else {
            self.props = props;
            self.delete_confirm.clear();
            self.is_changed = false;
            self.loading = false;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onchange_permission_level = self.link.callback(|ev: ChangeData| {
            if let ChangeData::Select(el) = ev {
                Msg::ChangePermission(el.value().parse().unwrap_or(3))
            } else {
                Msg::ChangePermission(3)
            }
        });
        let current_type_access_id = self.props.access.permission.type_access_id;
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
            {show_notification(
                get_value_field(&214),
                "is-success",
                self.is_changed,
            )}
            <tr id={format!("user-rbac-{}-{}", self.props.component_uuid, self.props.number)}>
                <td>{self.props.number}</td>
                // <td>{truncate_uuid(&self.props.access.user.uuid)}</td>
                <td><GoToUser data={self.props.access.user.clone()}/></td>
                <td>{PermissionLevel::render_permission_select(
                    &self.props.permissions,
                    current_type_access_id,
                    onchange_permission_level
                )}</td>
                <td>{self.props.access.created_at.date_to_display()}</td>
                <td>
                    {ft_delete_pair_btn(
                        &format!("delete-user-access-{}", self.props.access.user.uuid),
                        self.link.callback(|confirm| Msg::RequestDelete(confirm)),
                        self.delete_confirm == self.props.access.user.uuid,
                        false,
                        classes!("is-small"),
                    )}
                </td>
            </tr>
        </>}
    }
}