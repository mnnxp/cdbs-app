use yew::{Component, Callback, Context, html, Html, Properties};
use yew_router::prelude::*;
use crate::routes::AppRoute::Login;
use crate::error::Error;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub error: Option<Error>,
    pub clear_error: Option<Callback<()>>,
}

pub enum Msg {
    CloseError,
    RedirectToLogin,
    Ignore,
}

pub struct ListErrors {
    error: Option<Error>
}

impl Component for ListErrors {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ListErrors {
            error: ctx.props().error.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CloseError => {
                self.error = None;
                if let Some(clear) = &ctx.props().clear_error {
                    clear.clone().emit(());
                };
            },
            Msg::RedirectToLogin => {
                // Redirect to login page
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&Login);
            },
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_close_error = ctx.link().callback(|_| Msg::CloseError);
        let onclick_route_to_login = ctx.link().callback(|_| Msg::RedirectToLogin);

        if let Some(error) = &self.error {
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
