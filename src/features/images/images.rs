use std::collections::HashMap;

use crate::api::figma::FigmaApi;
use crate::common::error::CommonError;
use crate::common::fetcher::fetch;
use crate::common::fileutils::{create_dir, move_file};
use crate::common::http_client::create_http_client;
use crate::common::renderer::Renderer;
use crate::common::res_name::to_res_name;
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

#[derive(Debug, Clone)]
struct ImageInfo {
    name: String,
    scale_name: String,
    scale_value: f32,
    format: ImageFormat,
    res_name: String,
}

pub fn export_images(token: &String, image_names: &Vec<String>, path_to_config: &String) {
    let renderer = Renderer();
    let api = FigmaApi::new(create_http_client(&token));

    let result = fetch(&api, &path_to_config, &renderer)
        .and_then(|e| Ok((e.app_config, e.images_name_to_id)))
        .and_then(|(app_config, images_table)| {
            let image_scales = &app_config.android.images.scales;
            for image_name in image_names {
                for (scale_name, scale_value) in image_scales {
                    renderer.render(View::FetchingImage(image_name.clone(), scale_name.clone()));
                    let image_info = ImageInfo {
                        name: image_name.clone(),
                        scale_name: scale_name.clone(),
                        scale_value: *scale_value,
                        format: app_config.android.images.format.clone(),
                        res_name: to_res_name(&image_name),
                    };
                    export_image(&api, &app_config, &image_info, &images_table, &renderer);
                }
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

fn export_image(
    api: &FigmaApi,
    app_config: &AppConfig,
    image_info: &ImageInfo,
    images_table: &HashMap<String, String>,
    renderer: &Renderer,
) {
    let file_id = &app_config.figma.file_id;
    let frame_name = &app_config.common.images.figma_frame_name;
    let quality = app_config.android.images.webp_options.quality;

    let result = images_table
        .get(&image_info.name)
        .ok_or_else(|| CommonError {
            message: format!(
                "occurred because an image `{}` is missing in frame `{}`",
                &image_info.name, &frame_name
            ),
            cause: None,
        })
        .and_then(|node_id| {
            api.get_image_download_url(file_id, node_id, image_info.scale_value, &image_info.format)
        })
        .and_then(|image_url| download_image(&app_config, &api, &image_url, &image_info, &renderer))
        .and_then(|image_file_name| {
            renderer.render(View::ImageDownloaded(
                image_info.name.clone(),
                image_info.scale_name.clone(),
            ));
            convert_to_webp_if_necessary(&image_info, image_file_name, quality, &renderer)
        })
        .and_then(|image_temp_path| {
            let res_path = &app_config.android.main_res;
            let full_final_image_dir = format!("{}/drawable-{}", &res_path, &image_info.scale_name);
            create_dir(&full_final_image_dir).map(|()| (image_temp_path, full_final_image_dir))
        })
        .and_then(|(image_temp_path, full_final_image_dir)| {
            let extension = image_info.format.extension();
            let full_final_image_path = format!(
                "{}/{}.{}",
                full_final_image_dir, &image_info.res_name, &extension
            );
            move_file(&image_temp_path, &full_final_image_path)
        })
        .and_then(|()| {
            renderer.render(View::ImageExported(
                image_info.name.clone(),
                image_info.scale_name.clone(),
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

fn convert_to_webp_if_necessary(
    image_info: &ImageInfo,
    image_file_name: String,
    quality: f32,
    renderer: &Renderer,
) -> Result<String, CommonError> {
    match image_info.format {
        ImageFormat::Webp => {
            renderer.render(View::ConvertingToWebp(
                image_info.name.clone(),
                image_info.scale_name.clone(),
            ));
            match webp::image_to_webp(&image_file_name, quality) {
                Some(new_image_path) => {
                    renderer.render(View::ConvertedToWebp(
                        image_info.name.clone(),
                        image_info.scale_name.clone(),
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

fn download_image(
    app_config: &AppConfig,
    api: &FigmaApi,
    image_url: &String,
    image_info: &ImageInfo,
    renderer: &Renderer,
) -> Result<String, CommonError> {
    renderer.render(View::DownloadingImage(
        image_info.name.clone(),
        image_info.scale_name.clone(),
    ));
    let image_format = &app_config.android.images.format;
    api.get_image(
        &image_url,
        &image_info.res_name,
        &image_info.scale_name,
        &image_format,
    )
}
