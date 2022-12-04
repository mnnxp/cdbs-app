use yew::{Component, Context, html, Html, Properties, classes, NodeRef};
use yew::html::{Scope, TargetCast};
use web_sys::{InputEvent, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use gloo_timers::callback::Timeout;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::fragments::company::{SpecsTags, SpecTagItem};
use crate::types::{Spec, SpecPathInfo, UUID};
use crate::services::get_value_field;
use crate::gqls::make_query;
use crate::gqls::relate::{SearchSpecs, search_specs};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub company_specs: Vec<Spec>,
    pub company_uuid: UUID,
}

pub struct SearchSpecsTags {
    company_uuid: UUID,
    company_specs: Vec<Spec>,
    ipt_timer: Option<Timeout>,
    ipt_ref: NodeRef,
    specs_search_loading: bool,
    search_specs: Vec<SpecPathInfo>,
    found_specs: Vec<Spec>,
    added_specs: Vec<Spec>,
}

pub enum Msg {
    ParseSpecs,
    AddedSpec(usize),
    SetIptTimer(String),
    GetSearchRes(String),
    DeleteNewSpec(usize),
    DeleteCurrentSpec(usize),
    Ignore,
}

impl Component for SearchSpecsTags {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            company_uuid: ctx.props().company_uuid.clone(),
            company_specs: ctx.props().company_specs.clone(),
            ipt_timer: None,
            ipt_ref: NodeRef::default(),
            specs_search_loading: false,
            search_specs: Vec::new(),
            found_specs: Vec::new(),
            added_specs: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::ParseSpecs => {
                let mut del_specs_ids: Vec<usize> = Vec::new(); // for collect ids for removing
                let count_old_found = self.found_specs.len(); // calculate and used twice
                let mut temp_found: Vec<Spec> = Vec::new(); // for save size found_specs array
                temp_found.resize(count_old_found, Spec::default());
                for spec in &self.company_specs {
                    del_specs_ids.push(spec.spec_id);
                }
                // debug!("self.added_specs: {:?}", self.added_specs);
                for spec in &self.added_specs {
                    del_specs_ids.push(spec.spec_id);
                }
                // debug!("del_specs_ids: {:?}", del_specs_ids);
                let mut flag: bool;
                for spec in &self.search_specs {
                    flag = true;
                    for del in &del_specs_ids {
                        if &spec.spec_id == del {
                            flag = false;
                            break;
                        }
                    }
                    if flag {
                        temp_found.push(Spec::from(spec));
                    }
                }
                match temp_found.len() == count_old_found {
                    true => self.found_specs = Vec::new(),
                    false => self.found_specs = temp_found,
                }
                debug!("ParseSpecs {:?}", self.found_specs);
            },
            Msg::AddedSpec(spec_id) => {
                let mut found_specs_empty = true;
                let mut found_specs: Vec<Spec> = Vec::new();
                for spec in &self.found_specs {
                    if spec.spec_id == spec_id {
                        found_specs.push(Spec::default());
                        self.added_specs.push(spec.clone());
                    } else {
                        found_specs.push(spec.clone());
                        found_specs_empty = false;
                    }
                }
                if found_specs_empty {
                    self.found_specs = Vec::new();
                } else {
                    self.found_specs = found_specs;
                }
                debug!("FoundSpecs {:?}", self.found_specs);
                debug!("AddedSpec {:?}", self.added_specs);
            },
            Msg::SetIptTimer(val) => {
                debug!("ipt_val: {:?}", val);
                if val.is_empty() {
                    self.ipt_timer = None;
                    self.search_specs = Vec::new();
                    return true;
                }
                self.specs_search_loading = true;
                self.ipt_timer = Some(Timeout::new(800, move || {
                        let ipt_val = val.clone();
                        let res_link = link.clone();
                        spawn_local(async move {
                            let ipt_search_spec_arg = search_specs::IptSearchSpecArg {
                                text: ipt_val.clone(),
                                split_char: None,
                                depth_level: None,
                                limit: None,
                                offset: None,
                            };
                            let res = make_query(SearchSpecs::build_query(search_specs::Variables {
                                ipt_search_spec_arg
                            })).await.unwrap();
                            res_link.send_message(Msg::GetSearchRes(res));
                        });
                        debug!("time up: {:?}", val.clone());
                    })
                );
            },
            Msg::GetSearchRes(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();
                let search_specs: Vec<SpecPathInfo> =
                    serde_json::from_value(res.get("searchSpecs").unwrap().clone()).unwrap();
                // debug!(
                //     "found_specs res:{:?} {:?}",
                //     search_specs.iter().map(|x| Spec::from(x.clone())).collect::<Vec<Spec>>(),
                //     Spec::from(search_specs[0].clone())
                // );
                self.specs_search_loading = false;
                self.search_specs = search_specs;
                link.send_message(Msg::ParseSpecs);
            },
            Msg::DeleteNewSpec(spec_id) => {
                // debug!("self.found_specs before delete: {:?}", self.found_specs);
                let mut added_specs_empty = true;
                let mut added_specs: Vec<Spec> = Vec::new();
                for spec in &self.added_specs {
                    if spec.spec_id == spec_id {
                        added_specs.push(Spec::default());
                        // return spec to found specs list
                        self.found_specs.push(spec.clone());
                    } else {
                        added_specs.push(spec.clone());
                        added_specs_empty = false;
                    }
                }
                if added_specs_empty {
                    self.added_specs = Vec::new();
                } else {
                    self.added_specs = added_specs;
                }
                debug!("self.added_specs after delete: {:?}", self.added_specs);
            },
            Msg::DeleteCurrentSpec(spec_id) => {
                let mut props_specs: Vec<Spec> = Vec::new();
                for spec in self.company_specs.iter() {
                    if spec.spec_id == spec_id {
                        props_specs.push(Spec::default());
                    } else {
                        props_specs.push(spec.clone());
                    }
                }
                self.company_specs = props_specs;
            },
            Msg::Ignore => {},
        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.company_uuid == ctx.props().company_uuid {
            false
        } else {
            self.company_uuid = ctx.props().company_uuid.clone();
            self.company_specs = ctx.props().company_specs.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{self.fieldset_manage_specs(ctx.link(), ctx.props())}
    }
}

