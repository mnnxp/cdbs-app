use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::{
    buttons::ft_delete_small_btn,
    list_errors::ListErrors,
    standard::ListItemStandard,
};
use crate::services::resp_parsing;
use crate::types::{UUID, ShowStandardShort};
use crate::gqls::make_query;
use crate::gqls::component::{DeleteStandardsComponent, delete_standards_component};

/// Standard card for show data on component page
pub struct ComponentStandardItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    open_standard_info: bool,
    get_result_delete: bool,
    get_confirm: UUID,
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
    ClearError,
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
            get_confirm: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ShowStandardCard => self.open_standard_info = !self.open_standard_info,
            Msg::RequestDeleteStandard => {
                let standard_uuid = self.props.standard_data.uuid.clone();
                if self.get_confirm == standard_uuid {
                    let component_uuid = self.props.component_uuid.clone();
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
                } else {
                    self.get_confirm = standard_uuid;
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDeleteStandardResult(res) => {
                match resp_parsing::<usize>(res, "deleteStandardsComponent") {
                    Ok(result) => {
                        debug!("deleteStandardsComponent: {:?}", result);
                        self.get_result_delete = result > 0;
                        if self.get_result_delete {
                            if let Some(delete_standard) = &self.props.delete_standard {
                                delete_standard.emit(self.props.standard_data.uuid.clone());
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ClearError => self.error = None,
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
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
            {match self.get_result_delete {
                true => html!{},
                false => self.show_standard(),
            }}
        </>}
    }
}

impl ComponentStandardItem {
    fn show_standard(&self) -> Html {
        let onclick_standard_data_info = self.link.callback(|_| Msg::ShowStandardCard);
        let onclick_delete_standard = self.link.callback(|_| Msg::RequestDeleteStandard);

        html!{<>
            {self.show_modal_standard_info()}
            <tr>
                <td><a onclick={onclick_standard_data_info}>
                    <span class="icon" >
                        <i class="fas fa-info" aria-hidden="true"></i>
                    </span>
                </a></td>
                {match self.props.show_delete_btn {
                    true => html!{<td>
                        {ft_delete_small_btn(
                            "component-standard-delete",
                            onclick_delete_standard,
                            self.get_confirm == self.props.standard_data.uuid,
                        )}
                    </td>},
                    false => html!{},
                }}
            </tr>
        </>}
    }
    fn show_modal_standard_info(&self) -> Html {
        let onclick_standard_data_info = self.link.callback(|_| Msg::ShowStandardCard);
        let class_modal = match &self.open_standard_info {
            true => "modal is-active",
            false => "modal",
        };

        html!{<div class={class_modal}>
          <div class="modal-background" onclick={onclick_standard_data_info.clone()} />
            // <div class="modal-content">
              <div class="card">
                <ListItemStandard
                    data={self.props.standard_data.clone()}
                    show_list={true}
                  />
              </div>
            // </div>
          <button class="modal-close is-large" aria-label="close" onclick={onclick_standard_data_info} />
        </div>}
    }
}
