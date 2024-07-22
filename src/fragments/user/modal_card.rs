use yew::{html, Html, Component, ComponentLink, Properties, ShouldRender};
use crate::routes::other_user::ListItemUser;
use crate::services::content_adapter::ContentDisplay;
use crate::types::ShowUserShort;

pub enum Msg {
    ShowOwnerUserCard,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowUserShort,
    // pub callback_onclick: Option<Callback<()>>,
}

pub struct ModalCardUser {
    link: ComponentLink<Self>,
    open_owner_user_info: bool,
    props: Props,
}

impl Component for ModalCardUser {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            open_owner_user_info: false,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
          Msg::ShowOwnerUserCard => self.open_owner_user_info = !self.open_owner_user_info,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
      false
    }

    fn view(&self) -> Html {
      let onclick_open_owner =
        self.link.callback(|_| Msg::ShowOwnerUserCard);
      let class_modal = match &self.open_owner_user_info {
        true => "modal is-active",
        false => "modal",
      };

      html!{<>
        <a class="id-box has-text-grey-light has-text-weight-bold"
            onclick={onclick_open_owner.clone()} >
          {self.props.data.to_display()}
        </a>
        <div class={class_modal}>
          <div class="modal-background" onclick={onclick_open_owner.clone()} />
          <div class="modal-content">
              <div class="card">
                <ListItemUser
                    data={self.props.data.clone()}
                    show_list={true}
                  />
              </div>
          </div>
          <button class="modal-close is-large" aria-label="close" onclick={onclick_open_owner} />
        </div>
      </>}
    }
}