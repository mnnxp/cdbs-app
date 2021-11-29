use yew::{
    html, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
use log::debug;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::error::{get_error, Error};
use crate::fragments::list_errors::ListErrors;
use crate::gqls::make_query;
use crate::types::ShowFileInfo;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
struct DeleteFile;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_delete_btn: bool,
    pub file: ShowFileInfo,
}

pub struct FileItem {
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
    get_result_delete: bool,
}

pub enum Msg {
    RequestDeleteFile,
    ResponseError(Error),
    GetDeleteFileResult(String),
}

impl Component for FileItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            error: None,
            props,
            link,
            get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDeleteFile => {
                let file_uuid = self.props.file.uuid.clone();
                spawn_local(async move {
                    let res = make_query(DeleteFile::build_query(
                        delete_file::Variables {
                            file_uuid,
                        }
                    )).await;
                    link.send_message(Msg::GetDeleteFileResult(res.unwrap()));
                })
            },
            Msg::ResponseError(err) => {
                self.error = Some(err);
            },
            Msg::GetDeleteFileResult(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res = data.as_object().unwrap().get("data").unwrap();

                match res.is_null() {
                    false => {
                        let result: bool = serde_json::from_value(res.get("deleteFile").unwrap().clone()).unwrap();
                        debug!("deleteFile: {:?}", result);
                        self.get_result_delete = result;
                    },
                    true => {
                        link.send_message(Msg::ResponseError(get_error(&data)));
                    }
                }
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {<>
            <ListErrors error=self.error.clone()/>
            {match self.get_result_delete {
                true => html! {},
                false => self.show_file(),
            }}
        </>}
    }
}

impl FileItem {
    fn show_file(&self) -> Html {
        let onclick_delete_file = self
            .link
            .callback(|_| Msg::RequestDeleteFile);

        match &self.props.show_delete_btn {
            true => html! {
                <div class="block">
                    <span class="icon">
                      <i class="fas fa-file"></i>
                    </span>
                    {self.props.file.filename.clone()}
                    <button class="delete is-small"
                        onclick=onclick_delete_file/>
                </div>
            },
            false => html! {
                <div class="block">
                    <span class="icon">
                      <i class="fas fa-file"></i>
                    </span>
                    {self.props.file.filename.clone()}
                </div>
            },
        }
    }
}
