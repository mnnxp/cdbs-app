use yew::{html, classes, Callback, Component, Properties, ComponentLink, Html, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{PaginateSet, SpecWithParent, SpecNode};
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
    ToggleExpand(usize),
    BlockExpand,
    Ignore,
}

pub struct CatalogSpec {
    error: Option<Error>,
    current_spec_id: usize,
    spec_tree: Vec<SpecNode>,
    final_section: bool,
    props: Props,
    link: ComponentLink<Self>,
    page_set: PaginateSet,
    expanded: bool,
}

impl Component for CatalogSpec {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CatalogSpec {
            error: None,
            current_spec_id: props.current_spec_id.unwrap_or(1),
            spec_tree: Vec::new(),
            final_section: false,
            props,
            link,
            page_set: PaginateSet::set(None, Some(100)),
            expanded: true,
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
                self.set_node_loading(set_spec_id, true);
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
                            let new_nodes: Vec<SpecNode> = get_specs.into_iter()
                                .filter(|s| s.spec_id != 1)
                                .map(|s| SpecNode {
                                    spec_id: s.spec_id,
                                    spec: s.spec.clone(),
                                    children: Vec::new(),
                                    parent_spec: s,
                                    expanded: false,
                                    loading: false,
                                })
                                .collect();
                            
                            if self.spec_tree.is_empty() {
                                self.spec_tree = new_nodes;
                            } else {
                                self.add_children_to_parent(self.current_spec_id, new_nodes);
                            }
                            self.toggle_node_expanded(self.current_spec_id);
                        }
                        self.set_node_loading(self.current_spec_id, false);
                    },
                    Err(err) => {
                        self.set_node_loading(self.current_spec_id, false);
                        link.send_message(Msg::ResponseError(err))
                    },
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
            Msg::ToggleExpand(spec_id) => {
                self.toggle_node_expanded(spec_id);
            },
            Msg::BlockExpand => {
                self.expanded = !self.expanded;
            },
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_toggle = self.link.callback(|_| Msg::BlockExpand);

        html!{
            <div class={"card"}>
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                <div class={"column is-flex is-justify-content-space-between is-align-items-center pointer"} onclick={onclick_toggle}>
                    <p class={"title is-5 select-title"} style="margin-bottom: 0px;">{get_value_field(&104)}</p>
                    <span class="icon is-clickable">
                        <i class={classes!("fas", if self.expanded { "fa-chevron-up" } else { "fa-chevron-down" })}></i>
                    </span>
                </div>
                {if self.expanded {
                  html!{
                      <>
                        <div class={"column"}>{self.filters()}</div>
                        <div class={"column"}>{self.result_area()}</div>
                      </>
                  }
                } else {
                    html!{}
                }}
            </div>
        }
    }
}

impl CatalogSpec {
    fn add_children_to_parent(&mut self, parent_id: usize, children: Vec<SpecNode>) {
        fn add_children_recursive(nodes: &mut Vec<SpecNode>, parent_id: usize, children: Vec<SpecNode>) -> bool {
            for node in nodes.iter_mut() {
                if node.spec_id == parent_id {
                    node.children = children;
                    return true;
                }
                if !node.children.is_empty() {
                    if add_children_recursive(&mut node.children, parent_id, children.clone()) {
                        return true;
                    }
                }
            }
            false
        }
        
        add_children_recursive(&mut self.spec_tree, parent_id, children);
    }

    fn result_area(&self) -> Html {
        html!{
            <aside class={"menu"}>
                {self.render_spec_tree(&self.spec_tree)}
            </aside>
        }
    }

    fn render_spec_tree(&self, nodes: &[SpecNode]) -> Html {
        html!{
            <ul class={"menu-list"}>
                {for nodes.iter().map(|node| self.render_spec_node(node))}
            </ul>
        }
    }

    fn render_spec_node(&self, node: &SpecNode) -> Html {
        let spec_id = node.spec_id;
        let mut p_class = classes!("is-7");
        let mut a_class = classes!("overflow-title", "is-flex-grow-1");
        
        if self.final_section && node.spec_id == self.current_spec_id {
            p_class.push("has-text-weight-bold");
        }

        if node.spec_id == self.current_spec_id {
            a_class.push("is-active");
        }

        html!{
            <li>
                <div class={p_class}>
                    <div class="icon-text is-flex-direction-row	is-align-items-center	">
                        <span class="icon is-small" onclick={self.link.callback(move |_| Msg::ToggleExpand(spec_id))}>
                            {if node.loading {
                                html!{<i class="fas fa-spinner fa-pulse"></i>}
                            } else if !node.children.is_empty() {
                                html!{<i class={classes!("fas", if node.expanded { "fa-chevron-down" } else { "fa-chevron-right" })}></i>}
                            } else {
                                html!{<i class="fas fa-minus"></i>}
                            }}
                        </span>
                        <a class={a_class} style="flex: 1;" onclick={self.link.callback(move |_| Msg::ChangeSpec(spec_id))}>
                            {format!{"{}", node.spec}}
                        </a>
                    </div>
                </div>
                {if node.expanded && !node.children.is_empty() {
                    self.render_spec_tree(&node.children)
                } else {
                    html!{}
                }}
            </li>
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

    fn set_node_loading(&mut self, spec_id: usize, loading: bool) {
        fn set_loading_recursive(nodes: &mut Vec<SpecNode>, spec_id: usize, loading: bool) -> bool {
            for node in nodes.iter_mut() {
                if node.spec_id == spec_id {
                    node.loading = loading;
                    return true;
                }
                if !node.children.is_empty() {
                    if set_loading_recursive(&mut node.children, spec_id, loading) {
                        return true;
                    }
                }
            }
            false
        }
        set_loading_recursive(&mut self.spec_tree, spec_id, loading);
    }

    fn toggle_node_expanded(&mut self, spec_id: usize) {
        fn toggle_expanded_recursive(nodes: &mut Vec<SpecNode>, spec_id: usize) -> bool {
            for node in nodes.iter_mut() {
                if node.spec_id == spec_id {
                    node.expanded = !node.expanded;
                    return true;
                }
                if !node.children.is_empty() {
                    if toggle_expanded_recursive(&mut node.children, spec_id) {
                        return true;
                    }
                }
            }
            false
        }
        toggle_expanded_recursive(&mut self.spec_tree, spec_id);
    }
}