mod parser_raw;

use std::collections::BTreeMap;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use parser_raw::parsing_text;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::ft_import_btn;
use crate::fragments::notification::show_notification;
use crate::types::{NewModificationsPreData, Param, ParamValue, UUID};
use crate::services::{get_value_field, resp_parsing};
use crate::services::content_adapter::Markdownable;
use crate::gqls::make_query;
use crate::gqls::relate::{RegisterParamsBulk, register_params_bulk};
use crate::gqls::component::{RegisterComponentModificationsBulk, register_component_modifications_bulk};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub component_uuid: UUID,
    pub callback_finish_import: Callback<()>,
}

pub struct ImportModificationsData {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    hide_import: bool,
    new_modifications_raw: String,
    value_column: Vec<ModificationValueColumn>,
    paramnames: Vec<String>,
    new_modifications: Vec<NewModificationsPreData>,
    params_with_ids: BTreeMap<usize, (String, usize)>,
    stat_info: String,
}

#[derive(Debug)]
pub enum ModificationValueColumn {
    // Uuid(usize),
    Name(usize),
    Description(usize),
    ActualStatusId(usize),
}

pub enum Msg {
    RequestRegisterParams,
    GetRegisterParamsResult(String),
    RequestRegisterModifications,
    GetRegisterModificationsResult(String),
    ResponseError(Error),
    ShowImport,
    UpdateData(String),
    Parsing,
    PreparingImport,
    ClearError,
}

