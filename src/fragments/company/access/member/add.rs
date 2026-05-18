use yew::{html, Component, Callback, ComponentLink, Html, Properties, ShouldRender, InputData, classes, ChangeData};
use yew::services::timeout::{TimeoutService, TimeoutTask};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use std::time::Duration;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_modal_cancel_save_btn;
use crate::fragments::switch_icon::res_loading_state;
use crate::services::content_adapter::ContentDisplay;
use crate::services::{get_value_field, resp_parsing, truncate_uuid, unique_id};
use crate::types::{UUID, UserInfoForMember, CompanyRole};
use crate::gqls::make_query;
use crate::gqls::rbac::{
    SearchUsersForAccess, search_users_for_access,
    AddCompanyMember, add_company_member,
};

/// Add company member modal
pub(crate) struct AddCompanyMemberModal {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    search_text: String,
    search_results: Vec<UserInfoForMember>,
    selected_user_uuid: Option<UUID>,
    selected_role_id: i64,
    debounce_timeout: Option<TimeoutTask>,
    search_loading: bool,
    adding: bool,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) existing_member_uuids: Vec<UUID>,
    pub(crate) company_roles: Vec<CompanyRole>,
    pub(crate) is_active: bool,
    pub(crate) on_close: Callback<()>,
    pub(crate) on_success: Callback<()>,
}

pub(crate) enum Msg {
    UpdateSearch(String),
    SearchUsers,
    SearchResults(String),
    SelectUser(UUID),
    UpdateRole(i64),
    AddMember,
    AddResult(String),
    Close,
    ResponseError(Error),
    ClearError,
}

