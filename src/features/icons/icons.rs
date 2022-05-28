use std::collections::HashMap;

use crate::api::figma::FigmaApi;
use crate::common::fetcher::fetch;
use crate::common::fileutils::{create_dir, move_file};
use crate::common::http_client::create_http_client;
use crate::common::renderer::Renderer;
use crate::common::res_name::to_res_name;
use crate::feature_icons::view::View;
use crate::models::config::{AppConfig, ImageFormat};

#[derive(Debug, Clone)]
struct IconInfo {
    name: String,
    res_name: String,
}

pub fn export_icons(token: &String, image_names: &Vec<String>, path_to_config: &String) {
    let renderer = Renderer();
    let api = FigmaApi::new(create_http_client(&token));
    let result = fetch(&api, &path_to_config, &renderer)
        .and_then(|e| Ok((e.app_config, e.images_name_to_id)))
        .and_then(|(app_config, images_table)| {
            for image_name in image_names {
                renderer.render(View::FetchingIcon(image_name.clone()));
                let icon_info = IconInfo {
                    name: image_name.clone(),
                    res_name: to_res_name(&image_name),
                };
                export_icon(&api, &app_config, &icon_info, &images_table, &renderer);
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

fn export_icon(
    api: &FigmaApi,
    app_config: &AppConfig,
    icon_info: &IconInfo,
    images_table: &HashMap<String, String>,
    renderer: &Renderer,
) {
    let file_id = &app_config.figma.file_id;
    let frame_name = &app_config.common.images.figma_frame_name;
    match images_table.get(&icon_info.name) {
        Some(node_id) => {
            let result = api
                .get_image_download_url(file_id, node_id, 1.0f32, &ImageFormat::Svg)
                .and_then(|image_url| {
                    renderer.render(View::DownloadingIcon(icon_info.name.clone()));
                    api.get_image(
                        &image_url,
                        &icon_info.res_name,
                        &String::new(),
                        &ImageFormat::Svg,
                    )
                })
                .and_then(|image_temp_path| {
                    renderer.render(View::IconDownloaded(icon_info.name.clone()));
                    let res_path = &app_config.android.main_res;
                    let full_final_image_dir = format!("{}/drawable", &res_path);
                    create_dir(&full_final_image_dir)
                        .map(|()| (image_temp_path, full_final_image_dir))
                })
                .and_then(|(image_temp_path, full_final_image_dir)| {
                    let full_final_image_path =
                        format!("{}/{}.svg", full_final_image_dir, &icon_info.res_name);
                    move_file(&image_temp_path, &full_final_image_path)
                })
                .and_then(|()| {
                    renderer.render(View::IconExported(icon_info.name.clone()));
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
                    &icon_info.name, &frame_name
                ),
            });
            renderer.new_line();
        }
    }
}
