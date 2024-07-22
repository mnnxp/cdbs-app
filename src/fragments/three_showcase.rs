use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender, classes};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::fragments::list_errors::ListErrors;
use crate::services::{three_detector, preview_model, get_value_field, resp_parsing};
use crate::error::Error;
use crate::types::{DownloadFile, UUID};
use crate::gqls::make_query;
use crate::gqls::component::{ComModFilesetFiles, com_mod_fileset_files};

// 1. Получаем UUID набора файлов
// 2. Запрашиваем файлы из набора файлов (если набор подходит)
// 3. Выбираем подходящий для отображения файл
// 4. Активируем кнопку просмотра
// 5. Запускаем просмотр (при нажатии кнопки)

pub struct ThreeShowcase {
    props: Props,
    link: ComponentLink<Self>,
    error: Option<Error>,
    selected_file: Option<DownloadFile>,
    full_screen: bool,
    file_arr: Vec<DownloadFile>,
}

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub fileset_uuid: UUID,
    // pub program_id: usize,
    // pub callback_three_view: Callback<bool>,
}

#[derive(Clone)]
pub enum Msg {
    RequestDownloadFilesetFiles,
    ResponseError(Error),
    GetDownloadFilesetFilesResult(String),
    ChangeTypeShow,
    ShowThree,
    ClearError,
}

impl Component for ThreeShowcase {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ThreeShowcase {
            props,
            link,
            error: None,
            selected_file: None,
            full_screen: false,
            file_arr: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestDownloadFilesetFiles);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFilesetFiles => {
                // if fileset without STL
                // if self.props.program_id != 38 {
                //     return false;
                // }
                debug!("Select fileset: {:?}", self.props.fileset_uuid);
                if self.props.fileset_uuid.len() == 36 {
                    // set active loading button
                    let ipt_file_of_fileset_arg = com_mod_fileset_files::IptFileOfFilesetArg{
                        filesetUuid: self.props.fileset_uuid.clone(),
                        fileUuids: None,
                        limit: None,
                        offset: None,
                    };
                    spawn_local(async move {
                        let res = make_query(ComModFilesetFiles::build_query(com_mod_fileset_files::Variables {
                            ipt_file_of_fileset_arg
                        })).await.unwrap();

                        link.send_message(Msg::GetDownloadFilesetFilesResult(res));
                    })
                }
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::GetDownloadFilesetFilesResult(res) => {
                match resp_parsing(res, "componentModificationFilesetFiles") {
                    Ok(result) => {
                        self.file_arr = result;
                        debug!("componentModificationFilesetFiles: {:?}", self.file_arr);
                        for file in self.file_arr.iter() {
                            if three_detector(&file.filename) {
                                self.selected_file = Some(file.clone());
                                debug!("Found file for show: {:?}", self.selected_file);
                                break;
                            }
                        }
                        if self.selected_file.is_some() {
                            link.send_message(Msg::ShowThree)
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ChangeTypeShow => {
                self.full_screen = !self.full_screen;
                link.send_message(Msg::ShowThree);
            },
            Msg::ShowThree => {
                let patch_to_model = self.selected_file.as_ref().map(|f| f.download_url.as_str()).unwrap_or_default();
                preview_model(patch_to_model, self.full_screen); // Starting 3D View
            },
            Msg::ClearError => self.error = None,
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.fileset_uuid == props.fileset_uuid {
            debug!("no change: {:?}", self.props.fileset_uuid);
            false
        } else {
            self.props = props;
            self.file_arr.clear();
            self.full_screen = false;
            self.selected_file = None;
            self.link.send_message(Msg::RequestDownloadFilesetFiles);
            debug!("change: {:?}", self.props.fileset_uuid);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_full_screen = self.link.callback(|_| Msg::ChangeTypeShow);
        let mut container_style = "display: block; width: 100%; height: 100%; min-height: 25vh";
        let mut b_container_style = "";
        // let mut scene_hull_class = classes!("column", "is-one-quarter");
        let scene_hull_class = classes!("column", "main");
        let mut class_icon = classes!("fas");
        let mut class_modal = classes!("modal");
        if self.selected_file.is_none() {
            container_style = "padding-left: 0.75rem;";
        }
        let text_full_screen = match self.full_screen {
            true => {
                // scene_hull_class.push("main");
                b_container_style = container_style;
                container_style = "padding-left: 0.75rem;";
                class_modal.push("is-active");
                class_icon.push("fa-compress-alt");
                get_value_field(&299)
            },
            false => {
                // scene_hull_class.push("is-one-quarter");
                class_icon.push("fa-expand-alt");
                get_value_field(&298)
            },
        };

        html!{<>
            <ListErrors error=self.error.clone() clear_error=onclick_clear_error.clone()/>
            <scene-hull class={scene_hull_class}>
                {match self.selected_file.is_none() {
                    true => html!{
                        <div class="text-center">
                            <span>{get_value_field(&297)}</span>
                        </div>
                    },
                    false => html!{
                        <button
                            id="three-size-button"
                            class={"button is-ghost"}
                            onclick={onclick_full_screen.clone()}
                            style={"position: absolute;"}
                            aria-label={text_full_screen} >
                          <span class="icon is-small">
                            <i class={class_icon} style="color: #1872f0;"></i>
                          </span>
                        //   <span>{text_full_screen}</span>
                        </button>
                    },
                }}
                // <PreviewModel/>
                <a-container style={container_style}></a-container>
                <div class={class_modal}>
                    <div class="modal-background" onclick={onclick_full_screen.clone()}></div>
                    <div class="modal-content" style="width: 90%; height: 90%; min-height: 50vh;">
                        <b-container style={b_container_style}></b-container>
                    </div>
                    // <button class="modal-close is-large" aria-label="close"></button>
                    <button
                        class={"button is-ghost modal-close"}
                        onclick={onclick_full_screen}
                        // style={"position: absolute;"}
                        aria-label={text_full_screen} >
                      <span class="icon is-small">
                        <i class={"fas fa-compress-alt"} style="color: #1872f0;"></i>
                      </span>
                    //   <span>{text_full_screen}</span>
                    </button>
                </div>
            </scene-hull>
        </>}
    }
}