impl Component for AddCompanyMemberModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let selected_role_id = props.company_roles.first().map(|r| r.role.role_member_id).unwrap_or(0);
        Self {
            error: None,
            props,
            link,
            search_text: String::new(),
            search_results: Vec::new(),
            selected_user_uuid: None,
            selected_role_id,
            debounce_timeout: None,
            search_loading: false,
            adding: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::UpdateSearch(text) => {
                self.search_text = text;
                self.selected_user_uuid = None;

                if !self.search_text.is_empty() {
                    let cb_link = link.clone();
                    self.debounce_timeout = Some(TimeoutService::spawn(
                        Duration::from_millis(500),
                        cb_link.callback(|_| Msg::SearchUsers),
                    ));
                } else {
                    self.search_results.clear();
                }
            }
            Msg::SearchUsers => {
                if self.search_text.is_empty() {
                    return true
                }
                self.search_loading = true;
                let search = self.search_text.clone();
                let exclude_uuids = Some(self.props.existing_member_uuids.clone());
                let ipt_paginate = Some(search_users_for_access::IptPaginate {
                    currentPage: 1,
                    perPage: 50,
                });
                spawn_local(async move {
                    let res = make_query(SearchUsersForAccess::build_query(
                        search_users_for_access::Variables {
                            search,
                            exclude_uuids,
                            ipt_paginate,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::SearchResults(res));
                });
            }
            Msg::SearchResults(res) => {
                self.search_loading = false;
                match resp_parsing(res, "users") {
                    Ok(users) => self.search_results = users,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::SelectUser(uuid) => self.selected_user_uuid = Some(uuid),
            Msg::UpdateRole(role_id) => self.selected_role_id = role_id,
            Msg::AddMember => {
                if self.selected_role_id < 1 {
                    return true;
                }
                if let Some(user_uuid) = &self.selected_user_uuid {
                    self.adding = true;
                    let var_add_company_member = add_company_member::IptCompanyMemberData {
                        companyUuid: self.props.company_uuid.clone(),
                        userUuid: user_uuid.clone(),
                        roleId: self.selected_role_id,
                    };
                    spawn_local(async move {
                        let res = make_query(AddCompanyMember::build_query(
                            add_company_member::Variables { var_add_company_member }
                        )).await.unwrap();
                        link.send_message(Msg::AddResult(res));
                    });
                }
            }
            Msg::AddResult(res) => {
                self.adding = false;
                match resp_parsing(res, "addCompanyMember") {
                    Ok(value) => {
                        debug!("Add member returned {}", value);
                        if value {
                            self.search_text.clear();
                            self.search_results.clear();
                            self.selected_user_uuid = None;
                            self.props.on_success.emit(());
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
        self.props.is_active == props.is_active &&
        self.props.company_roles == props.company_roles {
            false
        } else {
            self.selected_role_id = props.company_roles.first().map(|r| r.role.role_member_id).unwrap_or(1);
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let close_modal = self.link.callback(|_| Msg::Close);
        let class_modal = if self.props.is_active { "modal is-active" } else { "modal" };
        let onchange_role_member = self.link.callback(|ev: ChangeData| {
            if let ChangeData::Select(el) = ev {
                Msg::UpdateRole(el.value().parse().unwrap_or(0))
            } else {
                Msg::UpdateRole(0)
            }
        });
        let mut class_input = classes!("control");
        if self.search_loading {
            class_input.push("is-loading");
        };

        html!{
            <div class={class_modal}>
                <div class="modal-background" onclick={close_modal.clone()} />
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{get_value_field(&490)}</p>
                    </header>
                    <section class="modal-card-body">
                        <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
                        <div class={"columns"}>
                            <div class={"column"}>
                            <div class="field">
                                <label class="label">{get_value_field(&486)}</label>
                                <div class={class_input}>
                                    <input
                                        class="input"
                                        type="text"
                                        placeholder={get_value_field(&481)}
                                        value={self.search_text.clone()}
                                        oninput={self.link.callback(|ev: InputData| Msg::UpdateSearch(ev.value))}
                                    />
                                </div>
                            </div>
                        </div>
                        <div class={"column"}>
                        <div class="field">
                            <label class="label">{get_value_field(&471)}</label>
                            <div class="control">
                                <div class="select">
                                    <select
                                        value={self.selected_role_id.to_string()}
                                        onchange={onchange_role_member}
                                    >
                                        {for self.props.company_roles.iter().map(|rl| {
                                            let role_id = rl.role.role_member_id;
                                            html!{
                                                <option value={role_id.to_string()}>
                                                    {rl.role.name.clone()}
                                                </option>
                                            }
                                        })}
                                    </select>
                                </div>
                            </div>
                        </div>
                        </div>
                    </div>
                    <div class={"column"}>
                        {match self.search_results.is_empty() {
                            true => self.no_users_found(),
                            false => self.show_search_results(),
                        }}
                    </div>
                    </section>
                    <footer class="modal-card-foot">
                        {ft_modal_cancel_save_btn(
                            &unique_id("add-member"),
                            close_modal,
                            self.link.callback(|_| Msg::AddMember),
                            self.selected_user_uuid.is_none() || self.adding,
                        )}
                    </footer>
                </div>
            </div>
        }
    }
}

impl AddCompanyMemberModal {
    fn no_users_found(&self) -> Html {
        match self.search_text.is_empty() {
            true => html!{},
            false if self.search_loading => res_loading_state(),
            false => html!{<p>{get_value_field(&492)}</p>}
        }
    }

    fn show_search_results(&self) -> Html {
        html!{<>
                <p class="help is-info">
                    <span>{get_value_field(&493)}</span>
                    <span>{" "}</span>
                    <span>{self.search_results.len()}</span>
                </p>
                {for self.search_results.iter().map(|user|
                    self.show_result_item(user)
                )}
        </>}
    }

    fn show_result_item(&self, user: &UserInfoForMember) -> Html {
        let user_uuid = user.uuid.clone();
        let onclick_user_item = self.link.callback(move |_| Msg::SelectUser(user_uuid.clone()));
        let is_selected = self.selected_user_uuid.as_ref().map(|f| f == &user.uuid).unwrap_or_default();
        let mut class_item = classes!("is-flex", "is-align-items-center", "is-justify-content-space-between", "p-2", "is-clickable");
        if is_selected {
            class_item.push("has-background-primary-light");
        }

        html!{
            <div class={class_item} onclick={onclick_user_item}>
                <div class="is-flex is-align-items-center">
                    <div class="image is-32x32 mr-2">
                        <img src={user.image_file.download_url.clone()} alt={get_value_field(&494)} />
                    </div>
                    <div>
                        <p>{user.to_display()}</p>
                        <p class="is-size-7 has-text-grey ml-2">{truncate_uuid(&user.uuid)}</p>
                    </div>
                </div>
            </div>
        }
    }
}