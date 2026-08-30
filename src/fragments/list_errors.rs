use yew::{agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use crate::routes::AppRoute;
use crate::services::LocaleKey;
use crate::error::Error;

use super::header::CurrentPage;

pub struct ListErrors {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
    current_page: CurrentPage,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub error: Option<Error>,
    pub clear_error: Callback<()>,
}

pub enum Msg {
    CloseError,
    RedirectToLogin,
    Ignore,
}

impl Component for ListErrors {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ListErrors {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            props,
            current_page: CurrentPage::from_route(None),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CloseError => self.props.clear_error.emit(()),
            Msg::RedirectToLogin => {
                // Redirect to login page
                self.router_agent.send(ChangeRoute(
                    AppRoute::Login.into()
                ));
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick_close_error = self.link.callback(|_| Msg::CloseError);
        let onclick_route_to_login = self.link.callback(|_| Msg::RedirectToLogin);
        match &self.props.error {
            Some(Error::UnprocessableEntity(error_info)) => html!{
                <div class={vec!("notification", "custom-notif", "is-danger")}>
                    <button class="delete" onclick={onclick_close_error}/>
                    <table class="table is-fullwidth">
                        <tbody>
                            {for error_info.errors.iter().map(|(key, value)| {
                                html!{<tr>
                                    { key }
                                    {for value.iter().map(|e| {
                                        html!{<span class="ml-3">{e}</span>}
                                    })}
                                </tr>}
                            })}
                        </tbody>
                    </table>
                </div>
            },
            Some(Error::Unauthorized) => html!{
                match self.current_page {
                    CurrentPage::Login => html!{
                        <div class={vec!("notification", "custom-notif", "is-warning")}>
                            <button class="delete" onclick={onclick_close_error}/>
                            {LocaleKey::Unauthorized.get_value()}
                        </div>
                    },
                    _ => html!{
                        <div class={vec!("notification", "custom-notif", "is-warning")}>
                            <button class="delete" onclick={onclick_close_error}/>
                            <div class="columns">
                                <div class="column">
                                    <p>{LocaleKey::Unauthorized.get_value()}</p>
                                </div>
                                <div class="column">
                                    <a class="is-ghost" onclick={onclick_route_to_login}>
                                        <span>{LocaleKey::GoToAuthorization.get_value()}</span>
                                    </a>
                                </div>
                            </div>
                        </div>
                    },
                }
            },
            Some(error) => html!{
                <div class={vec!("notification", "custom-notif", "is-danger")}>
                    <button class="delete" onclick={onclick_close_error}/>
                    {error}
                </div>
            },
            None => html!{},
        }
    }
}
