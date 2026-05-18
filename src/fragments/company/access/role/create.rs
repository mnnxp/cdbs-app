use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_modal_cancel_save_btn;
use crate::services::{get_value_field, resp_parsing, unique_id};
use crate::types::UUID;
use crate::gqls::make_query;
use crate::gqls::rbac::{
    RegisterCompanyRole, register_company_role,
};

/// Add company member modal
pub(crate) struct CreateCompanyRoleModal {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    request_name: String,
    creating: bool,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) is_active: bool,
    pub(crate) on_close: Callback<()>,
    pub(crate) on_success: Callback<()>,
}

pub(crate) enum Msg {
    UpdateRoleName(String),
    CreateRole,
    CreateRoleResults(String),
    Close,
    ResponseError(Error),
    ClearError,
}

impl Component for CreateCompanyRoleModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            request_name: String::new(),
            creating: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::UpdateRoleName(text) => self.request_name = text,
            Msg::CreateRole => {
                if self.request_name.is_empty() {
                    return true;
                }
                self.creating = true;
                let var_register_company_role = register_company_role::IptRoleMemberData {
                    companyUuid: self.props.company_uuid.clone(),
                    langId: 1, // todo!(fix for different lang)
                    name: self.request_name.clone(),
                };
                spawn_local(async move {
                    let res = make_query(RegisterCompanyRole::build_query(
                        register_company_role::Variables { var_register_company_role }
                    )).await.unwrap();
                    link.send_message(Msg::CreateRoleResults(res));
                });
            }
            Msg::CreateRoleResults(res) => {
                self.creating = false;
                match resp_parsing::<usize>(res, "registerCompanyRole") {
                    Ok(value) => {
                        debug!("Create role with id {}", value);
                        if value > 1 {
                            self.request_name.clear();
                            self.props.on_success.emit(());
                            self.props.on_close.emit(());
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::Close => self.props.on_close.emit(()),
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.company_uuid == props.company_uuid &&
        self.props.is_active == props.is_active {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let close_modal = self.link.callback(|_| Msg::Close);
        let class_modal = if self.props.is_active { "modal is-active" } else { "modal" };

        html!{
            <div class={class_modal}>
                <div class="modal-background" onclick={close_modal.clone()} />
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{get_value_field(&470)}</p>
                    </header>
                    <section class="modal-card-body">
                        <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
                        <div class="field">
                            <label class="label">{get_value_field(&467)}</label>
                            <div class="control">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder={get_value_field(&472)}
                                    value={self.request_name.clone()}
                                    oninput={self.link.callback(|ev: InputData| Msg::UpdateRoleName(ev.value))}
                                />
                            </div>
                        </div>
                    </section>
                    <footer class="modal-card-foot">
                        {ft_modal_cancel_save_btn(
                            &unique_id("create-role"),
                            close_modal,
                            self.link.callback(|_| Msg::CreateRole),
                            self.request_name.is_empty() || self.creating,
                        )}
                    </footer>
                </div>
            </div>
        }
    }
}