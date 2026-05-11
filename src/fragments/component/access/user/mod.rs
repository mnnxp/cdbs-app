mod row;
mod modal;
use log::debug;
pub(crate) use row::UserAccessRow;
pub(crate) use modal::AddUserAccessModal;

use yew::{html, Callback, classes, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::fragments::buttons::ft_custom_btn;
use crate::fragments::paginate::Paginate;
use crate::services::{get_classes_table, get_value_field};
use crate::types::{PaginateSet, PermissionLevel, UserAccess, UUID};

/// Complete component access management component
pub(crate) struct UserAccessComponentTable {
    props: Props,
    link: ComponentLink<Self>,
    show_add_user_modal: bool,
    page_set: PaginateSet,
    paginated_users: Vec<UserAccess>,
}

pub(crate) enum Msg {
    ShowAddUserModal,
    DeleteUser(UUID),
    RefreshAccessData,
    ChangePaginate(PaginateSet),
    UpdatePaginatedList,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) component_uuid: UUID,
    pub(crate) users: Vec<UserAccess>,
    pub(crate) permissions: Vec<PermissionLevel>,
    pub(crate) need_update: Callback<()>
}

impl Component for UserAccessComponentTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let page_set = PaginateSet::new();
        let paginated_users = Self::get_paginated_users(&props.users, &page_set);

        Self {
            props,
            link,
            show_add_user_modal: false,
            page_set,
            paginated_users,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowAddUserModal => self.show_add_user_modal = !self.show_add_user_modal,
            Msg::DeleteUser(user_uuid) => {
                self.props.users.retain(|u| u.user.uuid != user_uuid);
                self.link.send_message(Msg::UpdatePaginatedList);
                self.props.need_update.emit(());
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
            Msg::UpdatePaginatedList => self.paginated_users = Self::get_paginated_users(&self.props.users, &self.page_set),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
            self.props.users.len() == props.users.len() {
            false
        } else {
            self.props = props;
            self.page_set = PaginateSet::set(Some(1), Some(self.page_set.per_page));
            self.link.send_message(Msg::UpdatePaginatedList);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_show_add_user_modal = self.link.callback(|_| Msg::ShowAddUserModal);
        let callback_refresh_data = self.link.callback(|_| Msg::RefreshAccessData);
        let onclick_paginate = self.link.callback(|page_set| Msg::ChangePaginate(page_set));

        html! {
            <div class="access-management">
                <div class="card mb-5">
                    <header class="card-header">
                        <div class="card-header-title">
                            <p class="is-size-5 has-text-weight-semibold mb-0">
                                {get_value_field(&459)}
                            </p>
                            <div class="buttons right-side">
                                {ft_custom_btn(
                                    &format!("add-user-access-{}", self.props.component_uuid),
                                    get_value_field(&484),
                                    classes!("is-success"),
                                    "fas fa-user-plus",
                                    onclick_show_add_user_modal,
                                    false
                                )}
                            </div>
                        </div>
                    </header>
                    <div class="card-content">
                        {if !self.props.users.is_empty() {
                            self.render_table()
                        } else {
                            html! {
                                <div class="notification is-info is-light">
                                    {get_value_field(&495)}
                                </div>
                            }
                        }}
                        <Paginate
                            callback_change={onclick_paginate}
                            current_items={self.paginated_users.len() as i64}
                            current_page={Some(self.page_set.current_page)}
                            per_page={Some(self.page_set.per_page)}
                            total_items={Some(self.props.users.len() as i64)}
                        />
                        <AddUserAccessModal
                            component_uuid={self.props.component_uuid.clone()}
                            existing_user_uuids={self.get_existing_user_uuids()}
                            permissions={self.props.permissions.clone()}
                            is_active={self.show_add_user_modal}
                            on_close={self.link.callback(|_| Msg::ShowAddUserModal)}
                            on_success={callback_refresh_data}
                        />
                    </div>
                </div>
            </div>
        }
    }
}

impl UserAccessComponentTable {
    fn render_table(&self) -> Html {
        let mut classes_table = get_classes_table(self.page_set.per_page as usize);
        classes_table.push("is-striped");
        let start_index = ((self.page_set.current_page - 1) * self.page_set.per_page) as usize;

        html! {
            <div class="table-container">
                <div class={"content"}>
                    <table class={classes_table}>
                        <thead>
                            <tr>
                                <th>{"\u{2116}"}</th>
                                <th>{get_value_field(&19)}</th>
                                <th>{get_value_field(&468)}</th>
                                <th>{get_value_field(&458)}</th>
                                <th>{get_value_field(&111)}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {for self.paginated_users.iter().enumerate().map(|(idx, access)| {
                                html! {
                                    <UserAccessRow
                                        component_uuid={self.props.component_uuid.clone()}
                                        access={access.clone()}
                                        permissions={self.props.permissions.clone()}
                                        on_delete={self.link.callback(|uuid| Msg::DeleteUser(uuid))}
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

    fn get_existing_user_uuids(&self) -> Vec<UUID> {
        self.props.users.iter().map(|u| u.user.uuid.clone()).collect()
    }

    fn get_paginated_users(users: &[UserAccess], page_set: &PaginateSet) -> Vec<UserAccess> {
        let start = ((page_set.current_page - 1) * page_set.per_page) as usize;
        let end = (start + page_set.per_page as usize).min(users.len());

        if start >= users.len() {
            Vec::new()
        } else {
            users[start..end].to_vec()
        }
    }
}