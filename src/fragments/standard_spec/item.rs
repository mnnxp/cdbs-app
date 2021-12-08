use yew::{
    html, Component, ComponentLink,
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
struct DeleteStandardSpecs;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub standard_uuid: UUID,
    pub spec: Spec,
}

pub struct SpecTagItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    get_result_delete: bool,
}

pub enum Msg {
    RequestDeleteSpec,
    ResponseError(Error),
    GetDeleteSpecResult(String),
}

impl Component for SpecTagItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            error: None,
            props,
            link,
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
            Msg::ResponseError(err) => {
                self.error = Some(err);
            },
            Msg::GetDeleteSpecResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: usize = serde_json::from_value(res.get("deleteStandardSpecs").unwrap().clone()).unwrap();
                        debug!("deleteStandardSpecs: {:?}", result);
                        self.get_result_delete = result > 0;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {<>
            <ListErrors error=self.error.clone()/>
            {match self.get_result_delete {
                true => html! {},
                false => self.show_spec(),
            }}
        </>}
    }
}

impl SpecTagItem {
    fn show_spec(
        &self,
    ) -> Html {
        let onclick_delete_spec = self
            .link
            .callback(|_| Msg::RequestDeleteSpec);

        match &self.props.show_delete_btn {
            true => html! {
                <div class="tag is-light">
                    {self.props.spec.spec.clone()}
                    <button class="delete is-small"
                        onclick=onclick_delete_spec/>
                </div>
            },
            false => html! {
                <div class="tag is-light">
                    {self.props.spec.spec.clone()}
                </div>
            },
        }
    }
}
