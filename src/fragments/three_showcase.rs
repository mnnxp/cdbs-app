use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;
use log::debug;
use crate::fragments::list_errors::ListErrors;
use crate::services::{get_value_field, is_gltf_resource, preview_model, resp_parsing, KeyboardGuard, ModelFormat, ResourceMapping};
use crate::error::Error;
use crate::types::{DownloadFile, PaginateSet, UUID};
use crate::gqls::make_query;
use crate::gqls::component::{ComModFilesetFiles, com_mod_fileset_files};

// 1. Get the UUID of the file set
// 2. Request files from the file set (if the set is suitable)
// 3. Select the files and resources suitable for display
// 4. Activate the view button
// 5. Start viewing (when the button is clicked)

pub struct ThreeShowcase {
    props: Props,
    link: ComponentLink<Self>,
    error: Option<Error>,
    page_set: PaginateSet,
    full_screen: bool,
    file_arr: Vec<DownloadFile>,
    selected_file: Option<(DownloadFile, ModelFormat)>,
    suitable_files: Vec<(DownloadFile, ModelFormat)>,
    resource_files: Vec<DownloadFile>,
    viewer: Option<JsValue>,
    _key_guard: Option<KeyboardGuard>,
}

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub fileset_uuid: UUID,
    pub on_exit_fullscreen: Callback<()>,
}

