// use yew_agent::Bridge;
use yew::{Component, Context, html, html::Scope, Html, Properties, classes};
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::error::{Error, get_error};
use crate::routes::AppRoute::ShowStandard;
use crate::fragments::{
    list_errors::ListErrors,
    switch_icon::res_btn,
};
use crate::types::{UUID, ShowStandardShort};
use crate::services::get_value_field;
use crate::gqls::make_query;
use crate::gqls::standard::{
    AddStandardFav, add_standard_fav,
    DeleteStandardFav, delete_standard_fav,
};

pub enum Msg {
    OpenStandard,
    TriggerFav,
    AddFav,
    DelFav,
    GetFavResult(String),
    Ignore,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub data: ShowStandardShort,
    pub show_list: bool,
}

pub struct ListItemStandard {
    error: Option<Error>,
    standard_uuid: UUID,
    show_list: bool,
    // router_agent: Box<dyn Bridge<AppRoute>>,
    is_followed: bool,
}

impl Component for ListItemStandard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            show_list: ctx.props().show_list,
            standard_uuid: ctx.props().data.uuid,
            // router_agent: AppRoute::bridge(ctx.link().callback(|_| Msg::Ignore)),
            is_followed: ctx.props().data.is_followed,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::OpenStandard => {
                // Redirect to standard page
                // self.router_agent.send(ShowStandard { uuid: ctx.props().data.uuid.to_string() });
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.replace(&ShowStandard { uuid: ctx.props().data.uuid.to_string() });
            },
            Msg::TriggerFav => {
                match &self.is_followed {
                    true => link.send_message(Msg::DelFav),
                    false => link.send_message(Msg::AddFav),
                }
            },
            Msg::AddFav => {
                let standard_uuid = ctx.props().data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(AddStandardFav::build_query(add_standard_fav::Variables{
                        standard_uuid
                    })).await;
                    link.send_message(Msg::GetFavResult(res.unwrap()));
                });
            },
            Msg::DelFav => {
                let standard_uuid = ctx.props().data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteStandardFav::build_query(delete_standard_fav::Variables{
                      standard_uuid
                    })).await;
                    link.send_message(Msg::GetFavResult(res.unwrap()));
                });
            },
            Msg::GetFavResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                debug!("res value: {:#?}", res_value);

                match res_value.is_null() {
                    false => {
                        let result: bool = match &self.is_followed {
                            true => serde_json::from_value(res_value.get("deleteStandardFav").unwrap().clone()).unwrap(),
                            false => serde_json::from_value(res_value.get("addStandardFav").unwrap().clone()).unwrap(),
                        };
                        debug!("Fav result: {:?}", result);
                        if result {
                            self.is_followed = !self.is_followed;
                        }
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.show_list == ctx.props().show_list ||
            self.standard_uuid == ctx.props().data.uuid {
            false
        } else {
            self.is_followed = ctx.props().data.is_followed;
            self.show_list = ctx.props().show_list;
            self.standard_uuid = ctx.props().data.uuid;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
      html!{<>
        <ListErrors error={self.error.clone()}/>
        {match ctx.props().show_list {
            true => { self.showing_in_list(ctx.link(), ctx.props()) },
            false => { self.showing_in_box(ctx.link(), ctx.props()) },
        }}
      </>}
    }
}

impl ListItemStandard {
    fn showing_in_list(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let ShowStandardShort {
            classifier,
            name,
            description,
            specified_tolerance,
            publication_at,
            image_file,
            owner_company,
            standard_status,
            updated_at,
            // is_followed,
            ..
        } = &props.data;

        let show_standard_btn = link.callback(|_| Msg::OpenStandard);
        let trigger_fab_btn = link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec!["fa-bookmark"];
        let mut class_color_btn = "";
        match &self.is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #1872F0;";
            },
            false => class_res_btn.push("far"),
        }

        html!{
          <div class="box itemBox">
              <article class="media center-media">
                  <div class="media-left">
                    <figure class="image is-96x96">
                      <div class="top-tag" >{standard_status.name.to_string()}</div>
                      // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                      <img
                        src={image_file.download_url.clone()} alt="Image standard"
                        loading="lazy"
                      />
                    </figure>
                  </div>
                  <div class="media-content">
                    <div class="columns is-gapless" style="margin-bottom:0">
                      <div class="column">
                          { get_value_field(&142) } // classifier
                          <span class="id-box has-text-grey-light has-text-weight-bold">{classifier}</span>
                      </div>
                      <div class="column">
                          { get_value_field(&144) } // specified tolerance
                          <span class="id-box has-text-grey-light has-text-weight-bold">{specified_tolerance}</span>
                      </div>
                    </div>
                    <div class="columns" style="margin-bottom:0">
                        <div class="column">
                            <div class="has-text-weight-bold is-size-4">{name}</div>
                            <div class="overflow-title">
                                {match &description.len() {
                                    0..=50 => description.clone(),
                                    _ => format!("{:.*}...", 50, description),
                                }}
                            </div>
                            <div class="overflow-title">
                                { get_value_field(&141) } // owner
                                <span class="has-text-weight-bold">
                                    {format!("{} {}",
                                            &owner_company.shortname,
                                            &owner_company.company_type.shortname
                                    )}
                                </span>
                            </div>
                        </div>
                        <div class="column buttons is-one-quarter flexBox" >
                          {res_btn(
                            classes!("fas", "fa-eye"),
                            show_standard_btn,
                            String::new()
                          )}
                          {res_btn(
                            classes!(class_res_btn),
                            trigger_fab_btn,
                            class_color_btn.to_string()
                          )}
                        </div>
                    </div>
                    <div class="columns is-gapless" style="margin-bottom:0">
                        <div class="column">
                          {format!("{}: {:.*}", get_value_field(&155), 10, publication_at.to_string())}
                        </div>
                        <div class="column">
                          {format!("{}: {:.*}", get_value_field(&156), 10, updated_at.to_string())}
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
        let ShowStandardShort {
            classifier,
            name,
            // specified_tolerance,
            // publication_at,
            image_file,
            owner_company,
            standard_status,
            // is_followed,
            ..
        } = props.data.clone();

        let show_standard_btn = link.callback(|_| Msg::OpenStandard);
        let trigger_fab_btn = link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec![];
        let mut class_color_btn = "";
        match &self.is_followed {
            true => {
                class_res_btn.push("fas");
                class_color_btn = "color: #1872F0;";
            },
            false => {
                class_res_btn.push("far");
            },
        }
        class_res_btn.push("fa-bookmark");

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              <div class="imgBox" >
                <div class="top-tag" >{standard_status.name.to_string()}</div>
                // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                <img
                  src={image_file.download_url.clone()} alt="Image standard"
                  loading="lazy"
                />
              </div>
              <div>
                { get_value_field(&142) } // classifier
                <span class="id-box has-text-grey-light has-text-weight-bold">{classifier}</span>
                <br/>
              </div>
              <div class="has-text-weight-bold is-size-4">{name}</div>
              <div class="overflow-title">
                { get_value_field(&141) } // owner
                  <span class="has-text-weight-bold">
                    {format!("{} {}",
                      &owner_company.shortname,
                      &owner_company.company_type.shortname
                    )}
                  </span>
                </div>
              <div class="btnBox">
                <button class="button is-light is-fullwidth has-text-weight-bold"
                    onclick={show_standard_btn}>
                    { get_value_field(&143) } // Show standard
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
