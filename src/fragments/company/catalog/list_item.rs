use yew::{agent::Bridged, classes, html, Bridge, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::RouteAgent,
};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::fragments::{
    buttons::ft_follow_btn,
    list_errors::ListErrors,
    switch_icon::{res_btn, res_fullwidth_btn},
};
use crate::types::{Pathname, ShowCompanyShort, UUID};
use crate::services::{get_value_field, resp_parsing};
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
    ClearError,
    Ignore,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowCompanyShort,
    pub show_list: bool,
}

pub struct ListItemCompany {
    error: Option<Error>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
    company_uuid: UUID,
    is_followed: bool,
}

impl Component for ListItemCompany {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let is_followed = props.data.is_followed;
        Self {
            error: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            props,
            company_uuid: String::new(),
            is_followed,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render || self.company_uuid != self.props.data.uuid {
            self.company_uuid = self.props.data.uuid.clone();
            self.is_followed = self.props.data.is_followed;
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::OpenCompany => {
                // Redirect to profile page
                self.router_agent.send(
                    ChangeRoute(AppRoute::ShowCompany(self.props.data.uuid.to_string()).into())
                );
            },
            Msg::TriggerFav => {
                match &self.is_followed {
                    true => link.send_message(Msg::DelFav),
                    false => link.send_message(Msg::AddFav),
                }
            },
            Msg::AddFav => {
                let company_uuid = self.props.data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(AddCompanyFav::build_query(add_company_fav::Variables{
                      company_uuid
                    })).await.unwrap();
                    link.send_message(Msg::GetFavResult(res));
                });
            },
            Msg::DelFav => {
                let company_uuid = self.props.data.uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteCompanyFav::build_query(delete_company_fav::Variables{
                      company_uuid
                    })).await.unwrap();
                    link.send_message(Msg::GetFavResult(res));
                });
            },
            Msg::GetFavResult(res) => {
                let target_key = match &self.is_followed {
                    true => "deleteCompanyFav",
                    false => "addCompanyFav",
                };
                match resp_parsing(res, target_key) {
                    Ok(result) => {
                        debug!("Fav result: {:?}", result);
                        if result {
                            self.is_followed = !self.is_followed;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list ||
            // self.is_followed != props.data.is_followed ||
                self.props.data.uuid != props.data.uuid {
            self.props.show_list = props.show_list;
            self.is_followed = props.data.is_followed;
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{<>
          <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
          {match self.props.show_list {
              true => { self.showing_in_list() },
              false => { self.showing_in_box() },
          }}
        </>}
    }
}

impl ListItemCompany {
    fn showing_in_list(&self) -> Html {
        let ShowCompanyShort {
            shortname,
            inn,
            description,
            image_file,
            company_type,
            is_supplier,
            ..
        } = &self.props.data;

        let onclick_open_company = self.link.callback(|_| Msg::OpenCompany);
        let trigger_fav_btn = self.link.callback(|_| Msg::TriggerFav);

        html!{
          <div class="box itemBox">
              <article class="media center-media">
                  <div class="media-left" onclick={onclick_open_company.clone()}>
                    <figure class="image is-96x96">
                        <div hidden={!is_supplier} class="top-tag" >{get_value_field(&3)}</div> // supplier
                        <img src={image_file.download_url.clone()} alt="Favicon company" loading="lazy" />
                    </figure>
                  </div>
                  <div class="media-content">
                    <div class="column fix-width mb-0 p-0" onclick={onclick_open_company.clone()}>
                        {company_type.to_dispaly_order(
                            classes!("is-size-4"),
                            html!{<span class={"has-text-weight-bold is-size-4"}>{shortname.clone()}</span>}
                        )}
                        <p class="overflow-title">{description.clone()}</p>
                    </div>
                    {match inn.is_empty() {
                        true => html!{},
                        false => html!{
                            <div class="column p-0 mb-0" onclick={onclick_open_company.clone()}>
                                {get_value_field(&163)}
                                <span class="id-box has-text-weight-bold">{inn.clone()}</span>
                            </div>
                        },
                    }}
                  </div>
                  <div class="column buttons flexBox p-0" >
                      {res_btn(
                          classes!("far", "fa-folder"),
                          onclick_open_company.clone(),
                          String::new(),
                          get_value_field(&315),
                          Pathname::Company(self.props.data.uuid.clone())
                      )}
                      {ft_follow_btn(
                          trigger_fav_btn,
                          self.is_followed,
                          String::new(),
                      )}
                  </div>
              </article>
            </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let ShowCompanyShort {
            uuid,
            shortname,
            image_file,
            is_supplier,
            ..
        } = self.props.data.clone();

        let onclick_open_company = self.link.callback(|_| Msg::OpenCompany);
        let trigger_fav_btn = self.link.callback(|_| Msg::TriggerFav);

        html!{
          <div class="boxItem" >
            <div class="innerBox" >
              <div onclick={onclick_open_company.clone()}>
                <div class="imgBox">
                  <div class="top-tag" hidden={!is_supplier} >{get_value_field(&3)}</div> // supplier
                  <img src={image_file.download_url.to_string()} alt="Favicon profile" loading="lazy" />
                </div>
                <p class="overflow-title has-text-weight-bold is-size-4">{shortname}</p>
              </div>
              <div class="btnBox">
                {res_fullwidth_btn(onclick_open_company, get_value_field(&165), Pathname::Company(uuid))}
                <div style="margin-left: 8px;">
                {ft_follow_btn(
                    trigger_fav_btn,
                    self.is_followed,
                    String::new(),
                )}
                </div>
              </div>
            </div>
          </div>
        }
    }
}
