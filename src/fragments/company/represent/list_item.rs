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
            // uuid,
            // company_uuid,
            region,
            representation_type,
            name,
            address,
            phone,
            ..
        } = &self.props.data;

        html!{
          <div class="box itemBox">
            <article class="media center-media">
              // <div class="media-left">
              //   <figure class="image is-96x96">
              //     // <div hidden={!is_supplier} class="top-tag" >{ get_value_field(&3) }</div> // supplier
              //     <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              //     // <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
              //   </figure>
              // </div>
              <div class="media-content" style="min-width: 0px;">
                <div class="content">
                  <p>
                  <div style="margin-bottom:0" >
                      <span class="title is-5">{name.to_string()}</span>
                      { get_value_field(&231) }
                      <span class="id-box has-text-grey-light has-text-weight-bold">{region.region.to_string()}</span>
                    </div>
                    {format!("{}: {}", get_value_field(&232), address)}
                  </p>
                </div>
              </div>
              <div class="overflow-title media-right overflow-title">
                  {format!("{}: {}", get_value_field(&235), representation_type.representation_type)}
                  <br/>
                  {format!("{}: {}", get_value_field(&234), phone)}
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

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              // <div class="imgBox" >
              //   // <div class="top-tag" hidden={!is_supplier} >{ get_value_field(&3) }</div> // supplier
              //   <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              //   // <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
              // </div>
              <div style="margin-bottom:0" >
                  <span class="title is-5">{name.to_string()}</span>
                  { get_value_field(&231) }
                  <span class="id-box has-text-grey-light has-text-weight-bold">{region.region.to_string()}</span>
              </div>
              <div class="overflow-title">{
                  format!("{}: {}", get_value_field(&232), address)
              }</div>
              <div class="overflow-title">{
                  format!("{}: {}", get_value_field(&233), representation_type.representation_type)
              }</div>
              <div class="overflow-title has-text-weight-bold">{
                  format!("{}: {}", get_value_field(&234), phone)
              }</div>
            </div>
          </div>
        }
    }
}
