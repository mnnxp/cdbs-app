// use yew::services::fetch::FetchTask;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
// use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use yew::services::ConsoleService;

use log::debug;
use chrono::NaiveDateTime;

use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use crate::gqls::make_query;

use crate::error::{Error, get_error};
use crate::fragments::list_errors::ListErrors;
// use crate::routes::AppRoute;
use crate::services::is_authenticated;
use crate::types::{
    ShowNotification, DegreeImportanceTranslateList,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct GetNotifications;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct SetReadNotifications;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
struct DeleteNotifications;

pub enum Menu {
    GetAll,
    GetNotRead,
    GetRead,
    GetByDegreeCritical,
    GetByDegreeError,
    GetByDegreeWarning,
    GetByDegreeSuccess,
    GetByDegreeInfo,
}

/// Update settings of the author or logout
pub struct Notifications {
    error: Option<Error>,
    notifications: Vec<ShowNotification>,
    link: ComponentLink<Self>,
    // props: Props,
    read_notification: Vec<i64>,
    delete_notification: Vec<i64>,
    select_menu: Menu,
}

pub enum Msg {
    SelectMenu(Menu),
    RequestReadNotification,
    RequestRemoveNotification,
    ReadOneNotificationIds(i64),
    RemoveOneNotificationIds(i64),
    GetAllNotification(String),
    // GetNotificationByDegree(String),
    // GetNotReadNotification(String),
    GetReadNotification(String),
    GetRemoveNotification(String),
    Ignore,
    GetCurrentData,
}

impl Component for Notifications {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Notifications {
            error: None,
            notifications: Vec::new(),
            link,
            // props,
            read_notification: Vec::new(),
            delete_notification: Vec::new(),
            select_menu: Menu::GetAll,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();

        if first_render && is_authenticated() {
            spawn_local(async move {
                let res = make_query(GetNotifications::build_query(
                    get_notifications::Variables {
                        ipt_notification_arg: None
                    }
                )).await.unwrap();
                link.send_message(Msg::GetAllNotification(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::SelectMenu(value) => {
                self.select_menu = value;
                self.rendered(false);
            },
            Msg::RequestReadNotification => {
                let read_notifications_ids = self.read_notification.clone();
                spawn_local(async move {
                    let res = make_query(SetReadNotifications::build_query(set_read_notifications::Variables{
                        read_notifications_ids
                    })).await.unwrap();
                    link.send_message(Msg::GetReadNotification(res));
                })
            },
            Msg::RequestRemoveNotification => {
                let delete_notifications_ids = self.delete_notification.clone();
                spawn_local(async move {
                    let res = make_query(DeleteNotifications::build_query(delete_notifications::Variables{
                        delete_notifications_ids
                    })).await.unwrap();
                    link.send_message(Msg::GetRemoveNotification(res));
                })
            },
            Msg::ReadOneNotificationIds(id) => {
                debug!("ReadOneNotificationIds: {}", id);
                self.read_notification.push(id);

                link.send_message(Msg::RequestReadNotification);
            },
            Msg::RemoveOneNotificationIds(id) => {
                debug!("RemoveOneNotificationIds: {}", id);
                self.delete_notification.push(id);

                link.send_message(Msg::RequestRemoveNotification);
            },
            Msg::GetAllNotification(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let notifications_data: Vec<ShowNotification> = serde_json::from_value(res_value.get("notifications").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("User notifications data: {:?}", notifications_data).as_ref());

                        self.notifications = notifications_data;
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
            },
            // Msg::GetNotificationByDegree(res) => {},
            // Msg::GetNotReadNotification(res) => {},
            Msg::GetReadNotification(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let read_notification: usize = serde_json::from_value(res_value.get("readNotifications").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("Read notifications data: {:?}", read_notification).as_ref());

                        if read_notification > 0 {
                            self.rendered(true);
                        }
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
            },
            Msg::GetRemoveNotification(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let delete_notification: usize = serde_json::from_value(res_value.get("deleteNotifications").unwrap().clone()).unwrap();
                        ConsoleService::info(format!("Read notifications data: {:?}", delete_notification).as_ref());

                        if delete_notification > 0 {
                            self.rendered(true);
                        }
                    },
                    true => {
                        self.error = Some(get_error(&data));
                    },
                }
            },
            Msg::Ignore => {},
            Msg::GetCurrentData => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <div class="settings-page">
                <ListErrors error=self.error.clone()/>
                <div class="container page">
                    <div class="row">
                        <div class="columns">
                            <div class="column is-one-quarter">
                                { self.view_menu() }
                            </div>
                            // <h1 class="title">{ "Your Notifications" }</h1>
                            <div class="column">
                                <>{for self.notifications.iter().rev().map(|notif_data| {
                                        { self.column_notification(notif_data) }
                                    })
                                }</>
                            </div>
                        </div>
                    // <hr />
                    </div>
                </div>
            </div>
        }
    }
}

impl Notifications {
    fn view_menu(&self) -> Html {
        let onclick_get_all = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::GetAll
            ));
        let onclick_get_not_read = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::GetNotRead
            ));
        let onclick_get_read = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::GetRead
            ));
        // let onclick_get_by_degree = self.link
        //     .callback(|_| Msg::SelectMenu(
        //         Menu::GetByDegree
        //     ));
        let onclick_get_by_degree_critical = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::GetByDegreeCritical
            ));
        let onclick_get_by_degree_error = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::GetByDegreeError
            ));
        let onclick_get_by_degree_warning = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::GetByDegreeWarning
            ));
        let onclick_get_by_degree_success = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::GetByDegreeSuccess
            ));
        let onclick_get_by_degree_info = self.link
            .callback(|_| Msg::SelectMenu(
                Menu::GetByDegreeInfo
            ));

        let mut active_get_all = "";
        let mut active_get_not_read = "";
        let mut active_get_read = "";
        let mut active_get_by_degree = "";
        let mut active_get_by_degree_critical = "";
        let mut active_get_by_degree_error = "";
        let mut active_get_by_degree_warning = "";
        let mut active_get_by_degree_success = "";
        let mut active_get_by_degree_info = "";

        match self.select_menu {
            Menu::GetAll => active_get_all = "is-active",
            Menu::GetNotRead => active_get_not_read = "is-active",
            Menu::GetRead => active_get_read = "is-active",
            Menu::GetByDegreeCritical => {
                active_get_by_degree = "is-active";
                active_get_by_degree_critical = "is-active";
            },
            Menu::GetByDegreeError => {
                active_get_by_degree = "is-active";
                active_get_by_degree_error = "is-active";
            },
            Menu::GetByDegreeWarning => {
                active_get_by_degree = "is-active";
                active_get_by_degree_warning = "is-active";
            },
            Menu::GetByDegreeSuccess => {
                active_get_by_degree = "is-active";
                active_get_by_degree_success = "is-active";
            },
            Menu::GetByDegreeInfo => {
                active_get_by_degree = "is-active";
                active_get_by_degree_info = "is-active";
            },
        }

        html!{
            <aside class="menu">
                <p class="menu-label">
                    {"User Notifications"}
                </p>
                <ul class="menu-list">
                    <li><a
                      id="get-all"
                      class=active_get_all
                      onclick=onclick_get_all>
                        { "All" }
                    </a></li>
                    <li><a
                      id="get-not-read"
                      class=active_get_not_read
                      onclick=onclick_get_not_read>
                        { "Not read" }
                    </a></li>
                    <li><a
                      id="get-read"
                      class=active_get_read
                      onclick=onclick_get_read>
                        { "Read" }
                    </a></li>
                    <li>
                        <a
                          id="get-by-degree"
                          class=active_get_by_degree
                          // onclick=onclick_get_by_degree
                          >
                            { "By degree" }
                        </a>
                        <ul>
                          <li><a
                              id="get-by-degree-critical"
                              class=active_get_by_degree_critical
                              onclick=onclick_get_by_degree_critical>
                            { "Critical" }
                          </a></li>
                          <li><a
                              id="get-by-degree-error"
                              class=active_get_by_degree_error
                              onclick=onclick_get_by_degree_error>
                            { "Error" }
                          </a></li>
                          <li><a
                              id="get-by-degree-warning"
                              class=active_get_by_degree_warning
                              onclick=onclick_get_by_degree_warning>
                            { "Warning" }
                          </a></li>
                          <li><a
                              id="get-by-degree-success"
                              class=active_get_by_degree_success
                              onclick=onclick_get_by_degree_success>
                            { "Success" }
                          </a></li>
                          <li><a
                              id="get-by-degree-info"
                              class=active_get_by_degree_info
                              onclick=onclick_get_by_degree_info>
                            { "Info" }
                          </a></li>
                        </ul>
                    </li>
                </ul>
            </aside>
        }
    }

    fn column_notification(
        &self,
        notification_data: &ShowNotification,
    ) -> Html {
        let ShowNotification {
            id,
            notification,
            degree_importance,
            created_at,
            is_read,
        } = notification_data;

        let DegreeImportanceTranslateList {
            degree_importance_id,
            degree,
            ..
        } = degree_importance;

        let notif_id: i64 = *id as i64;

        debug!("onclick_set_read: {}", notif_id);

        let onclick_set_read = self.link
            .callback(move |_| {
                Msg::ReadOneNotificationIds(notif_id.clone())
            });

        let onclick_delete_notif = self.link
            .callback(move |_| {
                Msg::RemoveOneNotificationIds(notif_id.clone())
            });

        let mut notification_class_degree = "";
        let mut notification_class_icon = "";

        match degree_importance_id {
            1..=2 => {
                notification_class_degree = "notification is-danger";
                notification_class_icon = "fas fa-ban";
            },
            3 =>  {
                notification_class_degree = "notification is-warning";
                notification_class_icon = "fas fa-exclamation-triangle";
            },
            4 => {
                notification_class_degree = "notification is-success";
                notification_class_icon = "fas fa-check";
            },
            5 => {
                notification_class_degree = "notification is-info";
                notification_class_icon = "fas fa-info-circle";
            },
            _ => {},
        };

        let notification_class_degree = match is_read {
            true => format!("{} is-light", notification_class_degree),
            false => notification_class_degree.to_string(),
        };

        html!{<>
            <div class="card">
                <div class={notification_class_degree}>
                    {match is_read {
                        true => html!{},
                        false => html!{<button class="delete" onclick=onclick_set_read />},
                    }}
                    <span class="icon">
                      <i class={notification_class_icon}> </i>
                    </span>
                    { notification }
                    <br/>
                    <span class="content is-small">
                        { format!("{} created at {:.*}",
                            degree,
                            19, created_at.to_string()) }
                    </span>
                    <a class="button"
                        onclick=onclick_delete_notif>
                        <i class="icon fas fa-trash is-danger"></i>
                    </a>
                </div>
            </div>
            <br/>
        </>}
    }
}
