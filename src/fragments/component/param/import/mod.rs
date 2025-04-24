mod parser_raw;

use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use parser_raw::parsing_single;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_import_btn;
use crate::fragments::notification::show_notification;
use crate::types::{Param, ParamValue, UUID};
use crate::services::{get_value_field, resp_parsing};
use crate::services::content_adapter::Markdownable;
use crate::gqls::make_query;
use crate::gqls::relate::{RegisterParamsBulk, register_params_bulk};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub component_uuid: UUID,
    pub callback_add_params: Callback<Vec<ParamValue>>,
}

pub struct ImportParamsData {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    hide_import: bool,
    new_params_raw: String,
    parsed_params: Vec<(String, String)>,
    new_params: Vec<ParamValue>,
    stat_info: String,
}

pub enum Msg {
    RequestRegisterParams,
    GetRegisterParamsResult(String),
    PreparingImport,
    ResponseError(Error),
    ShowImport,
    UpdateData(String),
    ClearError,
}

impl Component for ImportParamsData {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            hide_import: true,
            new_params_raw: String::new(),
            parsed_params: Vec::new(),
            new_params: Vec::new(),
            stat_info: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestRegisterParams => {
                debug!("Raw data: {}", self.new_params_raw);
                self.parsed_params = parsing_single(&self.new_params_raw);
                debug!("Complete self.parsed_params: {:?}", self.parsed_params);
                let mut ipt_params_translate_list_data = Vec::new();
                for (p, _) in &self.parsed_params {
                    ipt_params_translate_list_data.push(
                        register_params_bulk::IptParamTranslateListData{
                            langId: 1, // todo!(fix for different lang)
                            paramname: p.clone(),
                        }
                    );
                }
                spawn_local(async move {
                    let res = make_query(RegisterParamsBulk::build_query(
                        register_params_bulk::Variables { ipt_params_translate_list_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetRegisterParamsResult(res));
                })
            },
            Msg::GetRegisterParamsResult(res) => {
                match resp_parsing::<Vec<Param>>(res, "registerParamsBulk") {
                    Ok(result) => {
                        self.new_params.clear();
                        for (p, v) in self.parsed_params.iter() {
                            for r in &result {
                                if &r.paramname == p {
                                    self.new_params.push(
                                        ParamValue{ param_id: r.param_id, value: v.clone() }
                                    );
                                }
                            }
                        }
                        debug!("Complete self.new_params: {:?}", self.new_params);
                        link.send_message(Msg::PreparingImport);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::PreparingImport => {
                self.new_params_raw.clear();
                // self.stat_info = format!("{} {}", get_value_field(&213), result);
                self.hide_import = true;
                self.props.callback_add_params.emit(self.new_params.clone());
                self.new_params.clear();
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ShowImport => {
                self.stat_info.clear();
                self.hide_import = !self.hide_import;
            },
            Msg::UpdateData(value) => {
                self.new_params_raw = value;
                if self.new_params_raw.is_empty() {
                    self.stat_info.clear();
                    return true
                }
                let mut row_count = 0;
                for (_n, c) in self.new_params_raw.char_indices() {
                    if c == '\n' {
                        row_count += 1;
                    }
                }
                self.stat_info = format!("{}: {}", get_value_field(&346), row_count);
            },
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_show_import = self.link.callback(|_| Msg::ShowImport);
        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {show_notification(&self.stat_info, "is-success", self.hide_import && !self.stat_info.is_empty())}
            {match self.hide_import {
                true => ft_import_btn("open-import-btn", onclick_show_import, get_value_field(&209), false, false),
                false => self.show_import_modal(),
            }}
        </>}
    }
}

impl ImportParamsData {
    fn show_import_modal(&self) -> Html {
        let onclick_hide_modal = self.link.callback(|_| Msg::ShowImport);
        let oninput_data = self.link.callback(|ev: InputData| Msg::UpdateData(ev.value));
        let onclick_subbmit = self.link.callback(|_| Msg::RequestRegisterParams);
        let class_modal = match &self.hide_import {
            true => "modal",
            false => "modal is-active",
        };
        html!{
            <div class={class_modal}>
              <div class={"modal-background"} onclick={onclick_hide_modal.clone()} />
                <div class={"modal-content"}>
                  <div class={"card"}>
                    <header class={"modal-card-head"}>
                      <div class={"modal-card-title"}>
                        <p>{get_value_field(&209)}</p>
                      </div>
                      <button class={"delete"} aria-label="close" onclick={onclick_hide_modal.clone()} />
                    </header>
                    <section class={"modal-card-body"}>
                        <div class={"column"}>
                            <div class={"subtitle is-6"}>
                                {get_value_field(&234)}<br/>
                                {get_value_field(&235).to_markdown()}
                            </div>
                            <textarea
                                id={"update-description"}
                                class={"textarea"}
                                type={"text"}
                                placeholder={get_value_field(&208)}
                                value={self.new_params_raw.clone()}
                                oninput={oninput_data}
                                />
                            <p class={"help"}>{self.stat_info.clone()}</p>
                        </div>
                        <div class={"column"}>
                            {ft_import_btn(
                                "import-params-btn",
                                onclick_subbmit,
                                get_value_field(&209),
                                true,
                                self.new_params_raw.is_empty()
                            )}
                        </div>
                      </section>
                  </div>
                </div>
              </div>
        }
    }
}
