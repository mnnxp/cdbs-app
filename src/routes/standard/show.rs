use yew::{Component, Context, html, html::Scope, Html, Properties, classes};
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::routes::AppRoute::{Login, ShowCompany, StandardSettings};
use crate::error::Error;
use crate::fragments::switch_icon::res_btn;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::component::CatalogComponents;
use crate::fragments::standard::{StandardFilesCard, SpecsTags, KeywordsTags};
use crate::fragments::img_showcase::ImgShowcase;
use crate::services::{get_logged_user, get_value_field, resp_parsing, resp_parsing_item};
use crate::types::{UUID, StandardInfo, SlimUser, DownloadFile, ComponentsQueryArg};
use crate::gqls::make_query;
use crate::gqls::standard::{
    GetStandardData, get_standard_data,
    StandardFiles, standard_files,
    AddStandardFav, add_standard_fav,
    DeleteStandardFav, delete_standard_fav,
};

/// Standard with relate data
pub struct ShowStandard {
    error: Option<Error>,
    standard: Option<StandardInfo>,
    current_standard_uuid: UUID,
    current_user_owner: bool,
    subscribers: usize,
    is_followed: bool,
    show_full_description: bool,
    show_related_components: bool,
    file_arr: Vec<DownloadFile>,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub current_user: Option<SlimUser>,
    pub standard_uuid: UUID,
}

#[derive(Clone)]
pub enum Msg {
    RequestDownloadFiles,
    Follow,
    AddFollow(String),
    UnFollow,
    DelFollow(String),
    GetDownloadFilesResult(String),
    GetStandardData(String),
    ShowDescription,
    ShowComponentsList,
    OpenStandardOwner,
    OpenStandardSetting,
    ResponseError(Error),
    Ignore,
}

