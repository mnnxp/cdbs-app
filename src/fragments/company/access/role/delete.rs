use yew::{html, Component, Callback, ComponentLink, Html, Properties, ShouldRender, classes};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_delete_pair_btn;
use crate::services::resp_parsing;
use crate::types::{UUID, RoleInfo};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    DeleteCompanyRole, delete_company_role,
};

/// Add company member modal
pub(crate) struct DeleteCompanyRoleModal {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    removing: bool,
    delete_confirm: bool,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) role: RoleInfo,
    pub(crate) on_success: Callback<i64>,
}

pub(crate) enum Msg {
    DeleteRole,
    DeleteRoleResults(String),
    Close,
    ResponseError(Error),
    ClearError,
}

impl Component for DeleteCompanyRoleModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            removing: false,
            delete_confirm: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::DeleteRole => {
                if !self.delete_confirm {
                    self.delete_confirm = !self.delete_confirm;
                    return true;
                }
                self.removing = true;
                let var_delete_company_role = delete_company_role::DelRoleMemberData {
                    companyUuid: self.props.company_uuid.clone(),
                    roleId: self.props.role.role_member_id,
                };
                spawn_local(async move {
                    let res = make_query(DeleteCompanyRole::build_query(
                        delete_company_role::Variables { var_delete_company_role }
                    )).await.unwrap();
                    link.send_message(Msg::DeleteRoleResults(res));
                });
            }
            Msg::DeleteRoleResults(res) => {
                self.removing = false;
                match resp_parsing(res, "deleteCompanyRole") {
                    Ok(value) => {
                        debug!("Delete role returned {}", value);
                        if value {
                            self.props.on_success.emit(self.props.role.role_member_id);
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                self.delete_confirm = false;
            }
            Msg::Close => self.delete_confirm = !self.delete_confirm,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.company_uuid == props.company_uuid &&
        self.props.role.role_member_id == props.role.role_member_id {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_del =  self.link.callback(|confirm| {
            if confirm { Msg::DeleteRole } else { Msg::Close }
        });
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
            {ft_delete_pair_btn(
                &format!("btn-delete-role-id-{}-{}", self.props.company_uuid, self.props.role.role_member_id),
                onclick_del,
                self.delete_confirm,
                self.removing,
                classes!("is-small")
            )}
        </>}
    }
}