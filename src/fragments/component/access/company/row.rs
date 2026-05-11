use yew::{classes, html, Callback, ChangeData, Component, ComponentLink, Html, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_delete_pair_btn;
use crate::fragments::notification::show_notification;
use crate::services::{get_value_field, resp_parsing};
use crate::services::content_adapter::DateDisplay;
use crate::types::{CompanyAccess, PermissionLevel, UUID};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    SetCompanyAccessComponent, set_company_access_component,
    DeleteCompanyAccessComponent, delete_company_access_component,
};

/// Company access row component
pub(crate) struct CompanyAccessRow {
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
    pub(crate) company_access: CompanyAccess,
    pub(crate) permissions: Vec<PermissionLevel>,
    pub(crate) on_delete: Callback<UUID>,
    pub(crate) number: usize,
}

pub(crate) enum Msg {
    ChangePermission(i64),
    ChangePermissionResults(String),
    RequestDelete,
    ConfirmDelete(String),
    CancelDelete,
    ResponseError(Error),
    ClearError,
}

impl Component for CompanyAccessRow {
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
                let var_set_company_access_component = set_company_access_component::IptCompanyAccessComponentData {
                    componentUuid: self.props.component_uuid.clone(),
                    companyUuid: self.props.company_access.company.uuid.clone(),
                    typeAccessId: permission_id,
                };
                spawn_local(async move {
                    let res = make_query(SetCompanyAccessComponent::build_query(
                        set_company_access_component::Variables { var_set_company_access_component }
                    )).await.unwrap();
                    link.send_message(Msg::ChangePermissionResults(res));
                });
            }
            Msg::ChangePermissionResults(res) => {
                self.loading = false;
                match resp_parsing(res, "setCompanyAccessComponent") {
                    Ok(value) => {
                        debug!("Change user access returned {}", value);
                        self.is_changed = value;
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::RequestDelete => {
                if self.delete_confirm == self.props.company_access.company.uuid {
                    let var_delete_company_access_component = delete_company_access_component::DelCompanyAccessComponentData {
                        componentUuid: self.props.component_uuid.clone(),
                        companyUuid: self.props.company_access.company.uuid.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(DeleteCompanyAccessComponent::build_query(
                            delete_company_access_component::Variables { var_delete_company_access_component }
                        )).await.unwrap();
                        link.send_message(Msg::ConfirmDelete(res));
                    });
                } else {
                    self.delete_confirm = self.props.company_access.company.uuid.clone();
                }
            }
            Msg::ConfirmDelete(res) => {
                self.delete_confirm.clear();
                match resp_parsing(res, "deleteCompanyAccessComponent") {
                    Ok(value) => {
                        debug!("Delete company access returned {}", value);
                        if value {
                            self.props.on_delete.emit(self.props.company_access.company.uuid.clone());
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::CancelDelete => self.delete_confirm.clear(),
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
        self.props.company_access.company.uuid == props.company_access.company.uuid &&
        self.props.company_access.permission.type_access_id == props.company_access.permission.type_access_id {
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
        let current_type_access_id = self.props.company_access.permission.type_access_id;
        let onclick_del =  self.link.callback(|confirm| {
            if confirm { Msg::RequestDelete } else { Msg::CancelDelete }
        });
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
            {show_notification(
                get_value_field(&214),
                "is-success",
                self.is_changed,
            )}
            <tr id={format!("user-rbac-{}-{}", self.props.component_uuid, self.props.number)}>
                <td>{self.props.number}</td>
                // <td>{truncate_uuid(&self.props.company_access.company.uuid)}</td>
                <td>{self.props.company_access.company.shortname.clone()}</td>
                <td>{PermissionLevel::render_permission_select(
                    &self.props.permissions,
                    current_type_access_id,
                    onchange_permission_level
                )}</td>
                <td>{self.props.company_access.created_at.date_to_display()}</td>
                <td>
                    {ft_delete_pair_btn(
                        &format!("delete-company-access-{}", self.props.company_access.company.uuid),
                        onclick_del,
                        self.delete_confirm == self.props.company_access.company.uuid,
                        false,
                        classes!("is-small"),
                    )}
                </td>
            </tr>
        </>}
    }
}