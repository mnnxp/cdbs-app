use yew::{html, classes, Callback, Component, Properties, ComponentLink, Html, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{PaginateSet, SpecWithParent};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::relate::{GetSpecs, get_specs};

#[derive(Properties, Clone)]
pub struct Props {
    pub current_spec_id: Option<usize>,
    pub callback_select_spec: Callback<usize>,
}

#[derive(Clone)]
pub enum Msg {
    ChangeSpec(usize),
    GetSpecs,
    GetSpecsResult(String),
    ResponseError(Error),
    ClearError,
    Ignore,
}

pub struct CatalogSpec {
    error: Option<Error>,
    current_spec_id: usize,
    spec_list: Vec<SpecWithParent>,
    final_section: bool,
    props: Props,
    link: ComponentLink<Self>,
    page_set: PaginateSet,
}

impl Component for CatalogSpec {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CatalogSpec {
            error: None,
            current_spec_id: props.current_spec_id.unwrap_or(1),
            spec_list: Vec::new(),
            final_section: false,
            props,
            link,
            page_set: PaginateSet::set(None, Some(100)),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            debug!("First render catalog_specs");
            self.link.send_message(Msg::GetSpecs);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::ChangeSpec(set_spec_id) => {
                self.current_spec_id = set_spec_id;
                link.send_message(Msg::GetSpecs);
            },
            Msg::GetSpecs => {
                let ipt_spec_arg = Some(get_specs::IptSpecArg{
                    specIds: None,
                    specsLevels: Some(vec![self.current_spec_id as i64]),
                });
                let ipt_paginate = Some(get_specs::IptPaginate {
                    currentPage: self.page_set.current_page,
                    perPage: self.page_set.per_page,
                });
                spawn_local(async move {
                    let res = make_query(GetSpecs::build_query(
                        get_specs::Variables{ipt_spec_arg, ipt_paginate}
                    )).await.unwrap();
                    link.send_message(Msg::GetSpecsResult(res));
                })
            },
            Msg::GetSpecsResult(res) => {
                match resp_parsing::<Vec<SpecWithParent>>(res, "specs") {
                    Ok(get_specs) => {
                        debug!("get specs: {:?}", get_specs);
                        self.final_section = get_specs.is_empty();
                        self.props.callback_select_spec.emit(self.current_spec_id);
                        if !self.final_section {
                            self.spec_list = get_specs;
                            if let Some(x) = self.spec_list.get(0) {
                                if x.spec_id == 1 {
                                    self.spec_list.remove(0); // remove ROOT of list
                                }
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html!{
            <div class={"card"}>
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <div class={"column"}><p class={"title is-5 select-title"}>{get_value_field(&104)}</p></div>
                <div class={"column"}>{self.filters()}</div>
                <div class={"column"}>{self.result_area()}</div>
            </div>
        }
    }
}

impl CatalogSpec {
    fn result_area(&self) -> Html {
        let (parent_spec_id, parent_spec_name) = self.spec_list.first().map(|s| (s.parent_spec.spec_id, s.parent_spec.spec.clone())).unwrap_or_default();
        html!{
            <div class={"block"}>
                <div class={"column"} onclick={self.link.callback(move |_| Msg::ChangeSpec(parent_spec_id))}>
                    <p class={"subtitle is-6 overflow-title"}>{parent_spec_name}</p>
                </div>
                {for self.spec_list.iter().map(|sl| self.spec_item(sl))}
            </div>
        }
    }

    fn spec_item(&self, sl: &SpecWithParent) -> Html {
        let spec_id = sl.spec_id;
        let mut p_class = classes!("subtitle", "is-7", "overflow-title");
        if self.final_section && sl.spec_id == self.current_spec_id {
            p_class.push("has-text-weight-bold");
        }
        html!{
            <div class={"spec-item mb-1"} onclick={self.link.callback(move |_| Msg::ChangeSpec(spec_id))}>
                <p class={p_class}>
                    {format!{"{}:{}", sl.spec_id, sl.spec}}
                </p>
            </div>
        }
    }

    fn filters(&self) -> Html {
        html!{
            <div class={"columns"}>
                <div class={"column"} onclick={self.link.callback(move |_| Msg::ChangeSpec(1))}>
                    <p class={"subtitle is-7 overflow-title"}>
                        {"TOP"}
                    </p>
                </div>
            </div>
        }
    }
}