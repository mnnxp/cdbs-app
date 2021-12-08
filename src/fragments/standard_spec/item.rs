use yew::{
    html, Callback, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{UUID, Spec};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct AddStandardSpecs;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct DeleteStandardSpecs;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub standard_uuid: UUID,
    pub spec: Spec,
    pub is_added: bool,
    pub style_tag: Option<String>,
    pub added_spec: Option<Callback<usize>>,
    pub delete_spec: Option<Callback<usize>>,
}

pub struct SpecTagItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    is_added: bool,
    get_result_delete: bool,
}

pub enum Msg {
    RequestDeleteSpec,
    RequestAddSpec,
    ResponseError(Error),
    GetAddedSpecResult(String),
    GetDeleteSpecResult(String),
    ClearError,
}

impl Component for SpecTagItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let is_added = props.is_added;
        Self {
            error: None,
            props,
            link,
            is_added,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDeleteSpec => {
                let standard_uuid = self.props.standard_uuid.clone();
                let spec_id = self.props.spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_standard_specs_data = delete_standard_specs::IptStandardSpecsData{
                        standardUuid: standard_uuid,
                        specIds: vec![spec_id],
                    };
                    let res = make_query(DeleteStandardSpecs::build_query(
                        delete_standard_specs::Variables {
                            ipt_standard_specs_data,
                        }
                    )).await;
                    link.send_message(Msg::GetDeleteSpecResult(res.unwrap()));
                })
            },
            Msg::RequestAddSpec => {
                let standard_uuid = self.props.standard_uuid.clone();
                let spec_id = self.props.spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_standard_specs_data = add_standard_specs::IptStandardSpecsData {
                        standardUuid: standard_uuid,
                        specIds: vec![spec_id],
                    };
                    let res =
                        make_query(AddStandardSpecs::build_query(add_standard_specs::Variables {
                            ipt_standard_specs_data,
                        }))
                        .await;
                    link.send_message(Msg::GetAddedSpecResult(res.unwrap()));
                })
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
            },
            Msg::GetAddedSpecResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res.get("addStandardSpecs").unwrap().clone()).unwrap();
                        debug!("addStandardSpecs: {:?}", result);
                        // self.get_result_delete = result > 0;
                        match &self.props.added_spec {
                            Some(added_spec) => {
                                if result > 0 {
                                    self.is_added = true;
                                    self.get_result_delete = false;
                                    added_spec.emit(self.props.spec.spec_id);
                                };
                            },
                            None => self.is_added = result > 0,
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::GetDeleteSpecResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res.get("deleteStandardSpecs").unwrap().clone()).unwrap();
                        debug!("deleteStandardSpecs: {:?}", result);
                        // self.get_result_delete = result > 0;
                        match &self.props.delete_spec {
                            Some(delete_spec) => {
                                if result > 0 {
                                    self.is_added = false;
                                    self.get_result_delete = true;
                                    delete_spec.emit(self.props.spec.spec_id);
                                };
                            },
                            None => self.get_result_delete = result > 0,
                        }
                    },
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
                }
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link
            .callback(|_| Msg::ClearError);

        html! {<>
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            <br/>
            {match self.get_result_delete {
                true => html! {},
                false => self.show_spec(),
            }}
        </>}
    }
}

impl SpecTagItem {
    fn show_spec(&self) -> Html {
        let onclick_delete_spec = self.link.callback(|_| Msg::RequestDeleteSpec);
        let onclick_add_spec = self.link.callback(|_| Msg::RequestAddSpec);

        // let style_tag = match &self.props.is_added {
        //     true => "tag is-light is-info",
        //     false => "tag is-light is-success",
        // };
        let style_tag = match &self.props.style_tag {
            Some(style) => format!("tag is-light {}", style),
            None => "tag is-light".to_string(),
        };

        html!{<div class="control">
          <div class="tags has-addons">
            <span class={style_tag}>{self.props.spec.spec.clone()}</span>
            {if self.props.show_manage_btn {
                match &self.props.is_added {
                    true => html! {<a class="tag is-delete is-small is-light" onclick={onclick_delete_spec} />},
                    false => html! {<a class="tag is-small is-light is-success" onclick={onclick_add_spec}>
                        <i class="fas fa-plus" />
                    </a>},
                }
            } else {html!{}}}
          </div>
        </div>}
    }
}
