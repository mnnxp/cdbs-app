use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::{
    list_errors::ListErrors,
    standard::ListItemStandard,
};
use crate::types::{UUID, ShowStandardShort};
use crate::gqls::make_query;
use crate::gqls::component::{DeleteStandardsComponent, delete_standards_component};

/// Standard card for show data on component page
pub struct ComponentStandardItem {
    error: Option<Error>,
    standard_uuid: UUID,
    open_standard_info: bool,
    get_result_delete: bool,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub standard_data: ShowStandardShort,
    #[prop_or_default]
    pub delete_standard: Option<Callback<UUID>>,
}

#[derive(Clone)]
pub enum Msg {
    ShowStandardCard,
    RequestDeleteStandard,
    ResponseError(Error),
    GetDeleteStandardResult(String),
    Ignore,
}

impl Component for ComponentStandardItem {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            standard_uuid: ctx.props().standard_data.uuid,
            open_standard_info: false,
            get_result_delete: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::ShowStandardCard => self.open_standard_info = !self.open_standard_info,
            Msg::RequestDeleteStandard => {
                let component_uuid = ctx.props().component_uuid.clone();
                let standard_uuid = ctx.props().standard_data.uuid.clone();
                spawn_local(async move {
                    let del_standard_to_component_data = delete_standards_component::DelStandardToComponentData{
                        component_uuid,
                        standards_uuids: vec![standard_uuid],
                    };
                    let res = make_query(DeleteStandardsComponent::build_query(
                        delete_standards_component::Variables {
                            del_standard_to_component_data,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteStandardResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDeleteStandardResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res.get("deleteStandardsComponent").unwrap().clone()).unwrap();
                        debug!("deleteStandardsComponent: {:?}", result);
                        match &ctx.props().delete_standard {
                            Some(delete_standard) => {
                                if result > 0 {
                                    self.get_result_delete = true;
                                    delete_standard.emit(ctx.props().standard_data.uuid.clone());
                                };
                            },
                            None => self.get_result_delete = result > 0,
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::Ignore => {}
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        match self.standard_uuid == ctx.props().standard_data.uuid {
            true => false,
            false => {
                self.standard_uuid = ctx.props().standard_data.uuid;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<>
            <ListErrors error={self.error.clone()}/>
            {match self.get_result_delete {
                true => html!{},
                false => self.show_standard(ctx.link(), ctx.props()),
            }}
        </>}
    }
}

impl ComponentStandardItem {
    fn show_standard(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_standard_data_info = link.callback(|_| Msg::ShowStandardCard);
        let onclick_delete_standard = link.callback(|_| Msg::RequestDeleteStandard);

        html!{<>
            {self.show_modal_standard_info(link, props)}
            <tr>
                <td>{props.standard_data.classifier.clone()}</td>
                <td>{props.standard_data.specified_tolerance.clone()}</td>
                <td><a onclick={onclick_standard_data_info.clone()}>
                    <span class="icon" >
                        <i class="fas fa-info" aria-hidden="true"></i>
                    </span>
                </a></td>
                {match props.show_delete_btn {
                    true => html!{<td><a onclick={onclick_delete_standard.clone()}>
                        <span class="icon" >
                          <i class="fa fa-trash" aria-hidden="true"></i>
                        </span>
                    </a></td>},
                    false => html!{},
                }}
            </tr>
        </>}
    }
    fn show_modal_standard_info(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_standard_data_info = link.callback(|_| Msg::ShowStandardCard);

        let class_modal = match &self.open_standard_info {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_standard_data_info.clone()} />
            // <div class="modal-content">
              <div class="card">
                <ListItemStandard
                    data = {props.standard_data.clone()}
                    show_list = {true}
                  />
              </div>
            // </div>
          <button class="modal-close is-large" aria-label="close" onclick={onclick_standard_data_info} />
        </div>}
    }
}
