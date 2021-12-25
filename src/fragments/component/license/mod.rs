mod item;

pub use item::ComponentLicenseTag;

use std::collections::BTreeSet;
use chrono::NaiveDateTime;
use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, LicenseInfo};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct AddComponentLicense;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct GetComponentLicenses;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct GetLicenses;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub component_licenses: Vec<LicenseInfo>,
}

pub struct ComponentLicensesTags {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    license_ids: BTreeSet<usize>,
    component_licenses: Vec<LicenseInfo>,
    license_list: Vec<LicenseInfo>,
    request_add_license_id: usize,
    hide_add_license_modal: bool,
}

#[derive(Clone)]
pub enum Msg {
    DeleteComponentLicense(usize),
    RequestLicensesList,
    RequestAddLicense,
    RequestComponentLicenses,
    GetLicensesListResult(String),
    GetComponentLicensesResult(String),
    GetAddLicenseResult(String),
    UpdateSelectLicense(String),
    ChangeHideAddLicense,
    SetSelectLicense,
    ClearError,
    Ignore,
}

impl Component for ComponentLicensesTags {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut license_ids: BTreeSet<usize> = BTreeSet::new();

        for license in props.component_licenses.clone() {
            license_ids.insert(license.id);
        };

        let component_licenses = props.component_licenses.clone();

