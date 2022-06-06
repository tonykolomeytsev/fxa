use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;

use crate::common::error::AppError;

/// App config from YAML:
/// ```yaml
/// figma:
///     fileId: "..."
///     pageName: "..."
/// common:
///     images:
///         figmaFrameName: Images
///     icons:
///         figmaFrameName: Icons
/// android:
///     mainRes: "./main/res"
///     images:
///         mainRes: "./main/res"
///         scales:
///             mdpi: 1.0
///             hdpi: 1.5
///             xhdpi: 2.0
///             xxhdpi: 3.0
///         format: svg | png | webp
///         webpOptions:
///             quality: 0..100
///     icons:
///         mainRes: "./main/res"
///         format: svg | xml
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub figma: FigmaConfig,
    #[serde(default = "default_common_config")]
    pub common: CommonConfig,
    pub android: AndroidConfig,
}

fn default_common_config() -> CommonConfig {
    CommonConfig {
        images: default_common_images_config(),
        icons: default_common_icons_config(),
    }
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
///     icons:
///         figmaFrameName: Icons
/// ```
#[derive(Debug, Deserialize)]
pub struct CommonConfig {
    #[serde(default = "default_common_images_config")]
    pub images: CommonImagesConfig,
    #[serde(default = "default_common_icons_config")]
    pub icons: CommonIconsConfig,
}

fn default_common_images_config() -> CommonImagesConfig {
    CommonImagesConfig {
        figma_frame_name: "Images".to_string(),
    }
}

fn default_common_icons_config() -> CommonIconsConfig {
    CommonIconsConfig {
        figma_frame_name: "Icons".to_string(),
    }
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
/// images:
///     figmaFrameName: Images
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonIconsConfig {
    pub figma_frame_name: String,
}

/// Part of App config from YAML:
/// ```yaml
/// android:
///     mainRes: "./main/res"
///     images:
///         mainRes: "./main/res"
///         scales:
///             mdpi: 1.0
///             hdpi: 1.5
///             xhdpi: 2.0
///             xxhdpi: 3.0
///         format: svg | png | webp
///         webpOptions:
///             quality: 0..100
///     icons:
///         mainRes: "./main/res"
///         format: svg | xml
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidConfig {
    pub main_res: Option<String>,
    #[serde(default = "default_android_images_config")]
    pub images: AndroidImagesConfig,
    #[serde(default = "default_android_icons_config")]
    pub icons: AndroidIconsConfig,
}

fn default_android_images_config() -> AndroidImagesConfig {
    AndroidImagesConfig {
        main_res: None,
        scales: default_scales(),
        format: default_image_format(),
        webp_options: default_webp_options(),
    }
}

fn default_android_icons_config() -> AndroidIconsConfig {
    AndroidIconsConfig {
        main_res: None,
        format: IconFormat::Xml,
    }
}

/// Part of App config from YAML:
/// ```yaml
/// images:
///     mainRes: "./main/res"
///     scales:
///         mdpi: 1.0
///         hdpi: 1.5
///         xhdpi: 2.0
///         xxhdpi: 3.0
///     format: svg | png | webp
///     webpOptions:
///         quality: 0..100
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidImagesConfig {
    pub main_res: Option<String>,
    #[serde(default = "default_scales")]
    pub scales: HashMap<String, f32>,
    #[serde(default = "default_image_format")]
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

fn default_image_format() -> ImageFormat {
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

    pub fn is_svg(&self) -> bool {
        match &self {
            ImageFormat::Svg => true,
            _ => false,
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

/// Part of App config from YAML:
/// ```yaml
/// icons:
///     mainRes: "./main/res"
///     format: svg | xml
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidIconsConfig {
    pub main_res: Option<String>,
    #[serde(default = "default_icons_format")]
    pub format: IconFormat,
}

fn default_icons_format() -> IconFormat {
    IconFormat::Xml
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum IconFormat {
    Svg,
    Xml,
}

impl IconFormat {
    pub fn extension(&self) -> String {
        match &self {
            IconFormat::Svg => "svg".to_string(),
            IconFormat::Xml => "xml".to_string(),
        }
    }
}

impl AppConfig {
    pub fn from_file(yaml_config_path: &String) -> Result<Self, AppError> {
        let file = match File::open(yaml_config_path) {
            Ok(file) => file,
            Err(e) => return Err(AppError::AppConfigOpen(format!("{}", e))),
        };
        match serde_yaml::from_reader(&file) {
            Ok(app_config) => Ok(app_config),
            Err(e) => Err(AppError::AppConfigParse(e)),
        }
    }

    /// Returns the required mainRes path from config.
    pub fn main_res_images(&self) -> Option<String> {
        let common_main_res = self.android.main_res.clone();
        let images_main_res = self.android.images.main_res.clone();

        images_main_res.or(common_main_res)
    }

    /// Returns the required mainRes path from config.
    pub fn main_res_icons(&self) -> Option<String> {
        let common_main_res = self.android.main_res.clone();
        let icons_main_res = self.android.icons.main_res.clone();

        icons_main_res.or(common_main_res)
    }
}
