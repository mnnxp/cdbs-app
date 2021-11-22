use yew::prelude::*;
use crate::types::CompanyRepresentInfo;

pub enum Msg {}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: CompanyRepresentInfo,
    pub show_list: bool,
}

pub struct ListItem {
    props: Props
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {props}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list {
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
            // uuid,
            // company_uuid,
            region,
            representation_type,
            name,
            address,
            phone,
            ..
        } = &self.props.data;

        html! {
          <div class="box itemBox">
            <article class="media center-media">
              // <div class="media-left">
              //   <figure class="image is-96x96">
              //     // <div hidden={!is_supplier} class="top-tag" >{"supplier"}</div>
              //     <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              //     // <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
              //   </figure>
              // </div>
              <div class="media-content" style="min-width: 0px;">
                <div class="content">
                  <p>
                  <div style="overflow-title margin-bottom:0" >
                      <span class="title is-5 overflow-title">{name.to_string()}</span>
                      {" from "} <span class="id-box has-text-grey-light has-text-weight-bold">{region.region.to_string()}</span>
                    </div>
                    {format!("address: {}", address.to_string())}
                  </p>
                </div>
              </div>
              <div class="overflow-title media-right overflow-title">
                  {format!("representation type: {}", representation_type.representation_type.to_string())}
                  <br/>
                  {format!("phone: {}", phone.to_string())}
              </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let CompanyRepresentInfo {
            // uuid,
            // company_uuid,
            region,
            representation_type,
            name,
            address,
            phone,
            ..
        } = self.props.data.clone();

        html! {
          <div class="boxItem" >
            <div class="innerBox" >
              // <div class="imgBox" >
              //   // <div class="top-tag" hidden={!is_supplier} >{"supplier"}</div>
              //   <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              //   // <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
              // </div>
              <div style="overflow-title margin-bottom:0" >
                  <span class="title is-5 overflow-title">{name.to_string()}</span>
                  {" from "} <span class="id-box has-text-grey-light has-text-weight-bold">{region.region.to_string()}</span>
              </div>
              <div class="overflow-title">{
                  format!("address: {}", address.to_string())
              }</div>
              <div class="overflow-title">{
                  format!("type: {}", representation_type.representation_type.to_string())
              }</div>
              <div class="overflow-title has-text-weight-bold">{
                  format!("phone: {}", phone.to_string())
              }</div>
            </div>
          </div>
        }
    }
}
