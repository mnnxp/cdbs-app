use log::debug;
use yew::{html, classes, Callback, Children, Component, ComponentLink, Html, Properties, ShouldRender};
use crate::fragments::buttons::{ft_delete_class_btn, ft_delete_pair_btn, ft_modal_cancel_save_btn};
use crate::services::unique_id;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    pub(crate) modal_id: &'static str,
    pub(crate) title: String,
    pub(crate) is_active: bool,
    pub(crate) on_close: Callback<()>,
    pub(crate) on_cancel: Option<Callback<()>>,
    pub(crate) on_save: Option<Callback<()>>,
    pub(crate) save_disabled: bool,
    #[prop_or(None)]
    pub(crate) on_delete: Option<Callback<bool>>,
    #[prop_or(false)]
    pub(crate) delete_confirm: bool,
    #[prop_or(false)]
    pub(crate) delete_disabled: bool,
    pub(crate) children: Children,
}

/// Automatically adjusts layout structure (header visibility, footer styles, and internal scrollbars)
/// depending on whether it renders a compact informational card, a CRUD form, or a deletion prompt.
pub(crate) struct ModalBlock {
    props: Props,
    link: ComponentLink<Self>,
    generated_id: String,
}

pub(crate) enum Msg {
    OnClose,
    OnCancel,
    OnSave,
    OnDelete(bool),
}

impl Component for ModalBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let generated_id = unique_id(&props.modal_id);
        Self {
            props,
            link,
            generated_id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnClose => {
                self.props.on_close.emit(());
            },
            Msg::OnCancel => {
                if let Some(on_cancel) = &self.props.on_cancel {
                    on_cancel.emit(());
                } else {
                    self.props.on_close.emit(());
                }
            },
            Msg::OnSave => {
                if let Some(on_save) = &self.props.on_save {
                    on_save.emit(());
                }
            },
            Msg::OnDelete(is_confirmed) => {
                debug!("ModalBlock [{}] Delete is_confirmed: {}, props confirm: {}", self.props.modal_id, is_confirmed, self.props.delete_confirm);
                if let Some(on_delete) = &self.props.on_delete {
                    on_delete.emit(self.props.delete_confirm);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let modal_classes = classes!(
            "modal",
            "is-isolated-modal",
            if self.props.is_active { Some("is-active") } else { None }
        );
        let section_class = match self.props.title.is_empty() && self.props.on_save.is_none() {
            true => "",
            false => "modal-card-body"
        };
        let on_close_bg = self.link.callback(|_| Msg::OnClose);

        html! {
            <div id={self.generated_id.clone()} class={modal_classes}>
                <div class="modal-background" onclick={on_close_bg} />
                <div class="modal-card">
                    {match self.props.title.is_empty() {
                        true => html!{},
                        false => html!{
                            <header class="modal-card-head">
                                <p class="modal-card-title">{&self.props.title}</p>
                            </header>
                        },
                    }}
                    <section class={section_class}>
                        {self.props.children.clone()}
                    </section>
                    {self.modal_footer()}
                </div>
            </div>
        }
    }
}

impl ModalBlock {
    fn modal_footer(&self) -> Html {
        if self.props.on_save.is_none() && self.props.on_delete.is_none() {
            return html!{};
        }
        html! {
            <footer class="modal-card-foot">
                {match (&self.props.on_delete, &self.props.on_save) {
                    (Some(_), Some(_)) => html!{
                        <>
                            {self.delete_single_btn()}
                            <div class="right-side">
                                {self.cancel_save_btn()}
                            </div>
                        </>
                    },
                    (Some(_), None) => self.delete_pair_btn(),
                    (None, Some(_)) => self.cancel_save_btn(),
                    (None, None) => html!{},
                }}
            </footer>
        }
    }

    fn delete_pair_btn(&self) -> Html {
        let on_delete_action = self.link.callback(|is_confirmed: bool| Msg::OnDelete(is_confirmed));
        ft_delete_pair_btn(
            &format!("delete-btn-{}", self.props.modal_id),
            on_delete_action,
            self.props.delete_confirm,
            self.props.delete_disabled,
            classes!("is-half")
        )
    }

    fn delete_single_btn(&self) -> Html {
        let on_event_delete_action = self.link.callback(|_| Msg::OnDelete(true));
        ft_delete_class_btn(
            &format!("delete-btn-{}", self.props.modal_id),
            on_event_delete_action,
            self.props.delete_confirm,
            self.props.delete_disabled,
            classes!("is-half")
        )
    }

    fn cancel_save_btn(&self) -> Html {
        let on_cancel_action = self.link.callback(|_| Msg::OnCancel);
        let on_save_action = self.link.callback(|_| Msg::OnSave);
        ft_modal_cancel_save_btn(
            &self.generated_id,
            on_cancel_action,
            on_save_action,
            self.props.on_delete.is_none(),
            self.props.save_disabled,
        )
    }
}