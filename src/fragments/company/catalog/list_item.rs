use yew::{Component, Context, html, html::Scope, Html, Properties, classes};
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::error::Error;
use crate::routes::AppRoute::ShowCompany;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::switch_icon::res_btn;
use crate::types::{UUID, ShowCompanyShort};
use crate::services::{get_value_field, resp_parsing_item};
use crate::gqls::make_query;
use crate::gqls::company::{
    AddCompanyFav, add_company_fav,
    DeleteCompanyFav, delete_company_fav,
};

pub enum Msg {
    OpenCompany,
    TriggerFav,
    AddFav,
    DelFav,
    GetFavResult(String),
    ResponseError(Error),
    Ignore,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub data: ShowCompanyShort,
    pub show_list: bool,
}

pub struct ListItemCompany {
    error: Option<Error>,
    company_uuid: UUID,
    is_followed: bool,
    show_list: bool,
}

impl Component for ListItemCompany {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            company_uuid: ctx.props().data.uuid.clone(),
            is_followed: ctx.props().data.is_followed,
            show_list: ctx.props().show_list,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render || self.company_uuid != ctx.props().data.uuid {
            self.company_uuid = ctx.props().data.uuid.clone();
            self.is_followed = ctx.props().data.is_followed;
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::OpenCompany => {
                // Redirect to company page
                let navigator: Navigator = link.navigator().expect("какая-то ерунда с этим роутом...");
                navigator.replace(&ShowCompany { uuid: ctx.props().data.uuid.clone() });
            },
            Msg::TriggerFav => {
                match &self.is_followed {
                    true => link.send_message(Msg::DelFav),
                    false => link.send_message(Msg::AddFav),
                }
            },
            Msg::AddFav => {
                let company_uuid = ctx.props().data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(AddCompanyFav::build_query(add_company_fav::Variables{
                      company_uuid
                    })).await.unwrap();
                    link.send_message(Msg::GetFavResult(res));
                });
            },
            Msg::DelFav => {
                let company_uuid = ctx.props().data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteCompanyFav::build_query(delete_company_fav::Variables{
                      company_uuid
                    })).await.unwrap();
                    link.send_message(Msg::GetFavResult(res));
                });
            },
            Msg::GetFavResult(res) => {
                let key_word: &str = match &self.is_followed {
                    true => "deleteCompanyFav",
                    false => "addCompanyFav",
                };
                let result: bool = resp_parsing_item(res, key_word)
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if result {
                    self.is_followed = !self.is_followed;
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.company_uuid == ctx.props().data.uuid &&
            self.is_followed == ctx.props().data.is_followed &&
              self.show_list == ctx.props().show_list {
            false
        } else {
            self.company_uuid = ctx.props().data.uuid.clone();
            self.is_followed = ctx.props().data.is_followed;
            self.show_list = ctx.props().show_list;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // debug!("&ctx.props().data.shortname: {}", &ctx.props().data.shortname);
        // debug!("&ctx.props().data.is_followed: {}", &ctx.props().data.is_followed);
        html!{<>
          <ListErrors error={self.error.clone()}/>
          {match ctx.props().show_list {
              true => { self.showing_in_list(ctx.link(), ctx.props()) },
              false => { self.showing_in_box(ctx.link(), ctx.props()) },
          }}
        </>}
    }
}

impl ListItemCompany {
    fn showing_in_list(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let ShowCompanyShort {
            shortname,
            inn,
            description,
            image_file,
            region,
            company_type,
            is_supplier,
            // is_followed,
            updated_at,
            ..
        } = &props.data;

        let show_company_btn = link.callback(|_| Msg::OpenCompany);
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
                        <div hidden={!is_supplier} class="top-tag" >{ get_value_field(&3) }</div> // supplier
                        // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                        <img
                            src={image_file.download_url.clone()} alt="Favicon profile"
                            loading="lazy"
                        />
                    </figure>
                  </div>
                  <div class="media-content">
                    { get_value_field(&164) } <span class="id-box has-text-grey-light has-text-weight-bold">{region.region.clone()}</span>
                    <div class="columns" style="margin-bottom:0">
                        <div class="column">
                            <div class="overflow-title has-text-weight-bold is-size-4">{
                                format!("{} {}", &shortname, &company_type.shortname
                            )}</div>
                            <p class="overflow-title">
                                {match &description.len() {
                                    0..=50 => description.clone(),
                                    _ => format!("{:.*}...", 50, description),
                                }}
                            </p>
                        </div>
                        <div class="column buttons is-one-quarter flexBox" >
                            {res_btn(
                                classes!("fas", "fa-eye"),
                                show_company_btn,
                                String::new())}
                            {res_btn(
                                classes!(class_res_btn),
                                trigger_fab_btn,
                                class_color_btn.to_string()
                            )}
                        </div>
                    </div>
                    <div class="columns is-gapless">
                        <div class="column">
                            { get_value_field(&163) }
                            <span class="id-box has-text-grey-light has-text-weight-bold">{inn.clone()}</span>
                        </div>
                        <div class="column">
                          { get_value_field(&30) }
                          {format!("{:.*}", 10, updated_at.to_string())}
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
        let ShowCompanyShort {
            shortname,
            image_file,
            region,
            company_type,
            is_supplier,
            // is_followed,
            ..
        } = props.data.clone();

        let show_company_btn = link.callback(|_| Msg::OpenCompany);
        let trigger_fab_btn = link.callback(|_| Msg::TriggerFav);

        let mut class_res_btn = vec![];
        let mut class_color_btn = "";
        match &self.is_followed {
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
                <div class="top-tag" hidden={!is_supplier} >{ get_value_field(&3) }</div> // supplier
                // <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                <img
                    src={image_file.download_url.to_string()} alt="Favicon profile"
                    loading="lazy"
                />
              </div>
              <div>
                { get_value_field(&164) }<span class="id-box has-text-grey-light has-text-weight-bold">{region.region.to_string()}</span>
              </div>
              <div class="overflow-title has-text-weight-bold is-size-4">{shortname}</div>
              <div class="has-text-weight-bold">{company_type.shortname.to_string()}</div>
              <div class="btnBox">
                <button class="button is-light is-fullwidth has-text-weight-bold" onclick={show_company_btn}>
                    { get_value_field(&165) } // Show company
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
