use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::services::resp_parsing;
use crate::types::{UUID, LicenseInfo};
use crate::gqls::make_query;
use crate::gqls::component::{DeleteComponentLicense, delete_component_license};

/// License card for show data on component page
pub struct ComponentLicenseTag {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    open_license_info: bool,
    get_result_delete: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub license_data: LicenseInfo,
    pub delete_license: Option<Callback<usize>>,
}

#[derive(Clone)]
pub enum Msg {
    ShowLicenseCard,
    RequestDeleteLicense,
    ResponseError(Error),
    GetDeleteLicenseResult(String),
}

impl Component for ComponentLicenseTag {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ComponentLicenseTag {
            error: None,
            props,
            link,
            open_license_info: false,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::ShowLicenseCard => self.open_license_info = !self.open_license_info,
            Msg::RequestDeleteLicense => {
                let component_uuid = self.props.component_uuid.clone();
                let license_id = self.props.license_data.id as i64;
                spawn_local(async move {
                    let ipt_component_license_data = delete_component_license::IptComponentLicenseData{
                        componentUuid: component_uuid,
                        licenseId: license_id,
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
                match resp_parsing::<usize>(res, "deleteComponentLicense") {
                    Ok(result) => {
                        debug!("deleteComponentLicense: {:?}", result);
                        self.get_result_delete = result > 0;
                        if self.get_result_delete {
                            if let Some(delete_license) = &self.props.delete_license {
                                delete_license.emit(self.props.license_data.id);
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.license_data.id == props.license_data.id {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{<>
            <ListErrors error=self.error.clone()/>
            {match self.get_result_delete {
                true => html!{},
                false => self.show_license(),
            }}
        </>}
    }
}

impl ComponentLicenseTag {
    fn show_license(&self) -> Html {
        let onclick_license_data_info = self.link.callback(|_| Msg::ShowLicenseCard);
        let onclick_delete_license = self.link.callback(|_| Msg::RequestDeleteLicense);
        html!{<>
            // {self.show_modal_license_info()}
            {match self.props.show_delete_btn {
                true => html!{
                  <div class="tags has-addons" style="margin-left: 1rem" >
                    <span class="tag is-light" onclick={onclick_license_data_info}>
                      {self.props.license_data.keyword.clone()}
                    </span>
                    <a class="tag is-delete is-small is-light" onclick={onclick_delete_license} />
                  </div>
                },
                false => html!{
                  <span class="tag is-light" style="margin-left: 1rem" onclick={onclick_license_data_info} >
                    {self.props.license_data.keyword.clone()}
                  </span>
                },
            }}
        </>}
    }
    // fn show_modal_license_info(&self) -> Html {
    //     let onclick_license_data_info = self.link
    //         .callback(|_| Msg::ShowLicenseCard);
    //
    //     let class_modal = match &self.open_license_info {
    //         true => "modal is-active",
    //         false => "modal",
    //     };
    //
    //     html!{<div class=class_modal>
    //       <div class="modal-background" onclick=onclick_license_data_info.clone() />
    //       <div class="modal-content">
    //           <div class="card">
    //
    //           </div>
    //       </div>
    //       <button class="modal-close is-large" aria-label="close" onclick=onclick_license_data_info />
    //     </div>}
    // }
}
