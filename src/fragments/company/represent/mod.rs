mod add_represent;
mod edit;
mod list_item;
mod represent_form;

use add_represent::AddCompanyRepresentModal;
use list_item::ListItem;

use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::error::Error;
use crate::fragments::buttons::ft_change_view_btn;
use crate::fragments::{list_errors::ListErrors, list_empty::ListEmpty};
use crate::fragments::ListState;
use crate::services::{LocaleKey, get_from_value, get_value_response};
use crate::types::{CompanyRepresentInfo, Region, RepresentationType, UUID};
use crate::gqls::make_query;
use crate::gqls::company::{
    GetRepresentDataOpt, get_represent_data_opt,
};

pub(crate) enum Msg {
    UpdateListResult(String),
    SwitchShowType,
    NewRepresent(CompanyRepresentInfo),
    UpdatedRepresent(CompanyRepresentInfo),
    DeleteRepresent(UUID),
    ResponseError(Error),
    ClearError,
}

pub(crate) struct CompanyRepresents {
    error: Option<Error>,
    link: ComponentLink<Self>,
    props: Props,
    list: Vec<CompanyRepresentInfo>,
    regions: Vec<Region>,
    represent_types: Vec<RepresentationType>,
    show_type: ListState,
    loading: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    pub(crate) company_uuid: UUID,
    pub(crate) list: Vec<CompanyRepresentInfo>,
    pub(crate) show_manage_btn: bool,
}

impl Component for CompanyRepresents {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let list = props.list.clone();
        Self {
            error: None,
            link,
            props,
            list,
            regions: Vec::new(),
            represent_types: Vec::new(),
            show_type: ListState::get_from_storage(),
            loading: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let link = self.link.clone();
            self.loading = true;
            spawn_local(async move {
                let res = make_query(GetRepresentDataOpt::build_query(
                    get_represent_data_opt::Variables
                )).await.unwrap();
                link.send_message(Msg::UpdateListResult(res));
            })
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateListResult(res) => {
                match get_value_response(res) {
                    Ok(ref value) => {
                        self.regions = get_from_value(value, "regions").unwrap_or_default();
                        self.represent_types = get_from_value(value, "companyRepresentTypes").unwrap_or_default();
                    },
                    Err(err) => self.link.send_message(Msg::ResponseError(err)),
                }
                self.loading = false;
            },
            Msg::NewRepresent(new_item) => self.list.push(new_item),
            Msg::UpdatedRepresent(updated_item) => {
                if let Some(index) = self.list.iter().position(|x| x.uuid == updated_item.uuid) {
                    self.list[index] = updated_item;
                }
            },
            Msg::DeleteRepresent(uuid_to_delete) => {
                if let Some(index) = self.list.iter().position(|x| x.uuid == uuid_to_delete) {
                    self.list.remove(index);
                }
            },
            Msg::SwitchShowType => {
                match self.show_type {
                    ListState::Box => self.show_type = ListState::List,
                    _ => self.show_type = ListState::Box,
                }
                ListState::set_to_storage(&self.show_type);
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{
            <div id="represents-box">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
                <div class="is-flex is-align-items-center is-justify-content-between mb-4">
                    {match self.props.show_manage_btn {
                        true => html!{
                            <div>
                                <h4 id="updated-represents" class="title is-4 mb-0">
                                    { LocaleKey::Representations.get_value() }
                                </h4>
                            </div>
                        },
                        false => html!{},
                    }}
                    {self.show_actions()}
                </div>
                {if self.list.is_empty() {
                    html!{<>
                        <p class="subtitle is-6 has-text-grey-light">
                            {LocaleKey::NoRepresentations.get_value()}
                        </p>
                        <ListEmpty />
                    </>}
                } else {
                    {self.show_items()}
                }}
            </div>
        }
    }
}

impl CompanyRepresents {
    fn show_actions(&self) -> Html {
        let callback_add = self.link.callback(|new_item| Msg::NewRepresent(new_item));
        let onclick_change_view = self.link.callback(|_| Msg::SwitchShowType);
        html!{
            <div class="buttons mb-0 right-side">
                {match &self.props.show_manage_btn {
                    true => html!{
                        <AddCompanyRepresentModal
                            company_uuid={self.props.company_uuid.clone()}
                            regions={self.regions.clone()}
                            represent_types={self.represent_types.clone()}
                            on_add={callback_add}
                        />
                    },
                    false => html!{},
                }}
                {ft_change_view_btn(onclick_change_view, &self.show_type)}
            </div>
        }
    }

    fn show_items(&self) -> Html {
        let callback_update = self.link.callback(|updated_item| Msg::UpdatedRepresent(updated_item));
        let callback_delete = self.link.callback(|uuid_to_delete| Msg::DeleteRepresent(uuid_to_delete));

        html!{
            <div class={self.show_type.get_container_class()}>
                {for self.list.iter().map(|represent|
                    html!{
                        <ListItem
                            data={represent.clone()}
                            show_list={self.show_type == ListState::List}
                            regions={self.regions.clone()}
                            represent_types={self.represent_types.clone()}
                            show_manage={self.props.show_manage_btn}
                            on_update={callback_update.clone()}
                            on_delete={callback_delete.clone()}
                            />
                    }
                )}
            </div>
        }
    }
}