impl SearchSpecsTags {
    fn fieldset_manage_specs(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let ipt_ref = self.ipt_ref.clone();
        let oninput_spec = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::SetIptTimer(input.value())
        });
        let onclick_added_spec = link.callback(|value: usize| Msg::AddedSpec(value));
        let onclick_del_new_spec = link.callback(|value: usize| Msg::DeleteNewSpec(value));
        let onclick_del_old_spec = link.callback(|value: usize| Msg::DeleteCurrentSpec(value));

        let mut class_p = classes!("control", "has-icons-left");
        if self.specs_search_loading {
            class_p.push("is-loading");
        }

        html!{<>
            <div class="panel-block">
              <p class={class_p}>
                <input ref={ipt_ref}
                    oninput={oninput_spec}
                    class="input"
                    type="text"
                    placeholder={get_value_field(&192)} // Enter data for specifications search
                  />
                <span class="icon is-left">
                  <i class="fas fa-search" aria-hidden="true"></i>
                </span>
              </p>
            </div>
            <div class="panel-block">
                <div id="new-specs" class="field is-grouped is-grouped-multiline">
                    {for self.found_specs.iter().map(|spec| {
                        if spec.spec.is_empty() {
                            html!{}
                        } else {
                            html!{<SpecTagItem
                                show_manage_btn = true
                                active_info_btn = false
                                company_uuid = {props.company_uuid.clone()}
                                spec = {spec.clone()}
                                is_added = false
                                style_tag = {"is-success".to_string()}
                                added_spec = {Some(onclick_added_spec.clone())}
                                // delete_spec = None
                                />
                    }}})}
                </div>
            </div>
            <div class="panel-block">
                <div id="add-specs" class="field is-grouped is-grouped-multiline">
                    {for self.added_specs.iter().map(|st_spec| {
                        html!{<SpecTagItem
                            show_manage_btn = true
                            active_info_btn = false
                            company_uuid = {props.company_uuid.clone()}
                            spec = {st_spec.clone()}
                            is_added = true
                            style_tag = {"is-info".to_string()}
                            // added_spec = None
                            delete_spec = {Some(onclick_del_new_spec.clone())}
                            />}
                    })}
                </div>
            </div>
            <div class="panel-block">
                <SpecsTags
                    show_manage_btn = {true}
                    company_uuid = {props.company_uuid.clone()}
                    specs = {props.company_specs.clone()}
                    delete_spec = {Some(onclick_del_old_spec.clone())}
                    />
            </div>
        </>}
    }
}
