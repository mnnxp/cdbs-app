use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::{fragments::modal::ModalBlock, services::get_value_field};

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
                {get_value_field(&28)}
                {" ["}<a onclick={onclick_show_conditions}>{ get_value_field(&29)}</a>{"]"}
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
                title={get_value_field(&285)}
                is_active={self.show_conditions}
                on_close={callback_show_conditions}
                on_save={None}
                save_disabled={false}
            >
                <div class="content">
                    <span>{get_value_field(&251)}</span>
                    <br/>
                    <span class="has-text-weight-bold">{get_value_field(&287)} </span>
                    <a href="mailto:support@cadbase.rs">{"support@cadbase.rs"}</a>
                    <br/><br/>
                    <button class="button is-fullwidth is-large is-info" onclick={onclick_show_conditions}>
                        {get_value_field(&288)}
                    </button>
                </div>
            </ModalBlock>
        }
    }
}
