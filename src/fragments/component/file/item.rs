use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;

use crate::error::Error;
use crate::fragments::switch_icon::res_file_btn;
use crate::fragments::file::FileShowcase;
use crate::fragments::list_errors::ListErrors;
use crate::types::{UUID, ShowFileInfo};
use crate::services::resp_parsing;
use crate::gqls::make_query;
use crate::gqls::component::{DeleteComponentFile, delete_component_file};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub file: ShowFileInfo,
    pub callback_delete_file: Option<Callback<UUID>>,
}

pub struct ComponentFileItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    open_full_info_file: bool,
    get_result_delete: bool,
}

pub enum Msg {
    RequestDeleteFile(UUID),
    ResponseError(Error),
    GetDeleteFileResult(String, UUID),
    ClickFileInfo,
    ClearError,
}

impl Component for ComponentFileItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            link,
            open_full_info_file: false,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDeleteFile(file_uuid) => {
                let component_uuid = self.props.component_uuid.clone();
                // let file_uuid = self.props.file.uuid.clone();
                spawn_local(async move {
                    let delete_component_file_data = delete_component_file::DelComponentFileData{
                        fileUuid: file_uuid.clone(),
                        componentUuid: component_uuid,
                    };
                    let res = make_query(DeleteComponentFile::build_query(delete_component_file::Variables {
                        delete_component_file_data
                    })).await.unwrap();
                    link.send_message(Msg::GetDeleteFileResult(res, file_uuid));
                })
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDeleteFileResult(res, file_uuid) => {
                match resp_parsing(res, "deleteComponentFile") {
                    Ok(result) => {
                        if result && &file_uuid == &self.props.file.uuid {
                            self.get_result_delete = result;
                            if let Some(rollback) = &self.props.callback_delete_file {
                                rollback.emit(self.props.file.uuid.clone());
                            }
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ClickFileInfo => self.open_full_info_file = !self.open_full_info_file,
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.file.uuid == props.file.uuid {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_file_info = self.link.callback(|_| Msg::ClickFileInfo);
        let onclick_delete_btn =
            self.link.callback(|delete_file_uuid| Msg::RequestDeleteFile(delete_file_uuid));
        let onclick_delete_btn = match self.props.show_delete_btn {
            true => Some(onclick_delete_btn),
            false => None,
        };

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
            {match self.get_result_delete {
                true => html!{},
                false => html!{<>
                    <FileShowcase
                        file_info={self.props.file.clone()}
                        file_info_callback={onclick_file_info}
                        file_delete_callback={onclick_delete_btn}
                        open_modal_frame={self.open_full_info_file}
                        show_revisions={self.props.show_delete_btn}
                        />
                    {self.show_file()}
                </>},
            }}
        </>}
    }
}

impl ComponentFileItem {
    fn show_file(&self) -> Html {
        let onclick_file_info = self.link.callback(|_| Msg::ClickFileInfo);
        res_file_btn(onclick_file_info.clone(), self.props.file.filename.clone())
    }
}
