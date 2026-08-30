mod list_item;
pub use list_item::ListItem;

use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use yew_router::prelude::RouterAnchor;
use graphql_client::GraphQLQuery;
use log::debug;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::buttons::ft_change_view_btn;
use crate::fragments::modal::ModalBlock;
use crate::fragments::{list_errors::ListErrors, list_empty::ListEmpty};
use crate::routes::component::CreateComponent;
use crate::routes::AppRoute;
use crate::types::{ComponentsQueryArg, ShowComponentShort, UUID};
use crate::services::{LocaleKey, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::component::{
    GetComponentsShortList, get_components_short_list,
    AddComponentFav, add_component_fav,
    DeleteComponentFav, delete_component_fav,
};
use crate::fragments::ListState;

pub enum Msg {
    SwitchShowType,
    UpdateList(String),
    AddFav(UUID),
    DelFav(UUID),
    GetList,
    ShowAddComponentCard,
    ResponseError(Error),
    ClearError,
}

pub struct CatalogComponents {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    show_type: ListState,
    list: Vec<ShowComponentShort>,
    company_uuid: Option<UUID>,
    show_add_component: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_create_btn: bool,
    pub arguments: Option<ComponentsQueryArg>,
    pub component_list: Option<Vec<ShowComponentShort>>,
}

impl Component for CatalogComponents {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let company_uuid = props.arguments.as_ref().map(|arg| arg.company_uuid.clone()).unwrap_or_default();
        Self {
            error: None,
            link,
            props,
            show_type: ListState::get_from_storage(),
            list: Vec::new(),
            company_uuid,
            show_add_component: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.props.component_list.is_none() {
            self.link.send_message(Msg::GetList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::SwitchShowType => {
                match self.show_type {
                    ListState::Box => self.show_type = ListState::List,
                    _ => self.show_type = ListState::Box,
                }
                ListState::set_to_storage(&self.show_type);
            },
            Msg::GetList => {
                let ipt_components_arg = match &self.props.arguments {
                    Some(ref arg) => Some(get_components_short_list::IptComponentsArg {
                        componentsUuids: arg.components_uuids.clone(),
                        companyUuid: arg.company_uuid.to_owned(),
                        standardUuid: arg.standard_uuid.to_owned(),
                        serviceUuid: arg.service_uuid.to_owned(),
                        userUuid: arg.user_uuid.to_owned(),
                        specId: arg.spec_id.to_owned(),
                        favorite: arg.favorite,
                    }),
                    None => None,
                };
                let ipt_sort = Some(get_components_short_list::IptSort {
                    byField: "createdAt".to_string(),
                    asDesc: true,
                });
                // todo!(make paginate)
                let ipt_paginate = Some(get_components_short_list::IptPaginate {
                    currentPage: 1,
                    perPage: 25,
                });
                spawn_local(async move {
                    let res = make_query(GetComponentsShortList::build_query(
                        get_components_short_list::Variables {
                            ipt_components_arg,
                            ipt_sort,
                            ipt_paginate,
                        },
                    ))
                    .await
                    .unwrap();
                    // debug!("GetList res: {:?}", res);
                    link.send_message(Msg::UpdateList(res));
                });
            },
            Msg::UpdateList(res) => {
                match resp_parsing(res, "components") {
                    Ok(result) => self.list = result,
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::AddFav(component_uuid) => {
                spawn_local(async move {
                    make_query(AddComponentFav::build_query(add_component_fav::Variables {
                        component_uuid,
                    }))
                    .await
                    .unwrap();
                    // debug!("AddFav res: {:?}", res);
                    link.send_message(Msg::GetList);
                });
            },
            Msg::DelFav(component_uuid) => {
                spawn_local(async move {
                    make_query(DeleteComponentFav::build_query(
                        delete_component_fav::Variables { component_uuid },
                    ))
                    .await
                    .unwrap();
                    // debug!("DelFav res: {:?}", res);
                    link.send_message(Msg::GetList);
                });
            },
            Msg::ShowAddComponentCard => self.show_add_component = !self.show_add_component,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut flag_change = match (&self.props.arguments, &props.arguments) {
            (Some(self_arg), Some(arg)) => self_arg == arg,
            (None, None) => true,
            _ => false,
        };
        if flag_change {
            flag_change = match (&self.props.component_list, &props.component_list) {
                (Some(self_list), Some(list)) =>
                    self_list.len() == list.len() && self_list.first().map(|c| &c.uuid) == list.first().map(|c| &c.uuid),
                (None, None) => true,
                _ => false,
            };
        }
        debug!("self_arg == arg: {}", flag_change);
        if self.props.show_create_btn == props.show_create_btn && flag_change {
            false
        } else {
            self.props = props;
            if self.props.component_list.is_none() {
                self.link.send_message(Msg::GetList);
            }
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_change_view = self.link.callback(|_| Msg::SwitchShowType);

        html! {
            <div id="components-box">
              <ListErrors error={self.error.clone()} clear_error={onclick_clear_error} />
              <div class="level" >
                <div class="level-left ">
                </div>
                <div class="level-right">
                    <div class="buttons">
                        {match &self.props.show_create_btn {
                          true => self.create_component_block(),
                          false => html!{},
                        }}
                        {ft_change_view_btn(onclick_change_view, &self.show_type)}
                    </div>
                </div>
              </div>
              {match self.props.component_list {
                Some(ref list) => self.show_list(list),
                None => self.show_list(&self.list),
              }}
            </div>
        }
    }
}

impl CatalogComponents {
    fn show_list(&self, list: &[ShowComponentShort]) -> Html {
        if list.is_empty() {
            html!{<ListEmpty />}
        } else {
            html!{
                <div class={self.show_type.get_container_class()}>
                    {for list.iter().map(|x| self.show_card(&x))}
                </div>
            }
        }
    }

    fn create_component_block(&self) -> Html {
        let onclick_show_add_component = self.link.callback(|_| Msg::ShowAddComponentCard);
        html!{
            {match self.company_uuid.is_none() {
                true => html!{
                    <RouterAnchor<AppRoute> route={AppRoute::CreateComponent} classes={"button is-info"}>
                        {LocaleKey::CreateComponent.get_value()}
                    </RouterAnchor<AppRoute>>
                },
                false => html!{<>
                    {self.modal_add_component()}
                    <button class="button is-info" onclick={onclick_show_add_component}>
                        <span>{LocaleKey::CreateComponent.get_value()}</span>
                    </button>
                </>},
            }}
        }
    }

    fn modal_add_component(&self) -> Html {
        let onclick_show_add_component = self.link.callback(|_| Msg::ShowAddComponentCard);
        html! {
            <ModalBlock
                modal_id="add-component"
                title={LocaleKey::StandardStatus.get_value()}
                is_active={self.show_add_component}
                on_close={onclick_show_add_component}
                on_save={None}
                save_disabled={false}
            >
                <div class="box mb-0">
                    <CreateComponent company_uuid={self.company_uuid.clone()} />
                </div>
            </ModalBlock>
        }
    }

    fn show_card(&self, show_comp: &ShowComponentShort) -> Html {
        let onclick_add_fav = self.link.callback(|target_uuid| Msg::AddFav(target_uuid));
        let onclick_del_fav = self.link.callback(|target_uuid| Msg::DelFav(target_uuid));
        html! {
            <ListItem
                data={show_comp.clone()}
                show_list={self.show_type == ListState::List}
                add_fav={onclick_add_fav}
                del_fav={onclick_del_fav}
                />
        }
    }
}
