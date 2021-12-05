use graphql_client::GraphQLQuery;
use log::debug;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{Spec, UUID};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct DeleteStandardSpecs;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
struct AddStandardSpecs;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub standard_uuid: UUID,
    pub spec: Spec,
    pub is_added: bool,
    pub added_spec: Option<Callback<usize>>,
}

pub struct SpecTagItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    is_added: bool,
}

pub enum Msg {
    RequestDeleteSpec,
    ResponseError(Error),
    GetSpecResult(String, String),
    RequestAddSpec,
    Ignore,
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDeleteSpec => {
                let standard_uuid = self.props.standard_uuid.clone();
                let spec_id = self.props.spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_standard_specs_data = delete_standard_specs::IptStandardSpecsData {
                        standardUuid: standard_uuid,
                        specIds: vec![spec_id],
                    };
                    let res = make_query(DeleteStandardSpecs::build_query(
                        delete_standard_specs::Variables {
                            ipt_standard_specs_data,
                        },
                    ))
                    .await;
                    link.send_message(Msg::GetSpecResult(
                        res.unwrap(),
                        "deleteStandardSpecs".to_string(),
                    ));
                })
            }
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
                    link.send_message(Msg::GetSpecResult(
                        res.unwrap(),
                        "addStandardSpecs".to_string(),
                    ));
                });
                if let Some(added_spec) = &self.props.added_spec {
                    added_spec.emit(self.props.spec.spec_id)
                }
            }
            Msg::ResponseError(err) => {
                self.error = Some(err);
            }
            Msg::GetSpecResult(res, get_type) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: usize =
                            serde_json::from_value(res.get(get_type.clone()).unwrap().clone())
                                .unwrap();
                        debug!("{}: {:?}", get_type, result);
                        // self.is_added = result > 0;
                        self.is_added = !self.is_added;
                    }
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            }
            Msg::Ignore => {
                self.is_added = !self.is_added;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        debug!("change: {:?}, {:?}", props, self.props);
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {<>
            <ListErrors error=self.error.clone()/>
            {self.show_spec()}
        </>}
    }
}

impl SpecTagItem {
    fn show_spec(&self) -> Html {
        let onclick_delete_spec = self.link.callback(|_| Msg::RequestDeleteSpec);
        let onclick_add_spec = self.link.callback(|_| Msg::RequestAddSpec);

        match self.is_added {
            true => html!{<div class="tag is-light">
                {self.props.spec.spec.clone()}
                <button class="delete is-small"
                    onclick={onclick_delete_spec}
                />
            </div>},
            false => html!{<div class="tag is-light is-info button"
                onclick={onclick_add_spec} >
                {self.props.spec.spec.clone()}
            </div>},
        }
    }
}
