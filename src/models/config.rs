use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;

use crate::common::error::CommonError;

/// App config from YAML:
/// ```yaml
/// figma:
///     fileId: "..."
///     pageName: "..."
/// common:
///     images:
///         figmaFrameName: Images
/// android:
///     mainRes: "./main/res"
///     images:
///         scales:
///             mdpi: 1.0
///             hdpi: 1.5
///             xhdpi: 2.0
///             xxhdpi: 3.0
///         format: svg | png | webp
///         webpOptions:
///             quality: 0..100
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub figma: FigmaConfig,
    pub common: CommonConfig,
    pub android: AndroidConfig,
}

/// Part of App config from YAML:
/// ```yaml
/// figma:
///     fileId: "..."
///     pageName: "..."
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaConfig {
    pub file_id: String,
    pub page_name: Option<String>,
}

/// Part of App config from YAML:
/// ```yaml
/// common:
///     images:
///         figmaFrameName: Images
/// ```
#[derive(Debug, Deserialize)]
pub struct CommonConfig {
    pub images: CommonImagesConfig,
}

/// Part of App config from YAML:
/// ```yaml
/// images:
///     figmaFrameName: Images
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonImagesConfig {
    pub figma_frame_name: String,
}

/// Part of App config from YAML:
/// ```yaml
/// android:
///     mainRes: "./main/res"
///     images:
///         scales:
///             mdpi: 1.0
///             hdpi: 1.5
///             xhdpi: 2.0
///             xxhdpi: 3.0
///         format: png | webp
///         webpOptions:
///             quality: 0..100
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidConfig {
    pub main_res: String,
    #[serde(default = "default_android_images_config")]
    pub images: AndroidImagesConfig,
}

fn default_android_images_config() -> AndroidImagesConfig {
    AndroidImagesConfig {
        scales: default_scales(),
        format: default_format(),
        webp_options: default_webp_options(),
    }
}

/// Part of App config from YAML:
/// ```yaml
/// images:
///     scales:
///         mdpi: 1.0
///         hdpi: 1.5
///         xhdpi: 2.0
///         xxhdpi: 3.0
///     format: png | webp
///     webpOptions:
///         quality: 0..100
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidImagesConfig {
    #[serde(default = "default_scales")]
    pub scales: HashMap<String, f32>,
    #[serde(default = "default_format")]
    pub format: ImageFormat,
    #[serde(default = "default_webp_options")]
    pub webp_options: AndroidImagesWebpConfig,
}

fn default_scales() -> HashMap<String, f32> {
    [
        ("mdpi", 1.0f32),
        ("hdpi", 1.5f32),
        ("xhdpi", 2.0f32),
        ("xxhdpi", 3.0f32),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

fn default_format() -> ImageFormat {
    ImageFormat::Webp
}

fn default_webp_options() -> AndroidImagesWebpConfig {
    AndroidImagesWebpConfig { quality: 85f32 }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImageFormat {
    Webp,
    Png,
    Svg,
}

impl ImageFormat {
    pub fn extension(&self) -> String {
        match &self {
            ImageFormat::Png => "png".to_string(),
            ImageFormat::Svg => "svg".to_string(),
            ImageFormat::Webp => "webp".to_string(),
        }
    }
}

/// Part of App config from YAML:
/// ```yaml
/// webpOptions:
///     quality: 0..100
/// ```
#[derive(Debug, Deserialize)]
pub struct AndroidImagesWebpConfig {
    pub quality: f32,
}

impl AppConfig {
    pub fn from_file(yaml_config_path: &String) -> Result<Self, CommonError> {
        match File::open(yaml_config_path) {
            Ok(file) => match serde_yaml::from_reader(&file) {
                Ok(app_config) => Ok(app_config),
                Err(e) => {
                    let message = format!("while parsing config file {}", yaml_config_path);
                    let cause = Some(format!("{}", e));
                    Err(CommonError { message, cause })
                }
            },
            Err(e) => {
                let message = format!("while opening config file {}", yaml_config_path);
                let cause = Some(format!("{}", e));
                Err(CommonError { message, cause })
            }
        }
    }
}
