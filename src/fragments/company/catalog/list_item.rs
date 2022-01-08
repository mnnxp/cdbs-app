use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::gqls::make_query;
use crate::error::{Error, get_error};
use crate::routes::AppRoute;
use crate::fragments::{
    list_errors::ListErrors,
    switch_icon::res_btn,
};
use crate::types::{UUID, ShowCompanyShort};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct AddCompanyFav;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct DeleteCompanyFav;

pub enum Msg {
    OpenCompany,
    TriggerFav,
    AddFav,
    DelFav,
    GetFavResult(String),
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
                self.router_agent.send(ChangeRoute(AppRoute::ShowCompany(
                    self.props.data.uuid.to_string()
                ).into()));
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
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                debug!("res value: {:#?}", res_value);

                match res_value.is_null() {
                    false => {
                        let result: bool = match &self.is_followed {
                            true => serde_json::from_value(res_value.get("deleteCompanyFav").unwrap().clone()).unwrap(),
                            false => serde_json::from_value(res_value.get("addCompanyFav").unwrap().clone()).unwrap(),
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
        // debug!("&self.props.data.shortname: {}", &self.props.data.shortname);
        // debug!("&self.props.data.is_followed: {}", &self.props.data.is_followed);
        html!{<>
          <ListErrors error=self.error.clone()/>
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
            // image_file,
            region,
            company_type,
            is_supplier,
            // is_followed,
            updated_at,
            ..
        } = &self.props.data;

        let show_company_btn = self.link.callback(|_| Msg::OpenCompany);
        let trigger_fab_btn = self.link.callback(|_| Msg::TriggerFav);

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
                        <div hidden={!is_supplier} class="top-tag" >{"supplier"}</div>
                        <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                        // <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
                    </figure>
                  </div>
                  <div class="media-content">
                    <div class="columns is-gapless" style="margin-bottom:0">
                      <div class="column">
                          {"from "} <span class="id-box has-text-grey-light has-text-weight-bold">{region.region.clone()}</span>
                      </div>
                      <div class="column">
                          {"Reg.№: "} <span class="id-box has-text-grey-light has-text-weight-bold">{inn.clone()}</span>
                      </div>
                    </div>
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
                            {res_btn(classes!(
                                String::from("fas fa-building")),
                                show_company_btn,
                                String::new())}
                            {res_btn(
                                classes!(class_res_btn),
                                trigger_fab_btn,
                                class_color_btn.to_string()
                            )}
                        </div>
                    </div>
                    <div class="columns is-gapless" style="margin-bottom:0">
                        <div class="column">
                        // {format!("Reg.№: {}", inn.to_string())}
                        </div>
                        <div class="column">
                          {format!("Updated at: {:.*}", 10, updated_at.to_string())}
                        </div>
                    </div>
                  </div>
              </article>
            </div>
        }
    }

    fn showing_in_box(&self) -> Html {
        let ShowCompanyShort {
            shortname,
            // image_file,
            region,
            company_type,
            is_supplier,
            // is_followed,
            ..
        } = self.props.data.clone();

        let show_company_btn = self.link.callback(|_| Msg::OpenCompany);
        let trigger_fab_btn = self.link.callback(|_| Msg::TriggerFav);

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
                <div class="top-tag" hidden={!is_supplier} >{"supplier"}</div>
                <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                // <img src={image_file.download_url.to_string()} alt="Favicon profile"/>
              </div>
              <div>
                {"from "}<span class="id-box has-text-grey-light has-text-weight-bold">{region.region.to_string()}</span>
              </div>
              <div class="overflow-title has-text-weight-bold is-size-4">{shortname}</div>
              <div class="overflow-title has-text-weight-bold">{company_type.shortname.to_string()}</div>
              <div class="btnBox">
                <button class="button is-light is-fullwidth has-text-weight-bold"
                    onclick=show_company_btn
                    >{"Show company"}</button>
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
