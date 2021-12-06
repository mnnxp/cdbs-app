mod item;
pub use item::SpecTagItem;

use yew::{
    classes, NodeRef, html, Component, ComponentLink,
    InputData, Properties, ShouldRender, Html,
};
use yew::services::timeout::{TimeoutService, TimeoutTask};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use crate::gqls::make_query;
// use crate::error::{get_error, Error};
// use crate::fragments::standard_spec::{SpecsTags, SearchSpecsTags};
use crate::types::{StandardSpec, Spec, SearchSpec, UUID};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/specs.graphql",
    response_derives = "Debug"
)]
struct SearchSpecs;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub standard_specs: Vec<StandardSpec>,
    pub standard_uuid: UUID,
}

pub struct SearchSpecsTags {
    props: Props,
    link: ComponentLink<Self>,
    ipt_timer: Option<TimeoutTask>,
    ipt_ref: NodeRef,
    specs_search_loading: bool,
    search_specs: Vec<SearchSpec>,
    specs: Vec<Spec>,
}

pub enum Msg {
    ParseSpecs,
    AddedSpec(usize),
    SetIptTimer(String),
    GetSearchRes(String),
    Ignore,
}

impl Component for SearchSpecsTags {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            ipt_timer: None,
            ipt_ref: NodeRef::default(),
            specs_search_loading: false,
            search_specs: Vec::new(),
            specs: Vec::new(),
        }
    }

    // fn rendered(&mut self, first_render: bool) {}

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ParseSpecs => {
                self.specs = Vec::new(); // clear old result
                for spec in &self.search_specs {
                    self.specs.push(Spec::from(spec));
                }
                debug!("ParseSpecs {:?}", self.specs);
            }
            Msg::AddedSpec(spec_id) => {
                for spec in &self.search_specs {
                    if spec.spec_id == spec_id as i32 {
                        self.props.standard_specs.push(StandardSpec{
                            spec: spec.into(),
                            standard_uuid: self.props.standard_uuid.clone()
                        });
                        break;
                    }
                }
                debug!("AddedSpec {:?}", self.props.standard_specs);
            }
            Msg::SetIptTimer(val) => {
                debug!("ipt_val: {:?}", val);
                if val.is_empty() {
                    self.ipt_timer = None;
                    self.search_specs = Vec::new();
                    return true;
                }
                self.specs_search_loading = true;
                let cb_link = link.clone();
                self.ipt_timer = Some(TimeoutService::spawn(
                    Duration::from_millis(800),
                    cb_link.callback(move |_| {
                        let ipt_val = val.clone();
                        let res_link = link.clone();
                        spawn_local(async move {
                            let arg = Some(search_specs::IptSearchSpecArg {
                                text: ipt_val.clone(),
                                splitChar: None,
                                depthLevel: None,
                                limit: None,
                                offset: None,
                            });
                            let res =
                                make_query(SearchSpecs::build_query(search_specs::Variables {
                                    arg,
                                }))
                                .await
                                .unwrap();
                            res_link.send_message(Msg::GetSearchRes(res));
                        });
                        debug!("time up: {:?}", val.clone());
                        Msg::Ignore
                    }),
                ));
            }
            Msg::GetSearchRes(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();
                let search_specs: Vec<SearchSpec> =
                    serde_json::from_value(res.get("searchSpecs").unwrap().clone()).unwrap();
                // debug!(
                //     "specs res:{:?} {:?}",
                //     search_specs.iter().map(|x| Spec::from(x.clone())).collect::<Vec<Spec>>(),
                //     Spec::from(search_specs[0].clone())
                // );
                self.specs_search_loading = false;
                self.search_specs = search_specs;
                link.send_message(Msg::ParseSpecs);
            }
            Msg::Ignore => {}
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            {self.fieldset_manage_specs()}
        }
    }
}

impl SearchSpecsTags {
    fn fieldset_manage_specs(&self) -> Html {
        let ipt_ref = self.ipt_ref.clone();
        let onclick_added_spec = self.link
            .callback(|value: usize| Msg::AddedSpec(value));

        html! {<>
            <div class="panel-block">
              <p class=classes!(String::from("control has-icons-left"),if self.specs_search_loading {
                String::from("is-loading")
              } else {
                String::new()
              }) >
                <input ref=ipt_ref
                    oninput=self.link.callback(|ev: InputData| Msg::SetIptTimer(ev.value))
                    class="input"
                    type="text"
                    placeholder="Rounded input"
                  />
                <span class="icon is-left">
                  <i class="fas fa-search" aria-hidden="true"></i>
                </span>
              </p>
            </div>
            <div class="panel-block">
                <div id="search-specs" class="tags search_res_box">
                    {for self.specs.iter().map(|spec| {
                        if self.props.standard_specs.iter().any(|x| &x.spec.spec_id == &spec.spec_id) {
                            html!{}
                        } else {
                            html! {<SpecTagItem
                                standard_uuid = self.props.standard_uuid.clone()
                                spec = spec.clone()
                                is_added = false
                                added_spec = Some(onclick_added_spec.clone())
                                />}
                        }
                    })}
                </div>
            </div>
            <div class="panel-block">
                <div id="standard-specs" class="tags search_res_box">
                    {for self.props.standard_specs.iter().map(|st_spec| {
                        html! {<SpecTagItem
                            standard_uuid = self.props.standard_uuid.clone()
                            spec = st_spec.spec.clone()
                            is_added = self.props.standard_specs.iter().any(|x| &x.spec.spec_id == &st_spec.spec.spec_id)
                            />}
                    })}
                </div>
            </div>
        </>}
    }
}
