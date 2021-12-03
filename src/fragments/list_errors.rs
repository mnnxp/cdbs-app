use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::error::Error;

pub struct ListErrors {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub error: Option<Error>,
}

pub enum Msg {
    CloseError
}

impl Component for ListErrors {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ListErrors {
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CloseError => self.props.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick_close_error = self.link.callback(|_| Msg::CloseError);

        if let Some(error) = &self.props.error {
            match error {
                Error::UnprocessableEntity(error_info) => {
                    html! {<div class=vec!("notification", "is-danger")>
                        <button class="delete" onclick=onclick_close_error/>
                        <table class="table is-fullwidth">
                            <tbody>
                                {for error_info.errors.iter().map(|(key, value)| {
                                    html! {<tr>
                                        { key }
                                        {for value.iter().map(|e| {
                                            html! {<>{" "} {e}</>}
                                        })}
                                    </tr>}
                                })}
                            </tbody>
                        </table>
                    </div>}
                }
                _ => {
                    html! {
                        <div class=vec!("notification", "is-danger")>
                            <button class="delete" onclick=onclick_close_error/>
                            {error}
                        </div>
                    }
                }
            }
        } else {
            html! {}
        }
    }
}
