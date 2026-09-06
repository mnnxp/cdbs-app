use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_wasm_bindgen::to_value;
use log::debug;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::services::{Size, LocaleKey, ext_str};
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
            controls: LocaleKey::Controls.get_value().to_string(),
            material_folder: LocaleKey::Material.get_value().to_string(),
            lighting_folder: LocaleKey::Lighting.get_value().to_string(),
            model_info_folder: LocaleKey::ModelInfo.get_value().to_string(),
            axes: LocaleKey::CoordinateAxes.get_value().to_string(),
            rotation: LocaleKey::Rotation.get_value().to_string(),
            wireframe: LocaleKey::Frame.get_value().to_string(),
            original_textures: LocaleKey::OriginalTextures.get_value().to_string(),
            model_color: LocaleKey::ModelColor.get_value().to_string(),
            background_color: LocaleKey::BackgroundColor.get_value().to_string(),
            model_scale: LocaleKey::ModelScale.get_value().to_string(),
            failed_to_load_model: LocaleKey::ErrorLoadingModel.get_value().to_string(),
            format_not_supported: LocaleKey::UnsupportedFormat.get_value().to_string(),
            view: LocaleKey::ViewLabel.get_value().to_string(),
            active_layer: LocaleKey::ActiveLayer.get_value().to_string(),
            hide_travel_moves: LocaleKey::HideTravelMoves.get_value().to_string(),
            display: LocaleKey::Display.get_value().to_string(),
            play: LocaleKey::Play.get_value().to_string(),
            speed: LocaleKey::Speed.get_value().to_string(),
            select: LocaleKey::SelectLabel.get_value().to_string(),
            metalness: LocaleKey::Metalness.get_value().to_string(),
            roughness: LocaleKey::Roughness.get_value().to_string(),
            env_intensity: LocaleKey::EnvIntensity.get_value().to_string(),
            clearcoat: LocaleKey::Clearcoat.get_value().to_string(),
            clearcoat_rough: LocaleKey::ClearcoatRough.get_value().to_string(),
            ambient: LocaleKey::Ambient.get_value().to_string(),
            hemisphere: LocaleKey::Hemisphere.get_value().to_string(),
            key_light: LocaleKey::KeyLight.get_value().to_string(),
            fill_light: LocaleKey::FillLight.get_value().to_string(),
            rim_light: LocaleKey::RimLight.get_value().to_string(),
            key_light_position: LocaleKey::KeyLightPosition.get_value().to_string(),
            fill_light_position: LocaleKey::FillLightPosition.get_value().to_string(),
            rim_light_position: LocaleKey::RimLightPosition.get_value().to_string(),
            intensities: LocaleKey::Intensities.get_value().to_string(),
            positions: LocaleKey::Positions.get_value().to_string(),
            directional: LocaleKey::Directional.get_value().to_string(),
            light: LocaleKey::Light.get_value().to_string(),
            file: LocaleKey::File.get_value().to_string(),
            size: LocaleKey::SizeLabel.get_value().to_string(),
            hide_textures: LocaleKey::HideTextures.get_value().to_string(),
            display_all: LocaleKey::All.get_value().to_string(),
            display_up_to_current: LocaleKey::UpToCurrent.get_value().to_string(),
            display_current_only: LocaleKey::CurrentOnly.get_value().to_string(),
            view_perspective: LocaleKey::Perspective3D.get_value().to_string(),
            view_top: LocaleKey::TopXY.get_value().to_string(),
            view_bottom: LocaleKey::Bottom.get_value().to_string(),
            view_front: LocaleKey::FrontXZ.get_value().to_string(),
            view_back: LocaleKey::BackView.get_value().to_string(),
            view_left: LocaleKey::LeftYZ.get_value().to_string(),
            view_right: LocaleKey::Right.get_value().to_string(),
            view_isometric: LocaleKey::Isometric.get_value().to_string(),
        },
    };
    to_value(&config).map(|v| Some(v)).unwrap_or_default()
}