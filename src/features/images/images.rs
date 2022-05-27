use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use reqwest::blocking::Client;

use crate::api::figma::{FigmaApi, FigmaApiError, FIGMA_FILES_ENDPOINT};
use crate::common::fileutils::{create_dir, move_file, FileUtilsError};
use crate::common::renderer::Renderer;
use crate::common::webp;
use crate::feature_images::renderer::{FeatureImagesRenderer, View};
use crate::models::config::{AppConfig, ImageFormat, LoadAppConfigError};
use crate::models::figma::{Document, Frame};

#[derive(Debug)]
pub struct FeatureImagesError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for FeatureImagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nCaused by: {}", &self.message, &self.cause)
    }
}

trait IntoFeatureImagesError {
    fn map(self) -> FeatureImagesError;
}

impl ImageFormat {
    fn extension(&self) -> String {
        match &self {
            ImageFormat::Png => "png".to_string(),
            ImageFormat::Svg => "svg".to_string(),
            ImageFormat::Webp => "webp".to_string(),
        }
    }
}

pub fn export_images(token: &String, image_names: &Vec<String>, path_to_config: &String) {
    let mut renderer = FeatureImagesRenderer();
    renderer.new_line();
    // Read app config
    renderer.render(View::ReadingConfig {
        path: path_to_config.clone(),
    });
    let api = FigmaApi::new(create_http_client(&token));
    let mut url = String::new();
    let result = AppConfig::from_file(path_to_config)
        .map_err(|e| e.map())
        .map(|app_config| {
            renderer.render(View::ReceivedConfig {
                path: path_to_config.clone(),
            });
            url = format!("{}{}", FIGMA_FILES_ENDPOINT, &app_config.figma.file_id);
            app_config
        })
        .and_then(|app_config| {
            renderer.new_line();
            renderer.render(View::FetchingDom { url: url.clone() });
            fetch_dom(&api, &app_config).map(|(doc, from_cache)| (app_config, doc, from_cache))
        })
        .and_then(|(app_config, doc, from_cache)| {
            renderer.render(View::DomFetched {
                url: url.clone(),
                from_cache,
            });
            renderer.new_line();
            renderer.render(View::ProcessingDom);
            find_images_frame(doc, app_config)
        })
        .and_then(|(app_config, images_table)| {
            renderer.render(View::FoundImages(
                app_config.common.images.figma_frame_name.clone(),
            ));
            renderer.new_line();
            let image_scales = &app_config.android.images.scales;
            for image_name in image_names {
                image_scales.iter().for_each(|(scale_name, scale_value)| {
                    renderer.render(View::FetchingImage(image_name.clone(), scale_name.clone()));
                    export_image(
                        &api,
                        &app_config,
                        &image_name,
                        &scale_name,
                        *scale_value,
                        &images_table,
                        &mut renderer,
                    );
                });
            }
            Ok(images_table)
        });
    match result {
        Ok(_) => {
            renderer.render(View::Done { message: None });
        }
        Err(e) => {
            renderer.render(View::Error {
                description: format!("{}", e),
            });
        }
    }
}

fn create_http_client(token: &String) -> Client {
    let mut auth_headers = reqwest::header::HeaderMap::new();
    auth_headers.insert("X-FIGMA-TOKEN", token.parse().unwrap());
    reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(30, 0)))
        .default_headers(auth_headers)
        .build()
        .unwrap()
}

fn fetch_dom(
    api: &FigmaApi,
    app_config: &AppConfig,
) -> Result<(Document, bool), FeatureImagesError> {
    let file_id = &app_config.figma.file_id;
    api.get_document(&file_id).map_err(|e| e.map())
}

fn find_images_frame(
    document: Document,
    app_config: AppConfig,
) -> Result<(AppConfig, HashMap<String, String>), FeatureImagesError> {
    let frame = document
        .children
        .iter()
        .filter(|&canvas| {
            if let Some(desired_page_name) = &app_config.figma.page_name {
                desired_page_name == &canvas.name
            } else {
                true
            }
        })
        .flat_map(|canvas| &canvas.children)
        .find(|frame| &frame.name == &app_config.common.images.figma_frame_name);
    if let Some(frame) = frame {
        Ok((app_config, map_images_name_to_id(&frame)))
    } else {
        let message = format!(
            "during search frame with name `{}`",
            &app_config.common.images.figma_frame_name
        );
        let cause = "Make sure such a frame exists".to_string();
        Err(FeatureImagesError { message, cause })
    }
}

