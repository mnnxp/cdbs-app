use yew::{agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use crate::routes::AppRoute;
use crate::error::Error;

pub struct ListErrors {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
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
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CloseError => {
                self.props.error = None;
                self.props.clear_error.emit(());
            },
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

        if let Some(error) = &self.props.error {
            match error {
                Error::UnprocessableEntity(error_info) => {
                    html!{<div class=vec!("notification", "is-danger")>
                        <button class="delete" onclick=onclick_close_error/>
                        <table class="table is-fullwidth">
                            <tbody>
                                {for error_info.errors.iter().map(|(key, value)| {
                                    html!{<tr>
                                        { key }
                                        {for value.iter().map(|e| {
                                            html!{<>{" "} {e}</>}
                                        })}
                                    </tr>}
                                })}
                            </tbody>
                        </table>
                    </div>}
                },
                Error::Unauthorized => {
                    html!{
                        <div class=vec!("notification", "is-warning")>
                            <button class="delete" onclick=onclick_close_error/>
                            <div class="media">
                                <div class="media-content">{error}</div>
                                <div class="media-right">
                                    <button class="button is-ghost" onclick=onclick_route_to_login>
                                        <span>{"Open sign in page"}</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                },
                _ => {
                    html!{
                        <div class=vec!("notification", "is-danger")>
                            <button class="delete" onclick=onclick_close_error/>
                            {error}
                        </div>
                    }
                }
            }
        } else {
            html!{}
        }
    }
}
