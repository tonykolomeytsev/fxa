use std::collections::HashMap;
use std::time::Duration;

use reqwest::blocking::Client;

use crate::api::figma::FigmaApi;
use crate::common::fetcher::fetch;
use crate::common::fileutils::{create_dir, move_file};
use crate::common::renderer::Renderer;
use crate::feature_icons::view::View;
use crate::models::config::{AppConfig, ImageFormat};

pub fn export_icons(token: &String, image_names: &Vec<String>, path_to_config: &String) {
    let renderer = Renderer();
    let api = FigmaApi::new(create_http_client(&token));
    let result = fetch(&api, &path_to_config, &renderer)
        .and_then(|e| Ok((e.app_config, e.images_name_to_id)))
        .and_then(|(app_config, images_table)| {
            for image_name in image_names {
                renderer.render(View::FetchingIcon(image_name.clone()));
                export_icon(&api, &app_config, &image_name, &images_table, &renderer);
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

fn export_icon(
    api: &FigmaApi,
    app_config: &AppConfig,
    image_name: &String,
    images_table: &HashMap<String, String>,
    renderer: &Renderer,
) {
    let file_id = &app_config.figma.file_id;
    let frame_name = &app_config.common.images.figma_frame_name;
    match images_table.get(image_name) {
        Some(node_id) => {
            let result = api
                .get_image_download_url(file_id, node_id, 1.0f32)
                .and_then(|image_url| {
                    renderer.render(View::DownloadingIcon(image_name.clone()));
                    api.get_image(&image_url, &image_name, &String::new(), &ImageFormat::Svg)
                })
                .and_then(|image_temp_path| {
                    renderer.render(View::IconDownloaded(image_name.clone()));
                    let res_path = &app_config.android.main_res;
                    let full_final_image_dir = format!("{}/drawable", &res_path);
                    create_dir(&full_final_image_dir)
                        .map(|()| (image_temp_path, full_final_image_dir))
                })
                .and_then(|(image_temp_path, full_final_image_dir)| {
                    let full_final_image_path =
                        format!("{}/{}.svg", full_final_image_dir, &image_name);
                    move_file(&image_temp_path, &full_final_image_path)
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
