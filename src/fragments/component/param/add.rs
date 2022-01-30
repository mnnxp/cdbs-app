use yew::prelude::*;
use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct RegisterParam;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub callback_add_param: Callback<(usize, String)>,
}

pub struct RegisterParamnameBlock {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    request_new_paramname: String,
    set_param_value: String,
    active_loading_btn: bool,
    disable_btn: bool,
}

#[derive(Clone)]
pub enum Msg {
    RequestRegisterParamname,
    GetRegisterParamnameResult(String),
    UpdateParamname(String),
    UpdateParamValue(String),
    ClearError,
    Ignore,
}

impl Component for RegisterParamnameBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            request_new_paramname: String::new(),
            set_param_value: String::new(),
            active_loading_btn: false,
            disable_btn: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestRegisterParamname => {
                self.active_loading_btn = true;
                self.disable_btn = true;
                let ipt_param_translate_list_data = register_param::IptParamTranslateListData{
                    langId: 1, // todo!(fix for different lang)
                    paramname: self.request_new_paramname.clone(),
                };
                spawn_local(async move {
                    let res = make_query(RegisterParam::build_query(
                        register_param::Variables { ipt_param_translate_list_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetRegisterParamnameResult(res));
                })
            },
            Msg::GetRegisterParamnameResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let value: usize =
                            serde_json::from_value(res_value.get("registerParam").unwrap().clone()).unwrap();
                        debug!("registerParam: {:?}", value);
                        self.props.callback_add_param.emit((value, self.set_param_value.clone()));
                        self.active_loading_btn = false;
                    },
                    true => self.error = Some(get_error(&data)),
                }
            },
            Msg::UpdateParamname(data) => {
                self.disable_btn = false;
                self.request_new_paramname = data;
            },
            Msg::UpdateParamValue(data) => self.set_param_value = data,
            Msg::ClearError => self.error = None,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error=self.error.clone() clear_error=Some(onclick_clear_error.clone())/>
            {self.add_paramname()}
        </>}
    }
}

impl RegisterParamnameBlock {
    fn add_paramname(&self) -> Html {
        let onclick_register_paramname = self.link.callback(|_| Msg::RequestRegisterParamname);

        let oninput_set_paramname =
            self.link.callback(|ev: InputData| Msg::UpdateParamname(ev.value));

        let oninput_set_param_value =
            self.link.callback(|ev: InputData| Msg::UpdateParamValue(ev.value));

        let class_btn = match self.active_loading_btn {
            true => "button is-loading is-fullwidth",
            false => "button is-fullwidth",
        };

        html!{<>
            <div class="column">
                <label class="label">{"Set paramname (letter case has matter)"}</label>
                <input
                    id="paramname"
                    class="input is-fullwidth"
                    type="text"
                    placeholder="param name"
                    value={self.request_new_paramname.clone()}
                    oninput=oninput_set_paramname
                    />
            </div>
            <div class="column">
                <label class="label">{"Set value"}</label>
                <input
                    id="param-value"
                    class="input is-fullwidth"
                    type="text"
                    placeholder="value"
                    value={self.set_param_value.clone()}
                    oninput=oninput_set_param_value
                    />
            </div>
            <div class="column">
                <button
                    id="add-paramname"
                    class=class_btn
                    disabled={self.disable_btn || self.request_new_paramname.is_empty() ||
                        self.set_param_value.is_empty()}
                    onclick={onclick_register_paramname} >
                    {"Add"}
                </button>
            </div>
        </>}
    }
}
