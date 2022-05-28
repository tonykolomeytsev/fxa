use std::collections::HashMap;
use std::time::Duration;

use reqwest::blocking::Client;

use crate::api::figma::FigmaApi;
use crate::common::error::CommonError;
use crate::common::fetcher::fetch;
use crate::common::fileutils::{create_dir, move_file};
use crate::common::renderer::Renderer;
use crate::common::webp;
use crate::feature_images::view::View;
use crate::models::config::{AppConfig, ImageFormat};

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
    let renderer = Renderer();
    let api = FigmaApi::new(create_http_client(&token));
    let result = fetch(&api, &path_to_config, &renderer)
        .and_then(|e| Ok((e.app_config, e.images_name_to_id)))
        .and_then(|(app_config, images_table)| {
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
                        &renderer,
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

fn export_image(
    api: &FigmaApi,
    app_config: &AppConfig,
    image_name: &String,
    image_scale_name: &String,
    image_scale_value: f32,
    images_table: &HashMap<String, String>,
    renderer: &Renderer,
) {
    let file_id = &app_config.figma.file_id;
    let frame_name = &app_config.common.images.figma_frame_name;
    let image_format = &app_config.android.images.format;
    let quality = app_config.android.images.webp_options.quality;
    match images_table.get(image_name) {
        Some(node_id) => {
            let result = api
                .get_image_download_url(file_id, node_id, image_scale_value)
                .and_then(|image_url| {
                    renderer.render(View::DownloadingImage(
                        image_name.clone(),
                        image_scale_name.clone(),
                    ));
                    let image_format = &app_config.android.images.format;
                    api.get_image(&image_url, &image_name, &image_scale_name, &image_format)
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
                        &renderer,
                    )
                })
                .and_then(|image_temp_path| {
                    let res_path = &app_config.android.main_res;
                    let full_final_image_dir =
                        format!("{}/drawable-{}", &res_path, &image_scale_name);
                    create_dir(&full_final_image_dir)
                        .map(|()| (image_temp_path, full_final_image_dir))
                })
                .and_then(|(image_temp_path, full_final_image_dir)| {
                    let extension = image_format.extension();
                    let full_final_image_path =
                        format!("{}/{}.{}", full_final_image_dir, &image_name, &extension);
                    move_file(&image_temp_path, &full_final_image_path)
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

fn convert_image_to_webp_if_necessary(
    image_name: &String,
    image_scale_name: &String,
    image_file_name: String,
    image_format: &ImageFormat,
    quality: f32,
    renderer: &Renderer,
) -> Result<String, CommonError> {
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
                None => Err(CommonError {
                    message: "while converting PNG to WEBP".to_string(),
                    cause: Some("something went wrong in webp module".to_string()),
                }),
            }
        }
        _ => Ok(image_file_name),
    }
}
