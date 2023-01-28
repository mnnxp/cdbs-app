use yew::{Component, Callback, Context, html, Html, Properties};
use yew::html::{Scope, TargetCast};
use web_sys::{InputEvent, HtmlInputElement};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_value_field, resp_parsing_item};
use crate::gqls::make_query;
use crate::gqls::relate::{RegisterParam, register_param};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub callback_add_param: Callback<(usize, String)>,
}

pub struct RegisterParamnameBlock {
    error: Option<Error>,
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
    ResponseError(Error),
    Ignore,
}

impl Component for RegisterParamnameBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            request_new_paramname: String::new(),
            set_param_value: String::new(),
            active_loading_btn: false,
            disable_btn: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::RequestRegisterParamname => {
                self.active_loading_btn = true;
                self.disable_btn = true;
                let ipt_param_translate_list_data = register_param::IptParamTranslateListData{
                    lang_id: 1, // todo!(fix for different lang)
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
                let value: usize = resp_parsing_item(res, "registerParam")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                ctx.props().callback_add_param.emit((value, self.set_param_value.clone()));
                self.active_loading_btn = false;
                // clear old data
                self.request_new_paramname.clear();
                self.set_param_value.clear();
            },
            Msg::UpdateParamname(data) => {
                self.disable_btn = false;
                self.request_new_paramname = data;
            },
            Msg::UpdateParamValue(data) => self.set_param_value = data,
            Msg::ClearError => self.error = None,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::Ignore => {},
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={Some(onclick_clear_error.clone())}/>
            {self.add_paramname(ctx.link())}
        </>}
    }
}

impl RegisterParamnameBlock {
    fn add_paramname(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_register_paramname = link.callback(|_| Msg::RequestRegisterParamname);
        let oninput_set_paramname = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateParamname(input.value())
        });
        let oninput_set_param_value = link.callback(|ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            Msg::UpdateParamValue(input.value())
        });
        let class_btn = match self.active_loading_btn {
            true => "button is-loading is-fullwidth",
            false => "button is-fullwidth",
        };

        html!{<>
            <div class="column">
                <label class="label">{ get_value_field(&205) }</label> // Set a paramname (letter case has matter)
                <input
                    id="paramname"
                    class="input is-fullwidth"
                    type="text"
                    placeholder={get_value_field(&205)}
                    value={self.request_new_paramname.clone()}
                    oninput={oninput_set_paramname}
                    />
            </div>
            <div class="column">
                <label class="label">{get_value_field(&133)}</label> // Set a value
                <input
                    id="param-value"
                    class="input is-fullwidth"
                    type="text"
                    placeholder={get_value_field(&133)}
                    value={self.set_param_value.clone()}
                    oninput={oninput_set_param_value}
                    />
            </div>
            <div class="column">
                <button
                    id="add-paramname"
                    class={class_btn}
                    disabled={self.disable_btn ||
                        self.request_new_paramname.is_empty() ||
                        self.set_param_value.is_empty()}
                    onclick={onclick_register_paramname} >
                    { get_value_field(&117) }
                </button>
            </div>
        </>}
    }
}
