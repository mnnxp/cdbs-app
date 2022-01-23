use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::*,
};
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::gqls::make_query;

use crate::routes::AppRoute;
use crate::fragments::switch_icon::res_btn;
use crate::types::{ShowComponentShort, DownloadFile, UUID};
use log::debug;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub struct ComponentFiles;

pub enum Msg {
    OpenComponent,
    TriggerFav,
    GetImg,
    GetDownloadFileResult(String),
    Ignore,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub data: ShowComponentShort,
    pub show_list: bool,
    pub add_fav: Callback<String>,
    pub del_fav : Callback<String>,
}

pub struct ListItem {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
    show_img: Option<String>,
}

impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
            props,
            show_img: None
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::GetImg);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::OpenComponent => {
                // Redirect to profile page
                self.router_agent.send(ChangeRoute(AppRoute::ShowComponent(
                    self.props.data.uuid.to_string()
                ).into()));
                // debug!("OpenComponent");
            },
            Msg::TriggerFav => {
                if !self.props.data.is_followed {
                    self.props.add_fav.emit(String::new());
                } else {
                    self.props.del_fav.emit(String::new());
                }
            },
            Msg::GetImg => {
                let component_uuid = self.props.data.uuid.clone();
                spawn_local(async move {
                    let ipt_component_files_arg = component_files::IptComponentFilesArg{
                        componentUuid: component_uuid,
                        filesUuids: None,
                        limit: Some(1),
                        offset: None,
                    };
                    let res = make_query(ComponentFiles::build_query(
                        component_files::Variables {
                            ipt_component_files_arg,
                        }
                    )).await;
                    debug!("res {:?}", res);
                    link.send_message(Msg::GetDownloadFileResult(res.unwrap()));
                });
            },
            Msg::GetDownloadFileResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let download_file: Vec<DownloadFile> =
                            serde_json::from_value(res_value.get("componentFiles").unwrap().clone())
                                .unwrap();
                        debug!("Download file: {:?}", download_file);
                        if download_file.len() > 0 {
                            self.show_img = Some(download_file[0].download_url.clone());
                        }
                    }
                    true => {},
                }
            },
            Msg::Ignore => (),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.show_list != props.show_list || self.props.data.is_followed != props.data.is_followed || self.props.data.uuid != props.data.uuid {
            self.props.show_list = props.show_list;
            self.props.data = props.data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
      match self.props.show_list {
        true => self.showing_in_list(),
        false => self.showing_in_box(),
      }
    }
}

impl ListItem {

    fn show_img(&self) -> String {
      if let Some(src) = self.show_img.clone() {
        src
      }else{
        "https://bulma.io/images/placeholders/128x128.png".to_string()
      }
    }

    fn showing_in_list(&self) -> Html {
        let ShowComponentShort {
            // uuid,
            name,
            description,
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
        } = &self.props.data;

        let onclick_open_component = self.link
            .callback(|_| Msg::OpenComponent);

        let trigger_fab_btn = self.link.callback(|_| Msg::TriggerFav);

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
                  <div hidden={!is_base} class="top-tag" >{"standard"}</div>
                  // {match files.first() {
                  //     Some(f) => html!{<img src={f.download_url.clone()} alt="Image" />},
                  //     None => html!{<img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />},
                  // }}
                  <img src={self.show_img()} alt="Image" />
                </figure>
              </div>
              <div class="media-content">
                <div class="columns is-gapless" style="margin-bottom:0">
                    <div class="column">
                        {match component_suppliers.first() {
                            Some(x) => html!{<>
                                {"supplier "}
                                <span class="id-box has-text-grey-light has-text-weight-bold">
                                  {x.supplier.shortname.clone()}
                                </span>
                            </>},
                            None => html!{<>
                                {"uploaded from "}
                                <span class="id-box has-text-grey-light has-text-weight-bold">
                                  {format!("@{}",&owner_user.username)}
                                </span>
                            </>},
                        }}
                    </div>
                    <div class="column">
                        {"actual status "}<span class="id-box has-text-grey-light has-text-weight-bold">{
                            actual_status.name.clone()
                        }</span>
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
                            Some(l) => html!{format!("License: {}", &l.name)},
                            None => html!{},
                        }}
                        // <div class="tags">
                        //   {for licenses.iter().map(|x| html!{
                        //       <span class="tag is-info is-light">{x.name.clone()}</span>
                        //   })}
                        // </div>
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
        let ShowComponentShort {
            is_base,
            is_followed,
            name,
            ..
        } = self.props.data.clone();
        let onclick_open_component = self.link
            .callback(|_| Msg::OpenComponent);

        let trigger_fab_btn = self.link
            .callback(|_| Msg::TriggerFav);

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
                <div class="top-tag" hidden={!is_base} >{"standart"}</div>
                <img src={self.show_img()} alt="Image" />
              </div>
              <div>
                {"manufactured by "}<span class="id-box has-text-grey-light has-text-weight-bold">{"Alphametall"}</span>
              </div>
              <div class="overflow-title has-text-weight-bold	is-size-4" >{name}</div>
                <div class="btnBox">
                  <button class="button is-light is-fullwidth has-text-weight-bold"
                        onclick={onclick_open_component} >
                    {"Open"}
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
