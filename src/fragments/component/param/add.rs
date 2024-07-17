use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html, InputData};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_save_btn;
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::relate::{RegisterParam, register_param};

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
    ResponseError(Error),
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
                match resp_parsing(res, "registerParam") {
                    Ok(result) => {
                        debug!("registerParam: {:?}", result);
                        self.props.callback_add_param.emit((result, self.set_param_value.clone()));
                        self.active_loading_btn = false;
                        // clear old data
                        self.request_new_paramname.clear();
                        self.set_param_value.clear();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::UpdateParamname(data) => {
                self.disable_btn = false;
                self.request_new_paramname = data;
            },
            Msg::UpdateParamValue(data) => self.set_param_value = data,
            Msg::ResponseError(err) => self.error = Some(err),
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
        let oninput_set_paramname = self.link.callback(|ev: InputData| Msg::UpdateParamname(ev.value));
        let oninput_set_param_value = self.link.callback(|ev: InputData| Msg::UpdateParamValue(ev.value));

        html!{<>
            <div class="column">
                <label class="label">{ get_value_field(&205) }</label> // Set a paramname (letter case has matter)
                <input
                    id="paramname"
                    class="input is-fullwidth"
                    type="text"
                    placeholder=get_value_field(&205)
                    value={self.request_new_paramname.clone()}
                    oninput=oninput_set_paramname
                    />
            </div>
            <div class="column">
                <label class="label">{ get_value_field(&133) }</label> // Set a value
                <input
                    id="param-value"
                    class="input is-fullwidth"
                    type="text"
                    placeholder=get_value_field(&133)
                    value={self.set_param_value.clone()}
                    oninput=oninput_set_param_value
                    />
            </div>
            <div class="column">
                {ft_save_btn(
                    "add-paramname",
                    onclick_register_paramname,
                    self.disable_btn || self.request_new_paramname.is_empty() ||
                        self.set_param_value.is_empty(),
                    self.set_param_value.is_empty()
                )}
            </div>
        </>}
    }
}
