use yew::{
    html, Callback, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, Spec, SpecPathInfo};
use crate::services::{get_value_field, resp_parsing};
use crate::gqls::{
    make_query,
    relate::{GetSpecsPaths, get_specs_paths},
    component::{
        AddComponentSpecs, add_component_specs,
        DeleteComponentSpecs, delete_component_specs,
    },
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_manage_btn: bool,
    pub active_info_btn: bool,
    pub component_uuid: UUID,
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
    spec_data: Option<SpecPathInfo>,
    open_spec_info: bool,
    is_added: bool,
    get_result_delete: bool,
}

pub enum Msg {
    RequestSpecInfo,
    RequestDeleteSpec,
    RequestAddSpec,
    ResponseError(Error),
    GetSpecInfoResult(String),
    GetAddedSpecResult(String),
    GetDeleteSpecResult(String),
    ClickSpecInfo,
    ClearError,
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
            spec_data: None,
            open_spec_info: false,
            is_added,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestSpecInfo => {
                let spec_id = self.props.spec.spec_id as i64;
                spawn_local(async move {
                    let arguments = get_specs_paths::IptSpecPathArg{
                        specIds: Some(vec![spec_id]),
                        splitChar: None,
                        depthLevel: None,
                    };
                    let res = make_query(GetSpecsPaths::build_query(get_specs_paths::Variables {
                        ipt_spec_path_arg: Some(arguments),
                    })).await.unwrap();
                    link.send_message(Msg::GetSpecInfoResult(res));
                })
            },
            Msg::RequestDeleteSpec => {
                let component_uuid = self.props.component_uuid.clone();
                let spec_id = self.props.spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_component_specs_data = delete_component_specs::IptComponentSpecsData{
                        componentUuid: component_uuid,
                        specIds: vec![spec_id],
                    };
                    let res = make_query(DeleteComponentSpecs::build_query(delete_component_specs::Variables {
                        ipt_component_specs_data
                    })).await.unwrap();
                    link.send_message(Msg::GetDeleteSpecResult(res));
                })
            },
            Msg::RequestAddSpec => {
                let component_uuid = self.props.component_uuid.clone();
                let spec_id = self.props.spec.spec_id as i64;
                spawn_local(async move {
                    let ipt_component_specs_data = add_component_specs::IptComponentSpecsData {
                        componentUuid: component_uuid,
                        specIds: vec![spec_id],
                    };
                    let res = make_query(AddComponentSpecs::build_query(add_component_specs::Variables {
                        ipt_component_specs_data
                    })).await.unwrap();
                    link.send_message(Msg::GetAddedSpecResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetSpecInfoResult(res) => {
                match resp_parsing::<Vec<SpecPathInfo>>(res, "specsPaths") {
                    Ok(result) => {
                        debug!("specsPaths: {:?}", result);
                        if let Some(data) = result.first() {
                            self.spec_data = Some(data.clone());
                            self.open_spec_info = true;
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetAddedSpecResult(res) => {
                match resp_parsing::<usize>(res, "addComponentSpecs") {
                    Ok(result) => {
                        debug!("addComponentSpecs: {:?}", result);
                        match &self.props.added_spec {
                            Some(added_spec) => {
                                if result > 0 {
                                    self.is_added = true;
                                    self.get_result_delete = false;
                                    added_spec.emit(self.props.spec.spec_id);
                                }
                            },
                            None => self.is_added = result > 0,
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::GetDeleteSpecResult(res) => {
                match resp_parsing::<usize>(res, "deleteComponentSpecs") {
                    Ok(result) => {
                        debug!("deleteComponentSpecs: {:?}", result);
                        self.get_result_delete = result > 0;
                        if self.get_result_delete {
                            if let Some(delete_spec) = &self.props.delete_spec {
                                delete_spec.emit(self.props.spec.spec_id);
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ClickSpecInfo => {
                match self.spec_data {
                    Some(_) => self.open_spec_info = !self.open_spec_info,
                    None => link.send_message(Msg::RequestSpecInfo),
                }
            },
            Msg::ClearError => self.error = None,
            Msg::Ignore => (),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        match &self.error {
            Some(err) => html!{
                <ListErrors error={err.clone()}
                    clear_error={onclick_clear_error.clone()}
                  />
            },
            None => match self.get_result_delete {
                true => html!{},
                false => html!{<>
                    {self.show_spec_info()}
                    {self.show_spec()}
                </>},
            }
        }
    }
}

impl SpecTagItem {
    fn show_spec(&self) -> Html {
        let onclick_open_spec_info = match self.props.active_info_btn {
            true => self.link.callback(|_| Msg::ClickSpecInfo),
            false => self.link.callback(|_| Msg::Ignore),
        };
        let onclick_delete_spec = self.link.callback(|_| Msg::RequestDeleteSpec);
        let onclick_add_spec = self.link.callback(|_| Msg::RequestAddSpec);

        let mut style_tag = match &self.props.style_tag {
            Some(style) => format!("tag is-light {}", style),
            None => "tag is-light".to_string(),
        };

        if self.props.active_info_btn {
            style_tag += " button";
        }

        html!{<div class="control">
          <div class="tags has-addons">
            <span class={style_tag} onclick={onclick_open_spec_info} >
                {self.props.spec.spec.clone()}
            </span>
            {if self.props.show_manage_btn {
                match &self.props.is_added {
                    true => html!{<a class="tag is-delete is-small is-light" onclick={onclick_delete_spec} />},
                    false => html!{<a class="tag is-small is-light is-success" onclick={onclick_add_spec}>
                        <i class="fas fa-plus" />
                    </a>},
                }
            } else {html!{}}}
          </div>
        </div>}
    }

    fn show_spec_info(&self) -> Html {
        let onclick_spec_info =
            self.link.callback(|_| Msg::ClickSpecInfo);

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
                              <td>{get_value_field(&246)}</td>
                              <td>{data.spec_id.to_string()}</td>
                            </tr>
                            <tr>
                              <td>{get_value_field(&247)}</td>
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
