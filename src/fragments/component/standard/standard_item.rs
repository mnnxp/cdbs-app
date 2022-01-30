use yew::{
    html, Callback, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::gqls::make_query;
use crate::fragments::{
    list_errors::ListErrors,
    standard::ListItemStandard,
};
use crate::types::{UUID, ShowStandardShort};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
struct DeleteStandardsComponent;

/// Standard card for show data on component page
pub struct ComponentStandardItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    open_standard_info: bool,
    get_result_delete: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub standard_data: ShowStandardShort,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ComponentStandardItem {
            error: None,
            props,
            link,
            open_standard_info: false,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ShowStandardCard => self.open_standard_info = !self.open_standard_info,
            Msg::RequestDeleteStandard => {
                let component_uuid = self.props.component_uuid.clone();
                let standard_uuid = self.props.standard_data.uuid.clone();
                spawn_local(async move {
                    let del_standard_to_component_data = delete_standards_component::DelStandardToComponentData{
                        componentUuid: component_uuid,
                        standardsUuids: vec![standard_uuid],
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
                        match &self.props.delete_standard {
                            Some(delete_standard) => {
                                if result > 0 {
                                    self.get_result_delete = true;
                                    delete_standard.emit(self.props.standard_data.uuid.clone());
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props.standard_data.uuid == props.standard_data.uuid {
            true => false,
            false => {
                self.props = props;
                true
            },
        }
    }

    fn view(&self) -> Html {
        html!{<>
            <ListErrors error=self.error.clone()/>
            {match self.get_result_delete {
                true => html!{},
                false => self.show_standard(),
            }}
        </>}
    }
}

impl ComponentStandardItem {
    fn show_standard(&self) -> Html {
        let onclick_standard_data_info = self.link
            .callback(|_| Msg::ShowStandardCard);

        let onclick_delete_standard = self.link
            .callback(|_| Msg::RequestDeleteStandard);

        html!{<>
            {self.show_modal_standard_info()}
            <tr>
                <td>{self.props.standard_data.classifier.clone()}</td>
                <td>{self.props.standard_data.specified_tolerance.clone()}</td>
                <td><a onclick={onclick_standard_data_info.clone()}>
                    <span class="icon" >
                        <i class="fas fa-info" aria-hidden="true"></i>
                    </span>
                </a></td>
                {match self.props.show_delete_btn {
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
    fn show_modal_standard_info(&self) -> Html {
        let onclick_standard_data_info = self.link
            .callback(|_| Msg::ShowStandardCard);

        let class_modal = match &self.open_standard_info {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class=class_modal>
          <div class="modal-background" onclick=onclick_standard_data_info.clone() />
            // <div class="modal-content">
              <div class="card">
                <ListItemStandard
                    data = self.props.standard_data.clone()
                    show_list = true
                  />
              </div>
            // </div>
          <button class="modal-close is-large" aria-label="close" onclick=onclick_standard_data_info />
        </div>}
    }
}
