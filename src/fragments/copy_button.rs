use yew::{classes, html, Callback, Classes, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::services::timeout::{TimeoutService, TimeoutTask};
use std::time::Duration;
use crate::services::{set_clipboard, LocaleKey};

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    /// Text to copy
    pub(crate) text: String,
    /// Additional CSS classes for the button
    #[prop_or_default]
    pub(crate) add_classes: Classes,
    /// Show text label next to icon
    #[prop_or(true)]
    pub(crate) show_text: bool,
    /// Callback when copy is triggered
    #[prop_or_default]
    pub(crate) on_copy: Callback<()>,
    /// Time in milliseconds to show "Copied" state before resetting
    #[prop_or(2000)]
    pub(crate) reset_delay: u64,
}

pub(crate) enum Msg {
    Copy,
    Reset,
}

pub(crate) struct CopyButton {
    link: ComponentLink<Self>,
    props: Props,
    copied: bool,
    timeout_task: Option<TimeoutTask>,
}

impl Component for CopyButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            copied: false,
            timeout_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Copy => {
                set_clipboard(&self.props.text);
                self.copied = true;
                self.props.on_copy.emit(());
                // Cancel existing timer if any
                self.timeout_task = None;
                // Reset after specified delay
                let link = self.link.clone();
                self.timeout_task = Some(TimeoutService::spawn(
                    Duration::from_millis(self.props.reset_delay),
                    link.callback(|_| Msg::Reset),
                ));
                true
            }
            Msg::Reset => {
                self.copied = false;
                self.timeout_task = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.text != props.text {
            self.copied = false;
            self.timeout_task = None;
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let class_btn = classes!(
            "button",
            "is-info",
            self.props.add_classes.clone(),
        );
        let onclick = self.link.callback(|_| Msg::Copy);
        let icon_class = if self.copied {
            "fas fa-check"
        } else {
            "fas fa-copy"
        };
        let title_btn = if self.copied {
            LocaleKey::Copied.get_value()
        } else {
            LocaleKey::Copy.get_value()
        };

        html! {
            <button
                class={class_btn}
                onclick={onclick}
                title={title_btn}
            >
                <span class="icon">
                    <i class={icon_class}></i>
                </span>
                {if self.props.show_text {
                    html!{<span>{title_btn}</span>}
                } else {
                    html!{}
                }}
            </button>
        }
    }
}