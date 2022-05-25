use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;

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
///         scales: [1, 1.5, 2, 3]
///         format: png | webp
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
///         scales: [1, 1.5, 2, 3]
///         format: png | webp
///         webpOptions:
///             quality: 0..100
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidConfig {
    pub main_res: String,
    pub images: AndroidImagesConfig,
}

/// Part of App config from YAML:
/// ```yaml
/// images:
///     scales: [1, 1.5, 2, 3]
///     format: png | webp
///     webpOptions:
///         quality: 0..100
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidImagesConfig {
    pub scales: HashMap<String, f32>,
    pub format: ImageFormat,
    pub webp_options: AndroidImagesWebpConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImageFormat {
    Jpeg,
    Webp,
    Png,
    Svg,
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

#[derive(Debug, Deserialize)]
pub struct LoadAppConfigError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for LoadAppConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}; {}", &self.message, &self.cause)
    }
}

impl AppConfig {
    pub fn from_file(yaml_config_path: &String) -> Result<Self, LoadAppConfigError> {
        match File::open(yaml_config_path) {
            Ok(file) => match serde_yaml::from_reader(&file) {
                Ok(app_config) => Ok(app_config),
                Err(e) => {
                    let message = format!("while parsing config file {}", yaml_config_path);
                    let cause = format!("{}", e);
                    Err(LoadAppConfigError { message, cause })
                }
            },
            Err(e) => {
                let message = format!("while opening config file {}", yaml_config_path);
                let cause = format!("{}", e);
                Err(LoadAppConfigError { message, cause })
            }
        }
    }
}
