use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
use crate::fragments::catalog_standard::ListItemStandard;
use crate::types::ShowStandardShort;

/// Standard card for show data on component page
pub struct ComponentStandardItem {
    props: Props,
    link: ComponentLink<Self>,
    open_standard_info: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub standard_data: ShowStandardShort,
}

#[derive(Clone)]
pub enum Msg {
    ShowStandardCard,
    Ignore,
}

impl Component for ComponentStandardItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ComponentStandardItem {
            props,
            link,
            open_standard_info: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowStandardCard => self.open_standard_info = !self.open_standard_info,
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props.standard_data.uuid == props.standard_data.uuid {
            true => false,
            false => {
                self.props = props;
                true
            },
        }
    }

    fn view(&self) -> Html {
        let onclick_standard_data_info = self.link
            .callback(|_| Msg::ShowStandardCard);

        html!{<>
            {self.show_modal_standard_info()}
            <tr>
                <td>{self.props.standard_data.classifier.clone()}</td>
                <td>{self.props.standard_data.specified_tolerance.clone()}</td>
                <td><a onclick={onclick_standard_data_info.clone()}>{"info"}</a></td>
            </tr>
        </>}
    }
}

impl ComponentStandardItem {
    fn show_modal_standard_info(&self) -> Html {
        let onclick_standard_data_info = self.link
            .callback(|_| Msg::ShowStandardCard);

        let class_modal = match &self.open_standard_info {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class=class_modal>
          <div class="modal-background" onclick=onclick_standard_data_info.clone() />
          <div class="modal-content">
              <div class="card">
                <ListItemStandard
                    data = self.props.standard_data.clone()
                    show_list = true
                  />
              </div>
          </div>
          <button class="modal-close is-large" aria-label="close" onclick=onclick_standard_data_info />
        </div>}
    }
}
