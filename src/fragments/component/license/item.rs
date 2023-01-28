use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, LicenseInfo};
use crate::gqls::make_query;
use crate::gqls::component::{DeleteComponentLicense, delete_component_license};
use crate::services::resp_parsing_item;

/// License card for show data on component page
pub struct ComponentLicenseTag {
    error: Option<Error>,
    open_license_info: bool,
    get_result_delete: bool,
    license_data_id: usize,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub license_data: LicenseInfo,
    #[prop_or_default]
    pub delete_license: Option<Callback<usize>>,
}

#[derive(Clone)]
pub enum Msg {
    ShowLicenseCard,
    RequestDeleteLicense,
    ResponseError(Error),
    GetDeleteLicenseResult(String),
    Ignore,
}

impl Component for ComponentLicenseTag {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ComponentLicenseTag {
            error: None,
            open_license_info: false,
            get_result_delete: false,
            license_data_id: ctx.props().license_data.id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::ShowLicenseCard => self.open_license_info = !self.open_license_info,
            Msg::RequestDeleteLicense => {
                let component_uuid = ctx.props().component_uuid.clone();
                let license_id = ctx.props().license_data.id as i64;
                spawn_local(async move {
                    let ipt_component_license_data = delete_component_license::IptComponentLicenseData{
                        component_uuid,
                        license_id,
                    };
                    let res = make_query(DeleteComponentLicense::build_query(
                        delete_component_license::Variables {
                            ipt_component_license_data,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::GetDeleteLicenseResult(res));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDeleteLicenseResult(res) => {
                let result: usize = resp_parsing_item(res, "deleteComponentLicense")
                    .map_err(|err| link.send_message(Msg::ResponseError(err)))
                    .unwrap();
                match &ctx.props().delete_license {
                    Some(delete_license) => {
                        if result > 0 {
                            self.get_result_delete = true;
                            delete_license.emit(ctx.props().license_data.id);
                        }
                    },
                    None => self.get_result_delete = result > 0,
                }
            },
            Msg::Ignore => {}
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.license_data_id == ctx.props().license_data.id {
            false
        } else {
            self.license_data_id = ctx.props().license_data.id;
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<>
            <ListErrors error={self.error.clone()}/>
            {match self.get_result_delete {
                true => html!{},
                false => self.show_license(ctx.link(), ctx.props()),
            }}
        </>}
    }
}

impl ComponentLicenseTag {
    fn show_license(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_license_data_info = link.callback(|_| Msg::ShowLicenseCard);
        let onclick_delete_license = link.callback(|_| Msg::RequestDeleteLicense);

        html!{<>
            // {self.show_modal_license_info()}
            {match props.show_delete_btn {
                true => html!{<div class="tags has-addons" style="margin-left: 1rem" >
                  <span class="tag is-light" onclick={onclick_license_data_info}>
                    {props.license_data.keyword.clone()}
                  </span>
                  <a class="tag is-delete is-small is-light" onclick={onclick_delete_license} />
                </div>},
                false => html!{<span class="tag is-light"
                    style="margin-left: 1rem"
                    onclick={onclick_license_data_info} >
                  {props.license_data.keyword.clone()}
                </span>},
            }}
        </>}
    }
    // fn show_modal_license_info(
    //     &self,
    //     link: &Scope<Self>,=
    // ) -> Html {
    //     let onclick_license_data_info = link.callback(|_| Msg::ShowLicenseCard);
    //
    //     let class_modal = match &self.open_license_info {
    //         true => "modal is-active",
    //         false => "modal",
    //     };
    //
    //     html!{<div class={class_modal}>
    //       <div class="modal-background" onclick={onclick_license_data_info.clone()} />
    //       <div class="modal-content">
    //           <div class="card">
    //
    //           </div>
    //       </div>
    //       <button class="modal-close is-large" aria-label="close" onclick={onclick_license_data_info} />
    //     </div>}
    // }
}