impl Component for ShowStandard {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        ShowStandard {
            error: None,
            standard: None,
            current_standard_uuid: String::new(),
            current_user_owner: false,
            subscribers: 0,
            is_followed: false,
            show_full_description: false,
            show_related_components: false,
            file_arr: Vec::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if let None = get_logged_user() {
            // route to login page if not found token
            let navigator: Navigator = ctx.link().navigator().unwrap();
            navigator.replace(&Login);
        };
        // get standard uuid for request standard data
        let target_standard_uuid =
            ctx.link().location().unwrap().path().trim_start_matches("/standard/").to_string();
        // get flag changing current standard in route
        let not_matches_standard_uuid = target_standard_uuid != self.current_standard_uuid;
        // debug!("self.current_standard_uuid {:#?}", self.current_standard_uuid);
        if first_render || not_matches_standard_uuid {
            let link = ctx.link().clone();

            // update current_standard_uuid for checking change standard in route
            self.current_standard_uuid = target_standard_uuid.to_string();
            link.send_message(Msg::RequestDownloadFiles);
            spawn_local(async move {
                let res = make_query(GetStandardData::build_query(get_standard_data::Variables {
                    standard_uuid: target_standard_uuid,
                })).await.unwrap();

                link.send_message(Msg::GetStandardData(res));
            })
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let navigator: Navigator = ctx.link().navigator().unwrap();

        match msg {
            Msg::RequestDownloadFiles => {
                let standard_uuid = ctx.props().standard_uuid.clone();
                spawn_local(async move {
                    let ipt_standard_files_arg = standard_files::IptStandardFilesArg{
                        files_uuids: None,
                        standard_uuid,
                    };
                    let res = make_query(StandardFiles::build_query(standard_files::Variables{
                        ipt_standard_files_arg
                    })).await.unwrap();
                    link.send_message(Msg::GetDownloadFilesResult(res));
                })
            },
            Msg::Follow => {
                let standard_uuid_string = self.standard.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(AddStandardFav::build_query(add_standard_fav::Variables {
                        standard_uuid: standard_uuid_string,
                    })).await.unwrap();

                    link.send_message(Msg::AddFollow(res));
                })
            },
            Msg::AddFollow(res) => {
                let result: bool = resp_parsing_item(res, "addStandardFav")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if result {
                    self.subscribers += 1;
                    self.is_followed = true;
                }
            },
            Msg::UnFollow => {
                let standard_uuid_string = self.standard.as_ref().unwrap().uuid.to_string();

                spawn_local(async move {
                    let res = make_query(DeleteStandardFav::build_query(delete_standard_fav::Variables{
                        standard_uuid: standard_uuid_string,
                    })).await.unwrap();

                    link.send_message(Msg::DelFollow(res));
                })
            },
            Msg::DelFollow(res) => {
                let result: bool = resp_parsing_item(res, "deleteStandardFav")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if result {
                    self.subscribers -= 1;
                    self.is_followed = false;
                }
            },
            Msg::GetDownloadFilesResult(res) => {
                let mut result: Vec<DownloadFile> = resp_parsing(res, "standardFiles")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if !result.is_empty() {
                    // checkign have main image
                    if let Some(main_img) = self.file_arr.first() {
                        result.push(main_img.clone());
                    }
                    self.file_arr = result;
                }
            },
            Msg::GetStandardData(res) => {
                let standard_data: StandardInfo = resp_parsing_item(res, "standard")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                self.subscribers = standard_data.subscribers;
                self.is_followed = standard_data.is_followed;
                self.current_standard_uuid = standard_data.uuid.clone();
                if let Some(user) = get_logged_user() {
                    self.current_user_owner = standard_data.owner_user.uuid == user.uuid;
                }
                // description length check for show
                self.show_full_description = standard_data.description.len() < 250;
                // add main image
                self.file_arr.push(standard_data.image_file.clone());
                self.standard = Some(standard_data);
            },
            Msg::ShowDescription => self.show_full_description = !self.show_full_description,
            Msg::ShowComponentsList => self.show_related_components = !self.show_related_components,
            Msg::OpenStandardOwner => {
                if let Some(standard_data) = &self.standard {
                    // Redirect to ownercompany standard page
                    navigator.clone().replace(&ShowCompany { uuid: standard_data.owner_company.uuid.to_string() });
                }
            },
            Msg::OpenStandardSetting => {
                if let Some(standard_data) = &self.standard {
                    // Redirect to page for change and update standard
                    navigator.replace(&StandardSettings { uuid: standard_data.uuid.to_string() });
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::Ignore => {}
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.current_standard_uuid == ctx.props().standard_uuid {
            false
        } else {
            self.current_standard_uuid = ctx.props().standard_uuid.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.standard {
            Some(standard_data) => html!{
                <div class="standard-page">
                    <ListErrors error={self.error.clone()}/>
                    <div class="container page">
                        <div class="row">
                            <div class="card column">
                              {self.show_main_card(ctx.link(), standard_data)}
                            </div>
                            {match &self.show_related_components {
                                true => {self.show_related_components(&standard_data.uuid)},
                                false => html!{<>
                                    <br/>
                                    <div class="columns">
                                      {self.show_standard_params(standard_data)}
                                      {self.show_standard_files(standard_data)}
                                    </div>
                                    {self.show_standard_specs(standard_data)}
                                    <br/>
                                    {self.show_standard_keywords(standard_data)}
                                </>},
                            }}
                        </div>
                    </div>
                </div>
            },
            None => html!{<div>
                <ListErrors error={self.error.clone()}/>
                // <h1>{"Not data"}</h1>
            </div>},
        }
    }
}

impl ShowStandard {
    fn show_main_card(
        &self,
        link: &Scope<Self>,
        standard_data: &StandardInfo,
    ) -> Html {
        let onclick_open_owner_company = link.callback(|_| Msg::OpenStandardOwner);
        let show_description_btn = link.callback(|_| Msg::ShowDescription);

        html!{
            <div class="columns">
              <ImgShowcase
                object_uuid={self.current_standard_uuid.clone()}
                file_arr={self.file_arr.clone()}
              />
              // <div class="column is-one-quarter">
              //   <img class="imgBox" src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
              // </div>
              <div class="column">
                <div class="media">
                    <div class="media-content">
                        { get_value_field(&94) }
                        <a class="id-box has-text-grey-light has-text-weight-bold"
                              onclick={onclick_open_owner_company}
                            >{format!("{} {}",
                            &standard_data.owner_company.shortname,
                            &standard_data.owner_company.company_type.shortname
                        )}</a>
                    </div>
                    <div class="media-right" style="margin-right: 1rem">
                        { get_value_field(&145) } // type access
                        <span class="id-box has-text-grey-light has-text-weight-bold">
                            {standard_data.type_access.name.clone()}
                        </span>
                    </div>
                </div>
                // <h1>{"Standard"}</h1>
                <div class="has-text-weight-bold is-size-4">
                    {standard_data.name.clone()}
                </div>
                <div class="buttons flexBox">
                    {self.show_related_components_btn(link)}
                    // {self.show_download_btn()}
                    {self.show_setting_btn(link)}
                    {self.show_followers_btn(link)}
                    // {self.show_share_btn()}
                </div>
                <div class="standard-description">{
                    match self.show_full_description {
                        true => html!{<>
                          {standard_data.description.clone()}
                          {match standard_data.description.len() {
                              250.. => html!{<>
                                <br/>
                                <button class="button is-white" onclick={show_description_btn}>
                                    { get_value_field(&99) }
                                </button>
                              </>},
                              _ => html!{},
                          }}
                        </>},
                        false => html!{<>
                          {format!("{:.*}", 200, standard_data.description)}
                          <br/>
                          <button class="button is-white" onclick={show_description_btn}>
                            { get_value_field(&98) }
                          </button>
                        </>},
                    }
                }</div>
              </div>
            </div>
        }
    }

    fn show_standard_params(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2 class="has-text-weight-bold">{ get_value_field(&152) }</h2> // Сharacteristics of the standard
              <div class="card column">
                <table class="table is-fullwidth">
                    <tbody>
                      <tr>
                        <td>{ get_value_field(&146) }</td> // classifier
                        <td>{standard_data.classifier.clone()}</td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&147) }</td> // specified_tolerance
                        <td>{standard_data.specified_tolerance.clone()}</td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&148) }</td> // technical_committee
                        <td>{standard_data.technical_committee.clone()}</td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&149) }</td> // publication_at
                        <td>{format!("{:.*}", 10, standard_data.publication_at.to_string())}</td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&150) }</td> // standard_status
                        <td>{standard_data.standard_status.name.clone()}</td>
                      </tr>
                      <tr>
                        <td>{ get_value_field(&151) }</td> // region
                        <td>{standard_data.region.region.clone()}</td>
                      </tr>
                    </tbody>
                  </table>
              </div>
            </div>
        }
    }

    fn show_standard_files(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{
            <div class="column">
              <h2 class="has-text-weight-bold">{ get_value_field(&153) }</h2> // Files
              <StandardFilesCard
                  show_download_btn = {true}
                  show_delete_btn = {false}
                  standard_uuid = {standard_data.uuid.clone()}
                  files = {standard_data.standard_files.clone()}
                />
            </div>
        }
    }

    fn show_standard_specs(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{<>
              <h2 class="has-text-weight-bold">{ get_value_field(&104) }</h2> // Specs
              <div class="card column">
                <SpecsTags
                    show_manage_btn = {false}
                    standard_uuid = {standard_data.uuid.clone()}
                    specs = {standard_data.standard_specs.clone()}
                  />
              </div>
        </>}
    }

    fn show_standard_keywords(
        &self,
        standard_data: &StandardInfo,
    ) -> Html {
        html!{<>
              <h2 class="has-text-weight-bold">{ get_value_field(&105) }</h2> // Keywords
              <div class="card column">
                <KeywordsTags
                    show_delete_btn = {false}
                    standard_uuid = {standard_data.uuid.clone()}
                    keywords = {standard_data.standard_keywords.clone()}
                  />
              </div>
        </>}
    }

    fn show_followers_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let (class_fav, onclick_following) = match self.is_followed {
            true => ("fas fa-bookmark", link.callback(|_| Msg::UnFollow)),
            false => ("far fa-bookmark", link.callback(|_| Msg::Follow)),
        };

        html!{<>
            <button
                id="following-button"
                class="button"
                onclick={onclick_following} >
              <span class="icon is-small">
                <i class={class_fav}></i>
              </span>
              <span>{self.subscribers}</span>
            </button>
        </>}
    }

    // fn show_share_btn(&self) -> Html {
    //     html!{
    //         <div class="media-right flexBox" >
    //           <button
    //               id="share-button"
    //               class="button" >
    //             <span class="icon is-small">
    //               <i class="fas fa-share-alt"></i>
    //             </span>
    //           </button>
    //         </div>
    //     }
    // }

    fn show_related_components_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_related_components_btn = link.callback(|_| Msg::ShowComponentsList);
        let (text_btn, classes_btn) = match &self.show_related_components {
            true => (get_value_field(&295), "button"),
            false => (get_value_field(&296), "button is-info is-light"),
        };

        html!{
            <button class={classes_btn}
                onclick={onclick_related_components_btn} >
              <span class="has-text-black">{text_btn}</span>
            </button>
        }
    }

    fn show_related_components(
        &self,
        standard_uuid: &UUID,
    ) -> Html {
        html!{<>
            <br/>
            <h2 class="has-text-weight-bold">{ get_value_field(&154) }</h2> // Components
            <div class="card">
              <CatalogComponents
                  show_create_btn = {false}
                  arguments = {ComponentsQueryArg::set_standard_uuid(standard_uuid)}
                />
            </div>
        </>}
    }

    // fn show_download_btn(&self) -> Html {
    //     let onclick_download_standard_btn = link.callback(|_| Msg::RequestDownloadFiles);
    //
    //     html!{
    //         <button class="button is-info"
    //             onclick={onclick_download_standard_btn} >
    //           <span class="has-text-weight-bold">{ get_value_field(&126) }</span>
    //         </button>
    //     }
    // }

    fn show_setting_btn(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_setting_standard_btn = link.callback(|_| Msg::OpenStandardSetting);

        match &self.current_user_owner {
            true => {res_btn(
                classes!("fa", "fa-tools"),
                onclick_setting_standard_btn,
                String::new())},
            false => html!{},
        }
    }
}
