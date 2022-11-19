mod standard_item;

pub use standard_item::ComponentStandardItem;

use std::collections::BTreeSet;
use yew::prelude::*;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowStandardShort};
use crate::services::get_value_field;
use crate::gqls::{
    make_query,
    component::{
        AddStandardToComponent, add_standard_to_component,
        GetComponentStandards, get_component_standards,
    },
    standard::{GetStandardsShortList, get_standards_short_list},
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub component_standards: Vec<ShowStandardShort>,
    // pub delete_standard: Option<Callback<UUID>>,
}

pub struct ComponentStandardsCard {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    standard_uuids: BTreeSet<UUID>,
    component_standards: Vec<ShowStandardShort>,
    standard_list: Vec<ShowStandardShort>,
    request_add_standard_uuid: UUID,
    hide_add_standard_modal: bool,
}

#[derive(Clone)]
pub enum Msg {
    DeleteComponentStandard(UUID),
    RequestStandardsList,
    RequestAddStandard,
    RequestComponentStandards,
    GetStandardsListResult(String),
    GetComponentStandardsResult(String),
    GetAddStandardResult(String),
    UpdateSelectStandard(String),
    ChangeHideAddStandard,
    SetSelectStandard,
    ClearError,
    Ignore,
}

impl Component for ComponentStandardsCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut standard_uuids: BTreeSet<UUID> = BTreeSet::new();

        for standard in props.component_standards.clone() {
            standard_uuids.insert(standard.uuid.clone());
        };

        let component_standards = props.component_standards.clone();

        Self {
            error: None,
            props,
            link,
            standard_uuids,
            component_standards,
            standard_list: Vec::new(),
            request_add_standard_uuid: String::new(),
            hide_add_standard_modal: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::DeleteComponentStandard(standard_uuid) => {
                self.standard_uuids.remove(&standard_uuid);
                link.send_message(Msg::SetSelectStandard);
            },
            Msg::RequestStandardsList => {
                spawn_local(async move {
                    let res = make_query(GetStandardsShortList::build_query(
                        get_standards_short_list::Variables { ipt_standards_arg: None }
                    )).await.unwrap();
                    link.send_message(Msg::GetStandardsListResult(res));
                })
            },
            Msg::RequestAddStandard => {
                let ipt_standard_to_component_data = add_standard_to_component::IptStandardToComponentData{
                    componentUuid: self.props.component_uuid.clone(),
                    standardUuid: self.request_add_standard_uuid.clone(),
                };
                spawn_local(async move {
                    let res = make_query(AddStandardToComponent::build_query(
                        add_standard_to_component::Variables { ipt_standard_to_component_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetAddStandardResult(res));
                })
            },
            Msg::RequestComponentStandards => {
                let component_uuid = self.props.component_uuid.clone();
                spawn_local(async move {
                    let res = make_query(GetComponentStandards::build_query(
                        get_component_standards::Variables { component_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::GetComponentStandardsResult(res));
                })
            },
            Msg::GetStandardsListResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<ShowStandardShort> =
                            serde_json::from_value(res_value.get("standards").unwrap().clone()).unwrap();
                        debug!("standards: {:?}", result);
                        self.standard_list = result;
                        link.send_message(Msg::SetSelectStandard);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetAddStandardResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: bool =
                            serde_json::from_value(res_value.get("addStandardToComponent").unwrap().clone()).unwrap();
                        debug!("addStandardToComponent: {:?}", result);
                        self.hide_add_standard_modal = result;
                        link.send_message(Msg::RequestComponentStandards);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::GetComponentStandardsResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: Vec<ShowStandardShort> =
                            serde_json::from_value(res_value.get("component").unwrap()
                                .get("componentStandards").unwrap().clone()).unwrap();
                        debug!("componentStandards: {:?}", result);
                        self.component_standards = result;
                        self.standard_uuids = BTreeSet::new();
                        for standard in self.component_standards.clone() {
                            self.standard_uuids.insert(standard.uuid.clone());
                        };
                        link.send_message(Msg::SetSelectStandard);
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateSelectStandard(data) => self.request_add_standard_uuid = data,
            Msg::ChangeHideAddStandard => {
                if self.hide_add_standard_modal && self.standard_list.is_empty() {
                    link.send_message(Msg::RequestStandardsList)
                }
                self.hide_add_standard_modal = !self.hide_add_standard_modal
            },
            Msg::SetSelectStandard => {
                self.request_add_standard_uuid = String::new();
                for standard in self.standard_list.iter() {
                    if let None = self.standard_uuids.get(&standard.uuid) {
                        self.request_add_standard_uuid = standard.uuid.clone();
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
             self.props.component_standards.len() == props.component_standards.len() {
            false
        } else {
            self.standard_uuids = BTreeSet::new();
            for standards in props.component_standards.iter() {
                self.standard_uuids.insert(standards.uuid.clone());
            };
            self.hide_add_standard_modal = true;
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            {self.show_standards()}
        </>}
    }
}

impl ComponentStandardsCard {
    fn show_standards(&self) -> Html {
        let onclick_delete_standard = self.link
            .callback(|value: UUID| Msg::DeleteComponentStandard(value));

        let onclick_action_btn = self.link
            .callback(|_| Msg::ChangeHideAddStandard);

        html!{<div class="card column">
          <table class="table is-fullwidth">
            <tbody>
               <th>{ get_value_field(&112) }</th> // Classifier
               <th>{ get_value_field(&113) }</th> // Specified tolerance
               <th>{ get_value_field(&111) }</th> // Action
               {match self.props.show_delete_btn {
                   true => html!{<th>{ get_value_field(&135) }</th>},
                   false => html!{},
               }}
               {for self.component_standards.iter().map(|data| {
                   match self.standard_uuids.get(&data.uuid) {
                       Some(_) => html!{<ComponentStandardItem
                           show_delete_btn = {self.props.show_delete_btn}
                           component_uuid = {self.props.component_uuid.clone()}
                           standard_data = {data.clone()}
                           delete_standard = {Some(onclick_delete_standard.clone())}
                         />},
                       None => html!{},
                   }
               })}
            </tbody>
          </table>
          {self.modal_add_standard()}
          <button
                id="add-standard-component"
                class="button is-fullwidth"
                onclick={onclick_action_btn} >
              <span class="icon" >
                  <i class="fas fa-plus" aria-hidden="true"></i>
              </span>
              <span>{ get_value_field(&191) }</span> // Add a standard to a component
          </button>
        </div>}
    }

    fn modal_add_standard(&self) -> Html {
        let onclick_add_standard = self.link
            .callback(|_| Msg::RequestAddStandard);

        let onclick_hide_modal = self.link
            .callback(|_| Msg::ChangeHideAddStandard);

        let onchange_select_add_standard = self.link
            .callback(|ev: ChangeData| Msg::UpdateSelectStandard(match ev {
              ChangeData::Select(el) => el.value(),
              _ => String::new(),
          }));

        let class_modal = match &self.hide_add_standard_modal {
            true => "modal",
            false => "modal is-active",
        };

        html!{
            <div class={class_modal}>
              <div class="modal-background" onclick={onclick_hide_modal.clone()} />
                <div class="modal-content">
                  <div class="card">
                    <header class="modal-card-head">
                      <p class="modal-card-title">{ get_value_field(&191) }</p> // Add a standard to a component
                      <button class="delete" aria-label="close" onclick={onclick_hide_modal.clone()} />
                    </header>
                    <section class="modal-card-body">
                        <label class="label">{ get_value_field(&212) }</label> // Select standard
                        // <div class="columns">
                            <div class="column">
                                <div class="select">
                                  <select
                                      id="add-standard"
                                      select={self.request_add_standard_uuid.clone()}
                                      onchange={onchange_select_add_standard}
                                    >
                                  { for self.standard_list.iter().map(|x|
                                      match self.standard_uuids.get(&x.uuid) {
                                          Some(_) => html!{}, // this standard already has
                                          None => html!{ <option value={x.uuid.to_string()}>{
                                              format!("{} ({})", &x.classifier, &x.name)
                                          }</option> },
                                      }
                                  )}
                                  </select>
                                </div>
                            </div>
                            <div class="column">
                                <button
                                    id="standard-component"
                                    class="button is-fullwidth"
                                    disabled={self.request_add_standard_uuid.is_empty()}
                                    onclick={onclick_add_standard} >
                                    { get_value_field(&117) }
                                </button>
                            </div>
                        // </div>
                    </section>
                  </div>
                </div>
              </div>
        }
    }
}
