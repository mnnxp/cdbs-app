use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, Spec, SpecPathInfo};
use crate::services::{get_value_field, resp_parsing, resp_parsing_item};
use crate::gqls::make_query;
use crate::gqls::relate::{GetSpecsPaths, get_specs_paths};
use crate::gqls::standard::{
    AddStandardSpecs, add_standard_specs,
    DeleteStandardSpecs, delete_standard_specs,
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_manage_btn: bool,
    pub active_info_btn: bool,
    pub standard_uuid: UUID,
    pub spec: Spec,
    pub is_added: bool,
    pub style_tag: Option<String>,
    pub added_spec: Option<Callback<usize>>,
    pub delete_spec: Option<Callback<usize>>,
}

pub struct SpecTagItem {
    error: Option<Error>,
    spec_data: Option<SpecPathInfo>,
    open_spec_info: bool,
    is_added: bool,
    get_result_delete: bool,
}

pub enum Msg {
    RequestSpecInfo,
    RequestDeleteSpec,
    RequestAddSpec,
    GetSpecInfoResult(String),
    GetAddedSpecResult(String),
    GetDeleteSpecResult(String),
    ClickSpecInfo,
    ClearError,
    ResponseError(Error),
    Ignore,
}

impl Component for SpecTagItem {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            spec_data: None,
            open_spec_info: false,
            is_added: ctx.props().is_added,
            get_result_delete: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::RequestSpecInfo => {
                let spec_id = ctx.props().spec.spec_id as i64;
                spawn_local(async move {
                    let arguments = get_specs_paths::IptSpecPathArg{
                        spec_ids: Some(vec![spec_id]),
                        split_char: None,
                        depth_level: None,
                        limit: None,
                        offset: None,
                    };
                    let res = make_query(GetSpecsPaths::build_query(get_specs_paths::Variables {
                        ipt_spec_path_arg: Some(arguments)
                    })).await.unwrap();
                    link.send_message(Msg::GetSpecInfoResult(res));
                })
            },
            Msg::RequestDeleteSpec => {
                let standard_uuid = ctx.props().standard_uuid.clone();
                let spec_id = ctx.props().spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_standard_specs_data = delete_standard_specs::IptStandardSpecsData{
                        standard_uuid,
                        spec_ids: vec![spec_id],
                    };
                    let res = make_query(DeleteStandardSpecs::build_query(delete_standard_specs::Variables {
                        ipt_standard_specs_data
                    })).await.unwrap();
                    link.send_message(Msg::GetDeleteSpecResult(res));
                })
            },
            Msg::RequestAddSpec => {
                let standard_uuid = ctx.props().standard_uuid.clone();
                let spec_id = ctx.props().spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_standard_specs_data = add_standard_specs::IptStandardSpecsData {
                        standard_uuid,
                        spec_ids: vec![spec_id],
                    };
                    let res =
                        make_query(AddStandardSpecs::build_query(add_standard_specs::Variables {
                            ipt_standard_specs_data,
                        }))
                        .await;
                    link.send_message(Msg::GetAddedSpecResult(res.unwrap()));
                })
            },
            Msg::GetSpecInfoResult(res) => {
                let result: Vec<SpecPathInfo> = resp_parsing(res, "specsPaths")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                if let Some(data) = result.first() {
                    self.spec_data = Some(data.clone());
                    self.open_spec_info = true;
                }
            },
            Msg::GetAddedSpecResult(res) => {
                let result: usize = resp_parsing_item(res, "addStandardSpecs")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                // self.get_result_delete = result > 0;
                match &ctx.props().added_spec {
                    Some(added_spec) => {
                        if result > 0 {
                            self.is_added = true;
                            self.get_result_delete = false;
                            added_spec.emit(ctx.props().spec.spec_id);
                        }
                    },
                    None => self.is_added = result > 0,
                }
            },
            Msg::GetDeleteSpecResult(res) => {
                let result: usize = resp_parsing_item(res, "deleteStandardSpecs")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                // self.get_result_delete = result > 0;
                match &ctx.props().delete_spec {
                    Some(delete_spec) => {
                        if result > 0 {
                            self.is_added = false;
                            self.get_result_delete = true;
                            delete_spec.emit(ctx.props().spec.spec_id);
                        }
                    },
                    None => self.get_result_delete = result > 0,
                }
            },
            Msg::ClickSpecInfo => {
                match self.spec_data {
                    Some(_) => self.open_spec_info = !self.open_spec_info,
                    None => link.send_message(Msg::RequestSpecInfo),
                }
            },
            Msg::ClearError => self.error = None,
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::Ignore => (),
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_clear_error = ctx.link().callback(|_| Msg::ClearError);
        match &self.error {
            Some(err) => html!{
                <ListErrors error={err.clone()} clear_error={Some(onclick_clear_error.clone())} />
            },
            None => match self.get_result_delete {
                true => html!{},
                false => html!{<>
                    {self.show_spec_info(ctx.link())}
                    {self.show_spec(ctx.link(), ctx.props())}
                </>},
            }
        }
    }
}

impl SpecTagItem {
    fn show_spec(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_open_spec_info = match props.active_info_btn {
            true => link.callback(|_| Msg::ClickSpecInfo),
            false => link.callback(|_| Msg::Ignore),
        };
        let onclick_delete_spec = link.callback(|_| Msg::RequestDeleteSpec);
        let onclick_add_spec = link.callback(|_| Msg::RequestAddSpec);
        let mut style_tag = match &props.style_tag {
            Some(style) => format!("tag is-light {}", style),
            None => "tag is-light".to_string(),
        };
        if props.active_info_btn {
            style_tag += " button";
        }

        html!{<div class="control">
          <div class="tags has-addons">
            <span class={style_tag} onclick={onclick_open_spec_info} >
                {props.spec.spec.clone()}
            </span>
            {if props.show_manage_btn {
                match &props.is_added {
                    true => html!{<a class="tag is-delete is-small is-light" onclick={onclick_delete_spec} />},
                    false => html!{<a class="tag is-small is-light is-success" onclick={onclick_add_spec}>
                        <i class="fas fa-plus" />
                    </a>},
                }
            } else {html!{}}}
          </div>
        </div>}
    }

    fn show_spec_info(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_spec_info = link.callback(|_| Msg::ClickSpecInfo);
        let class_modal = match &self.open_spec_info {
            true => "modal is-active",
            false => "modal",
        };

        match &self.spec_data {
            Some(data) => html!{
                <div class={class_modal}>
                  <div class="modal-background" onclick={onclick_spec_info.clone()} />
                  <div class="modal-content">
                      <div class="card column">
                        <table class="table is-fullwidth">
                          <tbody>
                            <tr>
                              <td>{ get_value_field(&246) }</td>
                              <td>{data.spec_id.to_string()}</td>
                            </tr>
                            <tr>
                              <td>{ get_value_field(&247) }</td>
                              <td>{data.path.clone()}</td>
                            </tr>
                          </tbody>
                        </table>
                      </div>
                  </div>
                  <button class="modal-close is-large" aria-label="close" onclick={onclick_spec_info} />
                </div>
            },
            None => html!{},
        }
    }
}
