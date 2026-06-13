use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_wasm_bindgen::to_value;
use log::debug;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::services::{Size, get_value_field, ext_str};
use crate::types::DownloadFile;

/// Array of file extensions used as resources
const GLTF_RESOURCE_EXTS: &[&str] = &[".bin", ".png", ".jpg", ".jpeg", ".webp", ".hdr", ".exr", ".ktx", ".ktx2", ".basis"];

thread_local! {
    static MODEL_CACHE: RefCell<HashMap<String, JsValue>> = RefCell::new(HashMap::new());
}

/// Checks if the filename has a resource extension for GLTF
pub(crate) fn is_gltf_resource(filename: &str) -> bool {
    let ext = ext_str(filename).to_lowercase();
    GLTF_RESOURCE_EXTS.contains(&ext.as_str())
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) enum ModelFormat {
    STL,
    GLTF,
    GLB,
    STEP,
    GCode,
    IFC,
    Unknown,
}

impl ModelFormat {
    pub(crate) fn from_filename(filename: &str) -> Self {
        match ext_str(filename).to_lowercase().as_str() {
            ".stl" => Self::STL,
            ".gltf" => Self::GLTF,
            ".glb" => Self::GLB,
            ".step" | ".stp" => Self::STEP,
            ".gcode" => Self::GCode,
            ".ifc" => Self::IFC,
            _ => Self::Unknown,
        }
    }

