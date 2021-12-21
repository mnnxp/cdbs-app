use graphql_client::GraphQLQuery;
use log::debug;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::{Spec, UUID};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct DeleteCompanySpecs;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
struct AddCompanySpecs;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    // pub show_delete_btn: bool,
    pub company_uuid: UUID,
    pub spec: Spec,
    pub is_added: bool,
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
                let company_uuid = self.props.company_uuid.clone();
                let spec_id = self.props.spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_company_spec_data = delete_company_specs::IptCompanySpecData {
                        companyUuid: company_uuid,
                        specIds: vec![spec_id],
                    };
                    let res = make_query(DeleteCompanySpecs::build_query(delete_company_specs::Variables {
                        ipt_company_spec_data,
                    })).await.unwrap();
                    link.send_message(Msg::GetSpecResult(res, "deleteCompanySpecs".to_string()));
                })
            }
            Msg::RequestAddSpec => {
                let company_uuid = self.props.company_uuid.clone();
                let spec_id = self.props.spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_company_spec_data = add_company_specs::IptCompanySpecData {
                        companyUuid: company_uuid,
                        specIds: vec![spec_id],
                    };
                    let res = make_query(AddCompanySpecs::build_query(add_company_specs::Variables {
                        ipt_company_spec_data,
                    })).await.unwrap();
                    link.send_message(Msg::GetSpecResult(res, "addCompanySpecs".to_string()));
                })
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
                    true => link.send_message(Msg::ResponseError(get_error(&data))),
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
            // {match self.is_added {
            //     true => html! {},
            //     false => self.show_spec(),
            // }}
        </>}
    }
}

impl SpecTagItem {
    fn show_spec(&self) -> Html {
        let onclick_delete_spec = self.link.callback(|_| Msg::RequestDeleteSpec);
        let onclick_add_spec = self.link.callback(|_| Msg::RequestAddSpec);
        // let show_btn = self.props.show_delete_btn;
        // debug!("show_btn: {:?}", show_btn);

        html! {
            <div class="tag is-light">
                {self.props.spec.spec.clone()}
                <div>
                  <button class=classes!("delete","is-small", if self.is_added {
                      ""
                  } else {
                    "to_add"
                  })
                  onclick={if self.is_added {
                    onclick_delete_spec
                  } else {
                    onclick_add_spec
                  }}
                  />
                </div>
            </div>
        }
    }
}