#[derive(Clone)]
pub enum Msg {
    RequestDownloadFilesetFiles,
    ResponseError(Error),
    GetDownloadFilesetFilesResult(String),
    ViewerReady(JsValue),
    ChangeTypeShow,
    ShowThree,
    Close,
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
            page_set: PaginateSet::set(Some(1), Some(50)),
            full_screen: false,
            file_arr: Vec::new(),
            selected_file: None,
            suitable_files: Vec::new(),
            resource_files: Vec::new(),
            viewer: None,
            _key_guard: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestDownloadFilesetFiles);
            // hotkey listener
            let link = self.link.clone();
            self.setup_global_hotkeys(link);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::RequestDownloadFilesetFiles => {
                debug!("Select fileset: {:?}", self.props.fileset_uuid);
                if self.props.fileset_uuid.len() == 36 {
                    // set active loading button
                    let ipt_file_of_fileset_arg = com_mod_fileset_files::IptFileOfFilesetArg{
                        filesetUuid: self.props.fileset_uuid.clone(),
                        fileUuids: None,
                    };
                    let ipt_paginate = Some(com_mod_fileset_files::IptPaginate {
                        currentPage: self.page_set.current_page,
                        perPage: self.page_set.per_page,
                    });
                    spawn_local(async move {
                        let res = make_query(ComModFilesetFiles::build_query(com_mod_fileset_files::Variables {
                            ipt_file_of_fileset_arg,
                            ipt_paginate
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
                        self.suitable_files.clear();
                        self.resource_files.clear();
                        debug!("componentModificationFilesetFiles: {:?}", self.file_arr);
                        for file in self.file_arr.iter() {
                            let model_format = ModelFormat::from_filename(&file.filename);
                            if model_format.is_3d_format() {
                                self.suitable_files.push((file.clone(), model_format));
                            }
                        }
                        if let Some((file, model_format)) = self.suitable_files.first() {
                            self.selected_file = Some((file.clone(), *model_format));
                            debug!("Found {} files for show, selected: {:?}", self.suitable_files.len(), self.selected_file);
                            if matches!(model_format, ModelFormat::GLTF | ModelFormat::GLB) {
                                debug!("{:?} needs resources", model_format);
                                self.reassemble_resources();
                            }
                            debug!("Resource files: {:?}", self.resource_files);
                            link.send_message(Msg::ShowThree);
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ViewerReady(viewer_instance) => self.viewer = Some(viewer_instance),
            Msg::ChangeTypeShow => {
                self.destroy_viewer();
                self.full_screen = !self.full_screen;
                link.send_message(Msg::ShowThree);
            },
            Msg::ShowThree => {
                if let Some((df, model_format)) = &self.selected_file {
                    let df_clone = df.clone();
                    let format_clone = *model_format;
                    let size_clone = self.full_screen;
                    let resources: Vec<ResourceMapping> = self.resource_files
                        .iter()
                        .map(|rf| ResourceMapping {
                            filename: rf.filename.clone(),
                            download_url: rf.download_url.clone(),
                        })
                        .collect();
                    let link_clone = link.clone();
                    spawn_local(async move {
                        let viewer_instance_op = preview_model(&df_clone, format_clone, resources, size_clone).await;
                        if let Some(viewer_instance) = viewer_instance_op {
                            link_clone.send_message(Msg::ViewerReady(viewer_instance));
                        }
                    });
                }
            },
            Msg::Close => {
                self.destroy_viewer();
                self.full_screen = false;
                // hide 3D view to stop unnecessary rendering
                self.props.on_exit_fullscreen.emit(());
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
            self.destroy_viewer();
            self.props = props;
            self.file_arr.clear();
            self.full_screen = false;
            self.selected_file = None;
            self.suitable_files.clear();
            self.resource_files.clear();
            self.link.send_message(Msg::RequestDownloadFilesetFiles);
            debug!("change: {:?}", self.props.fileset_uuid);
            true
        }
    }

    fn destroy(&mut self) {
        debug!("Сleaning up viewer");
        self.destroy_viewer();
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        let onclick_full_screen = self.link.callback(|_| Msg::ChangeTypeShow);
        let mut container_style = "display: block; width: 100%; height: 100%; min-height: 25vh; overflow: hidden;";
        let mut b_container_style = "";
        let scene_hull_class = classes!("column", "main");
        let mut class_icon = classes!("fas");
        let mut class_modal = classes!("modal", "is-isolated-modal");
        if self.selected_file.is_none() {
            container_style = "padding-left: 0.75rem;";
        }
        let text_full_screen = match self.full_screen {
            true => {
                b_container_style = container_style;
                container_style = "padding-left: 0.75rem;";
                class_modal.push("is-active");
                class_icon.push("fa-compress-alt");
                get_value_field(&299)
            },
            false => {
                class_icon.push("fa-expand-alt");
                get_value_field(&298)
            },
        };

        html!{<>
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
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
                          <span class="help has-text-grey is-pulled-right mr-2 mt-2">{get_value_field(&436)}</span> // F: fullscreen | 1-5: views
                        </button>
                    },
                }}
                <a-container style={container_style}></a-container>
                <div class={class_modal}>
                    <div class="modal-background" onclick={onclick_full_screen.clone()}></div>
                    <div class="modal-content" style="width: 80vw; height: 80vh; min-height: 50vh;">
                        <b-container style={b_container_style}></b-container>
                    </div>
                    <button
                        id={"three-modal-close-btn"}
                        class={"button is-ghost modal-close"}
                        onclick={onclick_full_screen}
                        aria-label={text_full_screen} >
                      <span class="icon is-small">
                        <i class={"fas fa-compress-alt"} style="color: #1872f0;"></i>
                      </span>
                    </button>
                </div>
            </scene-hull>
        </>}
    }
}

impl ThreeShowcase {
    fn reassemble_resources(&mut self) {
        if let Some((select_df, select_mf)) = &self.selected_file {
            let base_name = select_df.filename.split('.').next().unwrap_or("");
            let needs_resources = matches!(select_mf, ModelFormat::GLTF | ModelFormat::GLB);
            if needs_resources {
                self.resource_files = self.file_arr
                    .iter()
                    .filter(|res|
                        base_name != res.filename &&
                        is_gltf_resource(&res.filename)
                    )
                    .cloned()
                    .collect();
            }
        }
    }

    fn setup_global_hotkeys(&mut self, link: ComponentLink<Self>) {
        let link_clone = link.clone();
        let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.code() == "KeyF" {
                e.prevent_default();
                link_clone.send_message(Msg::ChangeTypeShow);
            }
            if e.code() == "Escape" {
                link_clone.send_message(Msg::Close);
            }
        }) as Box<dyn FnMut(_)>);
        self._key_guard = Some(KeyboardGuard::new(closure));
    }

    fn destroy_viewer(&mut self) {
        if let Some(js_value) = self.viewer.take() {
            if let Ok(func_value) = js_sys::Reflect::get(&js_value, &JsValue::from_str("destroy")) {
                if let Ok(func) = func_value.dyn_into::<js_sys::Function>() {
                    let _ = js_sys::Reflect::apply(&func, &js_value, &js_sys::Array::new());
                    debug!("WebGL/IFC context released cleanly via duck typing");
                }
            }
        }
    }
}