impl Component for ImportModificationsData {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            hide_import: true,
            new_modifications_raw: String::new(),
            value_column: Vec::new(),
            paramnames: Vec::new(),
            new_modifications: Vec::new(),
            params_with_ids: BTreeMap::new(),
            stat_info: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::RequestRegisterParams => {
                let mut ipt_params_translate_list_data = Vec::new();
                for p in &self.paramnames {
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
                        self.params_with_ids.clear();
                        for (number, p) in self.paramnames.iter().enumerate() {
                            if &result[number].paramname == p {
                                self.params_with_ids.insert(
                                    number,
                                    (result[number].paramname.clone(), result[number].param_id)
                                );
                                continue;
                            }
                            debug!("Parameter named {} not found by index", p);
                            for r in &result {
                                if &r.paramname == p {
                                    self.params_with_ids.insert(
                                        number,
                                        (r.paramname.clone(), r.param_id)
                                    );
                                }
                            }
                        }
                        debug!("Complete self.params_with_ids: {:?}", self.params_with_ids);
                        link.send_message(Msg::PreparingImport);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::RequestRegisterModifications => {
                let mut modifications_data= Vec::new();
                for nm in &self.new_modifications {
                    let mut ipt_param_data = Vec::new();
                    for p in &nm.params {
                        ipt_param_data.push(
                            register_component_modifications_bulk::IptParamData {
                                paramId: p.param_id as i64,
                                value: p.value.clone()
                            }
                        );
                    }
                    modifications_data.push(register_component_modifications_bulk::IptModificationsData{
                        modificationName: nm.modification_name.clone(),
                        description: nm.description.clone(),
                        actualStatusId: nm.actual_status_id as i64,
                        parameters: ipt_param_data,
                    });
                }
                let ipt_multiple_modifications_data = register_component_modifications_bulk::IptMultipleModificationsData{
                    componentUuid: self.props.component_uuid.clone(),
                    modificationsData: modifications_data,
                };
                spawn_local(async move {
                    let res = make_query(RegisterComponentModificationsBulk::build_query(
                        register_component_modifications_bulk::Variables { ipt_multiple_modifications_data }
                    )).await.unwrap();
                    link.send_message(Msg::GetRegisterModificationsResult(res));
                })
            },
            Msg::GetRegisterModificationsResult(res) => {
                match resp_parsing::<Vec<UUID>>(res, "registerComponentModificationsBulk") {
                    Ok(result) => {
                        self.new_modifications_raw.clear();
                        self.stat_info = format!("{} {}", get_value_field(&213), result.len());
                        self.hide_import = true;
                        self.props.callback_finish_import.emit(());
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ShowImport => {
                self.stat_info.clear();
                self.hide_import = !self.hide_import;
            },
            Msg::UpdateData(value) => {
                self.new_modifications_raw = value;
                if self.new_modifications_raw.is_empty() {
                    self.stat_info.clear();
                    return true
                }
                let mut header_count = 1;
                let mut row_count = 0;
                let mut processing_head = true;
                for (_n, c) in self.new_modifications_raw.char_indices() {
                    if c == '\t' && processing_head {
                        header_count += 1;
                    }
                    if c == '\n' {
                        processing_head = false;
                        row_count += 1;
                    }
                }
                self.stat_info = format!("{}: {}, {}: {}", get_value_field(&345), header_count, get_value_field(&346), row_count);
            },
            Msg::Parsing => {
                debug!("Test data: {}", self.new_modifications_raw);
                let (headers, _) = parsing_text(&self.new_modifications_raw, true);
                self.paramnames.clear();
                self.value_column.clear();
                // parse the first line into parameter names
                for (number, h) in headers.into_iter().enumerate() {
                    // keyword research
                    match h {
                        // Some("[ModificationUuid]") => self.value_column.push(ModificationValueColumn::Uuid(number)),
                        Some("[ModificationName]") => self.value_column.push(ModificationValueColumn::Name(number)),
                        Some("[ModificationDescription]") => self.value_column.push(ModificationValueColumn::Description(number)),
                        Some("[ModificationActualStatusId]") => self.value_column.push(ModificationValueColumn::ActualStatusId(number)),
                        Some(value) => {
                            // todo!(обработка [ID:ЦИФРА])
                            self.paramnames.push(value.to_string());
                            continue;
                        },
                        None => (),
                    }
                    self.paramnames.push(String::new());
                }
                debug!("Complete self.paramnames: {:?}", self.paramnames);
                debug!("Complete self.value_column: {:?}", self.value_column);
                link.send_message(Msg::RequestRegisterParams);
            },
            Msg::PreparingImport => {
                // parsing and save to self.new_modifications (NewModificationsPreData)
                // let mut column_uuid = None;
                let mut column_name = None;
                let mut column_description = None;
                let mut column_actual_status_id = None;
                for column_data in &self.value_column {
                    match column_data {
                        // ModificationValueColumn::Uuid(number) => column_uuid = Some(*number),
                        ModificationValueColumn::Name(number) => column_name = Some(*number),
                        ModificationValueColumn::Description(number) => column_description = Some(*number),
                        ModificationValueColumn::ActualStatusId(number) => column_actual_status_id = Some(*number),
                    }
                }
                let (headers, values) = parsing_text(&self.new_modifications_raw, false);
                debug!("Preparing parameters, headers {:?}, values {:?}", headers, values);
                for row in values {
                    let mut new_modification = NewModificationsPreData::new();
                    let mut params = Vec::new();
                    for (number, value) in row.into_iter().enumerate() {
                        debug!("Preparing parameters, nuber {:?}, parameter {:?}", number, value);
                        if Some(number) == column_name {
                            new_modification.modification_name = value.map(|s| s.to_string()).unwrap_or_default();
                            continue;
                        }
                        if Some(number) == column_description {
                            new_modification.description = value.map(|s| s.to_string()).unwrap_or_default();
                            continue;
                        }
                        if Some(number) == column_actual_status_id {
                            new_modification.actual_status_id = value
                                .map(|s| s.parse::<usize>().unwrap_or(1))
                                .unwrap_or(1);
                            continue;
                        }
                        match (value, self.params_with_ids.get(&number)) {
                            (Some(v), Some((paramname, param_id))) => {
                                if param_id == &0 && v.is_empty() {
                                    debug!("Parameter name or value is empty");
                                    continue;
                                }
                                debug!("Add new param: paramname {:?}, param_id {:?}, value {:?}", paramname, param_id, v);
                                params.push(ParamValue{ param_id: *param_id, value: v.to_string() });
                            },
                            _ => debug!("Parameter name or value not found"),
                        }
                    }
                    new_modification.params = params;
                    // add a new modification with parameters for registration
                    self.new_modifications.push(new_modification);
                }
                link.send_message(Msg::RequestRegisterModifications);
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
                true => ft_import_btn("open-import-btn", onclick_show_import, get_value_field(&342), false, false),
                false => self.show_import_modal(),
            }}
        </>}
    }
}

impl ImportModificationsData {
    fn show_import_modal(&self) -> Html {
        let onclick_hide_modal = self.link.callback(|_| Msg::ShowImport);
        let oninput_data = self.link.callback(|ev: InputData| Msg::UpdateData(ev.value));
        let onclick_subbmit = self.link.callback(|_| Msg::Parsing);
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
                      <p class={"modal-card-title"}>{get_value_field(&342)}</p>
                      <button class={"delete"} aria-label="close" onclick={onclick_hide_modal.clone()} />
                    </header>
                    <section class={"modal-card-body"}>
                        <div class={"column"}>
                            <div class={"subtitle is-6"}>
                                {get_value_field(&234)}<br/>
                                {get_value_field(&343).to_markdown()}
                            </div>
                            <textarea
                                id={"update-description"}
                                class={"textarea"}
                                type={"text"}
                                placeholder={format!("{}\n{}", get_value_field(&208), get_value_field(&344))}
                                value={self.new_modifications_raw.clone()}
                                oninput={oninput_data}
                                />
                            <p class={"help"}>{self.stat_info.clone()}</p>
                        </div>
                        <div class={"column"}>
                            {ft_import_btn(
                                "import-modifications-btn",
                                onclick_subbmit,
                                get_value_field(&342),
                                true,
                                self.new_modifications_raw.is_empty()
                            )}
                        </div>
                      </section>
                  </div>
                </div>
              </div>
        }
    }
}