fn map_images_name_to_id(frame: &Frame) -> HashMap<String, String> {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    match &frame.children {
        Some(children) => {
            children.iter().for_each(|frame| {
                hash_map.insert(frame.name.clone(), frame.id.clone());
            });
        }
        None => (),
    };
    hash_map
}

fn export_image(
    api: &FigmaApi,
    app_config: &AppConfig,
    image_name: &String,
    image_scale_name: &String,
    image_scale_value: f32,
    images_table: &HashMap<String, String>,
    renderer: &mut FeatureImagesRenderer,
) {
    let file_id = &app_config.figma.file_id;
    let frame_name = &app_config.common.images.figma_frame_name;
    let image_format = &app_config.android.images.format;
    let quality = app_config.android.images.webp_options.quality;
    match images_table.get(image_name) {
        Some(node_id) => {
            let result = api
                .get_image_download_url(file_id, node_id, image_scale_value)
                .map_err(|e| e.map())
                .and_then(|image_url| {
                    renderer.render(View::DownloadingImage(
                        image_name.clone(),
                        image_scale_name.clone(),
                    ));
                    let image_format = &app_config.android.images.format;
                    api.get_image(&image_url, &image_name, &image_scale_name, &image_format)
                        .map_err(|e| e.map())
                })
                .and_then(|image_file_name| {
                    renderer.render(View::ImageDownloaded(
                        image_name.clone(),
                        image_scale_name.clone(),
                    ));
                    convert_image_to_webp_if_necessary(
                        &image_name,
                        &image_scale_name,
                        image_file_name,
                        image_format,
                        quality,
                        renderer,
                    )
                })
                .and_then(|image_temp_path| {
                    let res_path = &app_config.android.main_res;
                    let full_final_image_dir =
                        format!("{}/drawable-{}", &res_path, &image_scale_name);
                    create_dir(&full_final_image_dir)
                        .map_err(|e| e.map())
                        .map(|()| (image_temp_path, full_final_image_dir))
                })
                .and_then(|(image_temp_path, full_final_image_dir)| {
                    let extension = image_format.extension();
                    let full_final_image_path =
                        format!("{}/{}.{}", full_final_image_dir, &image_name, &extension);
                    move_file(&image_temp_path, &full_final_image_path).map_err(|e| e.map())
                })
                .and_then(|()| {
                    renderer.render(View::ImageExported(
                        image_name.clone(),
                        image_scale_name.clone(),
                    ));
                    renderer.new_line();
                    Ok(())
                });
            if let Err(e) = result {
                renderer.render(View::Error {
                    description: format!("{}", e),
                });
                renderer.new_line();
            }
        }
        None => {
            renderer.render(View::Error {
                description: format!(
                    "occurred because an image `{}` is missing in frame `{}`",
                    &image_name, &frame_name
                ),
            });
            renderer.new_line();
        }
    }
}

impl IntoFeatureImagesError for LoadAppConfigError {
    fn map(self) -> FeatureImagesError {
        FeatureImagesError {
            message: self.message,
            cause: self.cause,
        }
    }
}

impl IntoFeatureImagesError for FigmaApiError {
    fn map(self) -> FeatureImagesError {
        FeatureImagesError {
            message: self.message,
            cause: self.cause,
        }
    }
}

impl IntoFeatureImagesError for FileUtilsError {
    fn map(self) -> FeatureImagesError {
        FeatureImagesError {
            message: self.message,
            cause: self.cause,
        }
    }
}

fn convert_image_to_webp_if_necessary(
    image_name: &String,
    image_scale_name: &String,
    image_file_name: String,
    image_format: &ImageFormat,
    quality: f32,
    renderer: &mut FeatureImagesRenderer,
) -> Result<String, FeatureImagesError> {
    match image_format {
        ImageFormat::Webp => {
            renderer.render(View::ConvertingToWebp(
                image_name.clone(),
                image_scale_name.clone(),
            ));
            match webp::image_to_webp(&image_file_name, quality) {
                Some(new_image_path) => {
                    renderer.render(View::ConvertedToWebp(
                        image_name.clone(),
                        image_scale_name.clone(),
                    ));
                    Ok(new_image_path)
                }
                None => Err(FeatureImagesError {
                    message: "while converting PNG to WEBP".to_string(),
                    cause: "something went wrong in webp module".to_string(),
                }),
            }
        }
        _ => Ok(image_file_name),
    }
}