    pub(crate) fn is_3d_format(&self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ResourceMapping {
    pub(crate) filename: String,
    pub(crate) download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ViewerConfig {
    model: ShowModel,
    model_format: ModelFormat,
    resource_mapping: Vec<ResourceMapping>,
    size_flag: bool,
    labels: ViewerLabels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShowModel {
    filename: String,
    url: String,
    size: String,
    content_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ViewerLabels {
    controls: String,
    material_folder: String,
    lighting_folder: String,
    model_info_folder: String,
    axes: String,
    rotation: String,
    wireframe: String,
    original_textures: String,
    model_color: String,
    background_color: String,
    model_scale: String,
    failed_to_load_model: String,
    format_not_supported: String,
    view: String,
    active_layer: String,
    hide_travel_moves: String,
    display: String,
    play: String,
    speed: String,
    select: String,
    metalness: String,
    roughness: String,
    env_intensity: String,
    clearcoat: String,
    clearcoat_rough: String,
    ambient: String,
    hemisphere: String,
    key_light: String,
    fill_light: String,
    rim_light: String,
    key_light_position: String,
    fill_light_position: String,
    rim_light_position: String,
    intensities: String,
    positions: String,
    directional: String,
    light: String,
    file: String,
    size: String,
    hide_textures: String,
    display_all: String,
    display_up_to_current: String,
    display_current_only: String,
    view_perspective: String,
    view_top: String,
    view_bottom: String,
    view_front: String,
    view_back: String,
    view_left: String,
    view_right: String,
    view_isometric: String,
}

#[wasm_bindgen(module = "/assets/js/greatviewer.js")]
extern "C" {
    type GreatViewer;

    #[wasm_bindgen(constructor)]
    fn new(config: JsValue) -> GreatViewer;

    #[wasm_bindgen(method)]
    async fn starter(this: &GreatViewer);

    #[wasm_bindgen(method)]
    fn destroy(this: &GreatViewer);
}

#[wasm_bindgen(module = "/assets/js/greatviewer-ifc.js")]
extern "C" {
    type GreatViewerIFC;

    #[wasm_bindgen(constructor)]
    fn new(config: JsValue) -> GreatViewerIFC;

    #[wasm_bindgen(method)]
    async fn starter(this: &GreatViewerIFC);

    #[wasm_bindgen(method)]
    fn destroy(this: &GreatViewerIFC);
}

pub(crate) async fn preview_model(
    model_file: &DownloadFile,
    model_format: ModelFormat,
    resource_mapping: Vec<ResourceMapping>,
    size_flag: bool
) -> Option<JsValue> {
    debug!("viewer");
    let cache_key = format!("{}_{}_{}", model_file.download_url, size_flag, model_format as u8);
    let cached_config = MODEL_CACHE.with(|cache| {
        cache.borrow().get(&cache_key).cloned()
    });
    if let Some(config_js) = cached_config {
        spawn_local(async move {
            match model_format {
                ModelFormat::IFC => {
                    let viewer = GreatViewerIFC::new(config_js);
                    viewer.starter().await;
                },
                _ => {
                    let viewer = GreatViewer::new(config_js);
                    viewer.starter().await;
                },
            };
        });
        return None;
    }

    let Some(config_js) = get_js_value(model_file, model_format, resource_mapping, size_flag) else {
        debug!("Failed to create viewer config");
        return None;
    };

    MODEL_CACHE.with(|cache| {
        cache.borrow_mut().insert(cache_key, config_js.clone());
    });

    match model_format {
        ModelFormat::IFC => {
            let viewer = GreatViewerIFC::new(config_js);
            viewer.starter().await;
            Some(viewer.into())
        },
        _ => {
            let viewer = GreatViewer::new(config_js);
            viewer.starter().await;
            Some(viewer.into())
        },
    }
}

fn get_js_value(
    model_file: &DownloadFile,
    model_format: ModelFormat,
    resource_mapping: Vec<ResourceMapping>,
    size_flag: bool
) -> Option<JsValue> {
    let config = ViewerConfig {
        model: ShowModel {
            filename: model_file.filename.clone(),
            url: model_file.download_url.clone(),
            size: model_file.show_size(),
            content_length: model_file.filesize,
        },
        model_format,
        resource_mapping,
        size_flag,
        labels: ViewerLabels {
            controls: get_value_field(&252).to_string(),
            material_folder: get_value_field(&253).to_string(),
            lighting_folder: get_value_field(&255).to_string(),
            model_info_folder: get_value_field(&260).to_string(),
            axes: get_value_field(&302).to_string(),
            rotation: get_value_field(&303).to_string(),
            wireframe: get_value_field(&304).to_string(),
            original_textures: get_value_field(&250).to_string(),
            model_color: get_value_field(&305).to_string(),
            background_color: get_value_field(&306).to_string(),
            model_scale: get_value_field(&307).to_string(),
            failed_to_load_model: get_value_field(&248).to_string(),
            format_not_supported: get_value_field(&249).to_string(),
            view: get_value_field(&419).to_string(),
            active_layer: get_value_field(&420).to_string(),
            hide_travel_moves: get_value_field(&421).to_string(),
            display: get_value_field(&422).to_string(),
            play: get_value_field(&423).to_string(),
            speed: get_value_field(&424).to_string(),
            select: get_value_field(&425).to_string(),
            metalness: get_value_field(&426).to_string(),
            roughness: get_value_field(&427).to_string(),
            env_intensity: get_value_field(&428).to_string(),
            clearcoat: get_value_field(&429).to_string(),
            clearcoat_rough: get_value_field(&430).to_string(),
            ambient: get_value_field(&431).to_string(),
            hemisphere: get_value_field(&502).to_string(),
            key_light: get_value_field(&503).to_string(),
            fill_light: get_value_field(&504).to_string(),
            rim_light: get_value_field(&505).to_string(),
            key_light_position: get_value_field(&506).to_string(),
            fill_light_position: get_value_field(&507).to_string(),
            rim_light_position: get_value_field(&508).to_string(),
            intensities: get_value_field(&509).to_string(),
            positions: get_value_field(&510).to_string(),
            directional: get_value_field(&432).to_string(),
            light: get_value_field(&433).to_string(),
            file: get_value_field(&434).to_string(),
            size: get_value_field(&435).to_string(),
            hide_textures: get_value_field(&437).to_string(),
            display_all: get_value_field(&438).to_string(),
            display_up_to_current: get_value_field(&439).to_string(),
            display_current_only: get_value_field(&440).to_string(),
            view_perspective: get_value_field(&441).to_string(),
            view_top: get_value_field(&442).to_string(),
            view_bottom: get_value_field(&443).to_string(),
            view_front: get_value_field(&444).to_string(),
            view_back: get_value_field(&445).to_string(),
            view_left: get_value_field(&446).to_string(),
            view_right: get_value_field(&447).to_string(),
            view_isometric: get_value_field(&448).to_string(),
        },
    };
    to_value(&config).map(|v| Some(v)).unwrap_or_default()
}