        Self {
            error: None,
            props,
            link,
            license_ids,
            component_licenses,
            license_list: Vec::new(),
            request_add_license_id: 0,
            hide_add_license_modal: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::DeleteComponentLicense(license_id) => {
                self.license_ids.remove(&license_id);
                link.send_message(Msg::SetSelectLicense);
            },
            Msg::RequestLicensesList => {
                spawn_local(async move {
                    let res = make_query(GetLicenses::build_query(
                        get_licenses::Variables { ipt_license_arg: None }
                    )).await.unwrap();
                    link.send_message(Msg::GetLicensesListResult(res));
                })
            },
            Msg::RequestAddLicense => {
                let ipt_component_license_data = Some(add_component_license::IptComponentLicenseData{
                    componentUuid: self.props.component_uuid.clone(),
                    licenseId: self.request_add_license_id as i64,
                });
                spawn_local(async move {
                    let res = make_query(AddComponentLicense::build_query(
                        add_component_license::Variables { ipt_component_license_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetAddLicenseResult(res));
                })
            },
            Msg::RequestComponentLicenses => {
                let component_uuid = self.props.component_uuid.clone();
                spawn_local(async move {
                    let res = make_query(GetComponentLicenses::build_query(
                        get_component_licenses::Variables { component_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentLicensesResult(res));
                })
            },
            Msg::GetLicensesListResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<LicenseInfo> =
                            serde_json::from_value(res_value.get("licenses").unwrap().clone()).unwrap();
                        debug!("licenses: {:?}", result);
                        self.license_list = result;
                        link.send_message(Msg::SetSelectLicense);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetAddLicenseResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("addComponentLicense").unwrap().clone()).unwrap();
                        debug!("addComponentLicense: {:?}", result);
                        self.hide_add_license_modal = result;
                        link.send_message(Msg::RequestComponentLicenses);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetComponentLicensesResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<LicenseInfo> =
                            serde_json::from_value(res_value.get("component").unwrap()
                                .get("licenses").unwrap().clone()).unwrap();
                        debug!("licenses: {:?}", result);
                        self.component_licenses = result;
                        self.license_ids = BTreeSet::new();
                        for license in self.component_licenses.clone() {
                            self.license_ids.insert(license.id);
                        };
                        link.send_message(Msg::SetSelectLicense);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateSelectLicense(data) =>
                self.request_add_license_id = data.parse::<usize>().unwrap_or_default(),
            Msg::ChangeHideAddLicense => {
                if self.hide_add_license_modal && self.license_list.is_empty() {
                    link.send_message(Msg::RequestLicensesList)
                }
                self.hide_add_license_modal = !self.hide_add_license_modal
            },
            Msg::SetSelectLicense => {
                self.request_add_license_id = 0;
                for license in self.license_list.iter() {
                    if let None = self.license_ids.get(&license.id) {
                        self.request_add_license_id = license.id;
                        break;
                    }
                }
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
             self.props.component_licenses.len() == props.component_licenses.len() {
            false
        } else {
            self.license_ids = BTreeSet::new();
            for licenses in props.component_licenses.iter() {
                self.license_ids.insert(licenses.id);
            };
            self.hide_add_license_modal = true;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {self.modal_add_license()}
            {self.show_licenses()}
        </>}
    }
}

impl ComponentLicensesTags {
    fn show_licenses(&self) -> Html {
        let onclick_delete_license = self.link
            .callback(|value: usize| Msg::DeleteComponentLicense(value));

        let onclick_action_btn = self.link
            .callback(|_| Msg::ChangeHideAddLicense);

        html!{<div class="media">
            <div class="media-right" style="margin-left: 0rem">
                <span style="" class="icon is-small">
                    <i class="fa fa-balance-scale"></i>
                </span>
            </div>
            <div class="media-content">
                <div class="tags">
                    {for self.component_licenses.iter().map(|data| html!{
                        match self.license_ids.get(&data.id) {
                            Some(_) => html!{<ComponentLicenseTag
                                show_delete_btn = self.props.show_delete_btn
                                component_uuid = self.props.component_uuid.clone()
                                license_data = data.clone()
                                delete_license = Some(onclick_delete_license.clone())
                              />},
                            None => html!{},
                        }
                    })}
                    {match self.props.show_delete_btn {
                        true => html!{<div class="tags has-addons"
                                style="margin-left: 1rem; margin-bottom: 1rem" >
                            <span class="tag is-light is-success" onclick={onclick_action_btn}>
                                <i aria-hidden="true" class="fa fa-plus"></i>
                            </span>
                        </div>},
                        false => html!{<span class="tag is-light is-success"
                            style="margin-left: 1rem; margin-bottom: 1rem"
                            onclick={onclick_action_btn} >
                          <i aria-hidden="true" class="fa fa-plus"></i>
                        </span>},
                    }}
                </div>
            </div>
        </div>}
    }

    fn modal_add_license(&self) -> Html {
        let onclick_add_license = self.link
            .callback(|_| Msg::RequestAddLicense);

        let onclick_hide_modal = self.link
            .callback(|_| Msg::ChangeHideAddLicense);

        let onchange_select_add_license = self.link
            .callback(|ev: ChangeData| Msg::UpdateSelectLicense(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
          }));

        let class_modal = match &self.hide_add_license_modal {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class=class_modal>
              <div class="modal-background" onclick=onclick_hide_modal.clone() />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{"Add license for component"}</p>
                      <button class="delete" aria-label="close" onclick=onclick_hide_modal.clone() />
                    </header>
                    <section class="modal-card-body">
                        <label class="label">{"Select license"}</label>
                        <div class="select">
                          <select
                              id="add-license"
                              onchange=onchange_select_add_license
                            >
                          { for self.license_list.iter().map(|x|
                              match self.license_ids.get(&x.id) {
                                  Some(_) => html!{}, // this license already has
                                  None => match self.request_add_license_id == x.id {
                                      true => html!{ <option value={x.id.to_string()} selected=true>{
                                          format!("{} ({})", &x.name, &x.keyword)
                                      }</option> },
                                      false => html!{ <option value={x.id.to_string()}>{
                                          format!("{} ({})", &x.name, &x.keyword)
                                      }</option> },
                                  },
                              }
                          )}
                          </select>
                        </div>
                    </section>
                    <button
                        id="license-component"
                        class="button"
                        disabled={self.request_add_license_id == 0}
                        onclick={onclick_add_license} >
                        {"Add"}
                    </button>
                  </div>
                </div>
              </div>
        }
    }
}
