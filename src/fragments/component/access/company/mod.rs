mod row;
mod modal;
pub(crate) use row::CompanyAccessRow;
pub(crate) use modal::AddCompanyAccessModal;

use yew::{html, Callback, classes, Component, ComponentLink, Html, Properties, ShouldRender};
use log::debug;

use crate::fragments::buttons::ft_custom_btn;
use crate::fragments::paginate::Paginate;
use crate::services::{get_classes_table, LocaleKey};
use crate::types::{CompanyAccess, PaginateSet, PermissionLevel, UUID};


/// Complete component access management component
pub(crate) struct CompanyAccessComponentTable {
    props: Props,
    link: ComponentLink<Self>,
    show_add_company_modal: bool,
    page_set: PaginateSet,
    paginated_companies: Vec<CompanyAccess>,
}

pub(crate) enum Msg {
    ShowAddCompanyModal,
    DeleteCompany(UUID),
    RefreshAccessData,
    ChangePaginate(PaginateSet),
    UpdatePaginatedList,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) component_uuid: UUID,
    pub(crate) companies: Vec<CompanyAccess>,
    pub(crate) permissions: Vec<PermissionLevel>,
    pub(crate) need_update: Callback<()>
}

impl Component for CompanyAccessComponentTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let page_set = PaginateSet::new();
        let paginated_companies = Self::get_paginated_companies(&props.companies, &page_set);

        Self {
            props,
            link,
            show_add_company_modal: false,
            page_set,
            paginated_companies,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowAddCompanyModal => self.show_add_company_modal = !self.show_add_company_modal,
            Msg::DeleteCompany(company_uuid) => {
                self.props.companies.retain(|c| c.company.uuid != company_uuid);
                self.props.need_update.emit(());
                self.link.send_message(Msg::UpdatePaginatedList);
            }
            Msg::RefreshAccessData => {
                self.props.need_update.emit(());
                self.link.send_message(Msg::UpdatePaginatedList);
            }
            Msg::ChangePaginate(page_set) => {
                debug!("Change page_set, old: {:?}, new: {:?}", self.page_set, page_set);
                if self.page_set.compare(&page_set) {
                    return true;
                }
                self.page_set = page_set;
                self.link.send_message(Msg::UpdatePaginatedList);
            }
            Msg::UpdatePaginatedList => self.paginated_companies = Self::get_paginated_companies(&self.props.companies, &self.page_set),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
            self.props.companies.len() == props.companies.len() {
            false
        } else {
            self.props = props;
            self.page_set = PaginateSet::set(Some(1), Some(self.page_set.per_page));
            self.link.send_message(Msg::UpdatePaginatedList);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_show_add_company_modal = self.link.callback(|_| Msg::ShowAddCompanyModal);
        let callback_refresh_data = self.link.callback(|_| Msg::RefreshAccessData);
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));

        html! {
            <div class="access-management">
                <div class="card">
                    <header class="card-header">
                        <div class="card-header-title">
                            <p class="is-size-5 has-text-weight-semibold mb-0">
                                {LocaleKey::CompaniesAccess.get_value()}
                            </p>
                            <div class="buttons right-side">
                                {ft_custom_btn(
                                    &format!("add-company-access-{}", self.props.component_uuid),
                                    {LocaleKey::AddCompany.get_value()},
                                    classes!("is-success"),
                                    "fas fa-building",
                                    onclick_show_add_company_modal,
                                    false
                                )}
                            </div>
                        </div>
                    </header>
                    <div class="card-content">
                        {if !self.props.companies.is_empty() {
                            self.show_table()
                        } else {
                            html! {
                                <div class="notification is-info is-light">
                                    {LocaleKey::NoCompaniesAccess.get_value()}
                                </div>
                            }
                        }}
                        <Paginate
                            callback_change={onclick_paginate}
                            current_items={self.paginated_companies.len() as i64}
                            current_page={Some(self.page_set.current_page)}
                            per_page={Some(self.page_set.per_page)}
                            total_items={Some(self.props.companies.len() as i64)}
                        />
                        <AddCompanyAccessModal
                            component_uuid={self.props.component_uuid.clone()}
                            existing_company_uuids={self.get_existing_company_uuids()}
                            permissions={self.props.permissions.clone()}
                            is_active={self.show_add_company_modal}
                            on_close={self.link.callback(|_| Msg::ShowAddCompanyModal)}
                            on_success={callback_refresh_data}
                        />
                    </div>
                </div>
            </div>
        }
    }
}

impl CompanyAccessComponentTable {
    fn show_table(&self) -> Html {
        let mut classes_table = get_classes_table(self.page_set.per_page as usize);
        classes_table.push("is-striped");
        let start_index = ((self.page_set.current_page - 1) * self.page_set.per_page) as usize;

        html! {
            <div class="table-container">
                <div class="content">
                    <table class={classes_table}>
                        <thead>
                            <tr>
                                <th>{"\u{2116}"}</th>
                                <th>{LocaleKey::Company.get_value()}</th>
                                <th>{LocaleKey::AccessPermissions.get_value()}</th>
                                <th>{LocaleKey::GrantedAt.get_value()}</th>
                                <th>{LocaleKey::Action.get_value()}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {for self.paginated_companies.iter().enumerate().map(|(idx, company_access)| {
                                html! {
                                    <CompanyAccessRow
                                        component_uuid={self.props.component_uuid.clone()}
                                        company_access={company_access.clone()}
                                        permissions={self.props.permissions.clone()}
                                        on_delete={self.link.callback(|uuid| Msg::DeleteCompany(uuid))}
                                        number={start_index + idx + 1}
                                    />
                                }
                            })}
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }

    fn get_existing_company_uuids(&self) -> Vec<UUID> {
        self.props.companies.iter().map(|c| c.company.uuid.clone()).collect()
    }

    fn get_paginated_companies(companies: &[CompanyAccess], page_set: &PaginateSet) -> Vec<CompanyAccess> {
        let start = ((page_set.current_page - 1) * page_set.per_page) as usize;
        let end = (start + page_set.per_page as usize).min(companies.len());

        if start >= companies.len() {
            Vec::new()
        } else {
            companies[start..end].to_vec()
        }
    }
}