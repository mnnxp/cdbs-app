use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::{fragments::modal::ModalBlock, services::LocaleKey};

pub struct ConditionsBlock {
    link: ComponentLink<Self>,
    show_conditions: bool,
}

pub enum Msg {
    ShowConditions
}

impl Component for ConditionsBlock {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConditionsBlock {
            link,
            show_conditions: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowConditions => self.show_conditions = !self.show_conditions,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_show_conditions = self.link.callback(|_| Msg::ShowConditions);
        html!{<>
            {self.modal_conditions()}
            <span>
                {LocaleKey::AcceptTerms.get_value()}
                {" ["}<a onclick={onclick_show_conditions}>{ LocaleKey::TermsAndConditions.get_value()}</a>{"]"}
            </span>
        </>}
    }
}

impl ConditionsBlock {
    fn modal_conditions(&self) -> Html {
        let callback_show_conditions = self.link.callback(|_| Msg::ShowConditions);
        let onclick_show_conditions = self.link.callback(|_| Msg::ShowConditions);
        html! {
            <ModalBlock
                modal_id="conditions"
                title={LocaleKey::CADBaseConditions.get_value()}
                is_active={self.show_conditions}
                on_close={callback_show_conditions}
                on_save={None}
                save_disabled={false}
            >
                <div class="content">
                    <span>{LocaleKey::SoftwareLicense.get_value()}</span>
                    <br/>
                    <span class="has-text-weight-bold">{LocaleKey::SupportContact.get_value()} </span>
                    <a href="mailto:support@cadbase.rs">{"support@cadbase.rs"}</a>
                    <br/><br/>
                    <button class="button is-fullwidth is-large is-info" onclick={onclick_show_conditions}>
                        {LocaleKey::Great.get_value()}
                    </button>
                </div>
            </ModalBlock>
        }
    }
}
