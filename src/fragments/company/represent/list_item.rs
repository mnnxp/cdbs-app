use yew::prelude::*;
use crate::types::{UUID, CompanyRepresentInfo};
use crate::services::get_value_field;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub data: CompanyRepresentInfo,
    pub show_list: bool,
}

pub struct ListItem {
    pub data_uuid: UUID,
    pub show_list: bool,
}

impl Component for ListItem {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data_uuid: ctx.props().data.uuid,
            show_list: ctx.props().show_list,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.show_list == ctx.props().show_list ||
            self.data_uuid == ctx.props().data.uuid {
            false
        } else {
            ctx.props().show_list = self.show_list;
            ctx.props().data.uuid = self.data_uuid;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
      match ctx.props().show_list {
        true => { self.showing_in_list(ctx.props()) },
        false => { self.showing_in_box(ctx.props()) },
      }
    }
}

impl ListItem {
    fn showing_in_list(
        &self,
        props: &Props,
    ) -> Html {
        let CompanyRepresentInfo {
            // uuid,
            // company_uuid,
            region,
            representation_type,
            name,
            address,
            phone,
            ..
        } = &props.data;

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

    fn showing_in_box(
        &self,
        props: &Props,
    ) -> Html {
        let CompanyRepresentInfo {
            // uuid,
            // company_uuid,
            region,
            representation_type,
            name,
            address,
            phone,
            ..
        } = props.data.clone();

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
