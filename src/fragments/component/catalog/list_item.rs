use yew::{Component, Callback, Context, html, html::Scope, Html, Properties, classes};
// use yew_agent::Bridge;
// use log::debug;
use yew_router::prelude::*;
use crate::services::get_value_field;
use crate::routes::AppRoute::{self, ShowComponent};
use crate::fragments::switch_icon::res_btn;
use crate::types::{UUID, ShowComponentShort};

pub enum Msg {
    OpenComponent,
    TriggerFav,
    Ignore,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub data: ShowComponentShort,
    pub show_list: bool,
    pub add_fav: Callback<String>,
    pub del_fav : Callback<String>,
}

pub struct ListItem {
    // router_agent: Box<dyn Bridge<AppRoute>>,
    component_uuid: UUID,
    is_followed: bool,
    show_list: bool,
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            // router_agent: AppRoute::bridge(ctx.link().callback(|_| Msg::Ignore)),
            component_uuid: ctx.props().data.uuid,
            is_followed: ctx.props().is_followed,
            show_list: ctx.props().show_list,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenComponent => {
                // Redirect to component page
                // self.router_agent.send(ShowComponent { uuid: ctx.props().data.uuid.to_string() });
                // debug!("OpenComponent");
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&ShowComponent { uuid: ctx.props().data.uuid.to_string() });
            },
            Msg::TriggerFav => {
                if ctx.props().data.is_followed {
                    ctx.props().del_fav.emit(String::new());
                } else {
                    ctx.props().add_fav.emit(String::new());
                }
            },
            Msg::Ignore => (),
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.show_list == ctx.props().show_list ||
            self.component_uuid == ctx.props().data.uuid ||
            self.is_followed == ctx.props().data.is_followed {
            false
        } else {
            self.show_list = ctx.props().show_list;
            self.component_uuid = ctx.props().data.uuid;
            self.is_followed = ctx.props().is_followed;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
      match ctx.props().show_list {
        true => self.showing_in_list(ctx.link(), ctx.props()),
        false => self.showing_in_box(ctx.link(), ctx.props()),
      }
    }
}

impl ListItem {
    fn showing_in_list(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let ShowComponentShort {
            // uuid,
            name,
            description,
            image_file,
            owner_user,
            // type_access,
            // component_type,
            actual_status,
            is_followed,
            is_base,
            updated_at,
            licenses,
            // files,
            component_suppliers,
            ..
        } = &props.data;

        let onclick_open_component = link.callback(|_| Msg::OpenComponent);

        let trigger_fab_btn = link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec!["fa-bookmark"];
        let mut class_color_btn = "";

        match is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #1872F0;";
            },
            false => class_res_btn.push("far"),
        }

        html!{
          <div class="box itemBox componentListItem">
            <article class="media center-media">
              <div class="media-left">
                <figure class="image is-96x96">
                  <div hidden={!is_base} class="top-tag" >{ get_value_field(&157) }</div> // standard
                  <img src={image_file.download_url.clone()} alt="Image" />
                </figure>
              </div>
              <div class="media-content">
                <div class="columns is-gapless" style="margin-bottom:0">
                    <div class="column">
                        {match component_suppliers.first() {
                            Some(x) => html!{<>
                                { get_value_field(&158) } // supplier
                                <span class="id-box has-text-grey-light has-text-weight-bold">
                                  {x.supplier.shortname.clone()}
                                </span>
                            </>},
                            None => html!{<>
                                { get_value_field(&118) } // user uploaded
                                <span class="id-box has-text-grey-light has-text-weight-bold">
                                  {format!("@{}",&owner_user.username)}
                                </span>
                            </>},
                        }}
                    </div>
                    <div class="column">
                        { get_value_field(&159) } // actual status
                        <span class="id-box has-text-grey-light has-text-weight-bold">
                            {actual_status.name.clone()}
                        </span>
                    </div>
                  </div>
                  <div class="columns" style="margin-bottom:0">
                      <div class="column">
                          <div class="overflow-title has-text-weight-bold is-size-4">{name}</div>
                          <div class="overflow-title">
                            {match &description.len() {
                                0..=50 => description.clone(),
                                _ => format!("{:.*}...", 50, description),
                            }}
                          </div>
                      </div>
                      <div class="column buttons is-one-quarter flexBox" >
                          {res_btn(classes!("fas", "fa-eye"),
                              onclick_open_component,
                              String::new())}
                          {res_btn(
                              classes!(class_res_btn),
                              trigger_fab_btn,
                              class_color_btn.to_string()
                          )}
                      </div>
                  </div>
                  <div class="columns" style="margin-bottom:0">
                      <div class="column">
                        // {format!("Component type: {}", component_type.component_type.to_string())}
                        {match licenses.first() {
                            Some(l) => html!{format!("{}: {}", get_value_field(&162), &l.name)}, // License
                            None => html!{},
                        }}
                        // <div class="tags">
                        //   {for licenses.iter().map(|x| html!{
                        //       <span class="tag is-info is-light">{x.name.clone()}</span>
                        //   })}
                        // </div>
                      </div>
                      <div class="column">
                        {format!("{}{:.*}", get_value_field(&30), 10, updated_at.to_string())}
                      </div>
                  </div>
                </div>
            </article>
          </div>
        }
    }

    fn showing_in_box(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let ShowComponentShort {
            is_base,
            is_followed,
            image_file,
            name,
            ..
        } = props.data.clone();

        let component_supplier = props.data.component_suppliers
            .first()
            .map(|s| s.supplier.shortname.clone())
            .unwrap_or_default();

        let onclick_open_component = link.callback(|_| Msg::OpenComponent);

        let trigger_fab_btn = link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec![];
        let mut class_color_btn = "";
        match is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #1872F0;";
            },
            false => class_res_btn.push("far"),
        }
        class_res_btn.push("fa-bookmark");

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <div class="top-tag" hidden={!is_base} >{ get_value_field(&157) }</div> // standard
                <img src={image_file.download_url.clone()} alt="Image" />
              </div>
              <div>
                { get_value_field(&160) } // manufactured by
                <span class="id-box has-text-grey-light has-text-weight-bold">{component_supplier}</span>
              </div>
              <div class="overflow-title has-text-weight-bold is-size-4" >{name}</div>
                <div class="btnBox">
                  <button class="button is-light is-fullwidth has-text-weight-bold"
                        onclick={onclick_open_component} >
                    { get_value_field(&161) }
                  </button>
                  <div style="margin-left: 8px;">
                      {res_btn(
                          classes!(class_res_btn),
                          trigger_fab_btn,
                          class_color_btn.to_string()
                      )}
                  </div>
                </div>
            </div>
          </div>
        }
    }
}
