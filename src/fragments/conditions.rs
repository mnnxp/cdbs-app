use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::services::get_value_field;

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
        let onclick_show_conditions = self.link.callback(|_| Msg::ShowConditions);

        let class_modal = match &self.show_conditions {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_show_conditions.clone()} />
          <div class="modal-card">
            <header class="modal-card-head">
              <p class="modal-card-title">{get_value_field(&285)}</p>
              <button class="delete" aria-label="close" onclick={onclick_show_conditions.clone()} />
            </header>
            <section class="modal-card-body">
              <span>{get_value_field(&251)}</span>
              <br/>
              <span class="has-text-weight-bold">{get_value_field(&287)}</span>
              <a href="mailto:support@cadbase.rs">{"support@cadbase.rs"}</a>
            </section>
            <footer class="modal-card-foot">
              <button class="button is-fullwidth is-large" onclick={onclick_show_conditions}>
                {get_value_field(&288)}
              </button>
            </footer>
          </div>
        </div>}
    }
}
