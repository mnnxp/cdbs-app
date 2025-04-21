use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use crate::types::CompanyRepresentInfo;
use crate::services::get_value_field;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: CompanyRepresentInfo,
    pub show_list: bool,
}

pub struct ListItem {
    props: Props
}

impl Component for ListItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {props}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list || self.props.data.uuid != props.data.uuid {
            self.props.show_list = props.show_list;
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
      match self.props.show_list {
        true => { self.showing_in_list() },
        false => { self.showing_in_box() },
      }
    }
}

impl ListItem {
    fn showing_in_list(&self) -> Html {
        let CompanyRepresentInfo {
            region,
            representation_type,
            name,
            address,
            phone,
            ..
        } = &self.props.data;

        html!{
          <div class="box itemBox">
            <article class="column overflow-title m-0 p-0">
              <div class="column m-0 p-0">
                  <p class="title is-5">{name.to_string()}</p>
              </div>
              {match address.is_empty() {
                true => html!{},
                false => html!{
                  <div class="column m-0 p-0">
                    {format!("{}: {}", get_value_field(&232), address)}
                  </div>
                },
              }}
              <div class="column">
              <div class="columns">
                  <div class="column m-0 p-0">
                      {format!("{}: {}", get_value_field(&231), region.region)}
                  </div>
                  <div class="column m-0 p-0">
                      {format!("{}: {}", get_value_field(&235), representation_type.representation_type)}
                  </div>
                  <div class="column m-0 p-0">
                      {format!("{}: {}", get_value_field(&234), phone)}
                  </div>
              </div>
              </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let CompanyRepresentInfo {
            representation_type,
            name,
            phone,
            ..
        } = self.props.data.clone();

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              <div style="margin-bottom:0" >
                  <span class="title is-5">{name.to_string()}</span>
              </div>
              <div class="overflow-title">
                  {format!("{}: {}", get_value_field(&233), representation_type.representation_type)}
              </div>
              <div class="overflow-title">
                  {format!("{}: {}", get_value_field(&234), phone)}
              </div>
            </div>
          </div>
        }
    }
}
