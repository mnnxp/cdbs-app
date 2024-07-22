use yew::{agent::Bridged, html, Bridge, Component, ComponentLink, Html, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::routes::AppRoute;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::services::content_adapter::DateDisplay;
use crate::services::{get_logged_user, get_value_field, resp_parsing};
use crate::types::{ShowNotification, DegreeImportanceTranslateList};
use crate::gqls::make_query;
use crate::gqls::user::{
    GetNotifications, get_notifications,
    SetReadNotifications, set_read_notifications,
    DeleteNotifications, delete_notifications,
};

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
    router_agent: Box<dyn Bridge<RouteAgent>>,
    // props: Props,
    link: ComponentLink<Self>,
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
    // GetCurrentData,
    ResponseError(Error),
    ClearError,
    Ignore,
}

impl Component for Notifications {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Notifications {
            error: None,
            notifications: Vec::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            // props,
            link,
            read_notification: Vec::new(),
            delete_notification: Vec::new(),
            select_menu: Menu::GetAll,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();

        if let None = get_logged_user() {
            // route to login page if not found token
            self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
        };

        if first_render {
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
                match resp_parsing::<Vec<ShowNotification>>(res, "notifications") {
                    Ok(result) => self.notifications = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
                debug!("User notifications data: {:?}", self.notifications);
            },
            // Msg::GetNotificationByDegree(res) => {},
            // Msg::GetNotReadNotification(res) => {},
            Msg::GetReadNotification(res) => {
                match resp_parsing::<usize>(res, "readNotifications") {
                    Ok(result) => {
                        debug!("Read notifications data: {:?}", result);
                        if result > 0 {
                            self.rendered(true);
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetRemoveNotification(res) => {
                match resp_parsing::<usize>(res, "deleteNotifications") {
                    Ok(result) => {
                        debug!("Read notifications data: {:?}", result);
                        if result > 0 {
                            self.rendered(true);
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            // Msg::GetCurrentData => {},
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{
            <div class="settings-page">
            <ListErrors error=self.error.clone() clear_error=onclick_clear_error />
                <div class="container page">
                    <div class="row">
                        <h4 id="show-notifications" class="title is-4">{ get_value_field(&284) }</h4>
                        <div class="card">
                            <div class="column">
                                <>{for self.notifications.iter().rev().map(|notif_data|
                                    {self.notification_card(notif_data)}
                                )}</>
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
    fn notification_card(
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

        let onclick_set_read =
            self.link.callback(move |_| Msg::ReadOneNotificationIds(notif_id.clone()));

        let onclick_delete_notif =
            self.link.callback(move |_| Msg::RemoveOneNotificationIds(notif_id.clone()));

        let (class_degree, class_icon) = match degree_importance_id {
            1..=2 => ("notification is-danger", "fas fa-ban"),
            3 =>  ("notification is-warning", "fas fa-exclamation-triangle"),
            4 => ("notification is-success", "fas fa-check"),
            5 => ("notification is-info", "fas fa-info-circle"),
            _ => ("", ""),
        };

        let class_degree = match is_read {
            true => format!("{} is-light", class_degree),
            false => class_degree.to_string(),
        };

        html!{<>
            <div class="card">
                <div class={class_degree}>
                    <button class="delete" onclick=onclick_delete_notif />
                    <span class="icon">
                      <i class={class_icon}> </i>
                    </span>
                    { notification }
                    <br/>
                    <div class="media">
                        <div class="media-left">
                            <span class="content is-small">
                                {get_value_field(&276)}
                                {" "}
                                {created_at.date_to_display()}
                                {format!(" ({})", degree)}
                            </span>
                        </div>
                        <div class="media-content"></div>
                        <div class="media-right">
                            {match is_read {
                                true => html!{
                                    <button class="button is-light is-info"
                                        disabled=true >
                                        <span class="icon">
                                            <i class="fas fa-envelope-open"></i>
                                        </span>
                                    </button>
                                },
                                false => html!{
                                    <button class="button is-ghost is-info"
                                        onclick=onclick_set_read >
                                        <span class="icon">
                                            <i class="fas fa-envelope"></i>
                                        </span>
                                    </button>
                                },
                            }}
                        </div>
                    </div>
                </div>
            </div>
            <br/>
        </>}
    }
}
