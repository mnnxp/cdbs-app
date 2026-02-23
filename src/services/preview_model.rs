// use serde_json::json;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
// use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::to_value;
use log::debug;
use crate::services::{Size, get_value_field, ext_str};
use crate::types::DownloadFile;

/// Array of file extensions used as resources in GLTF
const GLTF_RESOURCE_EXTS: &[&str] = &[".bin", ".png", ".jpg", ".jpeg",  ".webp", ".hdr", ".exr", ".ktx", ".ktx2", ".basis"];

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
            ".gcode" => Self::GCode,
            ".ifc" => Self::IFC,
            _ => Self::Unknown,
        }
    }

    pub(crate) fn is_3d_format(&self) -> bool {
        match self {
            // Self::STL | Self::GLTF | Self::GLB | Self::GCode => true,
            Self::Unknown => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ViewerConfig {
    pub(crate) model: ShowModel,
    pub(crate) model_format: ModelFormat,
    pub(crate) resource_mapping: Vec<ResourceMapping>,
    pub(crate) size_flag: bool,
    pub(crate) labels: ViewerLabels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ShowModel {
    pub(crate) filename: String,
    pub(crate) url: String,
    pub(crate) size: String,
    pub(crate) content_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ResourceMapping {
    pub(crate) filename: String,
    pub(crate) download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ViewerLabels {
    pub(crate) controls: String,
    pub(crate) material_folder: String,
    pub(crate) lighting_folder: String,
    pub(crate) model_info_folder: String,
    pub(crate) axes: String,
    pub(crate) rotation: String,
    pub(crate) wireframe: String,
    pub(crate) original_textures: String,
    pub(crate) model_color: String,
    pub(crate) background_color: String,
    pub(crate) model_scale: String,
    pub(crate) failed_to_load_model: String,
    pub(crate) format_not_supported: String,
    pub(crate) view: String,
    pub(crate) active_layer: String,
    pub(crate) hide_travel_moves: String,
    pub(crate) display: String,
    pub(crate) play: String,
    pub(crate) speed: String,
    pub(crate) select: String,
    pub(crate) metalness: String,
    pub(crate) roughness: String,
    pub(crate) env_intensity: String,
    pub(crate) clearcoat: String,
    pub(crate) clearcoat_rough: String,
    pub(crate) ambient: String,
    pub(crate) directional: String,
    pub(crate) light: String,
    pub(crate) file: String,
    pub(crate) size: String,
    pub(crate) hide_textures: String,
    pub(crate) display_all: String,
    pub(crate) display_up_to_current: String,
    pub(crate) display_current_only: String,
    pub(crate) view_perspective: String,
    pub(crate) view_top: String,
    pub(crate) view_bottom: String,
    pub(crate) view_front: String,
    pub(crate) view_back: String,
    pub(crate) view_left: String,
    pub(crate) view_right: String,
    pub(crate) view_isometric: String,
}

#[wasm_bindgen(module = "/assets/js/greatviewer.js")]
extern "C" {
    type GreatViewer;

    #[wasm_bindgen(constructor)]
    fn new(config: JsValue) -> GreatViewer;

    #[wasm_bindgen(method)]
    async fn starter(this: &GreatViewer);
}

#[wasm_bindgen(module = "/assets/js/greatviewer-ifc.js")]
extern "C" {
    type GreatViewerIFC;

    #[wasm_bindgen(constructor)]
    fn new(config: JsValue) -> GreatViewerIFC;

    #[wasm_bindgen(method)]
    async fn starter(this: &GreatViewerIFC);
}

#[wasm_bindgen]
pub enum JsModelFormat {
    STL,
    GLTF,
    GLB,
    GCode,
    IFC,
    Unknown,
}

impl From<ModelFormat> for JsModelFormat {
    fn from(model_format: ModelFormat) -> Self {
        match model_format {
            ModelFormat::STL => Self::STL,
            ModelFormat::GLTF => Self::GLTF,
            ModelFormat::GLB => Self::GLB,
            ModelFormat::GCode => Self::GCode,
            ModelFormat::IFC => Self::IFC,
            ModelFormat::Unknown => Self::Unknown,
        }
    }
}

pub(crate) fn preview_model(
    model_file: &DownloadFile,
    model_format: ModelFormat,
    resource_mapping: Vec<ResourceMapping>,
    // suitable_files: Vec<(DownloadFile, ModelFormat)>,
    size_flag: bool
) {
    debug!("viewer");
    let Some(config_js) = get_js_value(model_file, model_format, resource_mapping, size_flag) else {
        debug!("Failed to create viewer config");
        return
    };
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