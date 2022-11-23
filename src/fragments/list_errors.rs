use yew::{Component, Callback, Context, html, html::Scope, Html, Properties, Event};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use crate::routes::AppRoute::Login;
use crate::error::Error;

pub struct ListErrors {
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub error: Option<Error>,
    pub clear_error: Option<Callback<()>>,
}

pub enum Msg {
    CloseError,
    RedirectToLogin,
    Ignore,
}

impl Component for ListErrors {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ListErrors {
            router_agent: RouteAgent::bridge(ctx.link().callback(|_| Msg::Ignore)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CloseError => {
                ctx.props().error = None;
                if let Some(clear) = &ctx.props().clear_error {
                    clear.emit(());
                };
            },
            Msg::RedirectToLogin => {
                // Redirect to login page
                self.router_agent.send(ChangeRoute(Login.into()));
            },
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_close_error = ctx.link().callback(|_| Msg::CloseError);
        let onclick_route_to_login = ctx.link().callback(|_| Msg::RedirectToLogin);

        if let Some(error) = &ctx.props().error {
            match error {
                Error::UnprocessableEntity(error_info) => {
                    html!{<div class={vec!("notification", "is-danger")}>
                        <button class="delete" onclick={onclick_close_error}/>
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
                        <div class={vec!("notification", "is-warning")}>
                            <button class="delete" onclick={onclick_close_error}/>
                            <div class="media">
                                <div class="media-content">{error}</div>
                                <div class="media-right">
                                    <button class="button is-ghost" onclick={onclick_route_to_login}>
                                        <span>{"Open sign in page"}</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                },
                _ => {
                    html!{
                        <div class={vec!("notification", "is-danger")}>
                            <button class="delete" onclick={onclick_close_error}/>
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
