use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use reqwest::blocking::Client;

use crate::api::figma::{FigmaApi, FigmaApiError, FIGMA_FILES_ENDPOINT};
use crate::common::fileutils::{create_dir, move_file, FileUtilsError};
use crate::common::renderer::Renderer;
use crate::feature_icons::renderer::{FeatureIconsRenderer, View};
use crate::models::config::{AppConfig, ImageFormat};
use crate::models::figma::{Document, Frame};

#[derive(Debug)]
pub struct FeatureIconsError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for FeatureIconsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nCaused by: {}", &self.message, &self.cause)
    }
}

pub fn export_icons(token: &String, image_names: &Vec<String>, path_to_config: &String) {
    let mut renderer = FeatureIconsRenderer();
    renderer.new_line();
    // Read app config
    renderer.render(View::ReadingConfig {
        path: path_to_config.clone(),
    });
    let api = FigmaApi::new(create_http_client(&token));
    let mut url = String::new();
    let result = AppConfig::from_file(path_to_config)
        .map_err(|e| FeatureIconsError {
            message: e.message,
            cause: e.cause,
        })
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
            renderer.render(View::FoundIcons(
                app_config.common.images.figma_frame_name.clone(),
            ));
            renderer.new_line();
            for image_name in image_names {
                renderer.render(View::FetchingIcon(image_name.clone()));
                export_icon(&api, &app_config, &image_name, &images_table, &mut renderer);
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
) -> Result<(Document, bool), FeatureIconsError> {
    let file_id = &app_config.figma.file_id;
    api.get_document(&file_id).map_err(|e| FeatureIconsError {
        message: e.message,
        cause: e.cause,
    })
}

fn find_images_frame<'a>(
    document: Document,
    app_config: AppConfig,
) -> Result<(AppConfig, HashMap<String, String>), FeatureIconsError> {
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
        Err(FeatureIconsError { message, cause })
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

fn export_icon(
    api: &FigmaApi,
    app_config: &AppConfig,
    image_name: &String,
    images_table: &HashMap<String, String>,
    renderer: &mut FeatureIconsRenderer,
) {
    let file_id = &app_config.figma.file_id;
    let frame_name = &app_config.common.images.figma_frame_name;
    match images_table.get(image_name) {
        Some(node_id) => {
            let result = api
                .get_image_download_url(file_id, node_id, 1.0f32)
                .map_err(&map_figma_api_error)
                .and_then(|image_url| {
                    renderer.render(View::DownloadingIcon(image_name.clone()));
                    api.get_image(&image_url, &image_name, &String::new(), &ImageFormat::Svg)
                        .map_err(&map_figma_api_error)
                })
                .and_then(|image_temp_path| {
                    renderer.render(View::IconDownloaded(image_name.clone()));
                    let res_path = &app_config.android.main_res;
                    let full_final_image_dir = format!("{}/drawable", &res_path);
                    create_dir(&full_final_image_dir)
                        .map_err(&map_fileutils_error)
                        .map(|()| (image_temp_path, full_final_image_dir))
                })
                .and_then(|(image_temp_path, full_final_image_dir)| {
                    let full_final_image_path =
                        format!("{}/{}.svg", full_final_image_dir, &image_name);
                    move_file(&image_temp_path, &full_final_image_path)
                        .map_err(&map_fileutils_error)
                })
                .and_then(|()| {
                    renderer.render(View::IconExported(image_name.clone()));
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
                    "occurred because an icon `{}` is missing in frame `{}`",
                    &image_name, &frame_name
                ),
            });
            renderer.new_line();
        }
    }
}

fn map_figma_api_error(e: FigmaApiError) -> FeatureIconsError {
    FeatureIconsError {
        message: e.message,
        cause: e.cause,
    }
}

fn map_fileutils_error(e: FileUtilsError) -> FeatureIconsError {
    FeatureIconsError {
        message: e.message,
        cause: e.cause,
    }
}
