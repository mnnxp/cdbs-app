use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use crate::{services::get_value_field, types::{PreServiceRequestData, UUID}};
use crate::routes::supplier_service::CreateService;

pub struct ServiceRequestBtn {
    open_window: bool,
    link: ComponentLink<Self>,
    pre_request_data: PreServiceRequestData,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub company_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    ShowModal,
    Ignore,
}

impl Component for ServiceRequestBtn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let pre_request_data = PreServiceRequestData {
            company_uuid: props.company_uuid.clone(),
            calc_params: Vec::new(),
            cost: 0.0
        };
        ServiceRequestBtn {
            link,
            open_window: false,
            pre_request_data,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::ShowModal => self.open_window = !self.open_window,
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_share_btn = self.link.callback(move |_| Msg::ShowModal);
        html!{<>
            {self.service_req_window()}
            <button id={"service-req-btn"} class={"button"} onclick={onclick_share_btn}>
              <span class={"icon is-small"} style={"color: #00ff10;"}><i class={"fas fa-clipboard-list"} /></span>
              <span>{get_value_field(&355)}</span>
            </button>
        </>}
    }
}

impl ServiceRequestBtn {
    fn service_req_window(&self) -> Html {
        let onclick_share_btn = self.link.callback(|_| Msg::ShowModal);
        let class_modal = match &self.open_window {
            true => "modal is-active",
            false => "modal",
        };
        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_share_btn.clone()} />
              <div class="modal-content">
                <div class="card column">
                  <div class="">
                    <CreateService pre_service_req={self.pre_request_data.clone()} />
                  </div>
                </div>
              </div>
              <button class="modal-close is-large" aria-label="close" onclick={onclick_share_btn} />
            </div>
        }
    }
}
